use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

pub const WORKFLOW_SCHEMA_VERSION_V1: &str = "cairn.workflow-schema.v1";
pub const WORKFLOW_STATUS_SCHEMA_V1: &str = "cairn.workflow-status.v1";
pub const WORKFLOW_INSTRUCTIONS_SCHEMA_V1: &str = "cairn.workflow-instructions.v1";

pub const WORKFLOW_STATE_DONE: &str = "done";
pub const WORKFLOW_STATE_READY: &str = "ready";
pub const WORKFLOW_STATE_BLOCKED: &str = "blocked";
pub const WORKFLOW_STATE_SKIPPED: &str = "skipped";
pub const WORKFLOW_SUPPORTED_STATES: [&str; 4] = [
    WORKFLOW_STATE_DONE,
    WORKFLOW_STATE_READY,
    WORKFLOW_STATE_BLOCKED,
    WORKFLOW_STATE_SKIPPED,
];

pub const WORKFLOW_ADMISSION_PROPOSAL: &str = "proposal";
pub const WORKFLOW_ADMISSION_DELTA_SPECS: &str = "delta_specs";
pub const WORKFLOW_ADMISSION_DESIGN: &str = "design";
pub const WORKFLOW_ADMISSION_TASKS: &str = "tasks";
pub const WORKFLOW_ADMISSION_MARKDOWN: &str = "markdown";

pub const WORKFLOW_PATH_KIND_FILE: &str = "file";
pub const WORKFLOW_PATH_KIND_SPEC_DIRECTORY: &str = "spec_directory";

pub const WORKFLOW_ISSUE_POLICY_SHAPE: &str = "cairn.workflow.policy.shape";
pub const WORKFLOW_ISSUE_POLICY_SCHEMA_VERSION: &str = "cairn.workflow.policy.schema_version";
pub const WORKFLOW_ISSUE_POLICY_DUPLICATE_SCHEMA: &str = "cairn.workflow.policy.duplicate_schema";
pub const WORKFLOW_ISSUE_POLICY_BOUNDS: &str = "cairn.workflow.policy.bounds";
pub const WORKFLOW_ISSUE_POLICY_STATES: &str = "cairn.workflow.policy.states";
pub const WORKFLOW_ISSUE_POLICY_DUPLICATE_ARTIFACT: &str =
    "cairn.workflow.policy.duplicate_artifact";
pub const WORKFLOW_ISSUE_POLICY_UNKNOWN_DEPENDENCY: &str =
    "cairn.workflow.policy.unknown_dependency";
pub const WORKFLOW_ISSUE_POLICY_DUPLICATE_DEPENDENCY: &str =
    "cairn.workflow.policy.duplicate_dependency";
pub const WORKFLOW_ISSUE_POLICY_SELF_DEPENDENCY: &str = "cairn.workflow.policy.self_dependency";
pub const WORKFLOW_ISSUE_POLICY_CYCLE: &str = "cairn.workflow.policy.cycle";
pub const WORKFLOW_ISSUE_POLICY_UNSAFE_PATH: &str = "cairn.workflow.policy.unsafe_path";
pub const WORKFLOW_ISSUE_POLICY_TEMPLATE: &str = "cairn.workflow.policy.template";
pub const WORKFLOW_ISSUE_POLICY_STAGE: &str = "cairn.workflow.policy.stage";
pub const WORKFLOW_ISSUE_POLICY_PATH_KIND: &str = "cairn.workflow.policy.path_kind";
pub const WORKFLOW_ISSUE_POLICY_ADMISSION: &str = "cairn.workflow.policy.admission";
pub const WORKFLOW_ISSUE_POLICY_PREREQUISITE: &str = "cairn.workflow.policy.prerequisite";
pub const WORKFLOW_ISSUE_OBSERVATION_UNKNOWN: &str = "cairn.workflow.observation.unknown";
pub const WORKFLOW_ISSUE_OBSERVATION_DUPLICATE: &str = "cairn.workflow.observation.duplicate";
pub const WORKFLOW_ISSUE_OBSERVATION_INCONSISTENT: &str = "cairn.workflow.observation.inconsistent";
pub const WORKFLOW_ISSUE_ARTIFACT_INVALID: &str = "cairn.workflow.artifact.invalid";
pub const WORKFLOW_ISSUE_DEPENDENCY_BLOCKED: &str = "cairn.workflow.artifact.dependency_blocked";
pub const WORKFLOW_ISSUE_ARTIFACT_UNKNOWN: &str = "cairn.workflow.artifact_unknown";

