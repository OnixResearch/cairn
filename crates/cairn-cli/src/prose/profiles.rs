//! Nickel-authored configuration; checking prose never runs an evaluator.
use std::{collections::BTreeSet, error::Error, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    nickel_export_boundary::export_json_dual_run,
    support::{
        has_flag, print_json, read_string_option, reject_positional_arguments,
        reject_unknown_options_with_flags, CapabilityRoot, CapabilityRootKind,
    },
};

pub(super) const MAX_PROFILE_BYTES: u64 = 65_536;
const SCHEMA: &str = "cairn.prose-profiles.v1";
const BUILTIN: &str = include_str!("../../../../cairn-policy/generated/prose.json");

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Profile {
    pub(super) name: String,
    pub(super) include_rules: Vec<String>,
    pub(super) exclude_rules: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Profiles {
    schema_version: String,
    profiles: Vec<Profile>,
}

pub(super) fn validate_selector(selector: &str) -> Result<(), String> {
    if selector == "warnings" || englishc::lints::info(selector).is_some() {
        Ok(())
    } else {
        Err(format!("unknown prose rule selector: {selector}"))
    }
}

fn admit(bytes: &[u8]) -> Result<Profiles, Box<dyn Error>> {
    if bytes.len() > MAX_PROFILE_BYTES as usize {
        return Err("prose profile collection exceeds its byte limit".into());
    }
    let profiles: Profiles = serde_json::from_slice(bytes)?;
    if profiles.schema_version != SCHEMA
        || profiles.profiles.is_empty()
        || profiles.profiles.len() > 16
    {
        return Err("unsupported or unbounded prose profile collection".into());
    }
    let mut names = BTreeSet::new();
    for profile in &profiles.profiles {
        validate(profile)?;
        if !names.insert(&profile.name) {
            return Err("duplicate prose profile name".into());
        }
    }
    Ok(profiles)
}

pub(super) fn validate(profile: &Profile) -> Result<(), Box<dyn Error>> {
    if profile.name.is_empty()
        || profile.name.len() > 64
        || !profile
            .name
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_'))
    {
        return Err("invalid prose profile name".into());
    }
    for rules in [&profile.include_rules, &profile.exclude_rules] {
        if rules.len() > 64 {
            return Err("prose profile exceeds its rule limit".into());
        }
        let mut seen = BTreeSet::new();
        for rule in rules {
            validate_selector(rule)?;
            if !seen.insert(rule) {
                return Err("duplicate prose rule selector".into());
            }
        }
    }
    Ok(())
}

pub(super) fn load(
    root: &CapabilityRoot,
    path: Option<&Path>,
    name: &str,
) -> Result<Profile, Box<dyn Error>> {
    let profiles = if let Some(path) = path {
        admit(&root.read_bounded(path, MAX_PROFILE_BYTES)?)?
    } else {
        admit(BUILTIN.as_bytes())?
    };
    profiles
        .profiles
        .into_iter()
        .find(|profile| profile.name == name)
        .ok_or_else(|| format!("unknown prose profile: {name}").into())
}

pub(super) fn identity(profile: &Profile) -> String {
    cairn_core::canonical_json_hash(&json!({"schema_version": SCHEMA, "profile": profile}))
}

pub(super) fn export(args: &[String]) -> Result<(), Box<dyn Error>> {
    reject_unknown_options_with_flags(args, &["--source", "--output"], &["--check"])?;
    reject_positional_arguments(args)?;
    let source =
        read_string_option(args, "--source").unwrap_or_else(|| "cairn-policy/prose.ncl".into());
    let output = read_string_option(args, "--output")
        .unwrap_or_else(|| "cairn-policy/generated/prose.json".into());
    let exported = export_json_dual_run(
        "cairn-prose-profiles",
        Path::new(&source),
        Path::new(&output),
        false,
    )?;
    let profiles = admit(&exported.bytes)?;
    // Serialize through Value for canonical field order, matching other generated JSON.
    let value: Value = serde_json::to_value(profiles)?;
    let bytes = (serde_json::to_string_pretty(&value)? + "\n").into_bytes();
    let (root, relative) = CapabilityRoot::open_file_parent(
        CapabilityRootKind::Repository,
        Path::new(&output),
        false,
    )?;
    if has_flag(args, "--check") {
        if root.read_bounded(&relative, MAX_PROFILE_BYTES)? != bytes {
            return Err("generated prose profiles are stale".into());
        }
    } else {
        root.write(&relative, &bytes)?;
    }
    print_json(
        json!({"schema_version":"cairn.prose-profile-export.v1", "advisory":true, "mutated":!has_flag(args,"--check"), "output":output, "profiles_blake3":blake3::hash(&bytes).to_hex().to_string()}),
    )
}
