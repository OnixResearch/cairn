use serde_json::{Map, Value};

use crate::verified::evidence::canonical_json_hash;

use super::common::{
    entropy_hash_material_messages, is_blake3_hex, strip_non_hash_fields, NO_CORRECTNESS_CLAIM,
};

pub const EXPORT_BUNDLE_SCHEMA_V1: &str = "octet.export-bundle.v1";

const EXPORT_BUNDLE_REQUIRED_FIELDS: &[&str] = &[
    "schema",
    "bundle_hash",
    "objects",
    "name_pointers",
    "replay",
    "compiler_ir",
    "mutation_manifest",
    "caveats",
];
const EXPORT_BUNDLE_REPLAY_REQUIRED_FIELDS: &[&str] = &["command", "args", "toolchain"];
const EXPORT_BUNDLE_ISSUE_CAPACITY: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportBundleIssue {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportBundleReport {
    pub valid: bool,
    pub verdict: String,
    pub issues: Vec<ExportBundleIssue>,
    pub computed_bundle_hash: String,
}

// Active Cairn refs: content_addressed_rust_evidence.export_bundle,
// content_addressed_rust_evidence.export_bundle.replay,
// content_addressed_rust_evidence.export_bundle.mutation_manifest,
// content_addressed_rust_evidence.compiler_ir.
// These become Tracey r[impl ...] refs only after the delta is synced into accepted specs.
pub fn validate_export_bundle_receipt(receipt: &Value) -> ExportBundleReport {
    let computed_bundle_hash = export_bundle_hash(receipt);
    let issues = export_bundle_issues(receipt, &computed_bundle_hash);
    let is_valid = issues.is_empty();
    ExportBundleReport {
        valid: is_valid,
        verdict: if is_valid { "PASS" } else { "FAIL" }.to_string(),
        issues,
        computed_bundle_hash,
    }
}

pub fn export_bundle_hash(receipt: &Value) -> String {
    canonical_json_hash(&export_bundle_hash_material(receipt))
}

pub fn export_bundle_hash_material(receipt: &Value) -> Value {
    let mut material = receipt.clone();
    strip_non_hash_fields(&mut material);
    material
}

fn export_bundle_issues(receipt: &Value, computed_bundle_hash: &str) -> Vec<ExportBundleIssue> {
    let Some(object) = receipt.as_object() else {
        return vec![bundle_issue(
            "export_bundle.not_object",
            "export bundle receipt must be a JSON object",
        )];
    };
    let mut issues = Vec::with_capacity(EXPORT_BUNDLE_ISSUE_CAPACITY);
    issues.extend(export_bundle_required_issues(object));
    issues.extend(export_bundle_hash_issues(object, computed_bundle_hash));
    let (object_hashes, object_issues) = export_bundle_object_issues(object);
    issues.extend(object_issues);
    issues.extend(export_bundle_pointer_issues(object, &object_hashes));
    issues.extend(export_bundle_replay_issues(object));
    issues.extend(export_bundle_compiler_ir_issues(object));
    issues.extend(export_bundle_manifest_issues(object));
    issues.extend(export_bundle_caveat_issues(object));
    issues.extend(export_bundle_entropy_issues(
        "$",
        &export_bundle_hash_material(receipt),
    ));
    issues
}

fn export_bundle_required_issues(object: &Map<String, Value>) -> Vec<ExportBundleIssue> {
    let mut issues = Vec::with_capacity(EXPORT_BUNDLE_REQUIRED_FIELDS.len() + 1);
    for field in EXPORT_BUNDLE_REQUIRED_FIELDS {
        if !object.contains_key(*field) {
            issues.push(bundle_issue(
                "export_bundle.missing_field",
                format!("missing required field: {field}"),
            ));
        }
    }
    if object.get("schema").and_then(Value::as_str) != Some(EXPORT_BUNDLE_SCHEMA_V1) {
        issues.push(bundle_issue(
            "export_bundle.unsupported_schema",
            "schema must be octet.export-bundle.v1",
        ));
    }
    issues
}

fn export_bundle_hash_issues(
    object: &Map<String, Value>,
    computed_bundle_hash: &str,
) -> Vec<ExportBundleIssue> {
    let stated_hash = object
        .get("bundle_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if stated_hash == computed_bundle_hash && is_blake3_hex(stated_hash) {
        return Vec::new();
    }
    vec![bundle_issue(
        "export_bundle.hash_mismatch",
        "bundle_hash must equal the BLAKE3 hash of canonical export-bundle content with non-hash fields excluded",
    )]
}

fn export_bundle_object_issues(
    object: &Map<String, Value>,
) -> (Vec<String>, Vec<ExportBundleIssue>) {
    let Some(objects) = object.get("objects").and_then(Value::as_array) else {
        return (
            Vec::new(),
            vec![bundle_issue(
                "export_bundle.invalid_objects",
                "objects must be an array",
            )],
        );
    };
    let mut object_hashes = Vec::with_capacity(objects.len());
    let mut issues = Vec::with_capacity(objects.len() + 1);
    let mut previous_hash = None::<String>;
    for entry in objects {
        let Some(object_entry) = entry.as_object() else {
            issues.push(bundle_issue(
                "export_bundle.invalid_object_entry",
                "each object membership entry must be a JSON object",
            ));
            continue;
        };
        let object_hash = object_entry
            .get("object_hash")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !is_blake3_hex(object_hash) {
            issues.push(bundle_issue(
                "export_bundle.invalid_object_hash",
                "objects.object_hash must be a BLAKE3 hex string",
            ));
        }
        if previous_hash
            .as_deref()
            .is_some_and(|previous| previous > object_hash)
        {
            issues.push(bundle_issue(
                "export_bundle.unsorted_objects",
                "objects must be sorted by object_hash",
            ));
        }
        previous_hash = Some(object_hash.to_string());
        object_hashes.push(object_hash.to_string());
    }
    (object_hashes, issues)
}

fn export_bundle_pointer_issues(
    object: &Map<String, Value>,
    object_hashes: &[String],
) -> Vec<ExportBundleIssue> {
    let Some(pointers) = object.get("name_pointers").and_then(Value::as_array) else {
        return vec![bundle_issue(
            "export_bundle.invalid_name_pointers",
            "name_pointers must be an array",
        )];
    };
    let mut issues = Vec::with_capacity(pointers.len() + 1);
    let mut previous_name = None::<String>;
    for entry in pointers {
        let Some(pointer) = entry.as_object() else {
            issues.push(bundle_issue(
                "export_bundle.invalid_pointer_entry",
                "each name pointer entry must be a JSON object",
            ));
            continue;
        };
        let name = pointer
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let object_hash = pointer
            .get("object_hash")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if name.is_empty() {
            issues.push(bundle_issue(
                "export_bundle.invalid_pointer_name",
                "name pointer entries must include a non-empty name",
            ));
        }
        if previous_name
            .as_deref()
            .is_some_and(|previous| previous > name)
        {
            issues.push(bundle_issue(
                "export_bundle.unsorted_name_pointers",
                "name_pointers must be sorted by name",
            ));
        }
        if !object_hashes
            .iter()
            .any(|known_hash| known_hash == object_hash)
        {
            issues.push(bundle_issue(
                "export_bundle.unknown_pointer_object",
                "name pointer object_hash must reference an object present in the bundle",
            ));
        }
        previous_name = Some(name.to_string());
    }
    issues
}

fn export_bundle_replay_issues(object: &Map<String, Value>) -> Vec<ExportBundleIssue> {
    let Some(replay) = object.get("replay").and_then(Value::as_object) else {
        return vec![bundle_issue(
            "export_bundle.missing_object",
            "replay must be a JSON object",
        )];
    };
    let mut issues = Vec::with_capacity(EXPORT_BUNDLE_REPLAY_REQUIRED_FIELDS.len() + 2);
    for field in EXPORT_BUNDLE_REPLAY_REQUIRED_FIELDS {
        if !replay.contains_key(*field) {
            issues.push(bundle_issue(
                "export_bundle.missing_field",
                format!("missing required field: replay.{field}"),
            ));
        }
    }
    if replay
        .get("command")
        .and_then(Value::as_str)
        .is_none_or(str::is_empty)
    {
        issues.push(bundle_issue(
            "export_bundle.invalid_replay_command",
            "replay.command must be a non-empty string",
        ));
    }
    if replay
        .get("args")
        .and_then(Value::as_array)
        .is_none_or(Vec::is_empty)
    {
        issues.push(bundle_issue(
            "export_bundle.invalid_replay_args",
            "replay.args must be a non-empty array",
        ));
    }
    issues
}

fn export_bundle_compiler_ir_issues(object: &Map<String, Value>) -> Vec<ExportBundleIssue> {
    let Some(compiler_ir) = object.get("compiler_ir").and_then(Value::as_object) else {
        return vec![bundle_issue(
            "export_bundle.missing_object",
            "compiler_ir must be a JSON object",
        )];
    };
    let mut issues = Vec::with_capacity(6);
    issues.extend(export_bundle_hir_issues(compiler_ir));
    issues.extend(export_bundle_mir_issues(compiler_ir));
    issues
}

fn export_bundle_hir_issues(compiler_ir: &Map<String, Value>) -> Vec<ExportBundleIssue> {
    let Some(hir) = compiler_ir.get("hir").and_then(Value::as_object) else {
        return vec![bundle_issue(
            "export_bundle.missing_object",
            "compiler_ir.hir must be a JSON object",
        )];
    };
    let mut issues = Vec::with_capacity(3);
    for field in ["rustc", "extractor", "dependency_status"] {
        if hir
            .get(field)
            .and_then(Value::as_str)
            .is_none_or(str::is_empty)
        {
            issues.push(bundle_issue(
                "export_bundle.missing_compiler_ir_field",
                format!("compiler_ir.hir.{field} must be a non-empty string"),
            ));
        }
    }
    if hir.get("dependency_status").and_then(Value::as_str) == Some("source-scanner-token-set") {
        issues.push(bundle_issue(
            "export_bundle.hir_dependency_overclaim_missing",
            "HIR evidence must report toolchain-scoped resolved dependency status, not source-token scanner status",
        ));
    }
    issues
}

fn export_bundle_mir_issues(compiler_ir: &Map<String, Value>) -> Vec<ExportBundleIssue> {
    let Some(mir) = compiler_ir.get("mir").and_then(Value::as_object) else {
        return vec![bundle_issue(
            "export_bundle.missing_object",
            "compiler_ir.mir must be a JSON object",
        )];
    };
    let mut issues = Vec::with_capacity(4);
    for field in ["rustc", "extractor", "summary_status"] {
        if mir
            .get(field)
            .and_then(Value::as_str)
            .is_none_or(str::is_empty)
        {
            issues.push(bundle_issue(
                "export_bundle.missing_compiler_ir_field",
                format!("compiler_ir.mir.{field} must be a non-empty string"),
            ));
        }
    }
    if mir.get("stability_claim").and_then(Value::as_str) != Some("toolchain-coupled") {
        issues.push(bundle_issue(
            "export_bundle.mir_stability_overclaim",
            "MIR summaries must declare toolchain-coupled stability",
        ));
    }
    issues
}

fn export_bundle_manifest_issues(object: &Map<String, Value>) -> Vec<ExportBundleIssue> {
    let Some(manifest) = object.get("mutation_manifest").and_then(Value::as_object) else {
        return vec![bundle_issue(
            "export_bundle.missing_object",
            "mutation_manifest must be a JSON object",
        )];
    };
    export_bundle_touched_file_issues(manifest)
}

fn export_bundle_touched_file_issues(manifest: &Map<String, Value>) -> Vec<ExportBundleIssue> {
    let Some(entries) = manifest.get("touched_files").and_then(Value::as_array) else {
        return vec![bundle_issue(
            "export_bundle.invalid_mutation_manifest",
            "mutation_manifest.touched_files must be an array",
        )];
    };
    let mut issues = Vec::with_capacity(entries.len() + 1);
    let mut previous_path = None::<&str>;
    for entry in entries {
        let Some(file) = entry.as_object() else {
            issues.push(bundle_issue(
                "export_bundle.invalid_mutation_manifest",
                "each touched file must be a JSON object",
            ));
            continue;
        };
        let path = file.get("path").and_then(Value::as_str).unwrap_or_default();
        if path.is_empty() || path.starts_with('/') || path.contains("..") {
            issues.push(bundle_issue(
                "export_bundle.invalid_mutation_manifest",
                "touched file paths must be non-empty safe relative paths",
            ));
        }
        if previous_path.is_some_and(|previous| previous > path) {
            issues.push(bundle_issue(
                "export_bundle.invalid_mutation_manifest",
                "mutation_manifest.touched_files must be sorted by path",
            ));
        }
        previous_path = Some(path);
        for hash_field in ["before_hash", "after_hash"] {
            if !file
                .get(hash_field)
                .and_then(Value::as_str)
                .is_some_and(is_blake3_hex)
            {
                issues.push(bundle_issue(
                    "export_bundle.invalid_mutation_manifest",
                    format!("touched_files.{hash_field} must be a BLAKE3 hex string"),
                ));
            }
        }
    }
    issues
}

fn export_bundle_caveat_issues(object: &Map<String, Value>) -> Vec<ExportBundleIssue> {
    let Some(caveats) = object.get("caveats").and_then(Value::as_object) else {
        return vec![bundle_issue(
            "export_bundle.missing_object",
            "caveats must be a JSON object",
        )];
    };
    let mut issues = Vec::with_capacity(3);
    if caveats.get("correctness_claim").and_then(Value::as_str) != Some(NO_CORRECTNESS_CLAIM) {
        issues.push(bundle_issue(
            "export_bundle.correctness_overclaim",
            "export-bundle hash agreement is replay/provenance evidence, not a correctness proof",
        ));
    }
    if caveats.get("compiler_ir_status").and_then(Value::as_str) == Some("stable-semantic-identity")
    {
        issues.push(bundle_issue(
            "export_bundle.compiler_ir_overclaim",
            "compiler IR evidence must remain toolchain-coupled and cannot claim stable semantic identity",
        ));
    }
    issues
}

fn export_bundle_entropy_issues(root_path: &str, root: &Value) -> Vec<ExportBundleIssue> {
    entropy_hash_material_messages(root_path, root)
        .into_iter()
        .map(|message| bundle_issue("export_bundle.ambient_entropy_in_hash_material", message))
        .collect()
}

fn bundle_issue(code: impl Into<String>, message: impl Into<String>) -> ExportBundleIssue {
    ExportBundleIssue {
        code: code.into(),
        message: message.into(),
    }
}