pub const MAX_WORKFLOW_SCHEMAS_LIMIT: usize = 32;
pub const MAX_WORKFLOW_ARTIFACTS_LIMIT: usize = 64;
pub const MAX_WORKFLOW_DEPENDENCIES_LIMIT: usize = 32;
pub const MAX_WORKFLOW_FILES_LIMIT: usize = 512;
pub const MAX_WORKFLOW_PATH_BYTES_LIMIT: usize = 4096;
pub const MAX_WORKFLOW_ARTIFACT_BYTES_LIMIT: usize = 16 * 1024 * 1024;
pub const MAX_WORKFLOW_TEMPLATE_BYTES_LIMIT: usize = 16 * 1024;
pub const MAX_WORKFLOW_CONSTRAINTS_LIMIT: usize = 64;
pub const MAX_WORKFLOW_NON_CLAIMS_LIMIT: usize = 64;
pub const MAX_WORKFLOW_ISSUES_LIMIT: usize = 512;
pub const MAX_WORKFLOW_DIRECTORY_DEPTH_LIMIT: usize = 16;
pub const MAX_WORKFLOW_DIRECTORY_ENTRIES_LIMIT: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowBounds {
    pub max_artifacts: usize,
    pub max_dependencies: usize,
    pub max_files_per_artifact: usize,
    pub max_path_bytes: usize,
    pub max_artifact_bytes: usize,
    pub max_template_bytes: usize,
    pub max_constraints: usize,
    pub max_non_claims: usize,
    pub max_issues: usize,
    pub max_directory_depth: usize,
    pub max_directory_entries: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowArtifactPolicy {
    pub id: String,
    pub generates: String,
    pub output_path: String,
    pub path_kind: String,
    pub structural_admission: String,
    #[serde(default)]
    pub requires: Vec<String>,
    pub stage: String,
    pub instruction_template: String,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub apply_prerequisites: Vec<String>,
    #[serde(default)]
    pub skipped: bool,
    #[serde(default)]
    pub skip_satisfies_dependencies: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowSchemaPolicy {
    pub schema_version: String,
    pub id: String,
    pub description: String,
    pub states: Vec<String>,
    pub bounds: WorkflowBounds,
    pub artifacts: Vec<WorkflowArtifactPolicy>,
    pub required_non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowArtifactFile {
    pub path: String,
    pub blake3: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowArtifactObservation {
    pub artifact_id: String,
    pub present: bool,
    pub structurally_valid: bool,
    pub files: Vec<WorkflowArtifactFile>,
    pub issue_codes: Vec<String>,
}

impl WorkflowArtifactObservation {
    #[must_use]
    pub fn absent(artifact_id: impl Into<String>) -> Self {
        Self {
            artifact_id: artifact_id.into(),
            present: false,
            structurally_valid: false,
            files: Vec::new(),
            issue_codes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowArtifactState {
    Done,
    Ready,
    Blocked,
    Skipped,
}

impl WorkflowArtifactState {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Done => WORKFLOW_STATE_DONE,
            Self::Ready => WORKFLOW_STATE_READY,
            Self::Blocked => WORKFLOW_STATE_BLOCKED,
            Self::Skipped => WORKFLOW_STATE_SKIPPED,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowArtifactStatus {
    pub id: String,
    pub state: WorkflowArtifactState,
    pub output_path: String,
    pub path_kind: String,
    pub structural_admission: String,
    pub stage: String,
    pub requires: Vec<String>,
    pub missing: Vec<String>,
    pub unlocks: Vec<String>,
    pub blockers: Vec<String>,
    pub files: Vec<WorkflowArtifactFile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowStatusReport {
    pub schema_version: String,
    pub schema_id: String,
    pub schema_description: String,
    pub valid: bool,
    pub artifacts: Vec<WorkflowArtifactStatus>,
    pub next_actions: Vec<String>,
    pub issues: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowUpstreamArtifact {
    pub artifact_id: String,
    pub state: WorkflowArtifactState,
    pub files: Vec<WorkflowArtifactFile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowInstructionEnvelope {
    pub schema_version: String,
    pub schema_id: String,
    pub artifact_id: String,
    pub state: WorkflowArtifactState,
    pub output_path: String,
    pub instruction_template: String,
    pub constraints: Vec<String>,
    pub requires: Vec<String>,
    pub apply_prerequisites: Vec<String>,
    pub blockers: Vec<String>,
    pub upstream: Vec<WorkflowUpstreamArtifact>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowAdmissionError {
    pub code: &'static str,
    pub path: String,
    pub message: String,
}

impl WorkflowAdmissionError {
    #[must_use]
    pub fn new(code: &'static str, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            path: path.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for WorkflowAdmissionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} at {}: {}",
            self.code, self.path, self.message
        )
    }
}

impl Error for WorkflowAdmissionError {}
