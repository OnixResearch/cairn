//! Consumer conformance for the exact published bootstrap mechanisms.
#![cfg(target_os = "linux")]
use cap_root_core::{AdmissionMode, RootRelativePath};
use durable_file_publication::{
    core::{
        CleanupDisposition, DurabilityMode, OperationPhase, PhaseObservation, PrimaryFailure,
        PublicationDisposition, PublicationRequest, ReplacementMode,
    },
    shell::{
        publish_one_file, LinuxDirectory, MechanicalIoFailure, PublicationIo, RenameDisposition,
        StageCreateDisposition, StageNameSource,
    },
};
use std::{error::Error, fs, path::Path};

struct Names;
impl StageNameSource for Names {
    fn next_stage_leaf(&mut self) -> String {
        ".cairn-stage".into()
    }
}
fn request() -> PublicationRequest {
    PublicationRequest {
        destination_leaf: "policy.json".into(),
        payload_bytes: 2,
        maximum_payload_bytes: 16,
        collision_attempt_limit: 2,
        final_mode: 0o600,
        replacement: ReplacementMode::NoReplace,
        durability: DurabilityMode::DurabilityRequired,
    }
}

// r[verify cairn.consumer_bootstrap.boundary]
#[test]
fn cap_root_admits_relative_paths_without_admitting_escapes() -> Result<(), Box<dyn Error>> {
    let path = RootRelativePath::new(".cairn/./policy.json", AdmissionMode::NonEmptyRelative)?;
    assert_eq!(path.as_path(), Path::new(".cairn/policy.json"));
    assert!(RootRelativePath::new_leaf("policy.json").is_ok());
    assert!(RootRelativePath::new_leaf("nested/policy.json").is_err());
    for path in [
        "",
        ".",
        "/policy.json",
        "../policy.json",
        "a/../policy.json",
    ] {
        assert!(RootRelativePath::new(path, AdmissionMode::NonEmptyRelative).is_err());
    }
    Ok(())
}

// r[verify cairn.consumer_bootstrap.publication]
#[test]
fn published_linux_adapter_preserves_existing_destination_and_symlink_target(
) -> Result<(), Box<dyn Error>> {
    let root =
        std::env::temp_dir().join(format!("cairn-publication-contract-{}", std::process::id()));
    fs::create_dir(&root)?;
    let mut io = LinuxDirectory::from_open_directory(fs::File::open(&root)?)?;
    let publish =
        |io: &mut LinuxDirectory| publish_one_file(io, &mut Names, &request(), b"{}").unwrap();
    assert_eq!(
        publish(&mut io).disposition,
        PublicationDisposition::CommittedAndParentSynchronized
    );
    assert_eq!(fs::read(root.join("policy.json"))?, b"{}");
    assert_eq!(
        publish(&mut io).disposition,
        PublicationDisposition::DestinationExists {
            cleanup: CleanupDisposition::Removed
        }
    );
    fs::remove_file(root.join("policy.json"))?;
    fs::write(root.join("unowned"), b"keep")?;
    std::os::unix::fs::symlink("unowned", root.join("policy.json"))?;
    assert_eq!(
        publish(&mut io).disposition,
        PublicationDisposition::DestinationExists {
            cleanup: CleanupDisposition::Removed
        }
    );
    assert_eq!(fs::read(root.join("unowned"))?, b"keep");
    assert!(!root.join(".cairn-stage").exists());
    fs::remove_dir_all(root)?;
    Ok(())
}

#[derive(Default)]
struct FaultIo {
    write_fails: bool,
    sync_fails: bool,
    cleanup_fails: bool,
    renamed: bool,
    stages: usize,
}
impl PublicationIo for FaultIo {
    fn supports_no_replace(&self) -> bool {
        true
    }
    fn supports_parent_sync(&self) -> bool {
        true
    }
    fn create_stage(
        &mut self,
        _: &str,
        _: u32,
    ) -> Result<StageCreateDisposition, MechanicalIoFailure> {
        self.stages += 1;
        Ok(StageCreateDisposition::Created)
    }
    fn write_all(&mut self, _: &[u8]) -> Result<(), MechanicalIoFailure> {
        if self.write_fails {
            Err(MechanicalIoFailure)
        } else {
            Ok(())
        }
    }
    fn apply_permissions(&mut self, _: u32) -> Result<(), MechanicalIoFailure> {
        Ok(())
    }
    fn sync_payload(&mut self) -> Result<(), MechanicalIoFailure> {
        Ok(())
    }
    fn rename_stage(
        &mut self,
        _: &str,
        _: &str,
        _: ReplacementMode,
    ) -> Result<RenameDisposition, MechanicalIoFailure> {
        self.renamed = true;
        Ok(RenameDisposition::Committed)
    }
    fn sync_parent(&mut self) -> Result<(), MechanicalIoFailure> {
        if self.sync_fails {
            Err(MechanicalIoFailure)
        } else {
            Ok(())
        }
    }
    fn cleanup_stage(&mut self, _: &str) -> Result<(), MechanicalIoFailure> {
        if self.cleanup_fails {
            Err(MechanicalIoFailure)
        } else {
            Ok(())
        }
    }
}

// r[verify cairn.consumer_bootstrap.publication]
#[test]
fn publication_contract_retains_interruption_cleanup_and_unknown_durability() {
    let mut io = FaultIo::default();
    let result = publish_one_file(&mut io, &mut Names, &request(), b"{}").unwrap();
    assert_eq!(
        result.disposition,
        PublicationDisposition::CommittedAndParentSynchronized
    );
    assert_eq!(
        result.observations.last(),
        Some(&PhaseObservation::ParentSynchronized)
    );
    let mut io = FaultIo {
        sync_fails: true,
        ..Default::default()
    };
    let result = publish_one_file(&mut io, &mut Names, &request(), b"{}").unwrap();
    assert_eq!(
        result.disposition,
        PublicationDisposition::CommittedDurabilityUnknown
    );
    assert!(io.renamed);
    assert_eq!(
        result.observations.last(),
        Some(&PhaseObservation::Failed(OperationPhase::ParentSync))
    );
    for (cleanup_fails, cleanup) in [
        (false, CleanupDisposition::Removed),
        (true, CleanupDisposition::Failed),
    ] {
        let mut io = FaultIo {
            write_fails: true,
            cleanup_fails,
            ..Default::default()
        };
        let result = publish_one_file(&mut io, &mut Names, &request(), b"{}").unwrap();
        assert_eq!(
            result.disposition,
            PublicationDisposition::NotCommitted {
                failure: PrimaryFailure::PayloadWrite,
                cleanup
            }
        );
        assert!(!io.renamed);
    }
    let mut invalid = request();
    invalid.destination_leaf = "../policy.json".into();
    let mut io = FaultIo::default();
    assert!(publish_one_file(&mut io, &mut Names, &invalid, b"{}").is_err());
    assert_eq!(io.stages, 0);
    assert!(publish_one_file(&mut io, &mut Names, &request(), b"wrong length").is_err());
    assert_eq!(io.stages, 0);
}
