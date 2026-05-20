use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use serde_json::{json, Value};

use crate::agent_receipt::{read_json_artifact, AGENT_RECEIPT_MAX_BYTES};
use crate::evidence::{evidence_index_verify_receipt, EvidenceIndexVerifyOptions};
use crate::{
    default_policy_path, load_policy_file, path_string, portable_json_hash, print_json,
    read_option, read_string_option, with_portable_receipt_hash,
};
use cairn_core::canonical_json_hash;

#[derive(Default)]
struct EvidenceRegistryOptions {
    root: PathBuf,
    indexes: Vec<PathBuf>,
    out: Option<PathBuf>,
    labels: EvidenceRegistryLabels,
}

#[derive(Default)]
pub(crate) struct EvidenceRegistryVerifyOptions {
    pub(crate) root: PathBuf,
    pub(crate) registry: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceRegistryMirrorVerifyOptions {
    root: PathBuf,
    registry: Option<PathBuf>,
    mirror: Option<PathBuf>,
}

#[derive(Default, Clone)]
struct EvidenceRegistryLabels {
    channel: Option<String>,
    release: Option<String>,
    platform: Option<String>,
    compatibility_class: Option<String>,
}

#[derive(Default, Clone)]
struct EvidenceRegistryConsumeSelectors {
    bundle_kind: Option<String>,
    policy_hash: Option<String>,
    channel: Option<String>,
    release: Option<String>,
    platform: Option<String>,
    compatibility_class: Option<String>,
    index_receipt_hash: Option<String>,
    index_verification_receipt_hash: Option<String>,
    bundle_receipt_hash: Option<String>,
    artifact_role: Option<String>,
    artifact_hash: Option<String>,
    coverage_audit_receipt_hash: Option<String>,
    release_readiness_receipt_hash: Option<String>,
}

#[derive(Default)]
struct EvidenceRegistrySelectOptions {
    root: PathBuf,
    registry: Option<PathBuf>,
    out: Option<PathBuf>,
    selectors: EvidenceRegistryConsumeSelectors,
}

#[derive(Default)]
struct EvidenceRegistryFetchPlanOptions {
    root: PathBuf,
    registry: Option<PathBuf>,
    out: Option<PathBuf>,
    selectors: EvidenceRegistryConsumeSelectors,
}

pub(crate) fn run_evidence_registry(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("evidence registry requires build or verify".into());
    };
    let receipt = match subcommand.as_str() {
        "build" => {
            let options = evidence_registry_options(&iter.collect::<Vec<_>>())?;
            evidence_registry_receipt(&options)
        }
        "verify" => {
            let options = evidence_registry_verify_options(&iter.collect::<Vec<_>>())?;
            evidence_registry_verify_receipt(&options)
        }
        "select" => {
            let options = evidence_registry_select_options(&iter.collect::<Vec<_>>())?;
            evidence_registry_select_receipt(&options)
        }
        "fetch-plan" => {
            let options = evidence_registry_fetch_plan_options(&iter.collect::<Vec<_>>())?;
            evidence_registry_fetch_plan_receipt(&options)
        }
        "mirror-verify" => {
            let options = evidence_registry_mirror_verify_options(&iter.collect::<Vec<_>>())?;
            evidence_registry_mirror_verify_receipt(&options)
        }
        unknown => return Err(format!("unknown evidence registry subcommand: {unknown}").into()),
    };
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn evidence_registry_options(args: &[String]) -> Result<EvidenceRegistryOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let indexes = read_repeated_path_options(args, "--index");
    let out = read_string_option(args, "--out").map(PathBuf::from);
    let labels = evidence_registry_labels_from_args(args);
    if indexes.is_empty() {
        return Err("evidence registry build requires at least one --index <file>".into());
    }
    if out.is_none() {
        return Err("evidence registry build requires --out <file>".into());
    }
    Ok(EvidenceRegistryOptions {
        root,
        indexes,
        out,
        labels,
    })
}

fn evidence_registry_verify_options(
    args: &[String],
) -> Result<EvidenceRegistryVerifyOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let registry = read_string_option(args, "--registry").map(PathBuf::from);
    if registry.is_none() {
        return Err("evidence registry verify requires --registry <file>".into());
    }
    Ok(EvidenceRegistryVerifyOptions { root, registry })
}

fn evidence_registry_mirror_verify_options(
    args: &[String],
) -> Result<EvidenceRegistryMirrorVerifyOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let registry = read_string_option(args, "--registry").map(PathBuf::from);
    let mirror = read_string_option(args, "--mirror").map(PathBuf::from);
    if registry.is_none() {
        return Err("evidence registry mirror-verify requires --registry <file>".into());
    }
    if mirror.is_none() {
        return Err("evidence registry mirror-verify requires --mirror <dir>".into());
    }
    Ok(EvidenceRegistryMirrorVerifyOptions {
        root,
        registry,
        mirror,
    })
}

