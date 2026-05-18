use std::{error::Error, fmt};

use serde_json::Value;

pub const SUPPORTED_POLICY_VERSION: u64 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub name: String,
    pub version: u64,
    pub task_markers_enabled: bool,
    pub task_marker_strictness: String,
    pub task_marker_tokens: Vec<TaskMarker>,
    pub gate_stages: Vec<String>,
    pub gate_ignore_archives: bool,
    pub gate_finding_scope: String,
    pub gate_finding_promoted: String,
    pub sync_require_gate: bool,
    pub archive_require_gate: bool,
    pub migrate_require_manifest: bool,
    pub require_receipt_hashes: bool,
    pub receipt_schemas: Vec<ReceiptSchema>,
    pub exemption_marker_policy: ExemptionMarkerPolicy,
    pub raw: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskMarker {
    pub id: String,
    pub token: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptSchema {
    pub command: String,
    pub top_level_keys: Vec<String>,
    pub action_keys: Vec<String>,
    pub issue_keys: Vec<String>,
    pub hash_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExemptionMarkerPolicy {
    pub required_fields: Vec<String>,
    pub date_fields: Vec<String>,
    pub optional_fields: Vec<String>,
}

#[derive(Debug)]
pub enum PolicyError {
    Parse { source: serde_json::Error },
    MissingField(&'static str),
    InvalidField(&'static str),
    UnsupportedVersion { found: u64, expected: u64 },
}

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse { source } => write!(f, "failed to parse policy: {source}"),
            Self::MissingField(field) => write!(f, "policy missing field {field}"),
            Self::InvalidField(field) => write!(f, "policy has invalid field {field}"),
            Self::UnsupportedVersion { found, expected } => {
                write!(f, "unsupported policy version {found}; expected {expected}")
            }
        }
    }
}

impl Error for PolicyError {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TaskMarkerFields {
    is_enabled: bool,
    strictness: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GateFields {
    stages: Vec<String>,
    ignore_archives: bool,
    finding_scope: String,
    finding_promoted: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MutationPolicyFields {
    sync_require_gate: bool,
    archive_require_gate: bool,
    migrate_require_manifest: bool,
}

pub fn parse_policy_json(text: &str) -> Result<Policy, PolicyError> {
    let value: Value =
        serde_json::from_str(text).map_err(|source| PolicyError::Parse { source })?;
    validate_policy_json(value)
}

pub fn validate_policy_json(value: Value) -> Result<Policy, PolicyError> {
    // r[impl cairn.supersede.nickel_contracts] r[impl cairn.supersede.nickel_contracts.boundary]
    // r[impl cairn.supersede.nickel_contracts.exemptions] r[impl cairn.supersede.nickel_contracts.export]
    let name = string_field(&value, "name")?;
    let version = version_field(&value)?;
    require_nonempty_array(&value, "schemas")?;
    let task_marker = task_marker_fields(&value)?;
    let task_marker_tokens = task_markers(&value)?;
    let gate = gate_fields(&value)?;
    let mutation = mutation_policy_fields(&value)?;
    let should_require_receipt_hashes = bool_field(
        value
            .get("receipt_hash_policy")
            .ok_or(PolicyError::MissingField("receipt_hash_policy"))?,
        "receipt_hash_policy.required",
    )?;
    let receipt_schemas = receipt_schemas(&value)?;
    let exemption_marker_policy = exemption_marker_policy(&value)?;

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
        sync_require_gate: mutation.sync_require_gate,
        archive_require_gate: mutation.archive_require_gate,
        migrate_require_manifest: mutation.migrate_require_manifest,
        require_receipt_hashes: should_require_receipt_hashes,
        receipt_schemas,
        exemption_marker_policy,
        raw: value,
    })
}

fn version_field(value: &Value) -> Result<u64, PolicyError> {
    let version = value
        .get("version")
        .and_then(Value::as_u64)
        .ok_or(PolicyError::MissingField("version"))?;
    if version != SUPPORTED_POLICY_VERSION {
        return Err(PolicyError::UnsupportedVersion {
            found: version,
            expected: SUPPORTED_POLICY_VERSION,
        });
    }
    Ok(version)
}

fn require_nonempty_array(value: &Value, field: &'static str) -> Result<(), PolicyError> {
    let values = value
        .get(field)
        .and_then(Value::as_array)
        .ok_or(PolicyError::MissingField(field))?;
    if values.is_empty() {
        return Err(PolicyError::InvalidField(field));
    }
    Ok(())
}

fn task_marker_fields(value: &Value) -> Result<TaskMarkerFields, PolicyError> {
    let marker_policy = value
        .get("task_marker_policy")
        .ok_or(PolicyError::MissingField("task_marker_policy"))?;
    Ok(TaskMarkerFields {
        is_enabled: marker_policy
            .get("enabled")
            .and_then(Value::as_bool)
            .ok_or(PolicyError::MissingField("task_marker_policy.enabled"))?,
        strictness: string_field(marker_policy, "task_marker_policy.strictness")?,
    })
}

fn task_markers(value: &Value) -> Result<Vec<TaskMarker>, PolicyError> {
    let marker_policy = value
        .get("task_marker_policy")
        .ok_or(PolicyError::MissingField("task_marker_policy"))?;
    let markers = marker_policy
        .get("markers")
        .and_then(Value::as_array)
        .ok_or(PolicyError::MissingField("task_marker_policy.markers"))?;
    if markers.is_empty() {
        return Err(PolicyError::InvalidField("task_marker_policy.markers"));
    }
    markers
        .iter()
        .map(|marker| {
            Ok(TaskMarker {
                id: string_field(marker, "task_marker_policy.markers.id")?,
                token: string_field(marker, "task_marker_policy.markers.token")?,
                kind: string_field(marker, "task_marker_policy.markers.kind")?,
            })
        })
        .collect()
}

fn gate_fields(value: &Value) -> Result<GateFields, PolicyError> {
    let gate_policy = value
        .get("gate_policy")
        .ok_or(PolicyError::MissingField("gate_policy"))?;
    let stages = gate_stages(gate_policy)?;
    let is_gate_ignore_archives = gate_policy
        .get("ignore_archives")
        .and_then(Value::as_bool)
        .ok_or(PolicyError::MissingField("gate_policy.ignore_archives"))?;
    let gate_finding = gate_policy
        .get("finding")
        .ok_or(PolicyError::MissingField("gate_policy.finding"))?;
    Ok(GateFields {
        stages,
        ignore_archives: is_gate_ignore_archives,
        finding_scope: string_field(gate_finding, "gate_policy.finding.scope")?,
        finding_promoted: string_field(gate_finding, "gate_policy.finding.promoted")?,
    })
}

fn mutation_policy_fields(value: &Value) -> Result<MutationPolicyFields, PolicyError> {
    let mutation = value
        .get("mutation_policy")
        .ok_or(PolicyError::MissingField("mutation_policy"))?;
    let sync = mutation
        .get("sync")
        .ok_or(PolicyError::MissingField("mutation_policy.sync"))?;
    let archive = mutation
        .get("archive")
        .ok_or(PolicyError::MissingField("mutation_policy.archive"))?;
    let migrate = mutation
        .get("migrate")
        .ok_or(PolicyError::MissingField("mutation_policy.migrate"))?;
    Ok(MutationPolicyFields {
        sync_require_gate: bool_field(sync, "mutation_policy.sync.require_passing_gate")?,
        archive_require_gate: bool_field(archive, "mutation_policy.archive.require_passing_gate")?,
        migrate_require_manifest: bool_field(migrate, "mutation_policy.migrate.require_manifest")?,
    })
}

fn bool_field(value: &Value, field: &'static str) -> Result<bool, PolicyError> {
    let lookup = field.rsplit('.').next().unwrap_or(field);
    value
        .get(lookup)
        .and_then(Value::as_bool)
        .ok_or(PolicyError::MissingField(field))
}

fn string_array_field(value: &Value, field: &'static str) -> Result<Vec<String>, PolicyError> {
    let lookup = field.rsplit('.').next().unwrap_or(field);
    value
        .get(lookup)
        .and_then(Value::as_array)
        .ok_or(PolicyError::MissingField(field))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .ok_or(PolicyError::InvalidField(field))
        })
        .collect()
}

