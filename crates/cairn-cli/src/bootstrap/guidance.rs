//! Project-local composition of the existing generated-workflow installer plan.
use super::*;
use cairn_core::{GeneratedWorkflowFreshness, GeneratedWorkflowInstallPlan, Policy};
use std::path::Path;

use cairn_core::verified::consumer_bootstrap::{
    BootstrapGuidanceDeclaration, BOOTSTRAP_GUIDANCE_LOCATOR,
};
pub(super) const DECLARATION: &str = BOOTSTRAP_GUIDANCE_LOCATOR;

// r[impl cairn.consumer_bootstrap.guidance]
pub(super) fn plan(
    root: &Path,
    policy: &Policy,
    bytes: &[u8],
) -> Result<GeneratedWorkflowInstallPlan, Box<dyn Error>> {
    let declaration: BootstrapGuidanceDeclaration =
        serde_json::from_slice(bytes).map_err(|_| "bootstrap.guidance.invalid_declaration")?;
    declaration.validate()?;
    crate::generated_agent_workflow_shell::bootstrap_plan(
        root,
        policy,
        &declaration.adapter,
        &declaration.destination,
    )
}

pub(super) fn extend(
    prepared: &mut Prepared,
    prior: Option<&state::Prior>,
    plan: &GeneratedWorkflowInstallPlan,
    declaration: &[u8],
) -> Result<(), Box<dyn Error>> {
    if plan.blocked() {
        return Err(format!("bootstrap.guidance.manual_repair: {:?}", plan.blockers).into());
    }
    let mut manifest: BootstrapCompleteSelection =
        serde_json::from_slice(&prepared.complete.bytes)?;
    let mut additions = plan
        .files
        .iter()
        .map(|file| {
            (
                file.relative_path.clone(),
                file.contents.as_bytes().to_vec(),
            )
        })
        .collect::<Vec<_>>();
    additions.push((
        plan.manifest_path.clone(),
        plan.manifest_bytes.as_bytes().to_vec(),
    ));
    additions.push((DECLARATION.into(), declaration.to_vec()));
    for (destination, bytes) in additions {
        if [COMPLETE, PREPARED, SELECTION, publication::LOCK].contains(&destination.as_str())
            || prepared
                .writes
                .iter()
                .any(|write| write.destination == destination)
        {
            return Err("bootstrap.guidance.destination_collision".into());
        }
        let owned = prior.and_then(|prior| {
            prior
                .manifest
                .artifacts
                .iter()
                .find(|owned| owned.destination == destination)
        });
        let identity = BootstrapIdentity::from_bytes(&bytes);
        if owned.is_none_or(|owned| owned.identity != identity) {
            prepared.writes.push(WriteIntent {
                destination: destination.clone(),
                prior: owned.map(|owned| owned.identity.clone()),
                bytes: bytes.clone(),
            });
        }
        manifest
            .artifacts
            .retain(|artifact| artifact.destination != destination);
        manifest.artifacts.push(BootstrapInstalledArtifact {
            destination,
            identity,
            bytes: bytes.len() as u64,
        });
    }
    manifest
        .artifacts
        .sort_by(|a, b| a.destination.cmp(&b.destination));
    let complete = serde_json::to_vec(&manifest)?;
    let mut selection: BootstrapProjectDeclaration =
        serde_json::from_slice(&prepared.selection.bytes)?;
    selection.complete_manifest = BootstrapIdentity::from_bytes(&complete);
    manifest.validate_for(&selection)?;
    prepared.complete.bytes = complete;
    prepared.selection.bytes = serde_json::to_vec(&selection)?;
    prepared.record["guidance"] = summary(plan);
    prepared.record["complete_manifest"] = json!(selection.complete_manifest);
    prepared.record["selection"] = json!(BootstrapIdentity::from_bytes(&prepared.selection.bytes));
    prepared
        .record
        .as_object_mut()
        .ok_or("bootstrap.plan.object")?
        .remove("plan_hash");
    prepared.record["plan_hash"] = json!(BootstrapIdentity::from_bytes(&serde_json::to_vec(
        &prepared.record
    )?));
    Ok(())
}

fn summary(plan: &GeneratedWorkflowInstallPlan) -> serde_json::Value {
    json!({ "valid": !plan.blocked() && plan.manifest_freshness == GeneratedWorkflowFreshness::Current && plan.files.iter().all(|file| file.freshness == GeneratedWorkflowFreshness::Current),
        "manifest": plan.manifest_path, "plan_hash": plan.plan_hash,
        "manifest_freshness": plan.manifest_freshness, "blockers": plan.blockers,
        "files": plan.files.iter().map(|file| json!({"path": file.relative_path, "freshness": file.freshness})).collect::<Vec<_>>() })
}

// r[impl cairn.consumer_bootstrap.guidance]
pub(crate) fn doctor(
    root: &Path,
    policy: &Policy,
) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
    let capability = open_root(root)?;
    let bytes = read_optional(&capability, DECLARATION, MAX_BUNDLE_BYTES)?;
    let manifest = read_optional(&capability, COMPLETE, MAX_BUNDLE_BYTES)?
        .map(|bytes| serde_json::from_slice::<BootstrapCompleteSelection>(&bytes))
        .transpose()?;
    let owned = manifest.as_ref().and_then(|manifest| {
        manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.destination == DECLARATION)
    });
    match (bytes, owned) {
        (None, None) => Ok(None),
        (None, Some(_)) => Err("bootstrap.guidance.missing_managed_declaration".into()),
        (Some(_), None) => Err("bootstrap.guidance.unmanaged_declaration".into()),
        (Some(bytes), Some(owned)) => {
            if BootstrapIdentity::from_bytes(&bytes) != owned.identity {
                return Err("bootstrap.guidance.modified_declaration".into());
            }
            Ok(Some(summary(&plan(root, policy, &bytes)?)))
        }
    }
}