fn evidence_registry_select_options(
    args: &[String],
) -> Result<EvidenceRegistrySelectOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let registry = read_string_option(args, "--registry").map(PathBuf::from);
    let out = read_string_option(args, "--out").map(PathBuf::from);
    if registry.is_none() {
        return Err("evidence registry select requires --registry <file>".into());
    }
    if out.is_none() {
        return Err("evidence registry select requires --out <file>".into());
    }
    Ok(EvidenceRegistrySelectOptions {
        root,
        registry,
        out,
        selectors: evidence_registry_consume_selectors(args),
    })
}

fn evidence_registry_fetch_plan_options(
    args: &[String],
) -> Result<EvidenceRegistryFetchPlanOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let registry = read_string_option(args, "--registry").map(PathBuf::from);
    let out = read_string_option(args, "--out").map(PathBuf::from);
    if registry.is_none() {
        return Err("evidence registry fetch-plan requires --registry <file>".into());
    }
    if out.is_none() {
        return Err("evidence registry fetch-plan requires --out <file>".into());
    }
    Ok(EvidenceRegistryFetchPlanOptions {
        root,
        registry,
        out,
        selectors: evidence_registry_consume_selectors(args),
    })
}

fn evidence_registry_consume_selectors(args: &[String]) -> EvidenceRegistryConsumeSelectors {
    EvidenceRegistryConsumeSelectors {
        bundle_kind: read_string_option(args, "--bundle-kind"),
        policy_hash: read_string_option(args, "--policy-hash"),
        channel: read_string_option(args, "--channel"),
        release: read_string_option(args, "--release"),
        platform: read_string_option(args, "--platform"),
        compatibility_class: read_string_option(args, "--compatibility-class"),
        index_receipt_hash: read_string_option(args, "--index-receipt-hash"),
        index_verification_receipt_hash: read_string_option(
            args,
            "--index-verification-receipt-hash",
        ),
        bundle_receipt_hash: read_string_option(args, "--bundle-receipt-hash"),
        artifact_role: read_string_option(args, "--artifact-role"),
        artifact_hash: read_string_option(args, "--artifact-hash"),
        coverage_audit_receipt_hash: read_string_option(args, "--coverage-audit-receipt-hash"),
        release_readiness_receipt_hash: read_string_option(
            args,
            "--release-readiness-receipt-hash",
        ),
    }
}

fn evidence_registry_labels_from_args(args: &[String]) -> EvidenceRegistryLabels {
    EvidenceRegistryLabels {
        channel: read_string_option(args, "--channel"),
        release: read_string_option(args, "--release"),
        platform: read_string_option(args, "--platform"),
        compatibility_class: read_string_option(args, "--compatibility-class"),
    }
}

fn evidence_registry_labels_json(labels: &EvidenceRegistryLabels) -> Value {
    json!({
        "channel": labels.channel.clone().unwrap_or_else(|| "unspecified".to_string()),
        "release": labels.release.clone().unwrap_or_else(|| "unspecified".to_string()),
        "platform": labels.platform.clone().unwrap_or_else(|| "unspecified".to_string()),
        "compatibility_class": labels
            .compatibility_class
            .clone()
            .unwrap_or_else(|| "unspecified".to_string()),
    })
}

fn evidence_registry_validate_labels(
    root: &Path,
    labels: &EvidenceRegistryLabels,
    prefix: &'static str,
    issues: &mut Vec<Value>,
) {
    let Ok(policy) = load_policy_file(default_policy_path().into()) else {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.label_policy_unreadable"),
            "registry_label_policy",
            "readable generated policy",
            "unreadable",
        ));
        return;
    };
    let policy_hash = canonical_json_hash(&policy.raw);
    if policy_hash.is_empty() || root.as_os_str().is_empty() {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.label_policy_hash_invalid"),
            "registry_label_policy",
            "BLAKE3 policy hash",
            "invalid",
        ));
    }
    for (field, value, policy_key) in [
        ("channel", labels.channel.as_deref(), "channels"),
        ("platform", labels.platform.as_deref(), "platforms"),
        (
            "compatibility_class",
            labels.compatibility_class.as_deref(),
            "compatibility_classes",
        ),
    ] {
        if let Some(value) = value {
            if evidence_registry_label_value_unsafe(value) {
                issues.push(evidence_registry_issue(
                    &format!("{prefix}.label_unsafe"),
                    field,
                    "stable non-path label value",
                    "unsafe",
                ));
                continue;
            }
            let allowed = registry_label_policy_values(&policy.raw, policy_key);
            if !allowed.iter().any(|allowed| allowed == value) {
                issues.push(evidence_registry_issue(
                    &format!("{prefix}.label_not_allowed"),
                    field,
                    "policy-owned allowed label",
                    value,
                ));
            }
        }
    }
    if let Some(release) = labels.release.as_deref() {
        if evidence_registry_label_value_unsafe(release) {
            issues.push(evidence_registry_issue(
                &format!("{prefix}.label_unsafe"),
                "release",
                "stable non-path label value",
                "unsafe",
            ));
        } else if !evidence_registry_release_label_valid(release) {
            issues.push(evidence_registry_issue(
                &format!("{prefix}.release_label_invalid"),
                "release",
                "YYYY-MM-DD release label",
                release,
            ));
        }
    }
    let label_count = [
        labels.channel.as_ref(),
        labels.release.as_ref(),
        labels.platform.as_ref(),
        labels.compatibility_class.as_ref(),
    ]
    .iter()
    .flatten()
    .count();
    if label_count > 0 && labels.channel.is_none() {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.channel_required_for_labeled_registry"),
            "channel",
            "channel label when registry labels are present",
            "missing",
        ));
    }
    // r[impl cairn.supersede.registry_channel_metadata.policy]
}

