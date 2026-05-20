use serde_json::{Map, Value};

use crate::verified::evidence::canonical_json_hash;

pub const FUNCTION_OBJECT_SCHEMA_V1: &str = "octet.function-object.v1";

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

    let mut issues = Vec::new();
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

    if object
        .get("normalization")
        .and_then(|value| value.get("content_kind"))
        .and_then(Value::as_str)
        != Some(SOURCE_TOKEN_CONTENT_KIND)
    {
        issues.push(issue(
            "function_object.normalization_not_source_token",
            "normalization.content_kind must declare source-token identity",
        ));
    }

    if let Some(caveats) = object.get("caveats").and_then(Value::as_object) {
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
        if caveats.get("dependency_status").and_then(Value::as_str)
            != Some(SOURCE_SCANNER_DEPENDENCIES)
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
    }

    let hash_material = function_object_hash_material(receipt);
    entropy_issues("$", &hash_material, &mut issues);
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
    match value {
        Value::Object(object) => {
            object.remove("object_hash");
            object.remove("display_root");
            object.remove("display_path");
            object.remove("display_only");
            for child in object.values_mut() {
                strip_non_hash_fields(child);
            }
        }
        Value::Array(entries) => {
            for child in entries {
                strip_non_hash_fields(child);
            }
        }
        _ => {}
    }
}

fn entropy_issues(path: &str, value: &Value, issues: &mut Vec<FunctionObjectIssue>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                let child_path = format!("{path}.{key}");
                let lowered = key.to_ascii_lowercase();
                if ENTROPY_KEY_FRAGMENTS
                    .iter()
                    .any(|fragment| lowered.contains(fragment))
                {
                    issues.push(issue(
                        "function_object.ambient_entropy_in_hash_material",
                        format!(
                            "ambient entropy field is not allowed in hash material: {child_path}"
                        ),
                    ));
                }
                entropy_issues(&child_path, child, issues);
            }
        }
        Value::Array(entries) => {
            for (index, child) in entries.iter().enumerate() {
                entropy_issues(&format!("{path}[{index}]"), child, issues);
            }
        }
        Value::String(text) if text.starts_with("/tmp/") || text.starts_with("/var/tmp/") => {
            issues.push(issue(
                "function_object.ambient_entropy_in_hash_material",
                format!("temporary absolute path is not allowed in hash material: {path}"),
            ));
        }
        _ => {}
    }
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
}
