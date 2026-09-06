use super::*;
use std::os::unix::fs::MetadataExt;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct WriteIntent {
    pub destination: String,
    pub prior: Option<BootstrapIdentity>,
    pub bytes: Vec<u8>,
}
pub(super) struct Prepared {
    pub record: serde_json::Value,
    pub writes: Vec<WriteIntent>,
    pub complete: WriteIntent,
    pub selection: WriteIntent,
}
pub(super) struct Prior {
    pub manifest: BootstrapCompleteSelection,
    pub complete_identity: BootstrapIdentity,
    pub selection_identity: BootstrapIdentity,
}

pub(super) fn load(root: &Dir) -> Result<Option<Prior>, Box<dyn Error>> {
    let complete = read_optional(root, COMPLETE, MAX_BUNDLE_BYTES)?;
    let selection = read_optional(root, SELECTION, MAX_BUNDLE_BYTES)?;
    let (complete, selection) = match (complete, selection) {
        (None, None) => return Ok(None),
        (Some(complete), Some(selection)) => (complete, selection),
        _ => return Err("bootstrap.state.incomplete_selection: manual repair required".into()),
    };
    let declaration: BootstrapProjectDeclaration = serde_json::from_slice(&selection)?;
    let manifest: BootstrapCompleteSelection = serde_json::from_slice(&complete)?;
    manifest.validate_for(&declaration)?;
    let complete_identity = BootstrapIdentity::from_bytes(&complete);
    if complete_identity != declaration.complete_manifest {
        return Err("bootstrap.state.complete_identity_mismatch".into());
    }
    if manifest.artifacts.is_empty() {
        return Err("bootstrap.state.missing_ownership".into());
    }
    for artifact in &manifest.artifacts {
        let bytes = read_required(root, &artifact.destination, MAX_ARTIFACT_BYTES)
            .map_err(|error| format!("bootstrap.state.missing_or_unsafe_managed: {error}"))?;
        if BootstrapIdentity::from_bytes(&bytes) != artifact.identity
            || bytes.len() as u64 != artifact.bytes
        {
            return Err("bootstrap.state.modified_managed: manual repair required".into());
        }
    }
    if !manifest.artifacts.iter().any(|artifact| {
        artifact.destination == manifest.policy_locator && artifact.identity == manifest.policy
    }) {
        return Err("bootstrap.state.policy_not_owned".into());
    }
    Ok(Some(Prior {
        manifest,
        complete_identity,
        selection_identity: BootstrapIdentity::from_bytes(&selection),
    }))
}

pub(super) fn finish_plan(
    root: &Dir,
    bundle: &BootstrapSourceBundle,
    prior: Option<&Prior>,
    artifacts: &[ManagedArtifact],
    payloads: &[Vec<u8>],
    actions: &[PublicationAction],
    record: &mut serde_json::Value,
) -> Result<Prepared, Box<dyn Error>> {
    let mut installed = prior.map_or_else(Vec::new, |prior| prior.manifest.artifacts.clone());
    let mut writes = Vec::new();
    for (((source, artifact), bytes), action) in bundle
        .artifacts
        .iter()
        .zip(artifacts)
        .zip(payloads)
        .zip(actions)
    {
        cairn_core::verified::consumer_bootstrap::admit_generation_write(
            prior.map(|prior| &prior.manifest),
            &source.destination,
            *action,
        )
        .map_err(|_| "bootstrap.upgrade.new_policy_generation_required")?;
        installed.retain(|owned| owned.destination != source.destination);
        installed.push(BootstrapInstalledArtifact {
            destination: source.destination.clone(),
            identity: artifact.desired.clone(),
            bytes: artifact.desired_bytes,
        });
        if *action != PublicationAction::Preserve {
            writes.push(WriteIntent {
                destination: source.destination.clone(),
                prior: artifact.previously_owned.clone(),
                bytes: bytes.clone(),
            });
        }
    }
    installed.sort_by(|left, right| left.destination.cmp(&right.destination));
    let manifest = BootstrapCompleteSelection {
        schema: "cairn.bootstrap-complete-selection.v1".into(),
        cohort: cohort_identity(&bundle.cohort),
        policy: bundle.cohort.policy.clone(),
        policy_locator: bundle.policy_locator.clone(),
        artifacts: installed,
    };
    let complete_bytes = serde_json::to_vec(&manifest)?;
    let declaration = BootstrapProjectDeclaration {
        schema: "cairn.bootstrap-project.v1".into(),
        cohort: manifest.cohort.clone(),
        policy: manifest.policy.clone(),
        policy_locator: manifest.policy_locator.clone(),
        complete_manifest: BootstrapIdentity::from_bytes(&complete_bytes),
    };
    manifest.validate_for(&declaration)?;
    let selection_bytes = serde_json::to_vec(&declaration)?;
    let metadata = root.try_clone()?.into_std_file().metadata()?;
    record["root_identity"] = json!({"device": metadata.dev(), "inode": metadata.ino()});
    record["prior_selection"] = json!(prior.map(|prior| &prior.selection_identity));
    record["complete_manifest"] = json!(BootstrapIdentity::from_bytes(&complete_bytes));
    record["selection"] = json!(BootstrapIdentity::from_bytes(&selection_bytes));
    record["plan_hash"] = json!(BootstrapIdentity::from_bytes(&serde_json::to_vec(record)?));
    Ok(Prepared {
        record: record.clone(),
        writes,
        complete: WriteIntent {
            destination: COMPLETE.into(),
            prior: prior.map(|prior| prior.complete_identity.clone()),
            bytes: complete_bytes,
        },
        selection: WriteIntent {
            destination: SELECTION.into(),
            prior: prior.map(|prior| prior.selection_identity.clone()),
            bytes: selection_bytes,
        },
    })
}

pub(super) fn inspect(
    root: &Dir,
    root_path: &std::path::Path,
    command: &str,
) -> Result<(), Box<dyn Error>> {
    let prepared = read_optional(root, PREPARED, MAX_BUNDLE_BYTES)?;
    let locked = read_optional(root, publication::LOCK, MAX_BUNDLE_BYTES)?.is_some();
    if prepared.is_some() || locked {
        return print_json(crate::support::with_receipt_hash(json!({
            "schema": "cairn.bootstrap-check.v1", "command": command,
            "valid": false, "blocked": true, "mutated": false,
            "category": "bootstrap.recovery.manual_repair",
            "prepared_identity": prepared.as_ref().map(|bytes| BootstrapIdentity::from_bytes(bytes)),
            "lock_present": locked,
            "repair": "Stop competing writers. Preserve prepared state and prior selection. Review exact artifact identities and synchronization before restoring a complete generation. Automatic recovery is not admitted.",
            "installation_receipt": false
        })));
    }
    let prior = load(root)?.ok_or("bootstrap.state.not_installed")?;
    let bytes = read_required(root, &prior.manifest.policy_locator, MAX_ARTIFACT_BYTES)?;
    let policy = validate_policy_json(serde_json::from_slice(&bytes)?)?;
    let guidance = guidance::doctor(root_path, &policy)?;
    let valid = guidance.as_ref().is_none_or(|value| value["valid"] == true);
    print_json(crate::support::with_receipt_hash(json!({
        "schema": "cairn.bootstrap-check.v1", "command": command,
        "valid": valid, "blocked": !valid, "mutated": false,
        "guidance": guidance,
        "complete_manifest": prior.complete_identity,
        "artifacts": prior.manifest.artifacts,
        "installation_receipt": false,
        "non_claims": ["Current byte identities do not establish historical durability or policy correctness."]
    })))
}
