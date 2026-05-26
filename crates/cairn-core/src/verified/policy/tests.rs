use serde_json::Value;

use super::*;

#[test]
fn parses_generated_default_policy_text() {
    // r[verify developer_experience.core_policy_parser_boundary_split.behavior]
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
