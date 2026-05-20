//! Pure, deterministic core types for cairn.
//!
//! Filesystem access, environment reads, process control, and presentation live
//! in shell crates. Modules under `verified/` are kept pure and are checked by
//! Slotcar's `verified_purity` lint.
//! r[impl cairn.supersede.boundary.pure_core]
//! r[impl cairn.supersede.migrate_openspec.boundary]
//! r[impl cairn.supersede.tracey_coverage.boundary]
//! r[impl cairn.supersede.gate_policy.references]
//! r[impl cairn.supersede.gate_policy.task_links]
//! r[impl cairn.supersede.gate_policy.exemptions]
//! r[impl cairn.supersede.doctor]
//! r[impl cairn.supersede.doctor.boundary]

pub mod verified;

pub use verified::agent_receipt::{
    agent_run_receipt_hash, agent_run_receipt_issues, evaluate_agent_run_completion,
    positive_agent_run_fixture, AgentRunGateIssue, AgentRunGateReport,
    AGENT_RUN_RECEIPT_SCHEMA_VERSION, NO_MUTATION_PLAN_HASH,
};
pub use verified::content_addressed::{
    function_object_hash, function_object_hash_material, name_pointer_hash,
    name_pointer_hash_material, validate_function_object_receipt, validate_name_pointer_receipt,
    FunctionObjectIssue, FunctionObjectReport, NamePointerIssue, NamePointerReport,
    FUNCTION_OBJECT_SCHEMA_V1, NAME_POINTER_SCHEMA_V1,
};
pub use verified::delta_merge::{merge_delta_spec, DeltaMerge};
pub use verified::doctor::{
    build_doctor_report, DoctorCheck, DoctorCheckStatus, DoctorReport, DoctorSnapshot,
};
pub use verified::evidence::{blake3_hex, canonical_json, canonical_json_hash};
pub use verified::lifecycle::{
    archive_dir, build_change_skeleton, build_change_skeleton_for, build_ironspec_migration_plan,
    build_openspec_migration_plan, changes_dir, design_template, init_artifacts,
    init_artifacts_for, layout_name, layout_root, proposal_template, specs_dir, tasks_template,
    validate_change_name, ChangeSkeleton, LifecycleArtifact, LifecycleArtifactKind,
    LifecycleLayout, MigrationAction, MigrationInput, MigrationPlan,
};
pub use verified::plan::{
    build_archive_plan, build_sync_plan, ArchivePlan, PlanAction, PlanActionKind, SyncPlan,
};
pub use verified::policy::{
    parse_policy_json, validate_policy_json, Policy, PolicyError, SUPPORTED_POLICY_VERSION,
};
pub use verified::repo::{
    build_change_detail, build_change_list, build_change_status, build_repo_status,
    build_spec_detail, build_spec_list, count_tasks, spec_title, ChangeDetail, ChangeList,
    ChangeSource, ChangeStatus, RepoStatus, SpecDetail, SpecList, SpecSource, SpecSummary,
    TaskCounts,
};
pub use verified::validation::{
    change_issues, evaluate_gate, evaluate_gate_snapshot, validate_changes, validate_spec_source,
    validate_spec_sources, GateArtifact, GateIssue, GateReport, GateSnapshot, GateStage,
};
