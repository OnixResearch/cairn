//! Pure request admission and phase classification.

use alloc::string::String;
use alloc::vec::Vec;

const UNIX_PERMISSION_MASK: u32 = 0o777;

/// Exact replacement behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplacementMode {
    /// Replace one existing non-directory entry through one rename.
    Replace,
    /// Refuse replacement through one race-free no-replace primitive.
    NoReplace,
}

/// Requested mechanical completion level.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DurabilityMode {
    /// Require atomic visibility but make no crash-durability claim.
    VisibilityOnly,
    /// Require real parent-directory synchronization after rename.
    DurabilityRequired,
}

/// One bounded publication request over already-admitted product facts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicationRequest {
    /// Caller-admitted destination leaf under one opened parent.
    pub destination_leaf: String,
    /// Exact payload byte count.
    pub payload_bytes: usize,
    /// Caller-selected payload byte limit.
    pub maximum_payload_bytes: usize,
    /// Caller-selected exclusive stage-name attempt limit.
    pub collision_attempt_limit: usize,
    /// Exact final Unix permission bits.
    pub final_mode: u32,
    /// Exact replacement behavior.
    pub replacement: ReplacementMode,
    /// Exact requested completion level.
    pub durability: DurabilityMode,
}

/// Stable request admission issue.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RequestIssue {
    /// Destination does not name one safe leaf.
    InvalidDestinationLeaf,
    /// Payload limit is zero.
    PayloadLimitZero,
    /// Supplied payload exceeds its explicit bound.
    PayloadLimitExceeded {
        /// Caller maximum.
        maximum: usize,
        /// Supplied bytes.
        actual: usize,
    },
    /// Collision-attempt limit is zero.
    CollisionLimitZero,
    /// Permission bits exceed the Unix permission mask.
    InvalidPermissionMode,
}

/// Named operation phase used by failure observations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationPhase {
    /// Exclusive stage creation.
    StageCreate,
    /// Complete payload write.
    PayloadWrite,
    /// Exact permission application.
    PermissionApply,
    /// Staged payload synchronization.
    PayloadSync,
    /// Same-parent rename.
    Rename,
    /// Parent-directory synchronization.
    ParentSync,
}

/// Typed primary failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimaryFailure {
    /// Platform lacks exact race-free no-replace.
    NoReplaceUnsupported,
    /// Parent capability cannot perform real synchronization.
    ParentSyncUnsupported,
    /// Exclusive stage creation failed.
    StageCreate,
    /// Every bounded stage-name attempt collided.
    CollisionExhausted,
    /// Complete payload write failed.
    PayloadWrite,
    /// Permission application failed.
    PermissionApply,
    /// Staged payload synchronization failed.
    PayloadSync,
    /// Rename failed before commit.
    Rename,
}

/// Cleanup observation after a pre-commit terminal result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CleanupDisposition {
    /// No stage existed.
    NotNeeded,
    /// The private stage was removed.
    Removed,
    /// Stage removal failed and needs consumer review.
    Failed,
}

/// One exact shell observation supplied to the pure core.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PhaseObservation {
    /// Exact no-replace is unavailable before mutation.
    NoReplaceUnsupported,
    /// Real parent synchronization is unavailable before mutation.
    ParentSyncUnsupported,
    /// One exclusive private stage was created.
    StageCreated {
        /// Caller-generated safe stage leaf.
        leaf: String,
    },
    /// Every bounded stage-name attempt collided.
    CollisionExhausted,
    /// The full payload was written.
    PayloadWritten {
        /// Observed bytes written.
        bytes: usize,
    },
    /// Exact final permissions were applied.
    PermissionsApplied,
    /// The complete stage payload was synchronized.
    PayloadSynchronized,
    /// No-replace found an existing destination without committing.
    DestinationExists,
    /// Rename committed the destination name.
    Renamed,
    /// The parent directory was synchronized after rename.
    ParentSynchronized,
    /// One named operation phase failed.
    Failed(OperationPhase),
    /// Stage cleanup was attempted.
    CleanupAttempted {
        /// True when removal completed.
        succeeded: bool,
    },
}

/// Stable mechanical disposition. It grants no product authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicationDisposition {
    /// No destination commit occurred.
    NotCommitted {
        /// Primary failure.
        failure: PrimaryFailure,
        /// Cleanup state.
        cleanup: CleanupDisposition,
    },
    /// No-replace observed an existing destination.
    DestinationExists {
        /// Cleanup state for the losing stage.
        cleanup: CleanupDisposition,
    },
    /// Rename committed under an explicit durability non-claim.
    CommittedVisibilityOnly,
    /// Rename committed and the parent synchronization call succeeded.
    CommittedAndParentSynchronized,
    /// Rename committed but parent synchronization failed.
    CommittedDurabilityUnknown,
}

