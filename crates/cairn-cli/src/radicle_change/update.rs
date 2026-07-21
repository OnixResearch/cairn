use std::{error::Error, path::PathBuf};

use cairn_core::{
    change_cob_update_plan_hash, plan_change_cob_update, validate_change_name, ChangeCobAction,
    ChangeCobUpdatePlan, ChangeCollaborationUpdateSnapshot,
};
use onix_cairn_radicle::{
    execute_profile_update, inspect_profile_status, ChangeImportError, ChangeStatusInspection,
    ChangeUpdateOutcome,
};
use serde_json::{json, Value};

use crate::{
    collaboration::{
        materialize::RepositoryCheckoutLock, portable_store::read_active_change_artifacts,
    },
    support::{
        default_policy_path, load_policy_file, path_string, print_json, with_portable_receipt_hash,
    },
};

use super::{read_unique_option_value, OPTION_TOKEN_WIDTH};

const RADICLE_BACKEND_V1: &str = "radicle.v1";
const UPDATE_NON_CLAIM_LOCAL: &str =
    "a signed typed update proves bounded local Radicle storage observations, not network publication, peer availability, synchronization, or remote trust";
const UPDATE_NON_CLAIM_LIFECYCLE: &str =
    "collaboration update does not prove gate validity, evidence truth, dependency readiness, source correctness, archive execution, or release eligibility";
const UPDATE_NON_CLAIM_AUTHORITY: &str =
    "current identity and signer checks prove point-in-time local authority observations, not future authorization";
const UPDATE_NON_CLAIM_CONFLICT: &str =
    "ordinary collaboration update refuses conflicts and does not prove or perform conflict resolution";
const BLOCKER_INPUT_DRIFT: &str =
    "active change, policy, current identity, object, or typed state changed after preview";

// r[impl cairn.collaboration_update.radicle_execution]
// r[impl cairn.collaboration_update.receipts]
// r[impl cairn.collaboration_update.non_claims]

#[derive(Debug, Clone, PartialEq, Eq)]
struct RadicleUpdateOptions {
    change: String,
    root: PathBuf,
    rad_home: PathBuf,
    repository_id: String,
    policy: PathBuf,
    execute: bool,
}

pub(super) fn run_update(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = parse_update_options(&args)?;
    let inspection =
        inspect_profile_status(&options.rad_home, &options.repository_id, &options.change)?;
    let policy = load_policy_file(options.policy.clone())?;
    let observed_policy_hash = cairn_core::canonical_json_hash(&policy.raw);
    let preview = match build_update_plan(&options, &inspection, &observed_policy_hash) {
        Ok(plan) => plan,
        Err(error) => {
            return print_json(with_portable_receipt_hash(
                &options.root,
                failure_receipt(&options, error.to_string()),
            ));
        }
    };
    let mut blockers = Vec::new();
    let mut outcome = None;

    if options.execute && !preview.is_noop() {
        match RepositoryCheckoutLock::acquire(&options.root, &options.change) {
            Ok(repository_guard) => {
                let mut fatal_error = None;
                match recheck_update_inputs(&options, &inspection, &preview) {
                    Ok((current, current_plan)) => match execute_profile_update(
                        &options.rad_home,
                        &options.repository_id,
                        &current_plan,
                        &current,
                    ) {
                        Ok(updated) => outcome = Some(updated),
                        Err(error) if is_pre_mutation_blocker(&error) => {
                            blockers.push(error.to_string());
                        }
                        Err(error) => fatal_error = Some(error),
                    },
                    Err(error) => blockers.push(error.to_string()),
                }
                if let Err(error) = repository_guard.release() {
                    blockers.push(error.to_string());
                }
                if let Some(error) = fatal_error {
                    return Err(error.into());
                }
            }
            Err(error) => blockers.push(error.to_string()),
        }
    }
    blockers.sort();
    blockers.dedup();
    let receipt = update_receipt(
        &options,
        &policy.name,
        &observed_policy_hash,
        &inspection,
        &preview,
        &blockers,
        outcome.as_ref(),
    )?;
    print_json(with_portable_receipt_hash(&options.root, receipt))
}

