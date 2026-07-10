//! Pure, deterministic core types for cairn.
//!
//! Filesystem access, environment reads, process control, and presentation live
//! in shell crates. Modules under `verified/` are kept pure and are checked by
//! Slotcar's `verified_purity` lint.
//! r[impl cairn.supersede.boundary.pure_core]
//! r[impl cairn.supersede.migrate_openspec.boundary]
//! r[impl cairn.supersede.tracey_coverage.boundary]
//! r[impl cairn.supersede.traceability_ownership.authoritative_checker]
//! r[impl cairn.supersede.traceability_ownership.configured_profiles]
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
pub use verified::ast_grep_gate::{
    ast_grep_receipt_hash, ast_grep_receipt_hash_material, default_ast_grep_gate_policy,
    is_ast_grep_gate_mode, validate_ast_grep_receipt, AstGrepGateIssue, AstGrepGatePolicy,
    AstGrepGateReport, AST_GREP_GATE_BOUNDARY, AST_GREP_GATE_ISSUE_BLOCKING_FINDING,
    AST_GREP_GATE_ISSUE_CODEMOD_REVIEW, AST_GREP_GATE_ISSUE_MALFORMED_FINDING,
    AST_GREP_GATE_ISSUE_MALFORMED_HASH, AST_GREP_GATE_ISSUE_MISSING_FIELD,
    AST_GREP_GATE_ISSUE_OVERCLAIM, AST_GREP_GATE_ISSUE_STALE_SCOPE,
    AST_GREP_GATE_ISSUE_UNSUPPORTED_MODE, AST_GREP_GATE_ISSUE_UNSUPPORTED_OUTPUT_FORMAT,
    AST_GREP_GATE_ISSUE_WEAK_NON_CLAIM, AST_GREP_GATE_MODE_ADVISORY, AST_GREP_GATE_MODE_BLOCKING,
    AST_GREP_GATE_MODE_CODEMOD_REVIEW, AST_GREP_GATE_MODE_INVENTORY,
};
pub use verified::change_dependencies::{
    build_change_dependency_graph, change_dependency_issue_messages,
    change_metadata_with_dependencies, dependency_blockers_for_change, next_ready_change,
    normalized_change_dependencies, plan_change_dependency_edit,
    render_change_dependency_graph_dot, render_change_dependency_graph_mermaid,
    render_change_metadata_json, ChangeDependencyBlockedChange, ChangeDependencyEdge,
    ChangeDependencyEditAction, ChangeDependencyEditPlan, ChangeDependencyGraph,
    ChangeDependencyIssue, ChangeDependencyNextReady, ChangeDependencyNode,
    DEPENDENCY_BLOCKER_UNSATISFIED, DEPENDENCY_EDIT_ISSUE_CHANGE_UNKNOWN,
    DEPENDENCY_EDIT_ISSUE_METADATA_INVALID, DEPENDENCY_EDIT_ISSUE_NOT_DECLARED,
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
pub use verified::formal_proof_readiness_fixture::{
    validate_formal_proof_readiness_fixture, FormalProofReadinessFixture,
    FormalProofReadinessIssue, FormalProofReadinessReport, FORMAL_PROOF_READINESS_BOUNDARY,
};
pub use verified::function_address_gates::{
    evaluate_function_address_evidence, function_address_digest_rules, is_function_address_mode,
    normalize_function_address_evidence_digests, FunctionAddressDigestNormalization,
    FunctionAddressDigestRule, FunctionAddressEvidence, FunctionAddressIssue,
    FunctionAddressPolicy, FunctionAddressReadiness, FUNCTION_ADDRESS_BOUNDARY,
    FUNCTION_ADDRESS_CLAIM_SCOPE_LIFECYCLE, FUNCTION_ADDRESS_DIGEST_ISSUE_CASE_UNSUPPORTED,
    FUNCTION_ADDRESS_DIGEST_ISSUE_MALFORMED, FUNCTION_ADDRESS_DIGEST_ISSUE_PREFIX_UNSUPPORTED,
    FUNCTION_ADDRESS_DIGEST_SPELLING_B3_PREFIX, FUNCTION_ADDRESS_DIGEST_SPELLING_BARE,
    FUNCTION_ADDRESS_MODE_DISABLED, FUNCTION_ADDRESS_MODE_OPTIONAL, FUNCTION_ADDRESS_MODE_REQUIRED,
};
pub use verified::hash_algorithm_policy::{
    hash_algorithm_policy_issues_json, validate_hash_algorithm_policy,
    validate_hash_algorithm_policy_value, HashAlgorithmException, HashAlgorithmPolicy,
    HashAlgorithmPolicyIssue, HashAlgorithmPolicyReport, HashFieldObservation,
    HASH_ALGORITHM_POLICY_BOUNDARY, HASH_ALGORITHM_POLICY_SCHEMA_V1,
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
pub use verified::lifecycle_receipt::{
    adapt_cairn_gate_receipt, adapt_cairn_sync_archive_receipt, adapt_cairn_validate_receipt,
    adapt_downstream_lifecycle_receipt, adapt_lifecycle_receipt_value,
    default_lifecycle_receipt_hash_profile, default_lifecycle_receipt_non_claims,
    lifecycle_receipt_envelope_from_value, lifecycle_receipt_hash, lifecycle_receipt_hash_material,
    validate_lifecycle_receipt_envelope, LifecycleReceiptCommandIdentity,
    LifecycleReceiptDiagnostic, LifecycleReceiptEnvelope, LifecycleReceiptEvidenceSummary,
    LifecycleReceiptHashField, LifecycleReceiptHashProfile, LifecycleReceiptIssue,
    LifecycleReceiptValidationInput, LifecycleReceiptValidationReport,
    LIFECYCLE_RECEIPT_CORE_BOUNDARY, LIFECYCLE_RECEIPT_CORE_SCHEMA_V1,
    LIFECYCLE_RECEIPT_HASH_FIELD_INPUT_HASH, LIFECYCLE_RECEIPT_HASH_FIELD_POLICY_HASH,
    LIFECYCLE_RECEIPT_HASH_FIELD_RECEIPT_HASH, LIFECYCLE_RECEIPT_HASH_PROFILE_CORE_V1,
    LIFECYCLE_RECEIPT_ISSUE_HASH_PROFILE, LIFECYCLE_RECEIPT_ISSUE_MALFORMED_HASH,
    LIFECYCLE_RECEIPT_ISSUE_MISSING_DIAGNOSTIC, LIFECYCLE_RECEIPT_ISSUE_MISSING_FIELD,
    LIFECYCLE_RECEIPT_ISSUE_MISSING_HASH, LIFECYCLE_RECEIPT_ISSUE_OVERCLAIM,
    LIFECYCLE_RECEIPT_ISSUE_SELF_HASH, LIFECYCLE_RECEIPT_ISSUE_STALE_HASH,
    LIFECYCLE_RECEIPT_ISSUE_UNSUPPORTED_VERDICT, LIFECYCLE_RECEIPT_ISSUE_WEAK_NON_CLAIM,
};
pub use verified::plan::{
    build_archive_plan, build_archive_plan_with_dependency_blockers, build_sync_plan,
    build_sync_plan_with_dependency_blockers, ArchivePlan, PlanAction, PlanActionKind, SyncPlan,
};
pub use verified::policy::{
    classify_policy_schema_compatibility, default_policy_schema_metadata, parse_policy_json,
    policy_schema_compatibility_diagnostic, policy_schema_metadata, validate_policy_json, Policy,
    PolicyError, PolicySchemaCompatibilityMetadata, PolicySchemaCompatibilityReport,
    SteelOrchestrationPolicy, SteelOrchestrationProfile, TraceabilityPolicy,
    CURRENT_POLICY_FIELD_SET_DIGEST, CURRENT_POLICY_SCHEMA_VERSION, DEFAULT_POLICY_REFRESH_COMMAND,
    POLICY_SCHEMA_METADATA_FIELD, SUPPORTED_POLICY_VERSION,
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
pub use verified::requirement_id_registry::{
    requirement_id_registry_issues_json, validate_requirement_id_registry,
    validate_requirement_id_registry_value, RequirementIdAlias, RequirementIdRegistry,
    RequirementIdRegistryIssue, RequirementIdRegistryReport, RequirementReference,
    RequirementRegistryRow, REQUIREMENT_ID_REGISTRY_BOUNDARY, REQUIREMENT_ID_REGISTRY_SCHEMA_V1,
};
pub use verified::runtime_evidence::{
    is_runtime_capture_origin, runtime_record_hash, verify_runtime_evidence,
    witness_checkpoint_hash, CaptureOriginSummary, RuntimeEvidenceIssue, RuntimeEvidencePolicy,
    RuntimeEvidenceReport, CAPTURE_ORIGIN_BOUNDARY, CAPTURE_ORIGIN_INGESTED,
    CAPTURE_ORIGIN_UNKNOWN, CAPTURE_ORIGIN_WRAPPER_ASSERTED,
    COMPLETENESS_ALTERED_BEFORE_CHECKPOINT, COMPLETENESS_CHECKPOINT_MALFORMED,
    COMPLETENESS_INTEGRITY_ONLY, COMPLETENESS_SUBJECT_MISMATCH, COMPLETENESS_TRUNCATED,
    COMPLETENESS_WITNESSED_PREFIX, DEFAULT_RUNTIME_SUMMARY_MAX_BYTES, RUNTIME_EVIDENCE_BOUNDARY,
    RUNTIME_EVIDENCE_SCHEMA_V1, RUNTIME_HASH_ALGORITHM_BLAKE3,
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
pub use verified::stack_provenance::{
    evaluate_stack_provenance, is_blake3_hex, is_stack_provenance_mode, StackProvenanceEvidence,
    StackProvenanceIssue, StackProvenancePolicy, StackProvenanceReadiness,
    STACK_PROVENANCE_BOUNDARY, STACK_PROVENANCE_CLAIM_SCOPE_LIFECYCLE,
    STACK_PROVENANCE_MODE_DISABLED, STACK_PROVENANCE_MODE_OPTIONAL, STACK_PROVENANCE_MODE_REQUIRED,
};
pub use verified::structured_lifecycle::{
    structured_lifecycle_bundle_hash, structured_lifecycle_issue_json,
    validate_structured_lifecycle_bundle, StructuredLifecycleIssue, StructuredLifecycleReport,
    STRUCTURED_LIFECYCLE_SCHEMA_V1,
};
pub use verified::toolchain_profile_matrix::{
    toolchain_profile_issues_json, validate_toolchain_profile_matrix,
    validate_toolchain_profile_matrix_value, ToolchainProfileIssue, ToolchainProfileMatrix,
    ToolchainProfileReport, ToolchainProfileRow, TOOLCHAIN_PROFILE_MATRIX_BOUNDARY,
    TOOLCHAIN_PROFILE_MATRIX_SCHEMA_V1,
};
pub use verified::traceability::{
    adapt_cairn_traceability_markers, adapt_octet_traceability_markers,
    adapt_valence_traceability_markers, classify_traceability_coverage,
    default_traceability_marker_vocabulary, traceability_id_is_valid, traceability_ref_markers,
    traceability_spec_marker, unique_sorted_strings, validate_traceability_marker_links,
    TraceabilityCoverageReport, TraceabilityFile, TraceabilityMarkerIssue, TraceabilityMarkerLink,
    TraceabilityMarkerValidationInput, TraceabilityMarkerValidationReport, TraceabilityProfile,
    TraceabilityReferenceMarker, TraceabilityReferenceOccurrence,
    TraceabilityRequirementOccurrence, TraceabilitySource, TRACEABILITY_COVERAGE_VERBS,
    TRACEABILITY_MARKER_CORE_BOUNDARY, TRACEABILITY_MARKER_NON_CLAIMS,
    TRACEABILITY_MARKER_VERB_REQUIREMENT, TRACEABILITY_PROFILE_CAIRN_DEFAULT,
    TRACEABILITY_PROFILE_TRELLIS_RUST, TRACEABILITY_VERB_DEPENDS, TRACEABILITY_VERB_IMPL,
    TRACEABILITY_VERB_RELATED, TRACEABILITY_VERB_VERIFY,
};
pub use verified::validation::{
    change_issues, classify_gate_evidence, evaluate_gate, evaluate_gate_snapshot, validate_changes,
    validate_changes_with_archives, validate_spec_source, validate_spec_sources,
    EvidenceStrengthSummary, GateArtifact, GateIssue, GateReport, GateSnapshot, GateStage,
};
pub use verified::workspace_contract_matrix::{
    evaluate_workspace_contract_matrix_readiness, is_workspace_contract_mode,
    validate_workspace_contract_matrix, validate_workspace_contract_matrix_value,
    workspace_contract_issues_json, workspace_contract_readiness_issues_json,
    workspace_contract_readiness_policy_from_value, WorkspaceArtifactObservation,
    WorkspaceContractEdge, WorkspaceContractIssue, WorkspaceContractMatrix,
    WorkspaceContractReadiness, WorkspaceContractReadinessPolicy, WorkspaceContractReport,
    WORKSPACE_CONTRACT_MATRIX_BOUNDARY, WORKSPACE_CONTRACT_MATRIX_RECEIPT_SCHEMA_V1,
    WORKSPACE_CONTRACT_MATRIX_SCHEMA_V1, WORKSPACE_CONTRACT_MODE_DISABLED,
    WORKSPACE_CONTRACT_MODE_OPTIONAL, WORKSPACE_CONTRACT_MODE_REQUIRED,
};