/// Successful deterministic classification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Classification {
    /// Mechanical disposition.
    pub disposition: PublicationDisposition,
    /// Exact observations retained in caller order.
    pub observations: Vec<PhaseObservation>,
}

/// Invalid observation stream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassificationError {
    /// Request admission failed before observation classification.
    InvalidRequest(Vec<RequestIssue>),
    /// Stage leaf is unsafe.
    InvalidStageLeaf,
    /// Payload write count does not match the admitted request.
    PayloadByteCountMismatch {
        /// Admitted request bytes.
        expected: usize,
        /// Observed bytes.
        actual: usize,
    },
    /// Observation is impossible in the current state.
    InvalidTransition {
        /// Ordered observation index.
        index: usize,
    },
    /// Stream ends before one terminal disposition.
    MissingTerminalObservation,
    /// A pre-commit stage failure did not report cleanup.
    MissingCleanupObservation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Planned,
    Staged,
    Written,
    PermissionsApplied,
    PayloadSynchronized,
    Renamed,
    AwaitingCleanup(FailureTerminal),
    Terminal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FailureTerminal {
    Primary(PrimaryFailure),
    DestinationExists,
}

/// Return true when text is one non-path leaf.
#[must_use]
pub fn valid_leaf(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && !value.contains('/')
        && !value.contains('\\')
        && !value.chars().any(char::is_control)
}

/// Validate one request without side effects.
#[must_use]
pub fn validate_request(request: &PublicationRequest) -> Vec<RequestIssue> {
    let mut issues = Vec::new();
    if !valid_leaf(&request.destination_leaf) {
        issues.push(RequestIssue::InvalidDestinationLeaf);
    }
    if request.maximum_payload_bytes == 0 {
        issues.push(RequestIssue::PayloadLimitZero);
    }
    if request.payload_bytes > request.maximum_payload_bytes {
        issues.push(RequestIssue::PayloadLimitExceeded {
            maximum: request.maximum_payload_bytes,
            actual: request.payload_bytes,
        });
    }
    if request.collision_attempt_limit == 0 {
        issues.push(RequestIssue::CollisionLimitZero);
    }
    if request.final_mode & !UNIX_PERMISSION_MASK != 0 {
        issues.push(RequestIssue::InvalidPermissionMode);
    }
    issues.sort();
    issues
}

fn failure_for_phase(phase: OperationPhase) -> Option<PrimaryFailure> {
    match phase {
        OperationPhase::StageCreate => Some(PrimaryFailure::StageCreate),
        OperationPhase::PayloadWrite => Some(PrimaryFailure::PayloadWrite),
        OperationPhase::PermissionApply => Some(PrimaryFailure::PermissionApply),
        OperationPhase::PayloadSync => Some(PrimaryFailure::PayloadSync),
        OperationPhase::Rename => Some(PrimaryFailure::Rename),
        OperationPhase::ParentSync => None,
    }
}

fn cleanup_disposition(succeeded: bool) -> CleanupDisposition {
    if succeeded {
        CleanupDisposition::Removed
    } else {
        CleanupDisposition::Failed
    }
}

fn terminal_without_stage(
    observation: &PhaseObservation,
    request: &PublicationRequest,
) -> Option<PublicationDisposition> {
    match observation {
        PhaseObservation::NoReplaceUnsupported
            if request.replacement == ReplacementMode::NoReplace =>
        {
            Some(PublicationDisposition::NotCommitted {
                failure: PrimaryFailure::NoReplaceUnsupported,
                cleanup: CleanupDisposition::NotNeeded,
            })
        }
        PhaseObservation::ParentSyncUnsupported
            if request.durability == DurabilityMode::DurabilityRequired =>
        {
            Some(PublicationDisposition::NotCommitted {
                failure: PrimaryFailure::ParentSyncUnsupported,
                cleanup: CleanupDisposition::NotNeeded,
            })
        }
        PhaseObservation::CollisionExhausted => Some(PublicationDisposition::NotCommitted {
            failure: PrimaryFailure::CollisionExhausted,
            cleanup: CleanupDisposition::NotNeeded,
        }),
        PhaseObservation::Failed(OperationPhase::StageCreate) => {
            Some(PublicationDisposition::NotCommitted {
                failure: PrimaryFailure::StageCreate,
                cleanup: CleanupDisposition::NotNeeded,
            })
        }
        _ => None,
    }
}

