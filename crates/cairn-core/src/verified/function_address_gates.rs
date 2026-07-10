use serde_json::{json, Value};

pub const FUNCTION_ADDRESS_MODE_DISABLED: &str = "disabled";
pub const FUNCTION_ADDRESS_MODE_OPTIONAL: &str = "optional";
pub const FUNCTION_ADDRESS_MODE_REQUIRED: &str = "required";
pub const FUNCTION_ADDRESS_CLAIM_SCOPE_LIFECYCLE: &str =
    "function_address.v1.lifecycle_policy_conformance";
pub const FUNCTION_ADDRESS_BOUNDARY: &str = "Cairn consumed Valence, Mantle, and Kamacite function-address evidence for lifecycle/release policy conformance only; it did not prove Rust semantic correctness, compiler correctness, whole-program safety, verifier soundness, or release eligibility";

const BLAKE3_HEX_LENGTH: usize = 64;
const DISPOSITION_DISABLED: &str = "disabled";
const DISPOSITION_ABSENT: &str = "absent";
const DISPOSITION_PRESENT: &str = "present";
const DISPOSITION_FAILED: &str = "failed";
const FUNCTION_ADDRESS_CHECK_NAME: &str = "function_address_evidence";
const GRAPH_U64_FIELDS: &[&str] = &[
    "function_count",
    "source_count",
    "dependency_edge_count",
    "recorded_only_count",
    "property_count",
    "failed_link_count",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionAddressPolicy {
    pub mode: String,
    pub allowed_claim_scopes: Vec<String>,
    pub accepted_valence_policy_hashes: Vec<String>,
    pub required_roles: Vec<String>,
    pub required_schemas: Vec<String>,
    pub required_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionAddressEvidence {
    pub valence_receipt: Option<Value>,
    pub mantle_binding: Option<Value>,
    pub kamacite_receipt: Option<Value>,
    pub graph_summary: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionAddressIssue {
    pub code: String,
    pub field_path: String,
    pub expected: String,
    pub actual: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionAddressReadiness {
    pub mode: String,
    pub required: bool,
    pub passed: bool,
    pub skipped: bool,
    pub disposition: String,
    pub issues: Vec<FunctionAddressIssue>,
    pub valence_receipt_hash: Option<String>,
    pub mantle_binding_hash: Option<String>,
    pub kamacite_receipt_hash: Option<String>,
    pub claim_scope: Option<String>,
    pub graph_summary: Option<Value>,
    pub boundary: String,
}

struct FunctionAddressReadinessParts {
    passed: bool,
    skipped: bool,
    disposition: &'static str,
    issues: Vec<FunctionAddressIssue>,
    valence_receipt_hash: Option<String>,
    mantle_binding_hash: Option<String>,
    kamacite_receipt_hash: Option<String>,
    claim_scope: Option<String>,
    graph_summary: Option<Value>,
}

impl FunctionAddressReadiness {
    #[must_use]
    pub fn to_json(&self) -> Value {
        json!({
            "name": FUNCTION_ADDRESS_CHECK_NAME,
            "mode": self.mode,
            "required": self.required,
            "passed": self.passed,
            "skipped": self.skipped,
            "disposition": self.disposition,
            "issues": self.issues.iter().map(FunctionAddressIssue::to_json).collect::<Vec<_>>(),
            "valence_receipt_hash": self.valence_receipt_hash,
            "mantle_binding_hash": self.mantle_binding_hash,
            "kamacite_receipt_hash": self.kamacite_receipt_hash,
            "claim_scope": self.claim_scope,
            "graph_summary": self.graph_summary,
            "boundary": self.boundary,
        })
    }
}

impl FunctionAddressIssue {
    #[must_use]
    pub fn to_json(&self) -> Value {
        json!({
            "code": self.code,
            "field_path": self.field_path,
            "expected": self.expected,
            "actual": self.actual,
        })
    }
}

#[must_use]
pub fn evaluate_function_address_evidence(
    policy: &FunctionAddressPolicy,
    evidence: &FunctionAddressEvidence,
) -> FunctionAddressReadiness {
    // r[impl cairn.function_address_gates.policy]
    // r[impl cairn.function_address_gates.receipt_validation]
    // r[impl cairn.function_address_gates.boundary]
    // r[impl cairn.function_address_gates.graph_summary]
    if policy.mode == FUNCTION_ADDRESS_MODE_DISABLED {
        return function_address_readiness(
            policy,
            FunctionAddressReadinessParts {
                passed: true,
                skipped: true,
                disposition: DISPOSITION_DISABLED,
                issues: Vec::new(),
                valence_receipt_hash: None,
                mantle_binding_hash: None,
                kamacite_receipt_hash: None,
                claim_scope: None,
                graph_summary: None,
            },
        );
    }

    let Some(valence_receipt) = evidence.valence_receipt.as_ref() else {
        if policy.mode == FUNCTION_ADDRESS_MODE_OPTIONAL {
            return function_address_readiness(
                policy,
                FunctionAddressReadinessParts {
                    passed: true,
                    skipped: true,
                    disposition: DISPOSITION_ABSENT,
                    issues: Vec::new(),
                    valence_receipt_hash: None,
                    mantle_binding_hash: None,
                    kamacite_receipt_hash: None,
                    claim_scope: None,
                    graph_summary: None,
                },
            );
        }
        return function_address_readiness(
            policy,
            FunctionAddressReadinessParts {
                passed: false,
                skipped: false,
                disposition: DISPOSITION_FAILED,
                issues: vec![function_address_issue(
                    "function_address.valence_receipt_missing",
                    "valence_receipt",
                    "Valence function-address receipt",
                    "missing",
                )],
                valence_receipt_hash: None,
                mantle_binding_hash: None,
                kamacite_receipt_hash: None,
                claim_scope: None,
                graph_summary: None,
            },
        );
    };

    let mut issues = Vec::new();
    let valence_receipt_hash = receipt_string(valence_receipt, "receipt_hash");
    let mantle_binding_hash = evidence
        .mantle_binding
        .as_ref()
        .and_then(|binding| receipt_string(binding, "receipt_hash"));
    let kamacite_receipt_hash = evidence
        .kamacite_receipt
        .as_ref()
        .and_then(|receipt| receipt_string(receipt, "receipt_hash"));
    let claim_scope = claim_scope(evidence, valence_receipt);

    validate_valence_receipt(policy, valence_receipt, &mut issues);
    validate_mantle_binding(policy, evidence.mantle_binding.as_ref(), &mut issues);
    validate_kamacite_receipt(evidence.kamacite_receipt.as_ref(), &mut issues);
    validate_cross_links(valence_receipt, evidence, &mut issues);
    validate_boundary(
        policy,
        valence_receipt,
        evidence.mantle_binding.as_ref(),
        &mut issues,
    );
    let graph_summary = validate_graph_summary(evidence.graph_summary.as_ref(), &mut issues);
    issues.sort_by(|left, right| {
        left.code
            .cmp(&right.code)
            .then(left.field_path.cmp(&right.field_path))
    });
    let passed = issues.is_empty();
    function_address_readiness(
        policy,
        FunctionAddressReadinessParts {
            passed,
            skipped: false,
            disposition: if passed {
                DISPOSITION_PRESENT
            } else {
                DISPOSITION_FAILED
            },
            issues,
            valence_receipt_hash,
            mantle_binding_hash,
            kamacite_receipt_hash,
            claim_scope,
            graph_summary,
        },
    )
}

fn function_address_readiness(
    policy: &FunctionAddressPolicy,
    parts: FunctionAddressReadinessParts,
) -> FunctionAddressReadiness {
    FunctionAddressReadiness {
        mode: policy.mode.clone(),
        required: policy.mode == FUNCTION_ADDRESS_MODE_REQUIRED,
        passed: parts.passed,
        skipped: parts.skipped,
        disposition: parts.disposition.to_string(),
        issues: parts.issues,
        valence_receipt_hash: parts.valence_receipt_hash,
        mantle_binding_hash: parts.mantle_binding_hash,
        kamacite_receipt_hash: parts.kamacite_receipt_hash,
        claim_scope: parts.claim_scope,
        graph_summary: parts.graph_summary,
        boundary: policy.required_boundary.clone(),
    }
}

fn validate_valence_receipt(
    policy: &FunctionAddressPolicy,
    receipt: &Value,
    issues: &mut Vec<FunctionAddressIssue>,
) {
    if receipt.get("valid").and_then(Value::as_bool) != Some(true)
        || !matches!(
            receipt.get("verdict").and_then(Value::as_str),
            Some("pass" | "PASS")
        )
    {
        issues.push(function_address_issue(
            "function_address.valence_verdict_failed",
            "valence_receipt.verdict",
            "passing Valence function-address verification receipt",
            "failed",
        ));
    }
    validate_hash_field(
        receipt,
        "receipt_hash",
        "valence_receipt.receipt_hash",
        "function_address.valence_receipt_hash_invalid",
        issues,
    );
    let policy_hash = receipt_string(receipt, "valence_policy_hash").unwrap_or_default();
    if !policy.accepted_valence_policy_hashes.is_empty()
        && !policy
            .accepted_valence_policy_hashes
            .iter()
            .any(|accepted| accepted == &policy_hash)
    {
        issues.push(function_address_issue(
            "function_address.valence_policy_stale",
            "valence_receipt.valence_policy_hash",
            "accepted Valence policy hash",
            if policy_hash.is_empty() {
                "missing"
            } else {
                "stale"
            },
        ));
    }
    validate_required_roles(policy, receipt, issues);
    validate_required_schemas(policy, receipt, issues);
    validate_verification_role_summary(receipt, issues);
}

fn validate_mantle_binding(
    policy: &FunctionAddressPolicy,
    binding: Option<&Value>,
    issues: &mut Vec<FunctionAddressIssue>,
) {
    let Some(binding) = binding else {
        issues.push(function_address_issue(
            "function_address.mantle_binding_missing",
            "mantle_binding",
            "Mantle function-address release binding",
            "missing",
        ));
        return;
    };
    if binding.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(function_address_issue(
            "function_address.mantle_binding_failed",
            "mantle_binding.valid",
            "valid Mantle release binding",
            "failed",
        ));
    }
    validate_hash_field(
        binding,
        "receipt_hash",
        "mantle_binding.receipt_hash",
        "function_address.mantle_binding_hash_invalid",
        issues,
    );
    let claim_scope = binding
        .get("claim_scope")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if !policy
        .allowed_claim_scopes
        .iter()
        .any(|allowed| allowed == claim_scope)
    {
        issues.push(function_address_issue(
            "function_address.claim_scope_unsupported",
            "mantle_binding.claim_scope",
            "policy-owned allowed function-address claim scope",
            if claim_scope.is_empty() {
                "missing"
            } else {
                "unsupported"
            },
        ));
    }
    for field in [
        "sidecar_digest",
        "valence_receipt_digest",
        "source_archive_digest",
        "release_binary_digest",
    ] {
        validate_hash_field(
            binding,
            field,
            &format!("mantle_binding.{field}"),
            "function_address.linkage_hash_invalid",
            issues,
        );
    }
}

fn validate_kamacite_receipt(receipt: Option<&Value>, issues: &mut Vec<FunctionAddressIssue>) {
    let Some(receipt) = receipt else {
        return;
    };
    if receipt.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(function_address_issue(
            "function_address.kamacite_receipt_failed",
            "kamacite_receipt.valid",
            "valid Kamacite function-address receipt",
            "failed",
        ));
    }
    validate_hash_field(
        receipt,
        "receipt_hash",
        "kamacite_receipt.receipt_hash",
        "function_address.kamacite_receipt_hash_invalid",
        issues,
    );
}

fn validate_cross_links(
    receipt: &Value,
    evidence: &FunctionAddressEvidence,
    issues: &mut Vec<FunctionAddressIssue>,
) {
    let Some(binding) = evidence.mantle_binding.as_ref() else {
        return;
    };
    validate_equal_hashes(
        receipt_string(binding, "valence_receipt_digest"),
        receipt_string(receipt, "receipt_hash"),
        "mantle_binding.valence_receipt_digest",
        "function_address.valence_receipt_digest_stale",
        issues,
    );
    if let Some(kamacite) = evidence.kamacite_receipt.as_ref() {
        validate_equal_hashes(
            receipt_string(receipt, "kamacite_receipt_hash"),
            receipt_string(kamacite, "receipt_hash"),
            "valence_receipt.kamacite_receipt_hash",
            "function_address.kamacite_receipt_hash_stale",
            issues,
        );
    }
}

fn validate_boundary(
    policy: &FunctionAddressPolicy,
    receipt: &Value,
    binding: Option<&Value>,
    issues: &mut Vec<FunctionAddressIssue>,
) {
    let receipt_boundary = receipt
        .get("non_claim_boundary")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if receipt_boundary != policy.required_boundary {
        issues.push(function_address_issue(
            "function_address.non_claim_boundary_weakened",
            "valence_receipt.non_claim_boundary",
            "policy-owned Cairn function-address non-claim boundary",
            if receipt_boundary.is_empty() {
                "missing"
            } else {
                "weakened"
            },
        ));
    }
    if let Some(binding) = binding {
        let binding_boundary = binding
            .get("non_claim_boundary")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if binding_boundary != policy.required_boundary {
            issues.push(function_address_issue(
                "function_address.non_claim_boundary_weakened",
                "mantle_binding.non_claim_boundary",
                "policy-owned Cairn function-address non-claim boundary",
                if binding_boundary.is_empty() {
                    "missing"
                } else {
                    "weakened"
                },
            ));
        }
    }
    for text in [
        receipt_boundary,
        binding
            .and_then(|value| value.get("non_claim_boundary"))
            .and_then(Value::as_str)
            .unwrap_or_default(),
    ] {
        if function_address_text_overclaims(text) {
            issues.push(function_address_issue(
                "function_address.overclaim_detected",
                "non_claim_boundary",
                "bounded function-address non-claim text",
                "overclaim",
            ));
        }
    }
}

fn validate_graph_summary(
    graph: Option<&Value>,
    issues: &mut Vec<FunctionAddressIssue>,
) -> Option<Value> {
    let graph = graph?;
    let mut valid = true;
    for field in GRAPH_U64_FIELDS {
        if graph.get(*field).and_then(Value::as_u64).is_none() {
            issues.push(function_address_issue(
                "function_address.graph_summary_malformed",
                &format!("graph_summary.{field}"),
                "non-negative integer function-address graph summary field",
                "malformed",
            ));
            valid = false;
        }
    }
    let report_hash = graph
        .get("report_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if !is_blake3_hex(report_hash) {
        issues.push(function_address_issue(
            "function_address.graph_summary_malformed",
            "graph_summary.report_hash",
            "BLAKE3 report hash",
            if report_hash.is_empty() {
                "missing"
            } else {
                "malformed"
            },
        ));
        valid = false;
    }
    valid.then(|| graph.clone())
}

fn validate_required_roles(
    policy: &FunctionAddressPolicy,
    receipt: &Value,
    issues: &mut Vec<FunctionAddressIssue>,
) {
    let roles = receipt
        .get("roles")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    for required in &policy.required_roles {
        if !roles
            .iter()
            .any(|role| role.as_str() == Some(required.as_str()))
        {
            issues.push(function_address_issue(
                "function_address.required_role_missing",
                "valence_receipt.roles",
                required,
                "missing",
            ));
        }
    }
}

fn validate_required_schemas(
    policy: &FunctionAddressPolicy,
    receipt: &Value,
    issues: &mut Vec<FunctionAddressIssue>,
) {
    let schemas = receipt
        .get("schemas")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    for required in &policy.required_schemas {
        if !schemas
            .iter()
            .any(|schema| schema.as_str() == Some(required.as_str()))
        {
            issues.push(function_address_issue(
                "function_address.required_schema_missing",
                "valence_receipt.schemas",
                required,
                "missing",
            ));
        }
    }
}

fn validate_verification_role_summary(receipt: &Value, issues: &mut Vec<FunctionAddressIssue>) {
    let Some(summary) = receipt.get("verification_role_summary") else {
        issues.push(function_address_issue(
            "function_address.verification_role_summary_missing",
            "verification_role_summary",
            "verification role summary",
            "missing",
        ));
        return;
    };
    let property_count = summary.get("property_count").and_then(Value::as_u64);
    let recorded_only_count = summary.get("recorded_only_count").and_then(Value::as_u64);
    let verified_count = summary.get("verified_count").and_then(Value::as_u64);
    if property_count.is_none()
        || recorded_only_count.is_none()
        || verified_count.is_none()
        || property_count != verified_count
    {
        issues.push(function_address_issue(
            "function_address.role_promotion_detected",
            "verification_role_summary.verified_count",
            "verified_count equal to property_count and recorded_only separate",
            "recorded-only-counted-verified",
        ));
    }
}

fn validate_hash_field(
    value: &Value,
    field: &str,
    field_path: &str,
    code: &str,
    issues: &mut Vec<FunctionAddressIssue>,
) {
    let actual = value.get(field).and_then(Value::as_str).unwrap_or_default();
    if !is_blake3_hex(actual) {
        issues.push(function_address_issue(
            code,
            field_path,
            "BLAKE3 hash",
            if actual.is_empty() {
                "missing"
            } else {
                "invalid"
            },
        ));
    }
}

fn validate_equal_hashes(
    left: Option<String>,
    right: Option<String>,
    field_path: &str,
    code: &str,
    issues: &mut Vec<FunctionAddressIssue>,
) {
    if left.is_none() || right.is_none() || left != right {
        issues.push(function_address_issue(
            code,
            field_path,
            "matching BLAKE3 hash",
            "stale",
        ));
    }
}

fn claim_scope(evidence: &FunctionAddressEvidence, receipt: &Value) -> Option<String> {
    evidence
        .mantle_binding
        .as_ref()
        .and_then(|binding| receipt_string(binding, "claim_scope"))
        .or_else(|| receipt_string(receipt, "claim_scope"))
}

fn function_address_issue(
    code: &str,
    field_path: &str,
    expected: &str,
    actual: &str,
) -> FunctionAddressIssue {
    FunctionAddressIssue {
        code: code.to_string(),
        field_path: field_path.to_string(),
        expected: expected.to_string(),
        actual: actual.to_string(),
    }
}

fn receipt_string(receipt: &Value, field: &str) -> Option<String> {
    receipt
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn function_address_text_overclaims(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "proves rust semantic correctness",
        "proves compiler correctness",
        "proves whole-program safety",
        "proves verifier soundness",
        "proves release eligibility",
    ]
    .iter()
    .any(|fragment| lower.contains(fragment))
}

#[must_use]
pub fn is_function_address_mode(mode: &str) -> bool {
    matches!(
        mode,
        FUNCTION_ADDRESS_MODE_DISABLED
            | FUNCTION_ADDRESS_MODE_OPTIONAL
            | FUNCTION_ADDRESS_MODE_REQUIRED
    )
}

#[must_use]
pub fn is_blake3_hex(value: &str) -> bool {
    value.len() == BLAKE3_HEX_LENGTH && value.chars().all(|character| character.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACCEPTED_POLICY_HASH: &str =
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const RECEIPT_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const MANTLE_HASH: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
    const KAMACITE_HASH: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
    const SIDE_CAR_HASH: &str = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
    const SOURCE_HASH: &str = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    const BINARY_HASH: &str = "1111111111111111111111111111111111111111111111111111111111111111";
    const GRAPH_HASH: &str = "2222222222222222222222222222222222222222222222222222222222222222";
    const STALE_HASH: &str = "3333333333333333333333333333333333333333333333333333333333333333";

    fn policy(mode: &str) -> FunctionAddressPolicy {
        FunctionAddressPolicy {
            mode: mode.to_string(),
            allowed_claim_scopes: vec![FUNCTION_ADDRESS_CLAIM_SCOPE_LIFECYCLE.to_string()],
            accepted_valence_policy_hashes: vec![ACCEPTED_POLICY_HASH.to_string()],
            required_roles: vec![
                "valence-function-address-evidence-profile".to_string(),
                "function-address-evidence-sidecar".to_string(),
            ],
            required_schemas: vec![
                "valence.function-address-evidence.v1".to_string(),
                "kamacite.function-address-receipt.v1".to_string(),
            ],
            required_boundary: FUNCTION_ADDRESS_BOUNDARY.to_string(),
        }
    }

    fn valence_receipt() -> Value {
        json!({
            "valid": true,
            "verdict": "PASS",
            "receipt_hash": RECEIPT_HASH,
            "valence_policy_hash": ACCEPTED_POLICY_HASH,
            "claim_scope": FUNCTION_ADDRESS_CLAIM_SCOPE_LIFECYCLE,
            "kamacite_receipt_hash": KAMACITE_HASH,
            "roles": ["valence-function-address-evidence-profile", "function-address-evidence-sidecar"],
            "schemas": ["valence.function-address-evidence.v1", "kamacite.function-address-receipt.v1"],
            "verification_role_summary": {
                "property_count": 0,
                "recorded_only_count": 2,
                "verified_count": 0
            },
            "non_claim_boundary": FUNCTION_ADDRESS_BOUNDARY
        })
    }

    fn mantle_binding() -> Value {
        json!({
            "valid": true,
            "receipt_hash": MANTLE_HASH,
            "claim_scope": FUNCTION_ADDRESS_CLAIM_SCOPE_LIFECYCLE,
            "sidecar_digest": SIDE_CAR_HASH,
            "valence_receipt_digest": RECEIPT_HASH,
            "source_archive_digest": SOURCE_HASH,
            "release_binary_digest": BINARY_HASH,
            "non_claim_boundary": FUNCTION_ADDRESS_BOUNDARY
        })
    }

    fn kamacite_receipt() -> Value {
        json!({
            "valid": true,
            "receipt_hash": KAMACITE_HASH,
            "non_claim_boundary": "not host-language semantic correctness"
        })
    }

    fn graph_summary() -> Value {
        json!({
            "function_count": 2,
            "source_count": 1,
            "dependency_edge_count": 1,
            "recorded_only_count": 2,
            "property_count": 0,
            "failed_link_count": 0,
            "report_hash": GRAPH_HASH
        })
    }

    fn evidence() -> FunctionAddressEvidence {
        FunctionAddressEvidence {
            valence_receipt: Some(valence_receipt()),
            mantle_binding: Some(mantle_binding()),
            kamacite_receipt: Some(kamacite_receipt()),
            graph_summary: Some(graph_summary()),
        }
    }

    #[test]
    fn disabled_mode_ignores_function_address_evidence() {
        // r[verify cairn.function_address_gates.policy.disabled]
        // r[verify cairn.function_address_gates.fixtures.positive]
        // r[verify cairn.function_address_gates.fixtures.positive.modes]
        let readiness = evaluate_function_address_evidence(
            &policy(FUNCTION_ADDRESS_MODE_DISABLED),
            &FunctionAddressEvidence {
                valence_receipt: Some(json!({ "valid": false })),
                mantle_binding: None,
                kamacite_receipt: None,
                graph_summary: None,
            },
        );
        assert!(readiness.passed);
        assert!(readiness.skipped);
        assert_eq!(readiness.disposition, DISPOSITION_DISABLED);
    }

    #[test]
    fn optional_absent_records_skipped_disposition() {
        // r[verify cairn.function_address_gates.policy.optional_absent]
        // r[verify cairn.function_address_gates.fixtures.positive]
        // r[verify cairn.function_address_gates.fixtures.positive.modes]
        let readiness = evaluate_function_address_evidence(
            &policy(FUNCTION_ADDRESS_MODE_OPTIONAL),
            &FunctionAddressEvidence {
                valence_receipt: None,
                mantle_binding: None,
                kamacite_receipt: None,
                graph_summary: None,
            },
        );
        assert!(readiness.passed);
        assert!(readiness.skipped);
        assert_eq!(readiness.disposition, DISPOSITION_ABSENT);
    }

    #[test]
    fn optional_present_verifies_function_address_wiring() {
        // r[verify cairn.function_address_gates.graph_summary.included]
        // r[verify cairn.function_address_gates.fixtures.positive]
        // r[verify cairn.function_address_gates.fixtures.positive.modes]
        let readiness = evaluate_function_address_evidence(
            &policy(FUNCTION_ADDRESS_MODE_OPTIONAL),
            &evidence(),
        );
        assert!(readiness.passed, "{:?}", readiness.issues);
        assert_eq!(readiness.disposition, DISPOSITION_PRESENT);
        assert!(readiness.graph_summary.is_some());
    }

    #[test]
    fn required_complete_chain_passes() {
        // r[verify cairn.function_address_gates.receipt_validation]
        // r[verify cairn.function_address_gates.receipt_validation.complete]
        // r[verify cairn.function_address_gates.graph_summary.included]
        // r[verify cairn.function_address_gates.fixtures.positive.modes]
        let readiness = evaluate_function_address_evidence(
            &policy(FUNCTION_ADDRESS_MODE_REQUIRED),
            &evidence(),
        );
        assert!(readiness.passed, "{:?}", readiness.issues);
        assert_eq!(
            readiness.valence_receipt_hash.as_deref(),
            Some(RECEIPT_HASH)
        );
        assert_eq!(readiness.mantle_binding_hash.as_deref(), Some(MANTLE_HASH));
        assert_eq!(
            readiness.kamacite_receipt_hash.as_deref(),
            Some(KAMACITE_HASH)
        );
    }

    #[test]
    fn required_missing_receipt_fails() {
        // r[verify cairn.function_address_gates.policy.required_missing]
        // r[verify cairn.function_address_gates.fixtures.negative]
        // r[verify cairn.function_address_gates.fixtures.negative.required]
        let readiness = evaluate_function_address_evidence(
            &policy(FUNCTION_ADDRESS_MODE_REQUIRED),
            &FunctionAddressEvidence {
                valence_receipt: None,
                mantle_binding: None,
                kamacite_receipt: None,
                graph_summary: None,
            },
        );
        assert!(!readiness.passed);
        assert_eq!(
            readiness.issues[0].code,
            "function_address.valence_receipt_missing"
        );
    }

    #[test]
    fn invalid_fixture_matrix_fails_closed() {
        // r[verify cairn.function_address_gates.receipt_validation.digest_mismatch]
        // r[verify cairn.function_address_gates.receipt_validation.failed_upstream]
        // r[verify cairn.function_address_gates.graph_summary.malformed]
        // r[verify cairn.function_address_gates.boundary.recorded_only]
        // r[verify cairn.function_address_gates.boundary.proof_separate]
        // r[verify cairn.function_address_gates.fixtures.negative]
        // r[verify cairn.function_address_gates.fixtures.negative.required]
        type Mutator = fn(&mut FunctionAddressEvidence);
        type Case = (&'static str, Mutator, &'static str);

        fn stale_valence_digest(evidence: &mut FunctionAddressEvidence) {
            evidence.mantle_binding.as_mut().unwrap()["valence_receipt_digest"] = json!(STALE_HASH);
        }
        fn failed_upstream(evidence: &mut FunctionAddressEvidence) {
            evidence.valence_receipt.as_mut().unwrap()["valid"] = json!(false);
        }
        fn stale_policy(evidence: &mut FunctionAddressEvidence) {
            evidence.valence_receipt.as_mut().unwrap()["valence_policy_hash"] = json!(STALE_HASH);
        }
        fn unsupported_claim_scope(evidence: &mut FunctionAddressEvidence) {
            evidence.mantle_binding.as_mut().unwrap()["claim_scope"] =
                json!("function-address.semantic-proof");
        }
        fn role_promotion(evidence: &mut FunctionAddressEvidence) {
            evidence.valence_receipt.as_mut().unwrap()["verification_role_summary"]
                ["verified_count"] = json!(2);
        }
        fn missing_role(evidence: &mut FunctionAddressEvidence) {
            evidence.valence_receipt.as_mut().unwrap()["roles"] =
                json!(["function-address-evidence-sidecar"]);
        }
        fn weakened_boundary(evidence: &mut FunctionAddressEvidence) {
            evidence.valence_receipt.as_mut().unwrap()["non_claim_boundary"] = json!("weakened");
        }
        fn overclaim(evidence: &mut FunctionAddressEvidence) {
            evidence.valence_receipt.as_mut().unwrap()["non_claim_boundary"] =
                json!("proves Rust semantic correctness");
        }
        fn failed_kamacite(evidence: &mut FunctionAddressEvidence) {
            evidence.kamacite_receipt.as_mut().unwrap()["valid"] = json!(false);
        }
        fn malformed_graph(evidence: &mut FunctionAddressEvidence) {
            evidence.graph_summary.as_mut().unwrap()["report_hash"] = json!("not-a-digest");
        }

        let cases: &[Case] = &[
            (
                "stale valence digest",
                stale_valence_digest,
                "function_address.valence_receipt_digest_stale",
            ),
            (
                "failed upstream",
                failed_upstream,
                "function_address.valence_verdict_failed",
            ),
            (
                "stale policy",
                stale_policy,
                "function_address.valence_policy_stale",
            ),
            (
                "unsupported claim scope",
                unsupported_claim_scope,
                "function_address.claim_scope_unsupported",
            ),
            (
                "role promotion",
                role_promotion,
                "function_address.role_promotion_detected",
            ),
            (
                "missing role",
                missing_role,
                "function_address.required_role_missing",
            ),
            (
                "weakened boundary",
                weakened_boundary,
                "function_address.non_claim_boundary_weakened",
            ),
            (
                "overclaim",
                overclaim,
                "function_address.overclaim_detected",
            ),
            (
                "failed Kamacite",
                failed_kamacite,
                "function_address.kamacite_receipt_failed",
            ),
            (
                "malformed graph",
                malformed_graph,
                "function_address.graph_summary_malformed",
            ),
        ];

        for (name, mutate, expected_code) in cases {
            let mut evidence = evidence();
            mutate(&mut evidence);
            let readiness = evaluate_function_address_evidence(
                &policy(FUNCTION_ADDRESS_MODE_REQUIRED),
                &evidence,
            );
            assert!(!readiness.passed, "{name} should fail");
            assert!(
                readiness
                    .issues
                    .iter()
                    .any(|issue| issue.code == *expected_code),
                "{name} expected {expected_code}: {:?}",
                readiness.issues
            );
        }
    }

    #[test]
    fn readiness_json_includes_visible_boundary() {
        // r[verify cairn.function_address_gates.boundary]
        let readiness = evaluate_function_address_evidence(
            &policy(FUNCTION_ADDRESS_MODE_REQUIRED),
            &evidence(),
        );
        let json = readiness.to_json();
        assert!(json["boundary"]
            .as_str()
            .expect("boundary text")
            .contains("did not prove Rust semantic correctness"));
    }
}
