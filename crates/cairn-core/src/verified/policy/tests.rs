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
    let metadata = policy
        .policy_schema_compatibility
        .expect("generated policy includes schema compatibility metadata");
    assert_eq!(metadata.schema_version, CURRENT_POLICY_SCHEMA_VERSION);
    assert_eq!(metadata.field_set_digest, CURRENT_POLICY_FIELD_SET_DIGEST);
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
fn rejects_unknown_runtime_capture_origin() {
    // r[verify cairn.runtime_evidence.policy_schema]
    // r[verify cairn.runtime_evidence.policy_schema.public]
    // r[verify cairn.runtime_evidence.policy_schema.unknown]
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .expect("generated policy json");
    value["runtime_evidence_policy"]["allowed_capture_origins"] =
        serde_json::json!(["boundary_captured", "ambient"]);

    assert!(matches!(
        validate_policy_json(value),
        Err(PolicyError::InvalidField(
            "runtime_evidence_policy.allowed_capture_origins"
        ))
    ));
}

#[test]
fn rejects_zero_runtime_summary_limit() {
    // r[verify cairn.runtime_evidence.policy_schema]
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .expect("generated policy json");
    value["runtime_evidence_policy"]["max_summary_bytes"] = Value::from(0);

    assert!(matches!(
        validate_policy_json(value),
        Err(PolicyError::InvalidField(
            "runtime_evidence_policy.max_summary_bytes"
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

#[test]
fn classifies_current_generated_policy_metadata() {
    // r[verify cairn.policy_schema_compatibility.fixtures.positive]
    // r[verify cairn.policy_schema_compatibility.classifier]
    // r[verify cairn.policy_schema_compatibility.validation]
    let value = generated_policy_value();
    let report = classify_policy_schema_compatibility(&value, None);
    assert_eq!(report.status, "current");
    assert_eq!(
        report.schema_version.as_deref(),
        Some(CURRENT_POLICY_SCHEMA_VERSION)
    );
    assert_eq!(
        report.field_set_digest.as_deref(),
        Some(CURRENT_POLICY_FIELD_SET_DIGEST)
    );
}

#[test]
fn classifies_missing_metadata_as_refresh_required() {
    // r[verify cairn.policy_schema_compatibility.fixtures.negative]
    // r[verify cairn.policy_schema_compatibility.validation.fixtures]
    let mut value = generated_policy_value();
    value
        .as_object_mut()
        .unwrap()
        .remove(POLICY_SCHEMA_METADATA_FIELD);
    let error = validate_policy_json(value.clone()).expect_err("missing metadata fails closed");
    let report = classify_policy_schema_compatibility(&value, Some(&error));
    assert_eq!(report.status, "refresh-required");
    assert_eq!(
        report.missing_field.as_deref(),
        Some(POLICY_SCHEMA_METADATA_FIELD)
    );
}

#[test]
fn classifies_missing_new_required_field_as_refresh_required() {
    // r[verify cairn.policy_schema_compatibility.fixtures.negative]
    // r[verify cairn.policy_schema_compatibility.diagnostics]
    let mut value = generated_policy_value();
    value
        .as_object_mut()
        .unwrap()
        .remove("steel_orchestration_policy");
    let error = validate_policy_json(value.clone()).expect_err("missing new field fails");
    let report = classify_policy_schema_compatibility(&value, Some(&error));
    assert_eq!(report.status, "refresh-required");
    assert_eq!(
        report.missing_field.as_deref(),
        Some("steel_orchestration_policy")
    );
    let diagnostic = policy_schema_compatibility_diagnostic(&value, &error)
        .expect("refresh diagnostic is available");
    assert!(diagnostic.contains("policy-refresh-required"));
    assert!(diagnostic.contains("missing_field=steel_orchestration_policy"));
}

#[test]
fn rejects_unsupported_policy_schema_metadata() {
    // r[verify cairn.policy_schema_compatibility.classifier.unsupported]
    // r[verify cairn.policy_schema_compatibility.fixtures.negative]
    let mut value = generated_policy_value();
    value[POLICY_SCHEMA_METADATA_FIELD]["schema_version"] =
        Value::String("cairn.policy-schema.v999".to_string());
    let error = validate_policy_json(value.clone()).expect_err("unsupported schema fails");
    assert!(matches!(error, PolicyError::UnsupportedPolicySchema { .. }));
    let report = classify_policy_schema_compatibility(&value, Some(&error));
    assert_eq!(report.status, "unsupported");
}

#[test]
fn rejects_malformed_policy_schema_metadata() {
    // r[verify cairn.policy_schema_compatibility.fixtures.negative]
    // r[verify cairn.policy_schema_compatibility.validation.fixtures]
    let mut value = generated_policy_value();
    value[POLICY_SCHEMA_METADATA_FIELD]["field_set_digest"] =
        Value::String("not-a-hash".to_string());
    let error = validate_policy_json(value.clone()).expect_err("malformed metadata fails");
    assert!(matches!(
        error,
        PolicyError::InvalidField("policy_schema_compatibility.field_set_digest")
    ));
    let report = classify_policy_schema_compatibility(&value, Some(&error));
    assert_eq!(report.status, "unsupported");
}

fn generated_policy_value() -> Value {
    serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .expect("generated policy json")
}
