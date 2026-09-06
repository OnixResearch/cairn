//! Checked project policy loading. This module never publishes or evaluates Nickel.
use crate::{
    bootstrap_fs::{open_root, read_external, read_optional, read_required},
    support::default_policy_path,
};
use cairn_core::{
    validate_policy_json,
    verified::consumer_bootstrap::{
        BootstrapCompleteSelection, BootstrapIdentity, BootstrapProjectDeclaration,
        BOOTSTRAP_COMPLETE_LOCATOR as COMPLETE, BOOTSTRAP_SELECTION_LOCATOR as DECLARATION,
        MAX_ARTIFACT_BYTES,
    },
    Policy,
};
use serde_json::{json, Value};
use std::{error::Error, path::Path};

const MAX_DECLARATION_BYTES: u64 = 65_536;

pub(crate) struct SelectedPolicy {
    pub(crate) policy: Policy,
    pub(crate) selection: Value,
}

fn parse(bytes: &[u8], locator: &str, source: &str) -> Result<SelectedPolicy, Box<dyn Error>> {
    let raw: Value = serde_json::from_slice(bytes)
        .map_err(|error| format!("failed to parse policy {locator}: {error}"))?;
    let policy = validate_policy_json(raw.clone()).map_err(|error| {
        let diagnostic = cairn_core::policy_schema_compatibility_diagnostic(&raw, &error)
            .unwrap_or_else(|| error.to_string());
        format!("failed to parse policy {locator}: {diagnostic}")
    })?;
    Ok(SelectedPolicy {
        policy,
        selection: json!({
            "source": source, "locator": locator,
            "blake3": BootstrapIdentity::from_bytes(bytes),
            "installation_receipt": false,
        }),
    })
}

fn explicit_policy(args: &[String]) -> Result<Option<&str>, Box<dyn Error>> {
    let mut selected = None;
    for (index, arg) in args.iter().enumerate() {
        if arg.starts_with("--policy=") {
            return Err("bootstrap.policy.use_separate_option_value".into());
        }
        if arg == "--policy" {
            let value = args
                .get(index + 1)
                .filter(|value| !value.is_empty() && !value.starts_with("--"))
                .ok_or("bootstrap.policy.missing_option_value")?;
            if selected.replace(value.as_str()).is_some() {
                return Err("bootstrap.policy.duplicate_option".into());
            }
        }
    }
    Ok(selected)
}

// r[impl cairn.consumer_bootstrap.policy]
// r[impl cairn.consumer_bootstrap.boundary]
pub(crate) fn load(root: &Path, args: &[String]) -> Result<SelectedPolicy, Box<dyn Error>> {
    if let Some(path) = explicit_policy(args)? {
        let bytes = read_external(Path::new(path), MAX_ARTIFACT_BYTES)
            .map_err(|error| format!("failed to read policy {path}: {error}"))?;
        return parse(&bytes, path, "explicit");
    }
    // Preserve the old default for roots without a declaration. A dangling
    // declaration symlink is present, not permission to fall back.
    let capability = open_root(root)?;
    let bytes = match read_optional(&capability, DECLARATION, MAX_DECLARATION_BYTES)? {
        None => {
            let path = default_policy_path();
            let bytes = read_external(Path::new(path), MAX_ARTIFACT_BYTES)
                .map_err(|error| format!("failed to read policy {path}: {error}"))?;
            return parse(&bytes, path, "legacy_default");
        }
        Some(bytes) => bytes,
    };
    let declaration: BootstrapProjectDeclaration = serde_json::from_slice(&bytes)?;
    declaration.validate()?;
    let manifest_bytes = read_required(&capability, COMPLETE, MAX_DECLARATION_BYTES)?;
    if BootstrapIdentity::from_bytes(&manifest_bytes) != declaration.complete_manifest {
        return Err("bootstrap.policy.complete_manifest_mismatch".into());
    }
    let manifest: BootstrapCompleteSelection = serde_json::from_slice(&manifest_bytes)?;
    manifest.validate_for(&declaration)?;
    let policy_bytes = read_required(&capability, &declaration.policy_locator, MAX_ARTIFACT_BYTES)?;
    if BootstrapIdentity::from_bytes(&policy_bytes) != declaration.policy {
        return Err("bootstrap.policy.identity_mismatch".into());
    }
    parse(&policy_bytes, &declaration.policy_locator, "project")
}
