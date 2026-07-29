use serde_json::Value;

use super::{
    fields::{
        change_metadata_fields, exemption_marker_policy, gate_fields, mutation_policy_fields,
        receipt_schemas, require_nonempty_array, task_marker_fields, task_markers, version_field,
    },
    function_address::function_address_policy,
    nominal_identity::nominal_identity_policy,
    runtime_evidence::runtime_evidence_policy,
    schema_compatibility::{policy_schema_metadata, POLICY_SCHEMA_METADATA_FIELD},
    stack_provenance::stack_provenance_policy,
    steel::steel_orchestration_policy,
    traceability::traceability_policy,
    Policy, PolicyError,
};

pub fn parse_policy_json(text: &str) -> Result<Policy, PolicyError> {
    let value: Value =
        serde_json::from_str(text).map_err(|source| PolicyError::Parse { source })?;
    validate_policy_json(value)
}

pub fn validate_policy_json(value: Value) -> Result<Policy, PolicyError> {
    // r[impl cairn.supersede.nickel_contracts] r[impl cairn.supersede.nickel_contracts.boundary]
    // r[impl cairn.supersede.nickel_contracts.exemptions] r[impl cairn.supersede.nickel_contracts.export]
    // r[impl developer_experience.core_policy_parser_boundary_split.modules]
    // r[impl developer_experience.core_policy_parser_boundary_split.behavior]
    let name = super::fields::string_field(&value, "name")?;
    let version = version_field(&value)?;
    require_nonempty_array(&value, "schemas")?;
    let task_marker = task_marker_fields(&value)?;
    let task_marker_tokens = task_markers(&value)?;
    let gate = gate_fields(&value)?;
    if !task_marker_tokens
        .iter()
        .any(|marker| marker.id == gate.substance.dependency_marker_id)
    {
        return Err(PolicyError::InvalidField(
            "gate_policy.substance.dependency_marker_id",
        ));
    }
    if gate.substance.dependency_target_class != "lifecycle-id" {
        return Err(PolicyError::InvalidField(
            "gate_policy.substance.dependency_target_class",
        ));
    }
    let mutation = mutation_policy_fields(&value)?;
    let should_require_receipt_hashes = super::fields::bool_field(
        value
            .get("receipt_hash_policy")
            .ok_or(PolicyError::MissingField("receipt_hash_policy"))?,
        "receipt_hash_policy.required",
    )?;
    let receipt_schemas = receipt_schemas(&value)?;
    let exemption_marker_policy = exemption_marker_policy(&value)?;
    let change_metadata = change_metadata_fields(&value)?;
    let steel_orchestration_policy = steel_orchestration_policy(&value)?;
    let traceability_policy = traceability_policy(&value)?;
    let stack_provenance_policy = stack_provenance_policy(&value)?;
    let function_address_policy = function_address_policy(&value)?;
    let runtime_evidence_policy = runtime_evidence_policy(&value)?;
    let nominal_identity_policy = nominal_identity_policy(&value)?;
    let policy_schema_compatibility = policy_schema_metadata(&value)?
        .ok_or(PolicyError::MissingField(POLICY_SCHEMA_METADATA_FIELD))?;

    Ok(Policy {
        name,
        version,
        task_markers_enabled: task_marker.is_enabled,
        task_marker_strictness: task_marker.strictness,
        task_marker_tokens,
        gate_stages: gate.stages,
        gate_ignore_archives: gate.ignore_archives,
        gate_finding_scope: gate.finding_scope,
        gate_finding_promoted: gate.finding_promoted,
        artifact_substance_policy: gate.substance,
        sync_require_gate: mutation.sync_require_gate,
        archive_require_gate: mutation.archive_require_gate,
        migrate_require_manifest: mutation.migrate_require_manifest,
        require_receipt_hashes: should_require_receipt_hashes,
        receipt_schemas,
        exemption_marker_policy,
        change_metadata_allowed_groups: change_metadata.allowed_groups,
        change_metadata_group_prefixes: change_metadata.group_prefixes,
        change_metadata_statuses: change_metadata.statuses,
        steel_orchestration_policy,
        traceability_policy,
        stack_provenance_policy,
        function_address_policy,
        runtime_evidence_policy,
        nominal_identity_policy,
        policy_schema_compatibility: Some(policy_schema_compatibility),
        raw: value,
    })
}
