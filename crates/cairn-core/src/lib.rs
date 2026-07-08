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
    agent_run_preserves_receipt_hash, agent_run_receipt_hash, agent_run_receipt_issues,
    evaluate_agent_run_completion, positive_agent_run_fixture, AgentRunGateIssue,
    AgentRunGateReport, AGENT_RUN_RECEIPT_SCHEMA_VERSION, NO_MUTATION_PLAN_HASH,
};
pub use verified::ai_spec_draft::{
    ai_spec_draft_hash, validate_ai_spec_draft, AiSpecDraftIssue, AiSpecDraftProvenance,
    AiSpecDraftReport, AI_SPEC_DRAFT_SCHEMA_V1,
};
pub use verified::change_dependencies::{
    build_change_dependency_graph, change_dependency_issue_messages,
    dependency_blockers_for_change, ChangeDependencyEdge, ChangeDependencyGraph,
    ChangeDependencyIssue, ChangeDependencyNode, DEPENDENCY_BLOCKER_UNSATISFIED,
    DEPENDENCY_ISSUE_CYCLE, DEPENDENCY_ISSUE_DUPLICATE, DEPENDENCY_ISSUE_EMPTY_ID,
    DEPENDENCY_ISSUE_INVALID_ID, DEPENDENCY_ISSUE_INVALID_ITEM, DEPENDENCY_ISSUE_INVALID_TYPE,
    DEPENDENCY_ISSUE_SELF, DEPENDENCY_ISSUE_UNKNOWN,
};
pub use verified::content_addressed::{
    circuit_primitive_graph_hash, circuit_primitive_hash_material, downstream_pilot_hash,
    downstream_pilot_hash_material, export_bundle_hash, export_bundle_hash_material,
    function_object_hash, function_object_hash_material, name_pointer_hash,
    name_pointer_hash_material, validate_circuit_primitive_receipt,
    validate_downstream_pilot_receipt, validate_export_bundle_receipt,
    validate_function_object_receipt, validate_name_pointer_receipt, CircuitPrimitiveIssue,
    CircuitPrimitiveReport, DownstreamPilotIssue, DownstreamPilotReport, ExportBundleIssue,
    ExportBundleReport, FunctionObjectIssue, FunctionObjectReport, NamePointerIssue,
    NamePointerReport, CIRCUIT_PRIMITIVE_SCHEMA_V1, DOWNSTREAM_PILOT_SCHEMA_V1,
    EXPORT_BUNDLE_SCHEMA_V1, FUNCTION_OBJECT_SCHEMA_V1, NAME_POINTER_SCHEMA_V1,
};
pub use verified::delta_merge::{merge_delta_spec, DeltaMerge};
pub use verified::developer_experience::{
    onboarding_templates, plan_prompt_scaffold, plan_reverse_spec_discovery,
    plan_template_application, DiscoveryCandidate, DiscoveryPlan, DiscoverySnapshot, FilePlan,
    FilePlanEntry, PlannedFile, ScaffoldPlan, TemplateProfile, DISCOVERY_MAX_BYTES,
    DISCOVERY_MAX_FILES, DISCOVERY_POLICY_ID, SCAFFOLD_POLICY_ID, TEMPLATE_POLICY_ID,
};
pub use verified::doctor::{
    build_doctor_report, DoctorCheck, DoctorCheckStatus, DoctorReport, DoctorSnapshot,
};
pub use verified::evidence::{
    blake3_hex, canonical_json, canonical_json_hash, canonical_preserves_bytes,
    canonical_preserves_hash, preserves_content_hash,
};
pub use verified::kani_receipt::{
    kani_receipt_hash, validate_kani_receipt, KaniReceiptIssue, KaniReceiptProfile,
    KaniReceiptReport, KANI_NON_CLAIM_BOUNDED_ONLY, KANI_NON_CLAIM_FFI_STUB,
    KANI_NON_CLAIM_VERIFIER_SOUNDNESS, KANI_NON_CLAIM_WHOLE_PROGRAM, KANI_RECEIPT_SCHEMA_V1,
};
pub use verified::lifecycle::{
    archive_dir, build_change_skeleton, build_change_skeleton_for, build_ironspec_migration_plan,
    build_openspec_migration_plan, changes_dir, design_template, init_artifacts,
    init_artifacts_for, layout_name, layout_root, proposal_template, specs_dir, tasks_template,
    validate_change_name, ChangeSkeleton, LifecycleArtifact, LifecycleArtifactKind,
    LifecycleLayout, MigrationAction, MigrationInput, MigrationPlan,
};
pub use verified::lifecycle_orchestration::{
    validate_steel_action_envelope, SteelActionIssue, SteelActionValidationReport,
    STEEL_ACTION_ENVELOPE_SCHEMA_V1, STEEL_ORCHESTRATION_RECEIPT_SCHEMA_V1,
};
pub use verified::plan::{
    build_archive_plan, build_archive_plan_with_dependency_blockers, build_sync_plan,
    build_sync_plan_with_dependency_blockers, ArchivePlan, PlanAction, PlanActionKind, SyncPlan,
};
pub use verified::policy::{
    parse_policy_json, validate_policy_json, Policy, PolicyError, SteelOrchestrationPolicy,
    SteelOrchestrationProfile, SUPPORTED_POLICY_VERSION,
};
pub use verified::probe::{
    all_probe_assertions_passed, evaluate_probe_assertions, probe_declaration_hash,
    probe_issue_json, probe_receipt_hash, validate_probe_declaration, validate_probe_receipt,
    ProbeIssue, PROBE_DECLARATION_SCHEMA_V1, PROBE_RECEIPT_SCHEMA_V1,
};
pub use verified::receipt_drift::{receipt_field_drift, DriftClass, DriftEntry, DriftReport};
pub use verified::repo::{
    build_change_detail, build_change_list, build_change_status, build_repo_status,
    build_spec_detail, build_spec_list, count_tasks, filter_changes_by_metadata, spec_title,
    ChangeDetail, ChangeList, ChangeMetadata, ChangeSource, ChangeStatus, RepoStatus, SpecDetail,
    SpecList, SpecSource, SpecSummary, TaskCounts,
};
pub use verified::semantic_review::{
    completeness_receipt, issues_json as semantic_review_issues_json, review_ledger_analytics,
    review_receipt_hash, semantic_review_policy_profile_json,
    validate_review_metrics_evidence_boundary, validate_review_metrics_fixture_pairs,
    validate_review_metrics_guidance_sync, validate_review_metrics_registry,
    validate_review_receipt, validate_revision_handoff, SemanticReviewIssue, SemanticReviewReport,
    REVIEW_ANALYTICS_SCHEMA_V1, REVIEW_CATEGORIES, REVIEW_COMPLETENESS_SCHEMA_V1, REVIEW_DECISIONS,
    REVIEW_KINDS, REVIEW_METRICS_EVIDENCE_BOUNDARY_SCHEMA_V1,
    REVIEW_METRICS_FIXTURE_COVERAGE_SCHEMA_V1, REVIEW_METRICS_GUIDANCE_SYNC_SCHEMA_V1,
    REVIEW_METRICS_REGRESSION_SCHEMA_V1, REVIEW_RECEIPT_SCHEMA_V1, REVIEW_SEVERITIES,
    REVIEW_SOURCES, REVISION_DISPOSITIONS, REVISION_HANDOFF_SCHEMA_V1,
};
pub use verified::structured_lifecycle::{
    structured_lifecycle_bundle_hash, structured_lifecycle_issue_json,
    validate_structured_lifecycle_bundle, StructuredLifecycleIssue, StructuredLifecycleReport,
    STRUCTURED_LIFECYCLE_SCHEMA_V1,
};
pub use verified::validation::{
    change_issues, classify_gate_evidence, evaluate_gate, evaluate_gate_snapshot, validate_changes,
    validate_changes_with_archives, validate_spec_source, validate_spec_sources,
    EvidenceStrengthSummary, GateArtifact, GateIssue, GateReport, GateSnapshot, GateStage,
};
