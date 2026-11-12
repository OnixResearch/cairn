//! Thin capability-relative filesystem orchestration shell.

use alloc::string::String;
use alloc::vec::Vec;

use crate::core::{
    Classification, ClassificationError, DurabilityMode, OperationPhase, PhaseObservation,
    PublicationRequest, ReplacementMode, classify_publication, valid_leaf, validate_request,
};

/// Private creation mode before the shell applies the final explicit mode.
pub const PRIVATE_STAGE_MODE: u32 = 0o600;

/// Exclusive stage creation result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StageCreateDisposition {
    /// The stage is exclusively owned by this operation.
    Created,
    /// The generated stage leaf already exists.
    AlreadyExists,
}

/// Rename result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenameDisposition {
    /// Rename committed the destination.
    Committed,
    /// Exact no-replace observed an existing destination.
    DestinationExists,
}

/// Product-neutral I/O failure. Product text remains in the adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MechanicalIoFailure;

/// Caller-injected bounded stage-name source.
pub trait StageNameSource {
    /// Return the next candidate leaf.
    fn next_stage_leaf(&mut self) -> String;
}

/// One already-opened parent capability used by the shell.
///
/// Implementations must keep every operation relative to the same opened
/// parent and must not interpret ambient product paths.
pub trait PublicationIo {
    /// Whether exact race-free no-replace is available.
    fn supports_no_replace(&self) -> bool;

    /// Whether real parent-directory synchronization is available.
    fn supports_parent_sync(&self) -> bool;

    /// Exclusively create one private no-follow regular stage.
    fn create_stage(
        &mut self,
        leaf: &str,
        private_mode: u32,
    ) -> Result<StageCreateDisposition, MechanicalIoFailure>;

    /// Write all bytes to the owned stage.
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), MechanicalIoFailure>;

    /// Apply exact final permission bits to the stage.
    fn apply_permissions(&mut self, mode: u32) -> Result<(), MechanicalIoFailure>;

    /// Synchronize the complete staged payload and metadata.
    fn sync_payload(&mut self) -> Result<(), MechanicalIoFailure>;

    /// Rename the stage to the destination under exact replacement semantics.
    fn rename_stage(
        &mut self,
        stage_leaf: &str,
        destination_leaf: &str,
        replacement: ReplacementMode,
    ) -> Result<RenameDisposition, MechanicalIoFailure>;

    /// Synchronize the opened parent directory.
    fn sync_parent(&mut self) -> Result<(), MechanicalIoFailure>;

    /// Attempt removal of the private stage.
    fn cleanup_stage(&mut self, stage_leaf: &str) -> Result<(), MechanicalIoFailure>;
}

fn classify_terminal(
    request: &PublicationRequest,
    observations: Vec<PhaseObservation>,
) -> Result<Classification, ClassificationError> {
    classify_publication(request, &observations)
}

fn fail_with_cleanup(
    io: &mut impl PublicationIo,
    request: &PublicationRequest,
    stage_leaf: &str,
    mut observations: Vec<PhaseObservation>,
    phase: OperationPhase,
) -> Result<Classification, ClassificationError> {
    observations.push(PhaseObservation::Failed(phase));
    observations.push(PhaseObservation::CleanupAttempted {
        succeeded: io.cleanup_stage(stage_leaf).is_ok(),
    });
    classify_terminal(request, observations)
}

fn preflight(io: &impl PublicationIo, request: &PublicationRequest) -> Option<PhaseObservation> {
    if request.replacement == ReplacementMode::NoReplace && !io.supports_no_replace() {
        return Some(PhaseObservation::NoReplaceUnsupported);
    }
    if request.durability == DurabilityMode::DurabilityRequired && !io.supports_parent_sync() {
        return Some(PhaseObservation::ParentSyncUnsupported);
    }
    None
}