// r[impl durable_publication.core.classification]
// r[impl durable_publication.outcome.commit_state]
/// Classify an exact finite observation stream.
///
/// # Errors
///
/// Returns [`ClassificationError`] for invalid requests, unsafe stage names,
/// impossible phase order, byte-count mismatch, or incomplete terminal facts.
pub fn classify_publication(
    request: &PublicationRequest,
    observations: &[PhaseObservation],
) -> Result<Classification, ClassificationError> {
    let request_issues = validate_request(request);
    if !request_issues.is_empty() {
        return Err(ClassificationError::InvalidRequest(request_issues));
    }
    let mut state = State::Planned;
    let mut disposition = None;
    for (index, observation) in observations.iter().enumerate() {
        if state == State::Terminal {
            return Err(ClassificationError::InvalidTransition { index });
        }
        if state == State::Planned {
            if let Some(terminal) = terminal_without_stage(observation, request) {
                disposition = Some(terminal);
                state = State::Terminal;
                continue;
            }
        }
        match (state, observation) {
            (State::Planned, PhaseObservation::StageCreated { leaf }) => {
                if !valid_leaf(leaf) || leaf == &request.destination_leaf {
                    return Err(ClassificationError::InvalidStageLeaf);
                }
                state = State::Staged;
            }
            (State::Staged, PhaseObservation::PayloadWritten { bytes }) => {
                if *bytes != request.payload_bytes {
                    return Err(ClassificationError::PayloadByteCountMismatch {
                        expected: request.payload_bytes,
                        actual: *bytes,
                    });
                }
                state = State::Written;
            }
            (State::Written, PhaseObservation::PermissionsApplied) => {
                state = State::PermissionsApplied;
            }
            (State::PermissionsApplied, PhaseObservation::PayloadSynchronized) => {
                state = State::PayloadSynchronized;
            }
            (State::Staged, PhaseObservation::Failed(OperationPhase::PayloadWrite))
            | (State::Written, PhaseObservation::Failed(OperationPhase::PermissionApply))
            | (State::PermissionsApplied, PhaseObservation::Failed(OperationPhase::PayloadSync))
            | (State::PayloadSynchronized, PhaseObservation::Failed(OperationPhase::Rename)) => {
                let PhaseObservation::Failed(phase) = observation else {
                    return Err(ClassificationError::InvalidTransition { index });
                };
                let Some(failure) = failure_for_phase(*phase) else {
                    return Err(ClassificationError::InvalidTransition { index });
                };
                state = State::AwaitingCleanup(FailureTerminal::Primary(failure));
            }
            (State::PayloadSynchronized, PhaseObservation::DestinationExists)
                if request.replacement == ReplacementMode::NoReplace =>
            {
                state = State::AwaitingCleanup(FailureTerminal::DestinationExists);
            }
            (State::PayloadSynchronized, PhaseObservation::Renamed) => {
                if request.durability == DurabilityMode::VisibilityOnly {
                    disposition = Some(PublicationDisposition::CommittedVisibilityOnly);
                    state = State::Terminal;
                } else {
                    state = State::Renamed;
                }
            }
            (State::Renamed, PhaseObservation::ParentSynchronized)
                if request.durability == DurabilityMode::DurabilityRequired =>
            {
                disposition = Some(PublicationDisposition::CommittedAndParentSynchronized);
                state = State::Terminal;
            }
            (State::Renamed, PhaseObservation::Failed(OperationPhase::ParentSync))
                if request.durability == DurabilityMode::DurabilityRequired =>
            {
                disposition = Some(PublicationDisposition::CommittedDurabilityUnknown);
                state = State::Terminal;
            }
            (State::Terminal, _) => {
                return Err(ClassificationError::InvalidTransition { index });
            }
            (
                State::AwaitingCleanup(terminal),
                PhaseObservation::CleanupAttempted { succeeded },
            ) => {
                let cleanup = cleanup_disposition(*succeeded);
                disposition = Some(match terminal {
                    FailureTerminal::Primary(failure) => {
                        PublicationDisposition::NotCommitted { failure, cleanup }
                    }
                    FailureTerminal::DestinationExists => {
                        PublicationDisposition::DestinationExists { cleanup }
                    }
                });
                state = State::Terminal;
            }
            _ => return Err(ClassificationError::InvalidTransition { index }),
        }
    }
    if matches!(state, State::AwaitingCleanup(_)) {
        return Err(ClassificationError::MissingCleanupObservation);
    }
    let disposition = disposition.ok_or(ClassificationError::MissingTerminalObservation)?;
    Ok(Classification {
        disposition,
        observations: observations.to_vec(),
    })
}