fn registry_label_policy_values(policy: &Value, key: &str) -> Vec<String> {
    policy
        .get("registry_label_policy")
        .and_then(|policy| policy.get(key))
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn evidence_registry_label_value_unsafe(value: &str) -> bool {
    value.is_empty()
        || value.starts_with('/')
        || value.contains("..")
        || value.contains('\\')
        || value.contains(':')
        || value.chars().any(char::is_whitespace)
}

fn evidence_registry_release_label_valid(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(idx, byte)| idx == 4 || idx == 7 || byte.is_ascii_digit())
}

fn entry_labels_from_registry_entry(entry: &Value) -> Option<EvidenceRegistryLabels> {
    let labels = entry.get("labels")?;
    Some(EvidenceRegistryLabels {
        channel: registry_entry_label(labels, "channel"),
        release: registry_entry_label(labels, "release"),
        platform: registry_entry_label(labels, "platform"),
        compatibility_class: registry_entry_label(labels, "compatibility_class"),
    })
}

fn registry_entry_label(labels: &Value, key: &str) -> Option<String> {
    labels
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| *value != "unspecified")
        .map(str::to_string)
}

fn evidence_registry_label_key(entry: &Value) -> Option<String> {
    let labels = entry.get("labels")?;
    let channel = labels
        .get("channel")
        .and_then(Value::as_str)
        .unwrap_or("unspecified");
    if channel == "unspecified" {
        return None;
    }
    let release = labels
        .get("release")
        .and_then(Value::as_str)
        .unwrap_or("unspecified");
    let platform = labels
        .get("platform")
        .and_then(Value::as_str)
        .unwrap_or("unspecified");
    let compatibility = labels
        .get("compatibility_class")
        .and_then(Value::as_str)
        .unwrap_or("unspecified");
    Some(format!("{channel}|{release}|{platform}|{compatibility}"))
}

fn evidence_registry_reject_label_conflicts(
    entries: &[Value],
    prefix: &'static str,
    issues: &mut Vec<Value>,
) {
    let mut seen = std::collections::BTreeMap::<String, String>::new();
    for entry in entries {
        let Some(key) = evidence_registry_label_key(entry) else {
            continue;
        };
        let hash = entry
            .get("index_receipt_hash")
            .and_then(Value::as_str)
            .unwrap_or("<missing>")
            .to_string();
        if let Some(previous) = seen.insert(key, hash.clone()) {
            if previous != hash {
                issues.push(evidence_registry_issue(
                    &format!("{prefix}.label_conflict"),
                    "labels",
                    "unique label tuple per index hash",
                    "conflict",
                ));
            }
        }
    }
    // r[impl cairn.supersede.registry_channel_metadata.build]
}

fn read_repeated_path_options(args: &[String], flag: &str) -> Vec<PathBuf> {
    let mut values = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == flag {
            if let Some(value) = iter.next() {
                values.push(PathBuf::from(value));
            }
        }
    }
    values
}

fn evidence_registry_receipt(options: &EvidenceRegistryOptions) -> Value {
    // r[impl cairn.supersede.evidence_registry_rail.build]
    // r[impl cairn.supersede.evidence_registry_rail.failures]
    let root = options.root.clone();
    let out = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let mut entries = Vec::new();
    let mut hashes = std::collections::BTreeSet::new();
    evidence_registry_validate_labels(&root, &options.labels, "evidence_registry", &mut issues);
    for (position, index_path) in options.indexes.iter().enumerate() {
        if let Some(entry) = evidence_registry_entry_from_index(
            &root,
            index_path,
            out.parent().unwrap_or_else(|| Path::new(".")),
            position,
            "evidence_registry",
            Some(&options.labels),
            &mut issues,
        ) {
            if let Some(hash) = entry.get("index_receipt_hash").and_then(Value::as_str) {
                if !hashes.insert(hash.to_string()) {
                    issues.push(evidence_registry_issue(
                        "evidence_registry.duplicate_index_hash",
                        "indexes",
                        "unique index receipt hashes",
                        "duplicate",
                    ));
                }
            }
            entries.push(entry);
        }
    }
    entries.sort_by_key(canonical_json_hash);
    evidence_registry_reject_label_conflicts(&entries, "evidence_registry", &mut issues);
    let mut receipt = evidence_registry_final_receipt(
        &root,
        "evidence registry build",
        issues,
        entries,
        "<out>",
        Value::Null,
    );
    if receipt.get("valid").and_then(Value::as_bool) == Some(true) {
        if let Some(parent) = out.parent() {
            if fs::create_dir_all(parent).is_err() {
                receipt = evidence_registry_final_receipt(
                    &root,
                    "evidence registry build",
                    vec![evidence_registry_issue(
                        "evidence_registry.output_create_failed",
                        "out",
                        "writable output parent",
                        "failed",
                    )],
                    Vec::new(),
                    "<out>",
                    Value::Null,
                );
            }
        }
        if receipt.get("valid").and_then(Value::as_bool) == Some(true)
            && fs::write(
                &out,
                serde_json::to_vec_pretty(&receipt).unwrap_or_default(),
            )
            .is_err()
        {
            receipt = evidence_registry_final_receipt(
                &root,
                "evidence registry build",
                vec![evidence_registry_issue(
                    "evidence_registry.receipt_write_failed",
                    "out",
                    "writable registry receipt",
                    "failed",
                )],
                Vec::new(),
                "<out>",
                Value::Null,
            );
        }
    }
    receipt
}