fn parse_update_options(args: &[String]) -> Result<RadicleUpdateOptions, Box<dyn Error>> {
    let Some(change) = args.first() else {
        return Err("radicle update requires a change name".into());
    };
    validate_change_name(change).map_err(|reason| format!("invalid change name: {reason}"))?;
    let mut root = None;
    let mut rad_home = None;
    let mut repository_id = None;
    let mut policy = None;
    let mut execute = false;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--root" => {
                root =
                    Some(read_unique_option_value(args, index, "--root", root.is_some())?.into());
                index = index.saturating_add(OPTION_TOKEN_WIDTH);
            }
            "--rad-home" => {
                rad_home = Some(
                    read_unique_option_value(args, index, "--rad-home", rad_home.is_some())?.into(),
                );
                index = index.saturating_add(OPTION_TOKEN_WIDTH);
            }
            "--rid" => {
                repository_id = Some(read_unique_option_value(
                    args,
                    index,
                    "--rid",
                    repository_id.is_some(),
                )?);
                index = index.saturating_add(OPTION_TOKEN_WIDTH);
            }
            "--policy" => {
                policy = Some(
                    read_unique_option_value(args, index, "--policy", policy.is_some())?.into(),
                );
                index = index.saturating_add(OPTION_TOKEN_WIDTH);
            }
            "--execute" => {
                if execute {
                    return Err("duplicate option: --execute".into());
                }
                execute = true;
                index = index.saturating_add(1);
            }
            unknown => return Err(format!("unknown radicle update argument: {unknown}").into()),
        }
    }
    Ok(RadicleUpdateOptions {
        change: change.clone(),
        root: root.unwrap_or_else(|| PathBuf::from(".")),
        rad_home: rad_home.ok_or("radicle update requires --rad-home PATH")?,
        repository_id: repository_id.ok_or("radicle update requires --rid RID")?,
        policy: policy.unwrap_or_else(|| PathBuf::from(default_policy_path())),
        execute,
    })
}

fn build_update_plan(
    options: &RadicleUpdateOptions,
    inspection: &ChangeStatusInspection,
    policy_hash: &str,
) -> Result<ChangeCobUpdatePlan, Box<dyn Error>> {
    let artifacts = read_active_change_artifacts(&options.root, &options.change)?;
    Ok(plan_change_cob_update(
        &inspection.state,
        ChangeCollaborationUpdateSnapshot {
            change_id: options.change.clone(),
            policy_hash: policy_hash.to_string(),
            actor: inspection.authority.actor.clone(),
            is_delegate: inspection.authority.is_delegate,
            proposal: artifacts.proposal,
            design: artifacts.design,
            tasks: artifacts.tasks,
            metadata: artifacts.metadata,
            delta_specs: artifacts.delta_specs,
        },
    )?)
}

fn recheck_update_inputs(
    options: &RadicleUpdateOptions,
    expected_inspection: &ChangeStatusInspection,
    expected_plan: &ChangeCobUpdatePlan,
) -> Result<(ChangeStatusInspection, ChangeCobUpdatePlan), Box<dyn Error>> {
    let current_policy = load_policy_file(options.policy.clone())?;
    let current_policy_hash = cairn_core::canonical_json_hash(&current_policy.raw);
    let current =
        inspect_profile_status(&options.rad_home, &options.repository_id, &options.change)?;
    let current_plan = build_update_plan(options, &current, &current_policy_hash)?;
    if &current != expected_inspection || &current_plan != expected_plan {
        return Err(BLOCKER_INPUT_DRIFT.into());
    }
    Ok((current, current_plan))
}

