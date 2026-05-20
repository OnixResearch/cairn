use serde_json::{Map, Value};

use crate::verified::evidence::canonical_json_hash;

pub const FUNCTION_OBJECT_SCHEMA_V1: &str = "octet.function-object.v1";
pub const NAME_POINTER_SCHEMA_V1: &str = "octet.name-pointer.v1";

const REQUIRED_TOP_LEVEL_FIELDS: &[&str] = &[
    "schema",
    "object_hash",
    "source_token",
    "signature_hash",
    "body_hash",
    "normalization",
    "toolchain",
    "caveats",
];
const REQUIRED_NORMALIZATION_FIELDS: &[&str] = &["version", "content_kind"];
const REQUIRED_TOOLCHAIN_FIELDS: &[&str] = &["rustc", "octet_extractor"];
const REQUIRED_CAVEAT_FIELDS: &[&str] = &[
    "identity",
    "dependency_status",
    "macro_status",
    "effect_status",
    "correctness_claim",
];
const SOURCE_TOKEN_IDENTITY: &str = "source-token";
const NO_CORRECTNESS_CLAIM: &str = "none";
const SOURCE_TOKEN_CONTENT_KIND: &str = "source-token";
const SOURCE_SCANNER_DEPENDENCIES: &str = "source-scanner-token-set";
const KNOWN_MACRO_STATUSES: &[&str] = &["none", "source-visible", "macro-hidden", "generated-code"];
const KNOWN_EFFECT_STATUSES: &[&str] = &[
    "unknown",
    "reviewed-pure",
    "reviewed-impure",
    "macro-hidden",
    "unresolved",
];
const ENTROPY_KEY_FRAGMENTS: &[&str] = &[
    "timestamp",
    "created_at",
    "updated_at",
    "absolute_path",
    "temp_path",
    "tmp_path",
    "environment",
    "env",
    "hostname",
    "username",
];
const FUNCTION_OBJECT_ISSUE_CAPACITY: usize = 16;
const NAME_POINTER_REQUIRED_FIELDS: &[&str] = &[
    "schema",
    "pointer_hash",
    "name",
    "old_object_hash",
    "new_object_hash",
    "reason",
    "diff_summary",
    "rollback_handle",
    "mutation_manifest",
    "drift_check",
    "caveats",
];
const NAME_POINTER_ISSUE_CAPACITY: usize = 16;
const MUTATION_MANIFEST_REQUIRED_FIELDS: &[&str] = &["touched_files", "manifest_hash"];
const DRIFT_VALID_STATUS: &str = "valid";
const PURE_CACHE_ELIGIBLE_EFFECTS: &[&str] = &["reviewed-pure"];
const HASH_TRAVERSAL_STACK_CAPACITY: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionObjectIssue {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionObjectReport {
    pub valid: bool,
    pub verdict: String,
    pub issues: Vec<FunctionObjectIssue>,
    pub computed_object_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamePointerIssue {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamePointerReport {
    pub valid: bool,
    pub verdict: String,
    pub issues: Vec<NamePointerIssue>,
    pub computed_pointer_hash: String,
}

// Active Cairn refs: content_addressed_rust_evidence.function_object,
// content_addressed_rust_evidence.function_object.source_token,
// content_addressed_rust_evidence.function_object.no_correctness_claim,
// content_addressed_rust_evidence.canonical_hash,
// content_addressed_rust_evidence.canonical_hash.portable,
// content_addressed_rust_evidence.canonical_hash.no_entropy.
// These become Tracey r[impl ...] refs only after the delta is synced into accepted specs.
pub fn validate_function_object_receipt(receipt: &Value) -> FunctionObjectReport {
    let computed_object_hash = function_object_hash(receipt);
    let issues = function_object_issues(receipt, &computed_object_hash);
    let is_valid = issues.is_empty();
    FunctionObjectReport {
        valid: is_valid,
        verdict: if is_valid { "PASS" } else { "FAIL" }.to_string(),
        issues,
        computed_object_hash,
    }
}

pub fn function_object_hash(receipt: &Value) -> String {
    canonical_json_hash(&function_object_hash_material(receipt))
}

pub fn function_object_hash_material(receipt: &Value) -> Value {
    let mut material = receipt.clone();
    strip_non_hash_fields(&mut material);
    material
}

fn function_object_issues(receipt: &Value, computed_object_hash: &str) -> Vec<FunctionObjectIssue> {
    let Some(object) = receipt.as_object() else {
        return vec![issue(
            "function_object.not_object",
            "function object receipt must be a JSON object",
        )];
    };

    let mut issues = Vec::with_capacity(FUNCTION_OBJECT_ISSUE_CAPACITY);
    issues.extend(required_top_level_issues(object));
    issues.extend(hash_issues(object, computed_object_hash));
    issues.extend(required_nested_object_issues(object));
    issues.extend(normalization_issues(object));
    issues.extend(caveat_issues(object));
    issues.extend(entropy_issues("$", &function_object_hash_material(receipt)));
    issues
}

fn required_top_level_issues(object: &Map<String, Value>) -> Vec<FunctionObjectIssue> {
    let mut issues = Vec::with_capacity(REQUIRED_TOP_LEVEL_FIELDS.len() + 1);
    for field in REQUIRED_TOP_LEVEL_FIELDS {
        if !object.contains_key(*field) {
            issues.push(issue(
                "function_object.missing_field",
                format!("missing required field: {field}"),
            ));
        }
    }
    if object.get("schema").and_then(Value::as_str) != Some(FUNCTION_OBJECT_SCHEMA_V1) {
        issues.push(issue(
            "function_object.unsupported_schema",
            "schema must be octet.function-object.v1",
        ));
    }
    issues
}

fn hash_issues(
    object: &Map<String, Value>,
    computed_object_hash: &str,
) -> Vec<FunctionObjectIssue> {
    let mut issues = Vec::with_capacity(3);
    let stated_hash = object
        .get("object_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if stated_hash != computed_object_hash || !is_blake3_hex(stated_hash) {
        issues.push(issue(
            "function_object.hash_mismatch",
            "object_hash must equal the BLAKE3 hash of canonical function-object content with non-hash fields excluded",
        ));
    }
    for field in ["signature_hash", "body_hash"] {
        if !object
            .get(field)
            .and_then(Value::as_str)
            .is_some_and(is_blake3_hex)
        {
            issues.push(issue(
                "function_object.invalid_hash_field",
                format!("{field} must be a BLAKE3 hex string"),
            ));
        }
    }
    issues
}

fn required_nested_object_issues(object: &Map<String, Value>) -> Vec<FunctionObjectIssue> {
    let mut issues = Vec::with_capacity(
        REQUIRED_NORMALIZATION_FIELDS.len()
            + REQUIRED_TOOLCHAIN_FIELDS.len()
            + REQUIRED_CAVEAT_FIELDS.len(),
    );
    issues.extend(required_object_issues(
        object.get("normalization"),
        "normalization",
        REQUIRED_NORMALIZATION_FIELDS,
    ));
    issues.extend(required_object_issues(
        object.get("toolchain"),
        "toolchain",
        REQUIRED_TOOLCHAIN_FIELDS,
    ));
    issues.extend(required_object_issues(
        object.get("caveats"),
        "caveats",
        REQUIRED_CAVEAT_FIELDS,
    ));
    issues
}

fn normalization_issues(object: &Map<String, Value>) -> Vec<FunctionObjectIssue> {
    let has_source_token_kind = object
        .get("normalization")
        .and_then(|value| value.get("content_kind"))
        .and_then(Value::as_str)
        == Some(SOURCE_TOKEN_CONTENT_KIND);
    if has_source_token_kind {
        return Vec::new();
    }
    vec![issue(
        "function_object.normalization_not_source_token",
        "normalization.content_kind must declare source-token identity",
    )]
}

fn caveat_issues(object: &Map<String, Value>) -> Vec<FunctionObjectIssue> {
    let Some(caveats) = object.get("caveats").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut issues = Vec::with_capacity(5);
    issues.extend(caveat_identity_issues(caveats));
    issues.extend(caveat_status_issues(caveats));
    issues
}

fn caveat_identity_issues(caveats: &Map<String, Value>) -> Vec<FunctionObjectIssue> {
    let mut issues = Vec::with_capacity(2);
    if caveats.get("identity").and_then(Value::as_str) != Some(SOURCE_TOKEN_IDENTITY) {
        issues.push(issue(
            "function_object.identity_overclaim",
            "caveats.identity must be source-token, not semantic compiler identity",
        ));
    }
    if caveats.get("correctness_claim").and_then(Value::as_str) != Some(NO_CORRECTNESS_CLAIM) {
        issues.push(issue(
            "function_object.correctness_overclaim",
            "function-object hash agreement is provenance/review evidence, not a correctness proof",
        ));
    }
    issues
}

fn caveat_status_issues(caveats: &Map<String, Value>) -> Vec<FunctionObjectIssue> {
    let mut issues = Vec::with_capacity(3);
    if caveats.get("dependency_status").and_then(Value::as_str) != Some(SOURCE_SCANNER_DEPENDENCIES)
    {
        issues.push(issue(
            "function_object.dependency_status_overclaim",
            "source-token function objects must use source-scanner-token-set dependency status",
        ));
    }
    if !known_value(caveats, "macro_status", KNOWN_MACRO_STATUSES) {
        issues.push(issue(
            "function_object.unknown_macro_status",
            "macro_status must use the closed function-object caveat vocabulary",
        ));
    }
    if !known_value(caveats, "effect_status", KNOWN_EFFECT_STATUSES) {
        issues.push(issue(
            "function_object.unknown_effect_status",
            "effect_status must use the closed function-object caveat vocabulary",
        ));
    }
    issues
}

fn required_object_issues(
    value: Option<&Value>,
    object_name: &str,
    required_fields: &[&str],
) -> Vec<FunctionObjectIssue> {
    let Some(object) = value.and_then(Value::as_object) else {
        return vec![issue(
            "function_object.missing_object",
            format!("{object_name} must be a JSON object"),
        )];
    };
    required_fields
        .iter()
        .filter(|field| !object.contains_key(**field))
        .map(|field| {
            issue(
                "function_object.missing_field",
                format!("missing required field: {object_name}.{field}"),
            )
        })
        .collect()
}

fn known_value(object: &Map<String, Value>, field: &str, known: &[&str]) -> bool {
    object
        .get(field)
        .and_then(Value::as_str)
        .is_some_and(|value| known.contains(&value))
}

fn strip_non_hash_fields(value: &mut Value) {
    let mut stack = Vec::with_capacity(HASH_TRAVERSAL_STACK_CAPACITY);
    stack.push(value);
    while let Some(current) = stack.pop() {
        match current {
            Value::Object(object) => {
                object.remove("object_hash");
                object.remove("pointer_hash");
                object.remove("display_root");
                object.remove("display_path");
                object.remove("display_only");
                for child in object.values_mut() {
                    stack.push(child);
                }
            }
            Value::Array(entries) => {
                for child in entries {
                    stack.push(child);
                }
            }
            _ => {}
        }
    }
}

fn entropy_issues(root_path: &str, root: &Value) -> Vec<FunctionObjectIssue> {
    let mut issues = Vec::with_capacity(FUNCTION_OBJECT_ISSUE_CAPACITY);
    let mut stack = Vec::with_capacity(HASH_TRAVERSAL_STACK_CAPACITY);
    stack.push((root_path.to_string(), root));
    while let Some((path, value)) = stack.pop() {
        match value {
            Value::Object(object) => {
                for (key, child) in object {
                    let child_path = format!("{path}.{key}");
                    if is_entropy_key(key) {
                        issues.push(entropy_key_issue(&child_path));
                    }
                    stack.push((child_path, child));
                }
            }
            Value::Array(entries) => {
                for (index, child) in entries.iter().enumerate() {
                    stack.push((format!("{path}[{index}]"), child));
                }
            }
            Value::String(text) => {
                if is_temporary_path(text) {
                    issues.push(entropy_temporary_path_issue(&path));
                }
            }
            _ => {}
        }
    }
    issues
}

fn is_entropy_key(key: &str) -> bool {
    let lowered = key.to_ascii_lowercase();
    ENTROPY_KEY_FRAGMENTS
        .iter()
        .any(|fragment| lowered.contains(fragment))
}

fn is_temporary_path(text: &str) -> bool {
    text.starts_with("/tmp/") || text.starts_with("/var/tmp/")
}

fn entropy_key_issue(path: &str) -> FunctionObjectIssue {
    issue(
        "function_object.ambient_entropy_in_hash_material",
        format!("ambient entropy field is not allowed in hash material: {path}"),
    )
}

fn entropy_temporary_path_issue(path: &str) -> FunctionObjectIssue {
    issue(
        "function_object.ambient_entropy_in_hash_material",
        format!("temporary absolute path is not allowed in hash material: {path}"),
    )
}

fn issue(code: impl Into<String>, message: impl Into<String>) -> FunctionObjectIssue {
    FunctionObjectIssue {
        code: code.into(),
        message: message.into(),
    }
}

fn is_blake3_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

// Active Cairn refs: content_addressed_rust_evidence.name_pointer,
// content_addressed_rust_evidence.name_pointer.move,
// content_addressed_rust_evidence.name_pointer.drift,
// content_addressed_rust_evidence.caveats,
// content_addressed_rust_evidence.caveats.dependency_status,
// content_addressed_rust_evidence.caveats.effects_fail_closed,
// content_addressed_rust_evidence.export_bundle.mutation_manifest.
// These become Tracey r[impl ...] refs only after the delta is synced into accepted specs.
pub fn validate_name_pointer_receipt(receipt: &Value) -> NamePointerReport {
    let computed_pointer_hash = name_pointer_hash(receipt);
    let issues = name_pointer_issues(receipt, &computed_pointer_hash);
    let is_valid = issues.is_empty();
    NamePointerReport {
        valid: is_valid,
        verdict: if is_valid { "PASS" } else { "FAIL" }.to_string(),
        issues,
        computed_pointer_hash,
    }
}

pub fn name_pointer_hash(receipt: &Value) -> String {
    canonical_json_hash(&name_pointer_hash_material(receipt))
}

pub fn name_pointer_hash_material(receipt: &Value) -> Value {
    let mut material = receipt.clone();
    strip_non_hash_fields(&mut material);
    material
}

fn name_pointer_issues(receipt: &Value, computed_pointer_hash: &str) -> Vec<NamePointerIssue> {
    let Some(object) = receipt.as_object() else {
        return vec![pointer_issue(
            "name_pointer.not_object",
            "name pointer receipt must be a JSON object",
        )];
    };
    let mut issues = Vec::with_capacity(NAME_POINTER_ISSUE_CAPACITY);
    issues.extend(name_pointer_required_issues(object));
    issues.extend(name_pointer_hash_issues(object, computed_pointer_hash));
    issues.extend(name_pointer_movement_issues(object));
    issues.extend(name_pointer_manifest_issues(object));
    issues.extend(name_pointer_drift_issues(object));
    issues.extend(name_pointer_caveat_issues(object));
    issues.extend(pointer_entropy_issues(
        "$",
        &name_pointer_hash_material(receipt),
    ));
    issues
}

fn name_pointer_required_issues(object: &Map<String, Value>) -> Vec<NamePointerIssue> {
    let mut issues = Vec::with_capacity(NAME_POINTER_REQUIRED_FIELDS.len() + 1);
    for field in NAME_POINTER_REQUIRED_FIELDS {
        if !object.contains_key(*field) {
            issues.push(pointer_issue(
                "name_pointer.missing_field",
                format!("missing required field: {field}"),
            ));
        }
    }
    if object.get("schema").and_then(Value::as_str) != Some(NAME_POINTER_SCHEMA_V1) {
        issues.push(pointer_issue(
            "name_pointer.unsupported_schema",
            "schema must be octet.name-pointer.v1",
        ));
    }
    issues
}

fn name_pointer_hash_issues(
    object: &Map<String, Value>,
    computed_pointer_hash: &str,
) -> Vec<NamePointerIssue> {
    let mut issues = Vec::with_capacity(3);
    let stated_hash = object
        .get("pointer_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if stated_hash != computed_pointer_hash || !is_blake3_hex(stated_hash) {
        issues.push(pointer_issue(
            "name_pointer.hash_mismatch",
            "pointer_hash must equal the BLAKE3 hash of canonical name-pointer content with non-hash fields excluded",
        ));
    }
    for field in ["old_object_hash", "new_object_hash"] {
        if !object
            .get(field)
            .and_then(Value::as_str)
            .is_some_and(is_blake3_hex)
        {
            issues.push(pointer_issue(
                "name_pointer.invalid_hash_field",
                format!("{field} must be a BLAKE3 hex string"),
            ));
        }
    }
    issues
}

fn name_pointer_movement_issues(object: &Map<String, Value>) -> Vec<NamePointerIssue> {
    let mut issues = Vec::with_capacity(4);
    for field in ["name", "reason", "diff_summary", "rollback_handle"] {
        if !non_empty_string(object, field) {
            issues.push(pointer_issue(
                "name_pointer.missing_movement_field",
                format!("{field} must be a non-empty string"),
            ));
        }
    }
    if object.get("old_object_hash") == object.get("new_object_hash") {
        issues.push(pointer_issue(
            "name_pointer.noop_move",
            "pointer movement must record a new object hash distinct from the old object hash",
        ));
    }
    issues
}

fn name_pointer_manifest_issues(object: &Map<String, Value>) -> Vec<NamePointerIssue> {
    let Some(manifest) = object.get("mutation_manifest").and_then(Value::as_object) else {
        return vec![pointer_issue(
            "name_pointer.missing_object",
            "mutation_manifest must be a JSON object",
        )];
    };
    let mut issues = Vec::with_capacity(6);
    for field in MUTATION_MANIFEST_REQUIRED_FIELDS {
        if !manifest.contains_key(*field) {
            issues.push(pointer_issue(
                "name_pointer.missing_field",
                format!("missing required field: mutation_manifest.{field}"),
            ));
        }
    }
    issues.extend(touched_file_issues(manifest));
    if !manifest
        .get("manifest_hash")
        .and_then(Value::as_str)
        .is_some_and(is_blake3_hex)
    {
        issues.push(pointer_issue(
            "name_pointer.invalid_manifest_hash",
            "mutation_manifest.manifest_hash must be a BLAKE3 hex string",
        ));
    }
    issues
}

fn touched_file_issues(manifest: &Map<String, Value>) -> Vec<NamePointerIssue> {
    let Some(entries) = manifest.get("touched_files").and_then(Value::as_array) else {
        return vec![pointer_issue(
            "name_pointer.invalid_touched_files",
            "mutation_manifest.touched_files must be an array",
        )];
    };
    let mut issues = Vec::with_capacity(entries.len() + 1);
    let mut previous_path = None::<&str>;
    for entry in entries {
        let Some(file) = entry.as_object() else {
            issues.push(pointer_issue(
                "name_pointer.invalid_touched_file",
                "each touched file must be a JSON object",
            ));
            continue;
        };
        let path = file.get("path").and_then(Value::as_str).unwrap_or_default();
        if path.is_empty() || path.starts_with('/') || path.contains("..") {
            issues.push(pointer_issue(
                "name_pointer.invalid_touched_file_path",
                "touched file paths must be non-empty safe relative paths",
            ));
        }
        if previous_path.is_some_and(|previous| previous > path) {
            issues.push(pointer_issue(
                "name_pointer.unsorted_mutation_manifest",
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
                issues.push(pointer_issue(
                    "name_pointer.invalid_touched_file_hash",
                    format!("touched_files.{hash_field} must be a BLAKE3 hex string"),
                ));
            }
        }
    }
    issues
}

fn name_pointer_drift_issues(object: &Map<String, Value>) -> Vec<NamePointerIssue> {
    let Some(drift) = object.get("drift_check").and_then(Value::as_object) else {
        return vec![pointer_issue(
            "name_pointer.missing_object",
            "drift_check must be a JSON object",
        )];
    };
    if drift.get("status").and_then(Value::as_str) == Some(DRIFT_VALID_STATUS) {
        return Vec::new();
    }
    vec![pointer_issue(
        "name_pointer.drift_detected",
        "pointer is invalid until an explicit update receipt records the new object hash",
    )]
}

fn name_pointer_caveat_issues(object: &Map<String, Value>) -> Vec<NamePointerIssue> {
    let Some(caveats) = object.get("caveats").and_then(Value::as_object) else {
        return vec![pointer_issue(
            "name_pointer.missing_object",
            "caveats must be a JSON object",
        )];
    };
    let mut issues = Vec::with_capacity(5);
    if caveats.get("dependency_status").and_then(Value::as_str) != Some(SOURCE_SCANNER_DEPENDENCIES)
    {
        issues.push(pointer_issue(
            "name_pointer.dependency_status_overclaim",
            "name-pointer receipts must preserve source-scanner-token-set dependency caveats",
        ));
    }
    if !known_value(caveats, "macro_status", KNOWN_MACRO_STATUSES) {
        issues.push(pointer_issue(
            "name_pointer.unknown_macro_status",
            "macro_status must use the closed caveat vocabulary",
        ));
    }
    let effect_status = caveats.get("effect_status").and_then(Value::as_str);
    if !effect_status.is_some_and(|value| KNOWN_EFFECT_STATUSES.contains(&value)) {
        issues.push(pointer_issue(
            "name_pointer.unknown_effect_status",
            "effect_status must use the closed caveat vocabulary",
        ));
    }
    let is_pure_cache_eligible = caveats
        .get("pure_cache_eligible")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let is_effect_cache_eligible =
        effect_status.is_some_and(|value| PURE_CACHE_ELIGIBLE_EFFECTS.contains(&value));
    if is_pure_cache_eligible && !is_effect_cache_eligible {
        issues.push(pointer_issue(
            "name_pointer.pure_cache_overclaim",
            "unknown, macro-hidden, or unresolved effects must not be pure-cache eligible",
        ));
    }
    issues
}

fn pointer_entropy_issues(root_path: &str, root: &Value) -> Vec<NamePointerIssue> {
    entropy_issues(root_path, root)
        .into_iter()
        .map(|issue| {
            pointer_issue(
                "name_pointer.ambient_entropy_in_hash_material",
                issue.message,
            )
        })
        .collect()
}

fn non_empty_string(object: &Map<String, Value>, field: &str) -> bool {
    object
        .get(field)
        .and_then(Value::as_str)
        .is_some_and(|value| !value.is_empty())
}

fn pointer_issue(code: impl Into<String>, message: impl Into<String>) -> NamePointerIssue {
    NamePointerIssue {
        code: code.into(),
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn hash_placeholder() -> String {
        "0".repeat(64)
    }

    fn base_receipt(display_root: &str) -> Value {
        json!({
            "schema": FUNCTION_OBJECT_SCHEMA_V1,
            "object_hash": hash_placeholder(),
            "source_token": {
                "crate": "demo",
                "module_path": "demo::math",
                "name": "add_one",
                "definition_span": "src/lib.rs:1:1-3:2"
            },
            "signature_hash": "2d1d6fd7359ba2e5e25737604b14fc6a46fec9c5d7be69f3e3fe021e54ffd0bc",
            "body_hash": "297b70644d4c9a63fb0df22a8f346b7e600a714c471454a7784713b3983aaf74",
            "normalization": {
                "version": "source-token-v1",
                "content_kind": SOURCE_TOKEN_CONTENT_KIND,
                "display_root": display_root
            },
            "toolchain": {
                "rustc": "rustc 1.87.0",
                "octet_extractor": "octet 0.1.0"
            },
            "caveats": {
                "identity": SOURCE_TOKEN_IDENTITY,
                "dependency_status": SOURCE_SCANNER_DEPENDENCIES,
                "macro_status": "source-visible",
                "effect_status": "unknown",
                "correctness_claim": NO_CORRECTNESS_CLAIM
            }
        })
    }

    fn with_current_hash(mut receipt: Value) -> Value {
        let hash = function_object_hash(&receipt);
        receipt["object_hash"] = Value::String(hash);
        receipt
    }

    #[test]
    fn accepts_source_token_function_object_with_caveats() {
        let receipt = with_current_hash(base_receipt("/workspace/one"));
        let report = validate_function_object_receipt(&receipt);
        assert_eq!(report.verdict, "PASS");
        assert!(report.valid, "{:?}", report.issues);
        assert_eq!(receipt["object_hash"], report.computed_object_hash);
    }

    #[test]
    fn object_hash_excludes_display_root_for_portability() {
        let left = base_receipt("/workspace/one");
        let right = base_receipt("/workspace/two");
        assert_eq!(function_object_hash(&left), function_object_hash(&right));
    }

    #[test]
    fn rejects_correctness_and_identity_overclaims() {
        let mut receipt = base_receipt("/workspace/one");
        receipt["caveats"]["identity"] = Value::String("semantic-compiler-identity".to_string());
        receipt["caveats"]["correctness_claim"] = Value::String("proves-behavior".to_string());
        let receipt = with_current_hash(receipt);
        let codes = validate_function_object_receipt(&receipt)
            .issues
            .into_iter()
            .map(|issue| issue.code)
            .collect::<Vec<_>>();
        assert!(codes.contains(&"function_object.identity_overclaim".to_string()));
        assert!(codes.contains(&"function_object.correctness_overclaim".to_string()));
    }

    #[test]
    fn rejects_ambient_entropy_in_hash_material() {
        let mut receipt = base_receipt("/workspace/one");
        receipt["source_token"]["captured_at_timestamp"] =
            Value::String("2026-05-19T00:00:00Z".to_string());
        receipt["source_token"]["temp_path"] = Value::String("/tmp/cairn-x/src/lib.rs".to_string());
        let receipt = with_current_hash(receipt);
        let report = validate_function_object_receipt(&receipt);
        assert!(!report.valid);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "function_object.ambient_entropy_in_hash_material"));
    }

    fn base_pointer_receipt() -> Value {
        json!({
            "schema": NAME_POINTER_SCHEMA_V1,
            "pointer_hash": hash_placeholder(),
            "name": "demo::math::add_one",
            "old_object_hash": "1111111111111111111111111111111111111111111111111111111111111111",
            "new_object_hash": "2222222222222222222222222222222222222222222222222222222222222222",
            "reason": "source-token body changed",
            "diff_summary": "body_hash changed; signature_hash unchanged",
            "rollback_handle": "pointer://demo::math::add_one@1111111111111111111111111111111111111111111111111111111111111111",
            "mutation_manifest": {
                "manifest_hash": "3333333333333333333333333333333333333333333333333333333333333333",
                "touched_files": [
                    {
                        "path": "octet/pointers/demo.math.add_one.json",
                        "before_hash": "4444444444444444444444444444444444444444444444444444444444444444",
                        "after_hash": "5555555555555555555555555555555555555555555555555555555555555555"
                    }
                ]
            },
            "drift_check": {
                "status": DRIFT_VALID_STATUS,
                "checked_object_hash": "2222222222222222222222222222222222222222222222222222222222222222"
            },
            "caveats": {
                "dependency_status": SOURCE_SCANNER_DEPENDENCIES,
                "macro_status": "source-visible",
                "effect_status": "unknown",
                "pure_cache_eligible": false
            }
        })
    }

    fn with_current_pointer_hash(mut receipt: Value) -> Value {
        let hash = name_pointer_hash(&receipt);
        receipt["pointer_hash"] = Value::String(hash);
        receipt
    }

    #[test]
    fn accepts_reviewable_name_pointer_move() {
        let receipt = with_current_pointer_hash(base_pointer_receipt());
        let report = validate_name_pointer_receipt(&receipt);
        assert_eq!(report.verdict, "PASS");
        assert!(report.valid, "{:?}", report.issues);
        assert_eq!(receipt["pointer_hash"], report.computed_pointer_hash);
    }

    #[test]
    fn rejects_pointer_drift_until_update_receipt_exists() {
        let mut receipt = base_pointer_receipt();
        receipt["drift_check"]["status"] = Value::String("drift".to_string());
        let receipt = with_current_pointer_hash(receipt);
        let report = validate_name_pointer_receipt(&receipt);
        assert!(!report.valid);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "name_pointer.drift_detected"));
    }

    #[test]
    fn rejects_unknown_effect_pure_cache_overclaim() {
        let mut receipt = base_pointer_receipt();
        receipt["caveats"]["effect_status"] = Value::String("unknown".to_string());
        receipt["caveats"]["pure_cache_eligible"] = Value::Bool(true);
        let receipt = with_current_pointer_hash(receipt);
        let report = validate_name_pointer_receipt(&receipt);
        assert!(!report.valid);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "name_pointer.pure_cache_overclaim"));
    }
}
