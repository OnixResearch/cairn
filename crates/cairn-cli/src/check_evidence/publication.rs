//! One-file mechanism adapter. An exclusive journal prevents implicit retry.
use cap_std::{ambient_authority, fs::Dir};
use durable_file_publication::{
    core::{DurabilityMode, PublicationDisposition, PublicationRequest, ReplacementMode},
    shell::{publish_one_file, LinuxDirectory, StageNameSource},
};
use std::{error::Error, path::Path};

const MAX_ARTIFACT_BYTES: usize = 4 * 1024 * 1024;
const COLLISION_ATTEMPTS: usize = 8;
const PRIVATE_FILE_MODE: u32 = 0o600;

pub(super) struct Publisher {
    root: Dir,
    io: LinuxDirectory,
    sequence: u64,
}
struct Names {
    sequence: u64,
}
impl StageNameSource for Names {
    fn next_stage_leaf(&mut self) -> String {
        self.sequence += 1;
        format!(".stage-{}-{}", std::process::id(), self.sequence)
    }
}
impl Publisher {
    pub(super) fn open(path: &Path) -> Result<Self, Box<dyn Error>> {
        if std::fs::symlink_metadata(path)?.is_symlink() {
            return Err("publication root cannot be a symbolic link".into());
        }
        let root = Dir::open_ambient_dir(path, ambient_authority())?;
        let io = LinuxDirectory::from_open_directory(root.open(".")?.into_std())?;
        Ok(Self {
            root,
            io,
            sequence: 0,
        })
    }
    pub(super) fn publish(&mut self, leaf: &str, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        let request = PublicationRequest {
            destination_leaf: leaf.into(),
            payload_bytes: bytes.len(),
            maximum_payload_bytes: MAX_ARTIFACT_BYTES,
            collision_attempt_limit: COLLISION_ATTEMPTS,
            final_mode: PRIVATE_FILE_MODE,
            replacement: ReplacementMode::NoReplace,
            durability: DurabilityMode::DurabilityRequired,
        };
        let mut names = Names {
            sequence: self.sequence,
        };
        let result = publish_one_file(&mut self.io, &mut names, &request, bytes)
            .map_err(|_| "publication contract rejected")?;
        self.sequence = names.sequence;
        if result.disposition != PublicationDisposition::CommittedAndParentSynchronized {
            return Err(
                "publication incomplete or durability unknown; inspect output before recovery"
                    .into(),
            );
        }
        self.verify(leaf, blake3::hash(bytes).to_hex().as_ref())
    }

    pub(super) fn verify(&self, leaf: &str, expected: &str) -> Result<(), Box<dyn Error>> {
        use std::io::Read;
        let metadata = self.root.symlink_metadata(leaf)?;
        if !metadata.is_file() || metadata.len() > u64::try_from(MAX_ARTIFACT_BYTES)? {
            return Err("publication readback is not a bounded regular file".into());
        }
        let mut bytes = Vec::new();
        self.root
            .open(leaf)?
            .take(u64::try_from(MAX_ARTIFACT_BYTES)? + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() > MAX_ARTIFACT_BYTES || blake3::hash(&bytes).to_hex().as_str() != expected {
            return Err("publication readback changed; inspect output before recovery".into());
        }
        Ok(())
    }
}