pub(crate) fn evidence_registry_verify_receipt(options: &EvidenceRegistryVerifyOptions) -> Value {
    // r[impl cairn.supersede.evidence_registry_rail.verify]
    // r[impl cairn.supersede.evidence_registry_rail.failures]
    let root = options.root.clone();
    let registry_path = options
        .registry
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let registry = match read_json_artifact(&registry_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            return evidence_registry_verify_final_receipt(
                &root,
                vec![evidence_registry_issue(
                    "evidence_registry_verify.registry_unreadable",
                    "registry",
                    "readable registry",
                    "unreadable",
                )],
                Value::Null,
                Vec::new(),
            )
        }
    };
    let mut issues = Vec::new();
    let stated_hash = registry.get("receipt_hash").cloned().unwrap_or(Value::Null);
    if let Some(hash) = stated_hash.as_str() {
        let mut hash_input = registry.clone();
        if let Some(object) = hash_input.as_object_mut() {
            object.remove("receipt_hash");
        }
        if portable_json_hash(&root, &hash_input) != hash {
            issues.push(evidence_registry_issue(
                "evidence_registry_verify.registry_hash_mismatch",
                "receipt_hash",
                "portable receipt hash",
                "mismatch",
            ));
        }
    } else {
        issues.push(evidence_registry_issue(
            "evidence_registry_verify.registry_hash_missing",
            "receipt_hash",
            "BLAKE3 hex",
            "missing",
        ));
    }
    let mut entries = Vec::new();
    let base = registry_path.parent().unwrap_or_else(|| Path::new("."));
    for (position, entry) in registry
        .get("entries")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .iter()
        .enumerate()
    {
        let Some(index_path) = evidence_registry_resolve_index_path(base, entry) else {
            issues.push(evidence_registry_issue(
                "evidence_registry_verify.index_path_unsafe",
                "entries.index_relative",
                "safe relative index path",
                "unsafe",
            ));
            continue;
        };
        let entry_labels = entry_labels_from_registry_entry(entry);
        if let Some(labels) = entry_labels.as_ref() {
            evidence_registry_validate_labels(
                &root,
                labels,
                "evidence_registry_verify",
                &mut issues,
            );
        }
        if let Some(current) = evidence_registry_entry_from_index(
            &root,
            &index_path,
            base,
            position,
            "evidence_registry_verify",
            entry_labels.as_ref(),
            &mut issues,
        ) {
            if !evidence_registry_entries_equal(entry, &current) {
                issues.push(evidence_registry_issue(
                    "evidence_registry_verify.entry_mismatch",
                    "entries",
                    "current index entry",
                    "mismatch",
                ));
            }
            entries.push(current);
        }
    }
    entries.sort_by_key(canonical_json_hash);
    evidence_registry_reject_label_conflicts(&entries, "evidence_registry_verify", &mut issues);
    evidence_registry_verify_final_receipt(&root, issues, stated_hash, entries)
}