fn receipt_schemas(value: &Value) -> Result<Vec<ReceiptSchema>, PolicyError> {
    let schemas = value
        .get("receipt_schemas")
        .and_then(Value::as_array)
        .ok_or(PolicyError::MissingField("receipt_schemas"))?;
    if schemas.is_empty() {
        return Err(PolicyError::InvalidField("receipt_schemas"));
    }
    schemas
        .iter()
        .map(|schema| {
            Ok(ReceiptSchema {
                command: string_field(schema, "receipt_schemas.command")?,
                top_level_keys: string_array_field(schema, "receipt_schemas.top_level_keys")?,
                action_keys: string_array_field(schema, "receipt_schemas.action_keys")?,
                issue_keys: string_array_field(schema, "receipt_schemas.issue_keys")?,
                hash_fields: string_array_field(schema, "receipt_schemas.hash_fields")?,
            })
        })
        .collect()
}

fn exemption_marker_policy(value: &Value) -> Result<ExemptionMarkerPolicy, PolicyError> {
    let marker_policy = value
        .get("exemption_marker_policy")
        .ok_or(PolicyError::MissingField("exemption_marker_policy"))?;
    Ok(ExemptionMarkerPolicy {
        required_fields: string_array_field(
            marker_policy,
            "exemption_marker_policy.required_fields",
        )?,
        date_fields: string_array_field(marker_policy, "exemption_marker_policy.date_fields")?,
        optional_fields: string_array_field(
            marker_policy,
            "exemption_marker_policy.optional_fields",
        )?,
    })
}

fn gate_stages(gate_policy: &Value) -> Result<Vec<String>, PolicyError> {
    let stages = gate_policy
        .get("stages")
        .and_then(Value::as_array)
        .ok_or(PolicyError::MissingField("gate_policy.stages"))?
        .iter()
        .map(|stage| {
            stage
                .as_str()
                .map(str::to_string)
                .ok_or(PolicyError::InvalidField("gate_policy.stages"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if stages.is_empty() {
        return Err(PolicyError::InvalidField("gate_policy.stages"));
    }
    Ok(stages)
}

fn string_field(value: &Value, field: &'static str) -> Result<String, PolicyError> {
    let lookup = field.rsplit('.').next().unwrap_or(field);
    value
        .get(lookup)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or(PolicyError::MissingField(field))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_generated_default_policy_text() {
        let text = include_str!("../../../../cairn-policy/generated/cairn-policy.json");
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
    }

    #[test]
    fn rejects_unsupported_version() {
        let value = serde_json::json!({
            "name": "bad",
            "version": SUPPORTED_POLICY_VERSION + 1,
            "schemas": [{}],
            "task_marker_policy": { "enabled": true, "strictness": "strict" },
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
}
