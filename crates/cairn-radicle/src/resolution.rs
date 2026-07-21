use std::{path::Path, str::FromStr};

use cairn_core::{
    change_cob_content_root, change_cob_state_hash, inspect_change_cob_conflicts,
    ChangeCobResolutionPlan,
};
use nonempty::NonEmpty;
use radicle::{
    cob::{store::access::WriteAs, store::Store as CobStore, ObjectId},
    crypto::{self, PublicKey},
    git,
    node::device::BoxedDevice,
    storage::{git::Repository, ReadStorage},
};

use crate::{
    import::{load_profile_at, parse_repository_id},
    inspect_change_status,
    update::{acquire_update_lock, require_update_authority, validate_update_context},
    Action, ChangeImportError, ChangeStatusInspection, RadicleChangeCob, TYPENAME,
};

// r[impl cairn.collaboration_resolution.radicle_execution]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeResolutionOutcome {
    pub object_id: String,
    pub content_root: String,
    pub state_hash: String,
}

pub fn create_change_resolution<Signer>(
    repo: &Repository,
    signer: &Signer,
    plan: &ChangeCobResolutionPlan,
    expected: &ChangeStatusInspection,
) -> Result<ChangeResolutionOutcome, ChangeImportError>
where
    Signer: crypto::signature::Keypair<VerifyingKey = PublicKey>,
    Signer: crypto::signature::Signer<crypto::Signature>,
    Signer: crypto::signature::Signer<crypto::ssh::ExtendedSignature>,
    Signer: crypto::signature::Verifier<crypto::Signature>,
{
    if plan.actions.is_empty() {
        return Err(ChangeImportError::EmptyUpdate);
    }
    let signer_key = signer.verifying_key();
    let current = inspect_change_status(repo, &signer_key, &plan.change_id)?;
    validate_update_context(&current, expected)?;
    require_update_authority(&current)?;
    validate_resolution_plan_context(plan, &current)?;
    if signer_key.to_string() != current.authority.actor {
        return Err(ChangeImportError::SignerMismatch {
            signer: signer_key.to_string(),
            inspected: current.authority.actor,
        });
    }
    let identity_oid = git::Oid::from_str(&current.authority.identity_oid).map_err(|_| {
        ChangeImportError::UpdateContextDrift {
            field: "identity_oid".to_string(),
        }
    })?;
    let object_id = ObjectId::from_str(&current.object_id).map_err(|error| {
        ChangeImportError::InvalidCobObjectId {
            object_id: current.object_id.clone(),
            reason: error.to_string(),
        }
    })?;
    let actions = NonEmpty::from_vec(
        plan.actions
            .iter()
            .cloned()
            .map(Action::from)
            .collect::<Vec<_>>(),
    )
    .ok_or(ChangeImportError::EmptyUpdate)?;
    let title = format!("Resolve Cairn change {} conflicts", plan.change_id);
    let mut store = CobStore::<RadicleChangeCob, _, _>::open(repo, WriteAs::new(signer))?
        .identity(identity_oid);
    store.update(&TYPENAME, object_id, &title, actions, Vec::new())?;
    let Some(reloaded) = store.get(&object_id)? else {
        return Err(ChangeImportError::UpdatedObjectMissing {
            object_id: object_id.to_string(),
        });
    };
    let conflicts = inspect_change_cob_conflicts(reloaded.state())?;
    if conflicts.has_conflicts() {
        return Err(ChangeImportError::UpdateContextDrift {
            field: "resolved_conflicts".to_string(),
        });
    }
    let content_root = change_cob_content_root(reloaded.state())?;
    if content_root != plan.expected_content_root {
        return Err(ChangeImportError::UpdatedContentRootMismatch {
            planned: plan.expected_content_root.clone(),
            updated: content_root,
        });
    }
    Ok(ChangeResolutionOutcome {
        object_id: object_id.to_string(),
        content_root,
        state_hash: change_cob_state_hash(reloaded.state())?,
    })
}

pub fn execute_profile_resolution(
    rad_home: &Path,
    repository_id: &str,
    plan: &ChangeCobResolutionPlan,
    expected: &ChangeStatusInspection,
) -> Result<ChangeResolutionOutcome, ChangeImportError> {
    if plan.actions.is_empty() {
        return Err(ChangeImportError::EmptyUpdate);
    }
    let profile = load_profile_at(rad_home)?;
    let repository_id = parse_repository_id(repository_id)?;
    let repo = profile.storage.repository(repository_id)?;
    let preflight = inspect_change_status(&repo, &profile.public_key, &plan.change_id)?;
    validate_update_context(&preflight, expected)?;
    require_update_authority(&preflight)?;
    validate_resolution_plan_context(plan, &preflight)?;
    let lock = acquire_update_lock(
        profile.home.path(),
        &preflight.repository_id,
        &plan.change_id,
    )?;
    let guarded = inspect_change_status(&repo, &profile.public_key, &plan.change_id);
    let result = match guarded {
        Ok(inspection) => validate_update_context(&inspection, expected)
            .and_then(|()| require_update_authority(&inspection))
            .and_then(|()| validate_resolution_plan_context(plan, &inspection))
            .and_then(|()| {
                let signer: BoxedDevice = profile.signer()?;
                create_change_resolution(&repo, &signer, plan, &inspection)
            }),
        Err(error) => Err(error),
    };
    let release_result = lock.release();
    match (result, release_result) {
        (Ok(outcome), Ok(())) => Ok(outcome),
        (Err(error), _) => Err(error),
        (Ok(_), Err(error)) => Err(ChangeImportError::ProfileIo(error)),
    }
}