fn evidence_registry_mirror_verify_receipt(options: &EvidenceRegistryMirrorVerifyOptions) -> Value {
    // r[impl cairn.supersede.registry_mirror_verification.command]
    // r[impl cairn.supersede.registry_mirror_verification.receipt]
    // r[impl cairn.supersede.registry_mirror_verification.failures]
    let root = options.root.clone();
    let registry_path = options
        .registry
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let mirror = options.mirror.clone().unwrap_or_else(|| PathBuf::from("."));
    let registry = match read_json_artifact(&registry_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            return evidence_registry_mirror_verify_final_receipt(
                &root,
                vec![evidence_registry_issue(
                    "evidence_registry_mirror_verify.registry_unreadable",
                    "registry",
                    "readable registry",
                    "unreadable",
                )],
                Value::Null,
                Value::Null,
                Vec::new(),
            )
        }
    };
    let mut issues = Vec::new();
    let registry_hash = registry.get("receipt_hash").cloned().unwrap_or(Value::Null);
    if let Some(hash) = registry_hash.as_str() {
        let mut hash_input = registry.clone();
        if let Some(object) = hash_input.as_object_mut() {
            object.remove("receipt_hash");
        }
        if portable_json_hash(&root, &hash_input) != hash {
            issues.push(evidence_registry_issue(
                "evidence_registry_mirror_verify.registry_hash_mismatch",
                "receipt_hash",
                "portable receipt hash",
                "mismatch",
            ));
        }
    } else {
        issues.push(evidence_registry_issue(
            "evidence_registry_mirror_verify.registry_hash_missing",
            "receipt_hash",
            "BLAKE3 hex",
            "missing",
        ));
    }
    let registry_verification = evidence_registry_verify_receipt(&EvidenceRegistryVerifyOptions {
        root: root.clone(),
        registry: Some(registry_path.clone()),
    });
    if registry_verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_registry_issue(
            "evidence_registry_mirror_verify.registry_verify_failed",
            "registry",
            "valid registry verification",
            "invalid",
        ));
    }
    let mut entries = Vec::new();
    for (position, entry) in registry
        .get("entries")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .iter()
        .enumerate()
    {
        let Some(index_path) = evidence_registry_resolve_index_path(&mirror, entry) else {
            issues.push(evidence_registry_issue(
                "evidence_registry_mirror_verify.index_path_unsafe",
                "entries.index_relative",
                "safe relative index path",
                "unsafe",
            ));
            continue;
        };
        if !index_path.exists() {
            issues.push(evidence_registry_issue(
                "evidence_registry_mirror_verify.index_missing",
                "entries.index_relative",
                "mirrored index file",
                "missing",
            ));
            continue;
        }
        let entry_labels = entry_labels_from_registry_entry(entry);
        if let Some(labels) = entry_labels.as_ref() {
            evidence_registry_validate_labels(
                &root,
                labels,
                "evidence_registry_mirror_verify",
                &mut issues,
            );
        }
        if let Some(current) = evidence_registry_entry_from_index(
            &root,
            &index_path,
            &mirror,
            position,
            "evidence_registry_mirror_verify",
            entry_labels.as_ref(),
            &mut issues,
        ) {
            if entry.get("index_receipt_hash") != current.get("index_receipt_hash") {
                issues.push(evidence_registry_issue(
                    "evidence_registry_mirror_verify.index_hash_mismatch",
                    "entries.index_receipt_hash",
                    "declared registry index hash",
                    "mismatch",
                ));
            } else if !evidence_registry_entries_equal(entry, &current) {
                issues.push(evidence_registry_issue(
                    "evidence_registry_mirror_verify.entry_mismatch",
                    "entries",
                    "current mirrored index entry",
                    "mismatch",
                ));
            }
            entries.push(current);
        }
    }
    entries.sort_by_key(canonical_json_hash);
    evidence_registry_reject_label_conflicts(
        &entries,
        "evidence_registry_mirror_verify",
        &mut issues,
    );
    evidence_registry_mirror_verify_final_receipt(
        &root,
        issues,
        registry_hash,
        registry_verification
            .get("receipt_hash")
            .cloned()
            .unwrap_or(Value::Null),
        entries,
    )
}

fn evidence_registry_mirror_verify_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    registry_hash: Value,
    registry_verification_receipt_hash: Value,
    entries: Vec<Value>,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence registry mirror-verify",
            "root": path_string(root),
            "registry": "<registry>",
            "mirror": "<mirror>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "registry_hash": registry_hash,
            "registry_verification_receipt_hash": registry_verification_receipt_hash,
            "entry_count": entries.len(),
            "entries": entries,
        }),
    )
}

fn evidence_registry_entry_from_index(
    root: &Path,
    index_path: &Path,
    base: &Path,
    position: usize,
    prefix: &'static str,
    labels: Option<&EvidenceRegistryLabels>,
    issues: &mut Vec<Value>,
) -> Option<Value> {
    let index = match read_json_artifact(index_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            issues.push(evidence_registry_issue(
                &format!("{prefix}.index_unreadable"),
                "index",
                "readable evidence index",
                "unreadable",
            ));
            return None;
        }
    };
    if index.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.index_invalid"),
            "index",
            "valid evidence index",
            "invalid",
        ));
    }
    let verify_options = EvidenceIndexVerifyOptions {
        root: root.to_path_buf(),
        index: Some(index_path.to_path_buf()),
    };
    let verify = evidence_index_verify_receipt(&verify_options);
    if verify.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.index_verify_failed"),
            "index",
            "valid index verification",
            "invalid",
        ));
    }
    let index_relative = evidence_registry_index_relative(base, index_path);
    let labels = labels
        .map(evidence_registry_labels_json)
        .unwrap_or_else(|| evidence_registry_labels_json(&EvidenceRegistryLabels::default()));
    Some(json!({
        "index": format!("<index:{position}>"),
        "index_relative": index_relative,
        "index_receipt_hash": index.get("receipt_hash").cloned().unwrap_or(Value::Null),
        "index_verification_receipt_hash": verify.get("receipt_hash").cloned().unwrap_or(Value::Null),
        "labels": labels,
        "bundle_kind": index.get("bundle_kind").cloned().unwrap_or(Value::Null),
        "bundle_receipt_hash": index.get("bundle_receipt_hash").cloned().unwrap_or(Value::Null),
        "receipt_chain": index.get("receipt_chain").cloned().unwrap_or_else(|| json!({})),
        "policy_hash": index.get("policy_hash").cloned().unwrap_or(Value::Null),
        "artifacts": index.get("artifacts").cloned().unwrap_or_else(|| json!([])),
    }))
}