fn update_receipt(
    options: &RadicleUpdateOptions,
    policy_name: &str,
    observed_policy_hash: &str,
    inspection: &ChangeStatusInspection,
    plan: &ChangeCobUpdatePlan,
    blockers: &[String],
    outcome: Option<&ChangeUpdateOutcome>,
) -> Result<Value, Box<dyn Error>> {
    let updates = plan
        .actions
        .iter()
        .filter_map(|action| match action {
            ChangeCobAction::SetArtifact {
                artifact,
                content,
                basis,
            } => Some(json!({
                "artifact": artifact.as_str(),
                "basis": basis,
                "content_blake3": content
                    .as_ref()
                    .map(|value| cairn_core::blake3_hex(value.as_bytes())),
                "content_bytes": content.as_ref().map(String::len),
                "removed": content.is_none(),
            })),
            ChangeCobAction::Init { .. }
            | ChangeCobAction::AttachEvidence { .. }
            | ChangeCobAction::SetLifecycle { .. } => None,
        })
        .collect::<Vec<_>>();
    let blocked = !blockers.is_empty();
    Ok(json!({
        "command": "collaboration update",
        "backend": RADICLE_BACKEND_V1,
        "root": path_string(&options.root),
        "change": options.change,
        "workspace_id": inspection.repository_id,
        "base_revision": {
            "kind": "git_oid",
            "value": inspection.repository_head_oid,
        },
        "object_id": inspection.object_id,
        "actor": {
            "id": plan.actor,
            "source": "radicle_profile_and_current_identity",
            "cryptographically_verified": outcome.is_some(),
            "currently_authorized": inspection.authority.is_delegate
                || inspection.authority.actor == inspection.state.creator,
            "authority_class": if inspection.authority.is_delegate {
                "current_delegate"
            } else {
                "signed_creator"
            },
            "canonical_identity_oid": inspection.authority.identity_oid,
        },
        "policy": policy_name,
        "policy_hash": inspection.state.policy_hash,
        "observed_policy_hash": observed_policy_hash,
        "policy_current": inspection.state.policy_hash == observed_policy_hash,
        "input_hash": plan.input_state_hash,
        "plan_hash": change_cob_update_plan_hash(plan)?,
        "operation_plan_id": plan.operation_id,
        "input_state_hash": plan.input_state_hash,
        "input_content_root": plan.input_content_root,
        "expected_content_root": plan.expected_content_root,
        "result_state_hash": outcome.map(|value| value.state_hash.as_str()),
        "result_content_root": outcome.map(|value| value.content_root.as_str()),
        "changed_artifacts": plan.changed_artifacts,
        "updates": updates,
        "noop": plan.is_noop(),
        "dry_run": !options.execute,
        "blocked": blocked,
        "blockers": blockers,
        "mutated": outcome.is_some(),
        "valid": !blocked,
        "verdict": if blocked { "FAIL" } else { "PASS" },
        "non_claims": non_claims(),
    }))
}

fn failure_receipt(options: &RadicleUpdateOptions, blocker: String) -> Value {
    json!({
        "command": "collaboration update",
        "backend": RADICLE_BACKEND_V1,
        "root": path_string(&options.root),
        "change": options.change,
        "dry_run": !options.execute,
        "blocked": true,
        "blockers": [blocker],
        "mutated": false,
        "valid": false,
        "verdict": "FAIL",
        "non_claims": non_claims(),
    })
}

fn is_pre_mutation_blocker(error: &ChangeImportError) -> bool {
    matches!(
        error,
        ChangeImportError::CurrentDelegateRequired { .. }
            | ChangeImportError::ConcurrentUpdate { .. }
            | ChangeImportError::EmptyUpdate
            | ChangeImportError::UpdateContextDrift { .. }
            | ChangeImportError::UpdateActorMismatch { .. }
            | ChangeImportError::ChangeObjectMissing { .. }
            | ChangeImportError::DuplicateChangeObjects { .. }
            | ChangeImportError::SignerMismatch { .. }
    )
}

