//! Saved reports are untrusted: rerun current inputs before reporting freshness.
use super::{
    check::{self, Request},
    model::{self, Bounds, Selection},
    profiles, select,
};
use crate::support::{
    print_json, read_string_option, reject_positional_arguments, reject_unknown_options,
    CapabilityRoot, CapabilityRootKind,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    error::Error,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedProfile {
    name: String,
    blake3: String,
    settings: profiles::Profile,
    source: Option<String>,
}

// r[impl cairn.prose_workflow.binding]
pub(super) fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    reject_unknown_options(args, &["--root", "--report"])?;
    reject_positional_arguments(args)?;
    let root_path = read_string_option(args, "--root").unwrap_or_else(|| ".".into());
    let report_path =
        read_string_option(args, "--report").ok_or("prose verify requires --report")?;
    let root = CapabilityRoot::open(CapabilityRootKind::Repository, Path::new(&root_path))?;
    let bytes = root.read_bounded(Path::new(&report_path), model::MAX_REPORT_BYTES as u64)?;
    let saved: Value = serde_json::from_slice(&bytes)?;
    if saved["schema_version"] != "cairn.prose-check.v2"
        || saved["advisory"] != true
        || saved["mutated"] != false
    {
        return Err("prose verify requires an advisory content-bound v2 report".into());
    }
    if saved["engine"] != json!({"name":"englishc", "revision":model::ENGINE_REVISION}) {
        return Err("saved prose report uses a different engine cohort".into());
    }
    let profile: SavedProfile = serde_json::from_value(saved["profile"].clone())?;
    profiles::validate(&profile.settings)?;
    if profile.name != profile.settings.name
        || profile.blake3 != profiles::identity(&profile.settings)
    {
        return Err("saved prose profile identity is invalid".into());
    }
    let current_profile = profiles::load(
        &root,
        profile.source.as_deref().map(Path::new),
        &profile.name,
    )?;
    if profile.blake3 != profiles::identity(&current_profile) {
        return Err("saved prose profile is stale".into());
    }
    let bounds: Bounds = serde_json::from_value(saved["bounds"].clone())?;
    bounds.validate()?;
    if bytes.len() > bounds.max_report_bytes {
        return Err("saved prose report exceeds its declared output limit".into());
    }
    let selection: Selection = serde_json::from_value(saved["selection"].clone())?;
    let rows = saved["files"]
        .as_array()
        .ok_or("saved prose report requires files")?;
    if rows.is_empty() || rows.len() > select::MAX_FILES {
        return Err("saved prose report has an invalid file count".into());
    }
    let paths = match &selection {
        Selection::Change { change } => select::change_files(&root, change)?,
        Selection::Explicit => rows
            .iter()
            .map(|row| {
                row["path"]
                    .as_str()
                    .map(PathBuf::from)
                    .ok_or_else(|| "saved prose report has an invalid path".into())
            })
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?,
    };
    let request = Request {
        selection,
        profile: current_profile,
        profile_source: profile.source,
        bounds,
    };
    let current = check::report(&root, paths, &request, true)?;
    // This rejects unknown fields, altered suggestions, false claims, stale file
    // sets and hashes. A recomputed attacker-controlled hash is not authority.
    if saved != current {
        return Err(
            "saved prose report is stale, altered, or does not match the closed current report"
                .into(),
        );
    }
    model::encoded(&current, request.bounds.max_report_bytes)?;
    print_json(json!({
        "schema_version":"cairn.prose-freshness.v1", "advisory":true, "mutated":false, "current":true,
        "report_blake3":current["report_blake3"],
        "non_claims":["current content and recomputed diagnostics only; not authenticity, approval, safe application, or lifecycle evidence", "all suggestions still require author review"],
    }))
}