fn evidence_registry_entries_equal(expected: &Value, current: &Value) -> bool {
    for key in [
        "index_receipt_hash",
        "labels",
        "bundle_kind",
        "bundle_receipt_hash",
        "receipt_chain",
        "policy_hash",
        "artifacts",
    ] {
        if expected.get(key) != current.get(key) {
            return false;
        }
    }
    true
}

fn evidence_registry_index_relative(base: &Path, index_path: &Path) -> Value {
    if let Ok(relative) = index_path.strip_prefix(base) {
        if let Some(text) = relative.to_str() {
            if !text.is_empty() && !text.starts_with('/') && !text.contains("..") {
                return json!(text);
            }
        }
    }
    if let (Some(parent), Some(name)) = (index_path.parent(), index_path.file_name()) {
        if parent == base {
            return json!(name.to_string_lossy().to_string());
        }
    }
    Value::Null
}

fn evidence_registry_resolve_index_path(base: &Path, entry: &Value) -> Option<PathBuf> {
    let relative = entry.get("index_relative").and_then(Value::as_str)?;
    if relative.is_empty() || relative.starts_with('/') || relative.contains("..") {
        return None;
    }
    Some(base.join(relative))
}

fn evidence_registry_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_registry_final_receipt(
    root: &Path,
    command: &str,
    mut issues: Vec<Value>,
    entries: Vec<Value>,
    out: &str,
    registry_hash: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": command,
            "root": path_string(root),
            "out": out,
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "entry_count": entries.len(),
            "entries": entries,
            "registry_hash": registry_hash,
        }),
    )
}

fn evidence_registry_verify_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    registry_hash: Value,
    entries: Vec<Value>,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence registry verify",
            "root": path_string(root),
            "registry": "<registry>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "registry_hash": registry_hash,
            "entry_count": entries.len(),
            "entries": entries,
        }),
    )
}

fn evidence_registry_select_receipt(options: &EvidenceRegistrySelectOptions) -> Value {
    // r[impl cairn.supersede.evidence_registry_consume.select]
    let root = options.root.clone();
    let registry_path = options
        .registry
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let out = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let (registry, verification, mut issues) =
        evidence_registry_consume_registry(&root, &registry_path, "evidence_registry_select");
    let entries = registry
        .get("entries")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let matched_entries =
        evidence_registry_matching_entries(&entries, &options.selectors, &mut issues);
    let mut selected_entry = Value::Null;
    if matched_entries.is_empty() {
        issues.push(evidence_registry_issue(
            "evidence_registry_select.no_match",
            "selectors",
            "exactly one matching registry entry",
            "none",
        ));
    } else if matched_entries.len() > 1 {
        issues.push(evidence_registry_issue(
            "evidence_registry_select.ambiguous_match",
            "selectors",
            "exactly one matching registry entry",
            "multiple",
        ));
    } else {
        selected_entry = matched_entries[0].clone();
    }

    if issues.is_empty() {
        let base = registry_path.parent().unwrap_or_else(|| Path::new("."));
        let Some(index_path) = evidence_registry_resolve_index_path(base, &selected_entry) else {
            return evidence_registry_select_final_receipt(
                &root,
                vec![evidence_registry_issue(
                    "evidence_registry_select.index_path_unsafe",
                    "selected.index_relative",
                    "safe relative index path",
                    "unsafe",
                )],
                &options.selectors,
                verification,
                Value::Null,
            );
        };
        match fs::read(&index_path) {
            Ok(bytes) => {
                if let Some(parent) = out.parent() {
                    if fs::create_dir_all(parent).is_err() {
                        issues.push(evidence_registry_issue(
                            "evidence_registry_select.output_create_failed",
                            "out",
                            "writable output parent",
                            "failed",
                        ));
                    }
                }
                if issues.is_empty() && fs::write(&out, bytes).is_err() {
                    issues.push(evidence_registry_issue(
                        "evidence_registry_select.output_write_failed",
                        "out",
                        "writable selected index artifact",
                        "failed",
                    ));
                }
            }
            Err(_) => issues.push(evidence_registry_issue(
                "evidence_registry_select.index_unreadable",
                "selected.index_relative",
                "readable selected index",
                "unreadable",
            )),
        }
    }

    evidence_registry_select_final_receipt(
        &root,
        issues,
        &options.selectors,
        verification,
        selected_entry,
    )
}

