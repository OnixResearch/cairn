pub use crate::verified::archive_compaction::{
    append_compaction_receipt, build_archive_manifest, build_compaction_plan,
    build_compaction_receipt, build_unpack_plan, build_verification_receipt, compaction_chain_hash,
    compaction_genesis_hash, compaction_ledger_hash, compaction_plan_hash, compaction_receipt_hash,
    empty_compaction_ledger, ledger_path, mutation_manifest_path, object_path, pack_manifest_path,
    pointer_path, prepared_pointer_path, receipt_path, safe_archive_id, safe_relative_path,
    unpack_plan_hash, validate_compaction_ledger, validate_compaction_plan,
    validate_compaction_receipt, validate_pack_manifest, verification_receipt_hash,
    verify_pack_objects, ArchiveFileInput, ArchiveInput, ArchiveManifest, ArchiveManifestEntry,
    ArchivePointer, CompactionIssue, CompactionLedger, CompactionLedgerEntry, CompactionPlan,
    CompactionReceipt, CompactionVerificationReceipt, MutationAction, PackManifest,
    PackObjectDescriptor, PackObjectInput, UnpackFile, UnpackPlan, COMPACTION_LEDGER_SCHEMA_V1,
    COMPACTION_NON_CLAIM, COMPACTION_PACK_SCHEMA_V1, COMPACTION_PLAN_SCHEMA_V1,
    COMPACTION_POINTER_SCHEMA_V1, COMPACTION_RECEIPT_SCHEMA_V1, COMPACTION_ROOT,
    COMPACTION_UNPACK_SCHEMA_V1, COMPACTION_VERIFY_SCHEMA_V1, HASH_HEX_LENGTH,
    LOSSLESS_PACK_PROFILE, LOSSY_SUMMARY_PROFILE,
};
pub use crate::verified::change_dependencies::{
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
pub use crate::verified::delta_merge::{
    merge_delta_spec, DeltaMerge, DeltaMergeDiagnostic, DeltaOperation, DeltaOperationKind,
    DeltaOperationOutcome, DeltaOperationState,
};
pub use crate::verified::doctor::{
    build_doctor_report, DoctorCheck, DoctorCheckStatus, DoctorReport, DoctorSnapshot,
};
pub use crate::verified::lifecycle::{
    archive_dir, build_cairn_migration_plan, build_change_skeleton, build_change_skeleton_for,
    build_ironspec_migration_plan, build_openspec_migration_plan, changes_dir, design_template,
    init_artifacts, init_artifacts_for, layout_name, layout_root, migration_plan_identity,
    proposal_template, specs_dir, tasks_template, validate_change_name, ChangeSkeleton,
    LifecycleArtifact, LifecycleArtifactKind, LifecycleLayout, MigrationAction, MigrationInput,
    MigrationPlan,
};
pub use crate::verified::lifecycle_orchestration::{
    validate_steel_action_envelope, SteelActionIssue, SteelActionValidationReport,
    STEEL_ACTION_ENVELOPE_SCHEMA_V1, STEEL_ORCHESTRATION_RECEIPT_SCHEMA_V1,
};
pub use crate::verified::lifecycle_receipt::{
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
pub use crate::verified::lifecycle_store_context::{
    build_lifecycle_store_context, normalize_direct_references, resolve_lifecycle_root,
    store_qualified_spec_fetch_command, AcceptedSpecMetadata, AcceptedSpecSnapshot,
    ContextBoundary, ContextObservations, ContextResult, ContextSpecResult,
    DirectReferenceNormalization, DirectStoreReference, DoctorFindingStatus,
    ExplicitLifecycleStoreSelector, ExplicitRootSelector, FallbackFacts, LifecycleAuthorityEffect,
    LifecycleStoreDoctorFinding, LocatorSnapshot, LocatorStoreObservation, NativeRootCandidate,
    ProjectPointerObservation, ReferenceContentBoundary, ReferenceTraversalBoundary,
    RegistryLookupBoundary, ResolvedLifecycleRoot, RootResolutionObservations,
    RootResolutionResult, RootResolutionSource, StoreIdentityObservation, VcsBoundaryFacts,
    REFERENCE_AVAILABLE, REFERENCE_DUPLICATE, REFERENCE_SELF, REFERENCE_SNAPSHOT_AMBIGUOUS,
    REFERENCE_SNAPSHOT_MISSING, REFERENCE_SNAPSHOT_STALE, REFERENCE_SPEC_DUPLICATE,
    REFERENCE_STORE_AMBIGUOUS, REFERENCE_STORE_MISSING, REFERENCE_STORE_STALE,
    ROOT_PROJECT_POINTER_STALE, ROOT_SELECTOR_CONFLICT, ROOT_STORE_AMBIGUOUS, ROOT_STORE_MISSING,
    ROOT_STORE_STALE,
};
#[cfg_attr(
    dylint_lib = "octet",
    allow(
        renamed_imports,
        reason = "Cairn core API: preserves the documented LifecycleStoreCheckoutPath root type."
    )
)]
pub use crate::verified::lifecycle_store_identity::CheckoutPath as LifecycleStoreCheckoutPath;
#[cfg_attr(
    dylint_lib = "octet",
    allow(
        renamed_imports,
        reason = "Cairn core API: preserves the documented LifecycleStoreBlake3Identity root type."
    )
)]
pub use crate::verified::lifecycle_store_identity::Digest as LifecycleStoreBlake3Identity;
#[cfg_attr(
    dylint_lib = "octet",
    allow(
        renamed_imports,
        reason = "Cairn core API: preserves the documented LifecycleStoreId root type."
    )
)]
pub use crate::verified::lifecycle_store_identity::Id as LifecycleStoreId;
#[cfg_attr(
    dylint_lib = "octet",
    allow(
        renamed_imports,
        reason = "Cairn core API: preserves the documented LifecycleStoreNonClaim root type."
    )
)]
pub use crate::verified::lifecycle_store_identity::NonClaim as LifecycleStoreNonClaim;
#[cfg_attr(
    dylint_lib = "octet",
    allow(
        renamed_imports,
        reason = "Cairn core API: preserves the documented LifecycleStoreRemoteHint root type."
    )
)]
pub use crate::verified::lifecycle_store_identity::RemoteHint as LifecycleStoreRemoteHint;
pub use crate::verified::lifecycle_store_identity::{
    admit_lifecycle_store_identity, admit_lifecycle_store_locator,
    apply_lifecycle_store_locator_plan, empty_lifecycle_store_locator,
    lifecycle_store_identity_material, lifecycle_store_locator_plan_hash,
    lifecycle_store_locator_portable_hash, lifecycle_store_locator_projection,
    lifecycle_store_source_identity, plan_lifecycle_store_registration,
    plan_lifecycle_store_unregistration, LifecycleStoreDiagnostic, LifecycleStoreIdentity,
    LifecycleStoreIdentityAdmission, LifecycleStoreIdentityProjection,
    LifecycleStoreLocatorAdmission, LifecycleStoreLocatorApplication,
    LifecycleStoreLocatorLocalFacts, LifecycleStoreLocatorMapping,
    LifecycleStoreLocatorMappingProjection, LifecycleStoreLocatorOperation,
    LifecycleStoreLocatorPlan, LifecycleStoreLocatorPortableFacts, LifecycleStoreLocatorProjection,
    LifecycleStoreLocatorState, BLAKE3_IDENTITY_HEX_LENGTH,
    LIFECYCLE_STORE_CHECKOUT_PATH_MAX_BYTES, LIFECYCLE_STORE_DIAGNOSTIC_MAX_COUNT,
    LIFECYCLE_STORE_IDENTITY_NON_CLAIMS, LIFECYCLE_STORE_IDENTITY_OBSERVATION_NON_CLAIM,
    LIFECYCLE_STORE_IDENTITY_READINESS_NON_CLAIM, LIFECYCLE_STORE_IDENTITY_REMOTE_NON_CLAIM,
    LIFECYCLE_STORE_IDENTITY_SCHEMA_V1, LIFECYCLE_STORE_ID_MAX_BYTES,
    LIFECYCLE_STORE_LOCAL_EVIDENCE_CLASS, LIFECYCLE_STORE_LOCATOR_MAX_MAPPINGS,
    LIFECYCLE_STORE_LOCATOR_NON_CLAIM, LIFECYCLE_STORE_LOCATOR_PLAN_SCHEMA_V1,
    LIFECYCLE_STORE_LOCATOR_SCHEMA_V1, LIFECYCLE_STORE_NON_CLAIMS_MAX_COUNT,
    LIFECYCLE_STORE_NON_CLAIM_MAX_BYTES, LIFECYCLE_STORE_PORTABLE_FACT_CLASS,
    LIFECYCLE_STORE_REMOTE_HINT_MAX_BYTES, LIFECYCLE_STORE_REQUIRED_NON_CLAIMS,
};
pub use crate::verified::lifecycle_worksets::{
    apply_workset_transition, opener_profile_identity, plan_workset_create, plan_workset_open,
    plan_workset_remove, recheck_workset_open_plan, workset_identity, workset_open_plan_identity,
    workset_portable_identity, workset_portable_state_identity, workset_state_identity,
    workset_transition_plan_identity, Workset, WorksetArtifactEffect, WorksetArtifactPlan,
    WorksetArtifactRole, WorksetCreateRequest, WorksetError, WorksetExecutionMode, WorksetId,
    WorksetLocalMemberFact, WorksetLocalReceiptFacts, WorksetMember, WorksetMemberLabel,
    WorksetOpenPlan, WorksetOpenRequest, WorksetOpenerPolicy, WorksetOpenerProfile,
    WorksetOpenerProfileId, WorksetPlanAction, WorksetPortableArg, WorksetReceiptOperation,
    WorksetRemoveRequest, WorksetState, WorksetTransitionOperation, WorksetTransitionPlan,
    BLAKE3_HEX_BYTES, MAX_ARG_BYTES, MAX_DISPLAY_PATH_BYTES, MAX_EXECUTABLE_BYTES, MAX_FIXED_ARGS,
    MAX_OPENER_PROFILES, MAX_OPENER_PROFILE_ID_BYTES, MAX_WORKSETS, MAX_WORKSET_ID_BYTES,
    MAX_WORKSET_MEMBERS, MAX_WORKSET_MEMBER_LABEL_BYTES, WORKSET_EVIDENCE_CLASS_LOCAL_ONLY,
    WORKSET_LOCAL_RECEIPT_SCHEMA_V1, WORKSET_NON_CLAIM_EDITOR_STATE,
    WORKSET_NON_CLAIM_LIFECYCLE_AUTHORITY, WORKSET_NON_CLAIM_MEMBER_EFFECTS,
    WORKSET_OPEN_PLAN_SCHEMA_V1, WORKSET_STATE_LOGICAL_PATH, WORKSET_STATE_SCHEMA_V1,
    WORKSET_TRANSITION_PLAN_SCHEMA_V1,
};
pub use crate::verified::plan::{
    build_archive_plan, build_archive_plan_for_effect_with_dependency_blockers,
    build_archive_plan_with_dependency_blockers, build_sync_plan,
    build_sync_plan_for_effect_with_dependency_blockers, build_sync_plan_with_dependency_blockers,
    ArchivePlan, PlanAction, PlanActionKind, SyncPlan,
};
pub use crate::verified::repo::{
    build_change_detail, build_change_list, build_change_status, build_repo_status,
    build_spec_detail, build_spec_list, count_tasks, filter_changes_by_metadata, spec_title,
    ChangeDetail, ChangeList, ChangeMetadata, ChangeSource, ChangeStatus, RepoStatus, SpecDetail,
    SpecList, SpecSource, SpecSummary, TaskCounts,
};