fn validate_resolution_plan_context(
    plan: &ChangeCobResolutionPlan,
    current: &ChangeStatusInspection,
) -> Result<(), ChangeImportError> {
    if plan.actor != current.authority.actor {
        return Err(ChangeImportError::UpdateActorMismatch {
            planned: plan.actor.clone(),
            current: current.authority.actor.clone(),
        });
    }
    if plan.change_id != current.state.change_id {
        return Err(ChangeImportError::UpdateContextDrift {
            field: "change_id".to_string(),
        });
    }
    let state_hash = change_cob_state_hash(&current.state)?;
    if plan.input_state_hash != state_hash {
        return Err(ChangeImportError::UpdateContextDrift {
            field: "typed_state_hash".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, BTreeSet},
        error::Error,
        fs, io,
    };

    use cairn_core::{
        change_cob_import_content_root, inspect_change_cob_conflicts, plan_change_cob_resolution,
        ChangeArtifactHeadSelection, ChangeArtifactId, ChangeCobAction, ChangeCobInit,
        ChangeCollaborationResolutionSnapshot, RADICLE_CHANGE_COB_SCHEMA_V1,
    };
    use radicle::{
        cob::{store::access::WriteAs, store::Store as CobStore, ObjectId},
        crypto::{test::signer::MockSigner, Seed},
        node::{device::Device, Alias},
        prelude::ReadRepository,
        profile::{Home, Profile},
        storage::ReadStorage,
        test::{fixtures, setup::NodeWithRepo},
    };

    use super::*;
    use crate::{create_change_import, inspect_change_import};

    const CHANGE_ID: &str = "radicle-conflict-resolution";
    const POLICY_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const PROPOSAL: &str = "# Proposal\n";
    const LEFT_PROPOSAL: &str = "# Left proposal\n";
    const RIGHT_PROPOSAL: &str = "# Right proposal\n";
    const PROFILE_TEST_DIRECTORY_PREFIX: &str = "cairn-radicle-resolution-profile";

    fn create_conflicted_change<Signer>(
        repo: &Repository,
        signer: &Signer,
        change_id: &str,
        policy_hash: &str,
    ) -> Result<ChangeStatusInspection, Box<dyn Error>>
    where
        Signer: crypto::signature::Keypair<VerifyingKey = PublicKey>,
        Signer: crypto::signature::Signer<crypto::Signature>,
        Signer: crypto::signature::Signer<crypto::ssh::ExtendedSignature>,
        Signer: crypto::signature::Verifier<crypto::Signature>,
    {
        let init = ChangeCobInit {
            schema: RADICLE_CHANGE_COB_SCHEMA_V1.to_string(),
            change_id: change_id.to_string(),
            repository_id: repo.id().to_string(),
            base_git_oid: repo.head()?.1.to_string(),
            policy_hash: policy_hash.to_string(),
            artifacts: BTreeMap::from([(ChangeArtifactId::proposal(), PROPOSAL.to_string())]),
        };
        let import = inspect_change_import(repo, &signer.verifying_key(), change_id)?;
        let content_root = change_cob_import_content_root(&init, &import.authority.actor)?;
        let created = create_change_import(repo, signer, init, &import, &content_root)?;
        let object_id = ObjectId::from_str(&created.object_id)?;
        let root_basis = BTreeSet::from([created.object_id]);
        let identity_oid = repo.identity_head()?;
        let mut store = CobStore::<RadicleChangeCob, _, _>::open(repo, WriteAs::new(signer))?
            .identity(identity_oid);
        for (title, content) in [
            ("Left conflicting proposal", LEFT_PROPOSAL),
            ("Right conflicting proposal", RIGHT_PROPOSAL),
        ] {
            store.update(
                &TYPENAME,
                object_id,
                title,
                NonEmpty::new(Action::from(ChangeCobAction::SetArtifact {
                    artifact: ChangeArtifactId::proposal(),
                    content: Some(content.to_string()),
                    basis: root_basis.clone(),
                })),
                Vec::new(),
            )?;
        }
        Ok(inspect_change_status(
            repo,
            &signer.verifying_key(),
            change_id,
        )?)
    }

    fn plan(
        inspection: &ChangeStatusInspection,
        claimed_delegate: bool,
    ) -> Result<ChangeCobResolutionPlan, Box<dyn Error>> {
        let conflicts = inspect_change_cob_conflicts(&inspection.state)?;
        let proposal = conflicts
            .artifact_conflicts
            .iter()
            .find(|conflict| conflict.artifact == ChangeArtifactId::proposal())
            .ok_or("proposal conflict is missing")?;
        let selected = proposal
            .heads
            .iter()
            .find(|head| {
                head.content_blake3.as_deref()
                    == Some(cairn_core::blake3_hex(LEFT_PROPOSAL.as_bytes()).as_str())
            })
            .ok_or("left proposal head is missing")?;
        Ok(plan_change_cob_resolution(
            &inspection.state,
            ChangeCollaborationResolutionSnapshot {
                change_id: inspection.state.change_id.clone(),
                policy_hash: inspection.state.policy_hash.clone(),
                actor: inspection.authority.actor.clone(),
                is_delegate: claimed_delegate,
                artifact_selections: vec![ChangeArtifactHeadSelection {
                    artifact: ChangeArtifactId::proposal(),
                    revision_id: selected.revision_id.clone(),
                }],
                lifecycle_status: None,
            },
        )?)
    }

    // r[verify cairn.collaboration_resolution.radicle_execution.success]
    #[test]
    fn signed_resolution_reloads_conflict_free_content_and_rejects_stale_plan(
    ) -> Result<(), Box<dyn Error>> {
        let fixture = NodeWithRepo::default();
        let inspection =
            create_conflicted_change(&fixture.repo, &fixture.node.signer, CHANGE_ID, POLICY_HASH)?;
        let resolution = plan(&inspection, inspection.authority.is_delegate)?;
        let outcome = create_change_resolution(
            &fixture.repo,
            &fixture.node.signer,
            &resolution,
            &inspection,
        )?;
        assert_eq!(outcome.content_root, resolution.expected_content_root);
        let reloaded =
            inspect_change_status(&fixture.repo, fixture.node.signer.public_key(), CHANGE_ID)?;
        assert!(!inspect_change_cob_conflicts(&reloaded.state)?.has_conflicts());
        assert!(matches!(
            create_change_resolution(
                &fixture.repo,
                &fixture.node.signer,
                &resolution,
                &inspection,
            ),
            Err(ChangeImportError::UpdateContextDrift { .. })
        ));
        Ok(())
    }

    // r[verify cairn.collaboration_resolution.radicle_execution.blocked]
    #[test]
    fn current_identity_rejects_outsider_resolution_claim() -> Result<(), Box<dyn Error>> {
        let fixture = NodeWithRepo::default();
        create_conflicted_change(&fixture.repo, &fixture.node.signer, CHANGE_ID, POLICY_HASH)?;
        let outsider = Device::<MockSigner>::default();
        let inspection = inspect_change_status(&fixture.repo, outsider.public_key(), CHANGE_ID)?;
        let resolution = plan(&inspection, true)?;
        assert!(matches!(
            create_change_resolution(&fixture.repo, &outsider, &resolution, &inspection),
            Err(ChangeImportError::CurrentDelegateRequired { .. })
        ));
        Ok(())
    }

    // r[verify cairn.collaboration_resolution.radicle_execution.success]
    #[test]
    fn explicit_profile_resolution_executes_end_to_end() -> Result<(), Box<dyn Error>> {
        let root = std::env::temp_dir().join(format!(
            "{PROFILE_TEST_DIRECTORY_PREFIX}-{}",
            std::process::id()
        ));
        match fs::remove_dir_all(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        fs::create_dir_all(&root)?;
        let home = Home::new(root.join("rad-home"))?;
        let profile = Profile::init(
            home,
            "cairn-resolution".parse::<Alias>()?,
            None,
            Seed::generate(),
        )?;
        let signer = profile.signer()?;
        let (repository_id, _, _, _) =
            fixtures::project(root.join("working"), &profile.storage, signer.as_ref())?;
        let repo = profile.storage.repository(repository_id)?;
        let inspection = create_conflicted_change(&repo, &signer, CHANGE_ID, POLICY_HASH)?;
        let resolution = plan(&inspection, inspection.authority.is_delegate)?;
        let home_path = profile.home.path().to_path_buf();
        let repository_id_text = repository_id.to_string();
        drop(repo);
        drop(signer);
        drop(profile);

        let outcome =
            execute_profile_resolution(&home_path, &repository_id_text, &resolution, &inspection)?;
        let reloaded = crate::inspect_profile_status(&home_path, &repository_id_text, CHANGE_ID)?;
        assert_eq!(outcome.state_hash, change_cob_state_hash(&reloaded.state)?);
        assert!(!inspect_change_cob_conflicts(&reloaded.state)?.has_conflicts());

        fs::remove_dir_all(&root)?;
        Ok(())
    }
}
