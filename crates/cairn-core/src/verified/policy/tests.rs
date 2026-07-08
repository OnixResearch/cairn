use serde_json::Value;

use super::*;

#[test]
fn parses_generated_default_policy_text() {
    // r[verify developer_experience.core_policy_parser_boundary_split.behavior]
    // r[verify cairn.supersede.traceability_ownership.configured_profiles.cairn_default]
    let text = include_str!("../../../../../cairn-policy/generated/cairn-policy.json");
    let policy = parse_policy_json(text).expect("generated policy parses");
    assert_eq!(policy.name, "cairn-default");
    assert_eq!(policy.version, SUPPORTED_POLICY_VERSION);
    assert!(policy.task_markers_enabled);
    assert_eq!(policy.gate_stages, vec!["proposal", "design", "tasks"]);
    assert!(policy.gate_ignore_archives);
    assert_eq!(policy.gate_finding_scope, "tasks");
    assert_eq!(policy.gate_finding_promoted, "no");
    assert!(policy.sync_require_gate);
    assert!(policy.archive_require_gate);
    assert!(policy.migrate_require_manifest);
    assert!(policy.require_receipt_hashes);
    assert!(policy
        .task_marker_tokens
        .iter()
        .any(|marker| marker.token == "[serial]"));
    assert!(policy
        .receipt_schemas
        .iter()
        .any(|schema| schema.command == "gate"
            && schema.hash_fields.contains(&"receipt_hash".to_string())));
    assert_eq!(
        policy.exemption_marker_policy.required_fields,
        vec!["reason", "owner", "until"]
    );
    assert!(!policy.steel_orchestration_policy.enabled);
    assert_eq!(policy.steel_orchestration_policy.profiles.len(), 1);
    assert_eq!(
        policy.steel_orchestration_policy.profiles[0].mode,
        "disabled"
    );
    assert_eq!(policy.traceability_policy.default_profile, "cairn-default");
    assert!(policy
        .traceability_policy
        .profiles
        .iter()
        .any(|profile| profile.id == "trellis-rust"));
    assert_eq!(policy.stack_provenance_policy.mode, "optional");
    assert!(policy
        .stack_provenance_policy
        .allowed_claim_scopes
        .contains(&"stack_provenance.v1.lifecycle_policy_conformance".to_string()));
}

#[test]
fn policy_module_split_preserves_public_api_exports() {
    // r[verify developer_experience.core_policy_parser_boundary_split.map]
    // r[verify developer_experience.core_policy_parser_boundary_split.modules]
    let _policy: Option<Policy> = None;
    let _task_marker: Option<TaskMarker> = None;
    let _receipt_schema: Option<ReceiptSchema> = None;
    let _exemption_policy: Option<ExemptionMarkerPolicy> = None;
    let _steel_policy: Option<SteelOrchestrationPolicy> = None;
    let _steel_profile: Option<SteelOrchestrationProfile> = None;
    let _traceability_policy: Option<TraceabilityPolicy> = None;
    let _stack_provenance_policy: Option<crate::verified::stack_provenance::StackProvenancePolicy> =
        None;
    assert_eq!(SUPPORTED_POLICY_VERSION, 1);
}

#[test]
fn rejects_enabled_steel_policy_without_profiles() {
    // r[verify developer_experience.core_policy_parser_boundary_split.tests]
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .expect("generated policy json");
    value["steel_orchestration_policy"] = serde_json::json!({
        "enabled": true,
        "profiles": []
    });

    assert!(matches!(
        validate_policy_json(value),
        Err(PolicyError::InvalidField(
            "steel_orchestration_policy.profiles"
        ))
    ));
}

#[test]
fn rejects_unknown_steel_profile_mode() {
    // r[verify developer_experience.core_policy_parser_boundary_split.tests]
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .expect("generated policy json");
    value["steel_orchestration_policy"]["profiles"][0]["mode"] =
        Value::String("ambient".to_string());

    assert!(matches!(
        validate_policy_json(value),
        Err(PolicyError::InvalidField(
            "steel_orchestration_policy.profiles.mode"
        ))
    ));
}

