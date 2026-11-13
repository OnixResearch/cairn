//! Imperative shell for bounded authoring checks.
use super::{
    analysis,
    model::{self, Bounds, Selection, ENGINE_REVISION, NON_CLAIMS},
    profiles::{self, Profile},
    render, select,
};
use crate::support::{reject_unknown_options, CapabilityRoot, CapabilityRootKind};
use serde_json::{json, Value};
use std::{
    error::Error,
    io::{self, Write},
    path::{Path, PathBuf},
};

pub(super) struct Request {
    pub(super) selection: Selection,
    pub(super) profile: Profile,
    pub(super) profile_source: Option<String>,
    pub(super) bounds: Bounds,
}

// r[impl cairn.prose_check.advisory] r[impl cairn.prose_check.inputs]
// r[impl cairn.prose_check.diagnostics] r[impl cairn.prose_check.read_only]
// r[impl cairn.prose_workflow.presentation] r[impl cairn.prose_workflow.bounds]
pub(super) fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    reject_unknown_options(
        args,
        &[
            "--root",
            "--change",
            "--format",
            "--report-version",
            "--profile",
            "--profiles",
            "--max-total-bytes",
            "--max-diagnostics",
            "--max-report-bytes",
        ],
    )?;
    let mut root_path = Path::new(".");
    let mut change = None;
    let mut format = "json";
    let mut version = "1";
    let mut profile_name = "default";
    let mut profile_source = None;
    let mut bounds = Bounds::default();
    let mut paths = Vec::new();
    let mut tokens = args.iter();
    while let Some(token) = tokens.next() {
        if token.starts_with('-') {
            let value = tokens.next().ok_or("prose option requires a value")?;
            match token.as_str() {
                "--root" => root_path = Path::new(value),
                "--change" => change = Some(value.as_str()),
                "--format" => format = value,
                "--report-version" => version = value,
                "--profile" => profile_name = value,
                "--profiles" => profile_source = Some(value.as_str()),
                "--max-total-bytes" => bounds.max_total_bytes = value.parse()?,
                "--max-diagnostics" => bounds.max_diagnostics = value.parse()?,
                "--max-report-bytes" => bounds.max_report_bytes = value.parse()?,
                _ => return Err("unknown prose option".into()),
            }
        } else {
            paths.push(PathBuf::from(token));
            if paths.len() > select::MAX_FILES {
                return Err("prose check accepts at most 64 files".into());
            }
        }
    }
    if !matches!(format, "json" | "text" | "github" | "sarif") || !matches!(version, "1" | "2") {
        return Err(
            "prose requires format json, text, github, or sarif and report version 1 or 2".into(),
        );
    }
    bounds.validate()?;
    let root = CapabilityRoot::open(CapabilityRootKind::Repository, root_path)?;
    let selection = if let Some(change) = change {
        if !paths.is_empty() {
            return Err("prose --change cannot be combined with explicit files".into());
        }
        paths = select::change_files(&root, change)?;
        Selection::Change {
            change: change.to_string(),
        }
    } else {
        Selection::Explicit
    };
    if paths.is_empty() {
        return Err("prose check requires explicit files or --change".into());
    }
    let profile = profiles::load(&root, profile_source.map(Path::new), profile_name)?;
    let request = Request {
        selection,
        profile,
        profile_source: profile_source.map(str::to_string),
        bounds,
    };
    let report = report(&root, paths, &request, version == "2" || format != "json")?;
    let output = if format == "json" {
        model::encoded(&report, request.bounds.max_report_bytes)?
    } else {
        let mut output = render::render(&report, format)?;
        if !output.ends_with('\n') {
            output.push('\n');
        }
        if output.len() > request.bounds.max_report_bytes {
            return Err("prose report exceeds its output limit".into());
        }
        output
    };
    // No stdout is emitted before every input, analysis and output bound passes.
    io::stdout().lock().write_all(output.as_bytes())?;
    Ok(())
}

// r[impl cairn.prose_workflow.binding]
pub(super) fn report(
    root: &CapabilityRoot,
    paths: Vec<PathBuf>,
    request: &Request,
    enhanced: bool,
) -> Result<Value, Box<dyn Error>> {
    if paths.is_empty() || paths.len() > select::MAX_FILES {
        return Err("prose selection requires 1 through 64 files".into());
    }
    request.bounds.validate()?;
    let mut remaining_bytes = request.bounds.max_total_bytes;
    let mut remaining_diagnostics = request.bounds.max_diagnostics;
    let mut files = Vec::with_capacity(paths.len());
    for path in paths {
        let relative = root.relative(&path)?;
        let path = select::path_text(&relative)?;
        let source = String::from_utf8(
            root.read_bounded(&relative, model::MAX_DOCUMENT_BYTES.min(remaining_bytes))?,
        )?;
        remaining_bytes -= source.len() as u64;
        let diagnostics = analysis::diagnostics(
            &source,
            &request.profile,
            enhanced,
            remaining_diagnostics,
            path,
        )?;
        remaining_diagnostics -= diagnostics.len();
        let mut file = json!({"path":path, "diagnostics":diagnostics});
        if enhanced {
            file["content_blake3"] = json!(blake3::hash(source.as_bytes()).to_hex().to_string());
        }
        files.push(file);
    }
    let mut report = json!({
        "schema_version": if enhanced {"cairn.prose-check.v2"} else {"cairn.prose-check.v1"},
        "advisory":true, "mutated":false,
        "engine":{"name":"englishc", "revision":ENGINE_REVISION},
        "files":files, "non_claims":NON_CLAIMS,
    });
    if enhanced {
        report["profile"] = json!({"name":request.profile.name, "blake3":profiles::identity(&request.profile), "settings":request.profile, "source":request.profile_source});
        report["bounds"] = serde_json::to_value(&request.bounds)?;
        report["selection"] = serde_json::to_value(&request.selection)?;
        model::bind_report(&mut report);
    }
    Ok(report)
}