fn evidence_registry_fetch_plan_receipt(options: &EvidenceRegistryFetchPlanOptions) -> Value {
    // r[impl cairn.supersede.evidence_registry_consume.fetch_plan]
    let root = options.root.clone();
    let registry_path = options
        .registry
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let out = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let (registry, verification, mut issues) =
        evidence_registry_consume_registry(&root, &registry_path, "evidence_registry_fetch_plan");
    let entries = registry
        .get("entries")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut plan_entries =
        evidence_registry_matching_entries(&entries, &options.selectors, &mut issues);
    plan_entries.sort_by_key(canonical_json_hash);
    let mut receipt = evidence_registry_fetch_plan_final_receipt(
        &root,
        issues,
        &options.selectors,
        verification,
        plan_entries,
    );
    if receipt.get("valid").and_then(Value::as_bool) == Some(true) {
        if let Some(parent) = out.parent() {
            if fs::create_dir_all(parent).is_err() {
                receipt = evidence_registry_fetch_plan_final_receipt(
                    &root,
                    vec![evidence_registry_issue(
                        "evidence_registry_fetch_plan.output_create_failed",
                        "out",
                        "writable output parent",
                        "failed",
                    )],
                    &options.selectors,
                    Value::Null,
                    Vec::new(),
                );
            }
        }
        if receipt.get("valid").and_then(Value::as_bool) == Some(true)
            && fs::write(
                &out,
                serde_json::to_vec_pretty(&receipt).unwrap_or_default(),
            )
            .is_err()
        {
            receipt = evidence_registry_fetch_plan_final_receipt(
                &root,
                vec![evidence_registry_issue(
                    "evidence_registry_fetch_plan.output_write_failed",
                    "out",
                    "writable fetch plan receipt",
                    "failed",
                )],
                &options.selectors,
                Value::Null,
                Vec::new(),
            );
        }
    }
    receipt
}

fn evidence_registry_consume_registry(
    root: &Path,
    registry_path: &Path,
    prefix: &'static str,
) -> (Value, Value, Vec<Value>) {
    let mut issues = Vec::new();
    let registry = match read_json_artifact(registry_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            issues.push(evidence_registry_issue(
                &format!("{prefix}.registry_unreadable"),
                "registry",
                "readable registry",
                "unreadable",
            ));
            return (Value::Null, Value::Null, issues);
        }
    };
    let verification = evidence_registry_verify_receipt(&EvidenceRegistryVerifyOptions {
        root: root.to_path_buf(),
        registry: Some(registry_path.to_path_buf()),
    });
    if verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.registry_verify_failed"),
            "registry",
            "valid registry verification",
            "invalid",
        ));
    }
    (registry, verification, issues)
}

fn evidence_registry_matching_entries(
    entries: &[Value],
    selectors: &EvidenceRegistryConsumeSelectors,
    issues: &mut Vec<Value>,
) -> Vec<Value> {
    if let Some(issue) = evidence_registry_selector_issue(selectors) {
        issues.push(issue);
        return Vec::new();
    }
    entries
        .iter()
        .filter(|entry| evidence_registry_entry_matches(entry, selectors))
        .cloned()
        .collect()
}