#[test]
fn rejects_empty_traceability_profile_id() {
    // r[verify cairn.supersede.traceability_ownership.configured_profiles]
    // r[verify cairn.supersede.traceability_ownership.configured_profiles.malformed]
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .expect("generated policy json");
    value["traceability_policy"]["profiles"][0]["id"] = Value::String(String::new());

    assert!(matches!(
        validate_policy_json(value),
        Err(PolicyError::InvalidField("traceability_policy.profiles.id"))
    ));
}

#[test]
fn rejects_unsupported_traceability_marker_verb() {
    // r[verify cairn.supersede.traceability_ownership.configured_profiles]
    // r[verify cairn.supersede.traceability_ownership.configured_profiles.malformed]
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .expect("generated policy json");
    value["traceability_policy"]["profiles"][0]["marker_verbs"] =
        serde_json::json!(["impl", "claims"]);

    assert!(matches!(
        validate_policy_json(value),
        Err(PolicyError::InvalidField(
            "traceability_policy.profiles.marker_verbs"
        ))
    ));
}

#[test]
fn rejects_unsafe_traceability_source_path() {
    // r[verify cairn.supersede.traceability_ownership.configured_profiles]
    // r[verify cairn.supersede.traceability_ownership.configured_profiles.malformed]
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .expect("generated policy json");
    value["traceability_policy"]["profiles"][0]["requirement_sources"][0]["root"] =
        Value::String("../requirements".to_string());

    assert!(matches!(
        validate_policy_json(value),
        Err(PolicyError::InvalidField(
            "traceability_policy.profiles.sources.root"
        ))
    ));
}

#[test]
fn rejects_unsupported_traceability_source_kind() {
    // r[verify cairn.supersede.traceability_ownership.configured_profiles]
    // r[verify cairn.supersede.traceability_ownership.configured_profiles.malformed]
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .expect("generated policy json");
    value["traceability_policy"]["profiles"][0]["evidence_sources"][0]["source_kind"] =
        Value::String("ambient_source".to_string());

    assert!(matches!(
        validate_policy_json(value),
        Err(PolicyError::InvalidField(
            "traceability_policy.profiles.sources.source_kind"
        ))
    ));
}

#[test]
fn rejects_unknown_stack_provenance_mode() {
    // r[verify cairn.stack_provenance_gates.policy]
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .expect("generated policy json");
    value["stack_provenance_gate"]["mode"] = Value::String("ambient".to_string());

    assert!(matches!(
        validate_policy_json(value),
        Err(PolicyError::InvalidField("stack_provenance_gate.mode"))
    ));
}

#[test]
fn rejects_stale_stack_provenance_policy_hash_shape() {
    // r[verify cairn.stack_provenance_gates.policy]
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .expect("generated policy json");
    value["stack_provenance_gate"]["accepted_valence_policy_hashes"] =
        serde_json::json!(["not-a-hash"]);

    assert!(matches!(
        validate_policy_json(value),
        Err(PolicyError::InvalidField(
            "stack_provenance_gate.accepted_valence_policy_hashes"
        ))
    ));
}

#[test]
fn rejects_unsupported_version() {
    // r[verify developer_experience.core_policy_parser_boundary_split.validation]
    let value = serde_json::json!({
        "name": "bad",
        "version": SUPPORTED_POLICY_VERSION + 1,
        "schemas": [{}],
        "task_marker_policy": { "enabled": true, "strictness": "strict" },
        "change_metadata_policy": {
            "allowed_groups": ["current", "future"],
            "group_prefixes": ["feature:"],
            "statuses": ["current", "planned", "blocked", "deferred"]
        },
        "gate_policy": {
            "stages": ["tasks"],
            "ignore_archives": true,
            "finding": { "scope": "tasks", "promoted": "no" }
        },
        "mutation_policy": {
            "sync": { "require_passing_gate": true },
            "archive": { "require_passing_gate": true },
            "migrate": { "require_manifest": true }
        },
        "receipt_hash_policy": { "required": true }
    });
    assert!(matches!(
        validate_policy_json(value),
        Err(PolicyError::UnsupportedVersion { .. })
    ));
}
