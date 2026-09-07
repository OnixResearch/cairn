//! Durable one-file mechanics composed into a fail-closed Cairn transaction.
#[cfg(test)]
mod tests;
use super::*;
use cap_fs_ext::DirExt;
use cap_root_core::{AdmissionMode, RootRelativePath};
use durable_file_publication::{
    core::{DurabilityMode, PublicationDisposition, PublicationRequest, ReplacementMode},
    shell::{publish_one_file, LinuxDirectory, StageNameSource},
};
use std::io::{self, Write};

pub(super) const LOCK: &str = cairn_core::verified::consumer_bootstrap::BOOTSTRAP_LOCK_LOCATOR;
const STAGE: &str = cairn_core::verified::consumer_bootstrap::BOOTSTRAP_STAGE_LEAF;
const COLLISION_ATTEMPTS: usize = 1;
const FINAL_MODE: u32 = 0o600;

pub(super) struct Lock<'a>(&'a Dir);
impl<'a> Lock<'a> {
    pub(super) fn acquire(root: &'a Dir) -> Result<Self, Box<dyn Error>> {
        let mut options = cap_std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        let mut file = root
            .open_with(LOCK, &options)
            .map_err(|_| "bootstrap.lock.unavailable: inspect owner before manual repair")?;
        let lock = Self(root);
        file.write_all(b"cairn bootstrap in progress\n")?;
        file.sync_all()?;
        Ok(lock)
    }
    pub(super) fn release(self) -> Result<(), Box<dyn Error>> {
        self.0.remove_file(LOCK)?;
        Ok(())
    }
}
// Errors retain the lock. In particular, failure to synchronize prepared-state
// removal cannot silently expose a complete installation after an error.
struct Names;
impl StageNameSource for Names {
    fn next_stage_leaf(&mut self) -> String {
        STAGE.into()
    }
}

pub(super) fn directory(root: &Dir, locator: &str) -> Result<Dir, Box<dyn Error>> {
    let admitted = RootRelativePath::new(locator, AdmissionMode::NonEmptyRelative)?;
    let mut dir = root.try_clone()?;
    for component in admitted.as_path().components() {
        let leaf = component.as_os_str();
        match dir.create_dir(leaf) {
            Ok(()) => {
                dir.open(".")?.sync_all()?;
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error.into()),
        }
        dir = dir.open_dir_nofollow(leaf)?;
    }
    Ok(dir)
}

// r[impl cairn.consumer_bootstrap.publication]
// r[impl cairn.consumer_bootstrap.boundary]
pub(super) fn publish(root: &Dir, write: &WriteIntent) -> Result<(), Box<dyn Error>> {
    let observed = read_optional(root, &write.destination, MAX_ARTIFACT_BYTES)?;
    if observed
        .as_ref()
        .map(|bytes| BootstrapIdentity::from_bytes(bytes))
        != write.prior
    {
        return Err("bootstrap.publication.destination_drift".into());
    }
    let admitted = RootRelativePath::new(&write.destination, AdmissionMode::NonEmptyRelative)?;
    let parent = match admitted
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
    {
        Some(path) => directory(root, path.to_str().ok_or("bootstrap.path.utf8")?)?,
        None => root.try_clone()?,
    };
    let leaf = admitted
        .file_name()
        .and_then(|leaf| leaf.to_str())
        .ok_or("bootstrap.path.leaf")?;
    let mut io = LinuxDirectory::from_open_directory(parent.open(".")?.into_std())?;
    let request = PublicationRequest {
        destination_leaf: leaf.into(),
        payload_bytes: write.bytes.len(),
        maximum_payload_bytes: usize::try_from(MAX_ARTIFACT_BYTES)?,
        collision_attempt_limit: COLLISION_ATTEMPTS,
        final_mode: FINAL_MODE,
        replacement: if write.prior.is_some() {
            ReplacementMode::Replace
        } else {
            ReplacementMode::NoReplace
        },
        durability: DurabilityMode::DurabilityRequired,
    };
    let result = publish_one_file(&mut io, &mut Names, &request, &write.bytes)
        .map_err(|error| format!("bootstrap.publication.admission: {error:?}"))?;
    require_durable(&result.disposition)?;
    if read_required(root, &write.destination, MAX_ARTIFACT_BYTES)? != write.bytes {
        return Err("bootstrap.publication.readback_mismatch".into());
    }
    Ok(())
}

pub(super) fn require_durable(disposition: &PublicationDisposition) -> Result<(), Box<dyn Error>> {
    match disposition {
        PublicationDisposition::CommittedAndParentSynchronized => Ok(()),
        PublicationDisposition::CommittedDurabilityUnknown => {
            Err("bootstrap.publication.durability_unknown: recovery required".into())
        }
        _ => {
            Err(format!("bootstrap.publication.failed: {disposition:?}; recovery required").into())
        }
    }
}

pub(super) fn execute(root: &Dir, prepared: &Prepared) -> Result<(), Box<dyn Error>> {
    execute_with(root, prepared, |_| Ok(()))
}

fn execute_with(
    root: &Dir,
    prepared: &Prepared,
    mut after_artifact: impl FnMut(&str) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    publish(
        root,
        &WriteIntent {
            destination: PREPARED.into(),
            prior: None,
            bytes: serde_json::to_vec(&prepared.record)?,
        },
    )?;
    for write in &prepared.writes {
        publish(root, write)?;
        after_artifact(&write.destination)?;
    }
    for locator in [".cairn/specs", ".cairn/changes", ".cairn/archive"] {
        directory(root, locator)?;
    }
    // Complete selection metadata is published before the policy-selection pointer.
    publish(root, &prepared.complete)?;
    publish(root, &prepared.selection)?;
    // Until removal is synchronized, check and normal policy loading remain blocked.
    let directory = directory(root, ".cairn/bootstrap")?;
    directory.remove_file("prepared.json")?;
    directory.open(".")?.sync_all()?;
    Ok(())
}