// r[impl durable_publication.scope.one_file]
// r[impl durable_publication.shell.preflight]
// r[impl durable_publication.shell.staging]
// r[impl durable_publication.shell.replacement]
// r[impl durable_publication.shell.durability]
// r[impl durable_publication.outcome.commit_state]
// r[impl durable_publication.boundary.non_authority]
/// Publish one payload through one caller-opened parent capability.
///
/// Request admission and platform preflight occur before stage creation.
///
/// # Errors
///
/// Returns [`ClassificationError`] only when request facts or the shell's own
/// observation stream are structurally invalid.
pub fn publish_one_file(
    io: &mut impl PublicationIo,
    names: &mut impl StageNameSource,
    request: &PublicationRequest,
    bytes: &[u8],
) -> Result<Classification, ClassificationError> {
    let issues = validate_request(request);
    if !issues.is_empty() {
        return Err(ClassificationError::InvalidRequest(issues));
    }
    if bytes.len() != request.payload_bytes {
        return Err(ClassificationError::PayloadByteCountMismatch {
            expected: request.payload_bytes,
            actual: bytes.len(),
        });
    }
    if let Some(observation) = preflight(io, request) {
        return classify_terminal(request, vec![observation]);
    }

    let mut selected_stage = None;
    for _attempt in 0..request.collision_attempt_limit {
        let candidate = names.next_stage_leaf();
        if !valid_leaf(&candidate) || candidate == request.destination_leaf {
            return Err(ClassificationError::InvalidStageLeaf);
        }
        match io.create_stage(&candidate, PRIVATE_STAGE_MODE) {
            Ok(StageCreateDisposition::Created) => {
                selected_stage = Some(candidate);
                break;
            }
            Ok(StageCreateDisposition::AlreadyExists) => {}
            Err(MechanicalIoFailure) => {
                return classify_terminal(
                    request,
                    vec![PhaseObservation::Failed(OperationPhase::StageCreate)],
                );
            }
        }
    }
    let Some(stage_leaf) = selected_stage else {
        return classify_terminal(request, vec![PhaseObservation::CollisionExhausted]);
    };
    let mut observations = vec![PhaseObservation::StageCreated {
        leaf: stage_leaf.clone(),
    }];

    if io.write_all(bytes).is_err() {
        return fail_with_cleanup(
            io,
            request,
            &stage_leaf,
            observations,
            OperationPhase::PayloadWrite,
        );
    }
    observations.push(PhaseObservation::PayloadWritten { bytes: bytes.len() });

    if io.apply_permissions(request.final_mode).is_err() {
        return fail_with_cleanup(
            io,
            request,
            &stage_leaf,
            observations,
            OperationPhase::PermissionApply,
        );
    }
    observations.push(PhaseObservation::PermissionsApplied);

    if io.sync_payload().is_err() {
        return fail_with_cleanup(
            io,
            request,
            &stage_leaf,
            observations,
            OperationPhase::PayloadSync,
        );
    }
    observations.push(PhaseObservation::PayloadSynchronized);

    match io.rename_stage(&stage_leaf, &request.destination_leaf, request.replacement) {
        Ok(RenameDisposition::Committed) => observations.push(PhaseObservation::Renamed),
        Ok(RenameDisposition::DestinationExists) => {
            observations.push(PhaseObservation::DestinationExists);
            observations.push(PhaseObservation::CleanupAttempted {
                succeeded: io.cleanup_stage(&stage_leaf).is_ok(),
            });
            return classify_terminal(request, observations);
        }
        Err(MechanicalIoFailure) => {
            return fail_with_cleanup(
                io,
                request,
                &stage_leaf,
                observations,
                OperationPhase::Rename,
            );
        }
    }

    if request.durability == DurabilityMode::VisibilityOnly {
        return classify_terminal(request, observations);
    }
    if io.sync_parent().is_ok() {
        observations.push(PhaseObservation::ParentSynchronized);
    } else {
        observations.push(PhaseObservation::Failed(OperationPhase::ParentSync));
    }
    classify_terminal(request, observations)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux {
    use std::fs::File;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;

    use rustix::fs::{
        AtFlags, Mode, OFlags, RenameFlags, fsync, openat, renameat, renameat_with, unlinkat,
    };

    use super::{
        MechanicalIoFailure, PublicationIo, RenameDisposition, ReplacementMode,
        StageCreateDisposition,
    };

    /// Linux/Android implementation over one caller-opened directory file.
    pub struct LinuxDirectory {
        directory: File,
        stage: Option<File>,
        parent_sync_supported: bool,
    }

    impl LinuxDirectory {
        /// Wrap one directory already opened and authorized by the consumer.
        ///
        /// # Errors
        ///
        /// Returns an I/O error when the supplied file is not a directory.
        /// Durability preflight remains unsupported when the directory file
        /// cannot complete an initial synchronization call, such as `O_PATH`.
        pub fn from_open_directory(directory: File) -> std::io::Result<Self> {
            if !directory.metadata()?.is_dir() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "publication parent is not a directory",
                ));
            }
            let parent_sync_supported = directory.sync_all().is_ok();
            Ok(Self {
                directory,
                stage: None,
                parent_sync_supported,
            })
        }
    }

    impl PublicationIo for LinuxDirectory {
        fn supports_no_replace(&self) -> bool {
            true
        }

        fn supports_parent_sync(&self) -> bool {
            self.parent_sync_supported
        }

        fn create_stage(
            &mut self,
            leaf: &str,
            private_mode: u32,
        ) -> Result<StageCreateDisposition, MechanicalIoFailure> {
            let flags =
                OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC;
            match openat(
                &self.directory,
                leaf,
                flags,
                Mode::from_raw_mode(private_mode),
            ) {
                Ok(descriptor) => {
                    self.stage = Some(File::from(descriptor));
                    Ok(StageCreateDisposition::Created)
                }
                Err(rustix::io::Errno::EXIST) => Ok(StageCreateDisposition::AlreadyExists),
                Err(_) => Err(MechanicalIoFailure),
            }
        }

        fn write_all(&mut self, bytes: &[u8]) -> Result<(), MechanicalIoFailure> {
            self.stage
                .as_mut()
                .ok_or(MechanicalIoFailure)?
                .write_all(bytes)
                .map_err(|_| MechanicalIoFailure)
        }

        fn apply_permissions(&mut self, mode: u32) -> Result<(), MechanicalIoFailure> {
            self.stage
                .as_ref()
                .ok_or(MechanicalIoFailure)?
                .set_permissions(std::fs::Permissions::from_mode(mode))
                .map_err(|_| MechanicalIoFailure)
        }

        fn sync_payload(&mut self) -> Result<(), MechanicalIoFailure> {
            self.stage
                .as_ref()
                .ok_or(MechanicalIoFailure)?
                .sync_all()
                .map_err(|_| MechanicalIoFailure)
        }

        fn rename_stage(
            &mut self,
            stage_leaf: &str,
            destination_leaf: &str,
            replacement: ReplacementMode,
        ) -> Result<RenameDisposition, MechanicalIoFailure> {
            self.stage = None;
            let result = match replacement {
                ReplacementMode::Replace => renameat(
                    &self.directory,
                    stage_leaf,
                    &self.directory,
                    destination_leaf,
                ),
                ReplacementMode::NoReplace => renameat_with(
                    &self.directory,
                    stage_leaf,
                    &self.directory,
                    destination_leaf,
                    RenameFlags::NOREPLACE,
                ),
            };
            match result {
                Ok(()) => Ok(RenameDisposition::Committed),
                Err(rustix::io::Errno::EXIST) if replacement == ReplacementMode::NoReplace => {
                    Ok(RenameDisposition::DestinationExists)
                }
                Err(_) => Err(MechanicalIoFailure),
            }
        }

        fn sync_parent(&mut self) -> Result<(), MechanicalIoFailure> {
            fsync(&self.directory).map_err(|_| MechanicalIoFailure)
        }

        fn cleanup_stage(&mut self, stage_leaf: &str) -> Result<(), MechanicalIoFailure> {
            self.stage = None;
            unlinkat(&self.directory, stage_leaf, AtFlags::empty()).map_err(|_| MechanicalIoFailure)
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use linux::LinuxDirectory;
