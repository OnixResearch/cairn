//! Shared positive and negative mechanical conformance corpus.

use crate::core::{DurabilityMode, PrimaryFailure, ReplacementMode};

/// Named injected fault.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FaultPoint {
    /// No injected primary fault.
    None,
    /// Stage creation fails.
    StageCreate,
    /// Complete write fails.
    PayloadWrite,
    /// Permission application fails.
    PermissionApply,
    /// Payload synchronization fails.
    PayloadSync,
    /// Rename fails before commit.
    Rename,
    /// Parent synchronization fails after commit.
    ParentSync,
}

/// Expected high-level classification for one case.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedDisposition {
    /// Visibility-only commit.
    Visible,
    /// Commit and parent synchronization.
    Durable,
    /// Existing destination under exact no-replace.
    DestinationExists,
    /// No commit with one exact primary failure.
    NotCommitted(PrimaryFailure),
    /// Commit with unknown durability.
    CommittedDurabilityUnknown,
    /// Core request admission rejects before I/O.
    InvalidRequest,
    /// Stage-name admission rejects before I/O.
    InvalidStageName,
}

/// One shared corpus row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CorpusCase {
    /// Stable case ID.
    pub id: &'static str,
    /// Replacement behavior.
    pub replacement: ReplacementMode,
    /// Completion requirement.
    pub durability: DurabilityMode,
    /// Platform no-replace support.
    pub no_replace_supported: bool,
    /// Platform parent-sync support.
    pub parent_sync_supported: bool,
    /// Existing destination observed by no-replace.
    pub destination_exists: bool,
    /// Number of stage-name collisions before success.
    pub collisions_before_success: usize,
    /// Primary fault.
    pub fault: FaultPoint,
    /// Whether cleanup fails after a pre-commit terminal event.
    pub cleanup_fails: bool,
    /// Whether the generated stage leaf is unsafe.
    pub unsafe_stage_name: bool,
    /// Whether payload exceeds its request bound.
    pub payload_over_bound: bool,
    /// Expected classification.
    pub expected: ExpectedDisposition,
}

const NO_COLLISIONS: usize = 0;
const ONE_COLLISION: usize = 1;
const EXHAUSTED_COLLISIONS: usize = 3;

/// Stable shared corpus used by the core, shell, and both adapters.
pub const CONFORMANCE_CORPUS: &[CorpusCase] = &[
    CorpusCase {
        id: "replace-visible",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::None,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::Visible,
    },
    CorpusCase {
        id: "replace-durable",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::DurabilityRequired,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::None,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::Durable,
    },
    CorpusCase {
        id: "no-replace-durable",
        replacement: ReplacementMode::NoReplace,
        durability: DurabilityMode::DurabilityRequired,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: ONE_COLLISION,
        fault: FaultPoint::None,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::Durable,
    },
    CorpusCase {
        id: "destination-exists",
        replacement: ReplacementMode::NoReplace,
        durability: DurabilityMode::DurabilityRequired,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: true,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::None,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::DestinationExists,
    },
    CorpusCase {
        id: "destination-race-winner",
        replacement: ReplacementMode::NoReplace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: true,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::None,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::DestinationExists,
    },
    CorpusCase {
        id: "unsafe-stage-name",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::None,
        cleanup_fails: false,
        unsafe_stage_name: true,
        payload_over_bound: false,
        expected: ExpectedDisposition::InvalidStageName,
    },
    CorpusCase {
        id: "collision-exhausted",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: EXHAUSTED_COLLISIONS,
        fault: FaultPoint::None,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::NotCommitted(PrimaryFailure::CollisionExhausted),
    },
    CorpusCase {
        id: "stage-create-failure",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::StageCreate,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::NotCommitted(PrimaryFailure::StageCreate),
    },
    CorpusCase {
        id: "write-failure-cleanup-success",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::PayloadWrite,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::NotCommitted(PrimaryFailure::PayloadWrite),
    },
    CorpusCase {
        id: "write-failure-cleanup-failure",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::PayloadWrite,
        cleanup_fails: true,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::NotCommitted(PrimaryFailure::PayloadWrite),
    },
    CorpusCase {
        id: "permission-failure",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::PermissionApply,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::NotCommitted(PrimaryFailure::PermissionApply),
    },
    CorpusCase {
        id: "payload-sync-failure",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::PayloadSync,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::NotCommitted(PrimaryFailure::PayloadSync),
    },
    CorpusCase {
        id: "rename-failure",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::Rename,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::NotCommitted(PrimaryFailure::Rename),
    },
    CorpusCase {
        id: "parent-sync-partial-commit",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::DurabilityRequired,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::ParentSync,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::CommittedDurabilityUnknown,
    },
    CorpusCase {
        id: "no-replace-unsupported",
        replacement: ReplacementMode::NoReplace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: false,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::None,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::NotCommitted(PrimaryFailure::NoReplaceUnsupported),
    },
    CorpusCase {
        id: "parent-sync-unsupported",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::DurabilityRequired,
        no_replace_supported: true,
        parent_sync_supported: false,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::None,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: false,
        expected: ExpectedDisposition::NotCommitted(PrimaryFailure::ParentSyncUnsupported),
    },
    CorpusCase {
        id: "payload-bound-exceeded",
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::VisibilityOnly,
        no_replace_supported: true,
        parent_sync_supported: true,
        destination_exists: false,
        collisions_before_success: NO_COLLISIONS,
        fault: FaultPoint::None,
        cleanup_fails: false,
        unsafe_stage_name: false,
        payload_over_bound: true,
        expected: ExpectedDisposition::InvalidRequest,
    },
];