fn non_claims() -> [&'static str; 4] {
    [
        UPDATE_NON_CLAIM_AUTHORITY,
        UPDATE_NON_CLAIM_LOCAL,
        UPDATE_NON_CLAIM_LIFECYCLE,
        UPDATE_NON_CLAIM_CONFLICT,
    ]
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, fs, io, path::Path};

    use cairn_core::{
        change_cob_import_content_root, change_cob_state_hash, initialize_change_cob,
        ChangeArtifactId, ChangeCobInit, ChangeCobOperation, RADICLE_CHANGE_COB_SCHEMA_V1,
    };
    use onix_cairn_radicle::CurrentDelegateAuthority;

    use super::*;

    const CHANGE_ID: &str = "radicle-update-preview";
    const REPOSITORY_ID: &str = "rad:repository";
    const BASE_GIT_OID: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const IDENTITY_OID: &str = "cccccccccccccccccccccccccccccccccccccccc";
    const ROOT_OPERATION: &str = "1111111111111111111111111111111111111111";
    const ACTOR: &str = "did:key:actor";
    const POLICY_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const TEST_POLICY_RELATIVE: &str = "../../cairn-policy/generated/cairn-policy.json";
    const TEST_DIRECTORY_PREFIX: &str = "cairn-radicle-update-preview";

    fn test_directory() -> PathBuf {
        std::env::temp_dir().join(format!("{TEST_DIRECTORY_PREFIX}-{}", std::process::id()))
    }

    fn remove_directory(path: &Path) -> Result<(), Box<dyn Error>> {
        match fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    fn inspection() -> ChangeStatusInspection {
        let state = initialize_change_cob(ChangeCobOperation {
            id: ROOT_OPERATION.to_string(),
            actor: ACTOR.to_string(),
            is_delegate: true,
            actions: vec![ChangeCobAction::Init {
                state: ChangeCobInit {
                    schema: RADICLE_CHANGE_COB_SCHEMA_V1.to_string(),
                    change_id: CHANGE_ID.to_string(),
                    repository_id: REPOSITORY_ID.to_string(),
                    base_git_oid: BASE_GIT_OID.to_string(),
                    policy_hash: POLICY_HASH.to_string(),
                    artifacts: BTreeMap::from([(
                        ChangeArtifactId::proposal(),
                        "# Proposal\n".to_string(),
                    )]),
                },
            }],
        })
        .unwrap();
        ChangeStatusInspection {
            authority: CurrentDelegateAuthority {
                actor: ACTOR.to_string(),
                identity_oid: IDENTITY_OID.to_string(),
                is_delegate: true,
            },
            repository_id: REPOSITORY_ID.to_string(),
            repository_head_oid: BASE_GIT_OID.to_string(),
            object_id: ROOT_OPERATION.to_string(),
            state,
        }
    }

    fn options(root: &Path, execute: bool) -> RadicleUpdateOptions {
        RadicleUpdateOptions {
            change: CHANGE_ID.to_string(),
            root: root.to_path_buf(),
            rad_home: root.join("radicle"),
            repository_id: REPOSITORY_ID.to_string(),
            policy: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(TEST_POLICY_RELATIVE),
            execute,
        }
    }

    // r[verify cairn.collaboration_update.receipts.modes]
    #[test]
    fn preview_plan_and_receipt_bind_exact_local_update() -> Result<(), Box<dyn Error>> {
        let root = test_directory();
        remove_directory(&root)?;
        let change = root.join("cairn/changes").join(CHANGE_ID);
        fs::create_dir_all(change.join("specs"))?;
        fs::write(change.join("proposal.md"), "# Updated proposal\n")?;
        let inspection = inspection();
        let plan = build_update_plan(&options(&root, false), &inspection, POLICY_HASH)?;
        assert_eq!(plan.changed_artifacts, vec!["proposal"]);
        assert_ne!(plan.input_content_root, plan.expected_content_root);
        let receipt = update_receipt(
            &options(&root, false),
            "default",
            POLICY_HASH,
            &inspection,
            &plan,
            &[],
            None,
        )?;
        assert_eq!(receipt["dry_run"], true);
        assert_eq!(receipt["mutated"], false);
        assert_eq!(receipt["plan_hash"], change_cob_update_plan_hash(&plan)?);
        assert_eq!(
            receipt["input_state_hash"],
            change_cob_state_hash(&inspection.state)?
        );
        assert_eq!(
            receipt["input_content_root"],
            change_cob_import_content_root(
                &ChangeCobInit {
                    schema: RADICLE_CHANGE_COB_SCHEMA_V1.to_string(),
                    change_id: CHANGE_ID.to_string(),
                    repository_id: REPOSITORY_ID.to_string(),
                    base_git_oid: BASE_GIT_OID.to_string(),
                    policy_hash: POLICY_HASH.to_string(),
                    artifacts: BTreeMap::from([(
                        ChangeArtifactId::proposal(),
                        "# Proposal\n".to_string(),
                    )]),
                },
                ACTOR,
            )?
        );
        assert!(receipt["non_claims"].as_array().is_some_and(|claims| claims
            .iter()
            .any(|claim| claim == UPDATE_NON_CLAIM_CONFLICT)));
        remove_directory(&root)?;
        Ok(())
    }

    #[test]
    fn update_arguments_require_explicit_radicle_context() {
        assert!(parse_update_options(&[CHANGE_ID.to_string()]).is_err());
        let parsed = parse_update_options(&[
            CHANGE_ID.to_string(),
            "--rad-home".to_string(),
            "/tmp/radicle".to_string(),
            "--rid".to_string(),
            REPOSITORY_ID.to_string(),
        ])
        .unwrap();
        assert_eq!(parsed.change, CHANGE_ID);
        assert!(!parsed.execute);
    }
}
