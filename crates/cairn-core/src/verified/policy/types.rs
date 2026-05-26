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
    pub change_metadata_allowed_groups: Vec<String>,
    pub change_metadata_group_prefixes: Vec<String>,
    pub change_metadata_statuses: Vec<String>,
    pub steel_orchestration_policy: SteelOrchestrationPolicy,
    pub raw: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteelOrchestrationPolicy {
    pub enabled: bool,
    pub profiles: Vec<SteelOrchestrationProfile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteelOrchestrationProfile {
    pub id: String,
    pub mode: String,
    pub allowed_host_functions: Vec<String>,
    pub deterministic_budgets: Vec<String>,
    pub redaction_classes: Vec<String>,
    pub fallback_modes: Vec<String>,
    pub receipt_schema_version: String,
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