fn evidence_registry_entry_matches(
    entry: &Value,
    selectors: &EvidenceRegistryConsumeSelectors,
) -> bool {
    if !evidence_registry_match_string(entry, "bundle_kind", selectors.bundle_kind.as_deref()) {
        return false;
    }
    if !evidence_registry_match_string(entry, "policy_hash", selectors.policy_hash.as_deref()) {
        return false;
    }
    if !evidence_registry_match_label(entry, "channel", selectors.channel.as_deref()) {
        return false;
    }
    if !evidence_registry_match_label(entry, "release", selectors.release.as_deref()) {
        return false;
    }
    if !evidence_registry_match_label(entry, "platform", selectors.platform.as_deref()) {
        return false;
    }
    if !evidence_registry_match_label(
        entry,
        "compatibility_class",
        selectors.compatibility_class.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_string(
        entry,
        "index_receipt_hash",
        selectors.index_receipt_hash.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_string(
        entry,
        "index_verification_receipt_hash",
        selectors.index_verification_receipt_hash.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_string(
        entry,
        "bundle_receipt_hash",
        selectors.bundle_receipt_hash.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_receipt_chain(
        entry,
        "coverage_audit_receipt_hash",
        selectors.coverage_audit_receipt_hash.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_receipt_chain(
        entry,
        "release_readiness_receipt_hash",
        selectors.release_readiness_receipt_hash.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_artifact(entry, selectors) {
        return false;
    }
    true
}

fn evidence_registry_match_string(entry: &Value, key: &str, selector: Option<&str>) -> bool {
    match selector {
        Some(expected) => entry.get(key).and_then(Value::as_str) == Some(expected),
        None => true,
    }
}

fn evidence_registry_match_label(entry: &Value, key: &str, selector: Option<&str>) -> bool {
    match selector {
        Some(expected) => {
            entry
                .get("labels")
                .and_then(|labels| labels.get(key))
                .and_then(Value::as_str)
                == Some(expected)
        }
        None => true,
    }
}

fn evidence_registry_match_receipt_chain(entry: &Value, key: &str, selector: Option<&str>) -> bool {
    match selector {
        Some(expected) => {
            entry
                .get("receipt_chain")
                .and_then(|chain| chain.get(key))
                .and_then(Value::as_str)
                == Some(expected)
        }
        None => true,
    }
}

fn evidence_registry_match_artifact(
    entry: &Value,
    selectors: &EvidenceRegistryConsumeSelectors,
) -> bool {
    if selectors.artifact_role.is_none() && selectors.artifact_hash.is_none() {
        return true;
    }
    let Some(artifacts) = entry.get("artifacts").and_then(Value::as_array) else {
        return false;
    };
    artifacts.iter().any(|artifact| {
        let role_ok = selectors
            .artifact_role
            .as_deref()
            .is_none_or(|role| artifact.get("role").and_then(Value::as_str) == Some(role));
        let hash_ok = selectors
            .artifact_hash
            .as_deref()
            .is_none_or(|hash| artifact.get("hash").and_then(Value::as_str) == Some(hash));
        role_ok && hash_ok
    })
}

fn evidence_registry_selector_issue(selectors: &EvidenceRegistryConsumeSelectors) -> Option<Value> {
    let selector_values = [
        selectors.bundle_kind.as_deref(),
        selectors.policy_hash.as_deref(),
        selectors.channel.as_deref(),
        selectors.release.as_deref(),
        selectors.platform.as_deref(),
        selectors.compatibility_class.as_deref(),
        selectors.index_receipt_hash.as_deref(),
        selectors.index_verification_receipt_hash.as_deref(),
        selectors.bundle_receipt_hash.as_deref(),
        selectors.artifact_role.as_deref(),
        selectors.artifact_hash.as_deref(),
        selectors.coverage_audit_receipt_hash.as_deref(),
        selectors.release_readiness_receipt_hash.as_deref(),
    ];
    if selector_values.iter().flatten().any(|value| {
        value.is_empty() || value.starts_with('/') || value.contains("..") || value.contains('\\')
    }) {
        return Some(evidence_registry_issue(
            "evidence_registry_consume.selector_unsafe",
            "selectors",
            "stable non-path selector values",
            "unsafe",
        ));
    }
    None
}

fn evidence_registry_selectors_json(selectors: &EvidenceRegistryConsumeSelectors) -> Value {
    let mut object = serde_json::Map::new();
    if let Some(value) = selectors.bundle_kind.as_ref() {
        object.insert("bundle_kind".to_string(), json!(value));
    }
    if let Some(value) = selectors.policy_hash.as_ref() {
        object.insert("policy_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.channel.as_ref() {
        object.insert("channel".to_string(), json!(value));
    }
    if let Some(value) = selectors.release.as_ref() {
        object.insert("release".to_string(), json!(value));
    }
    if let Some(value) = selectors.platform.as_ref() {
        object.insert("platform".to_string(), json!(value));
    }
    if let Some(value) = selectors.compatibility_class.as_ref() {
        object.insert("compatibility_class".to_string(), json!(value));
    }
    if let Some(value) = selectors.index_receipt_hash.as_ref() {
        object.insert("index_receipt_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.index_verification_receipt_hash.as_ref() {
        object.insert("index_verification_receipt_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.bundle_receipt_hash.as_ref() {
        object.insert("bundle_receipt_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.artifact_role.as_ref() {
        object.insert("artifact_role".to_string(), json!(value));
    }
    if let Some(value) = selectors.artifact_hash.as_ref() {
        object.insert("artifact_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.coverage_audit_receipt_hash.as_ref() {
        object.insert("coverage_audit_receipt_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.release_readiness_receipt_hash.as_ref() {
        object.insert("release_readiness_receipt_hash".to_string(), json!(value));
    }
    Value::Object(object)
}

fn evidence_registry_select_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    selectors: &EvidenceRegistryConsumeSelectors,
    verification: Value,
    selected_entry: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence registry select",
            "root": path_string(root),
            "registry": "<registry>",
            "out": "<out>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "selectors": evidence_registry_selectors_json(selectors),
            "registry_receipt_hash": verification.get("registry_hash").cloned().unwrap_or(Value::Null),
            "registry_verification_receipt_hash": verification.get("receipt_hash").cloned().unwrap_or(Value::Null),
            "selected_entry": selected_entry,
        }),
    )
}

fn evidence_registry_fetch_plan_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    selectors: &EvidenceRegistryConsumeSelectors,
    verification: Value,
    entries: Vec<Value>,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence registry fetch-plan",
            "root": path_string(root),
            "registry": "<registry>",
            "out": "<out>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "selectors": evidence_registry_selectors_json(selectors),
            "registry_receipt_hash": verification.get("registry_hash").cloned().unwrap_or(Value::Null),
            "registry_verification_receipt_hash": verification.get("receipt_hash").cloned().unwrap_or(Value::Null),
            "entry_count": entries.len(),
            "entries": entries,
        }),
    )
}
