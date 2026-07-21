use std::{error::Error, path::PathBuf};

use cairn_core::{
    change_cob_resolution_plan_hash, inspect_change_cob_conflict_value,
    inspect_change_cob_conflicts, plan_change_cob_resolution, validate_change_name,
    ChangeArtifactHeadSelection, ChangeArtifactId, ChangeCobAction, ChangeCobResolutionPlan,
    ChangeCollaborationConflictInspection, ChangeCollaborationResolutionSnapshot,
    ChangeLifecycleStatus, MAX_OPERATION_ID_BYTES,
};
use onix_cairn_radicle::{
    execute_profile_resolution, inspect_profile_status, ChangeImportError, ChangeResolutionOutcome,
    ChangeStatusInspection,
};
use serde_json::{json, Value};

use crate::{
    collaboration::materialize::RepositoryCheckoutLock,
    support::{
        default_policy_path, load_policy_file, path_string, print_json, with_portable_receipt_hash,
    },
};

use super::{read_unique_option_value, OPTION_TOKEN_WIDTH};

const RADICLE_BACKEND_V1: &str = "radicle.v1";
const BLOCKER_INPUT_DRIFT: &str =
    "policy, current identity, object, typed state, conflict heads, or resolution choices changed after preview";
const NON_CLAIM_AUTHORITY: &str =
    "current identity and signer checks prove point-in-time local authority observations, not future authorization";
const NON_CLAIM_LOCAL: &str =
    "signed conflict resolution proves bounded local Radicle storage observations only";
const NON_CLAIM_MERGE: &str =
    "selecting a conflict head does not prove semantic merge correctness or that losing heads are invalid";
const NON_CLAIM_LIFECYCLE: &str =
    "an explicit lifecycle reset to open does not prove review, approval, evidence truth, source correctness, archive execution, or release eligibility";
const NON_CLAIM_TRANSPORT: &str =
    "local signed Radicle resolution does not prove network publication, peer convergence, availability, or remote trust";
const NON_CLAIM_COUNT: usize = 5;

// r[impl cairn.collaboration_resolution.radicle_execution]
// r[impl cairn.collaboration_resolution.receipts]
// r[impl cairn.collaboration_resolution.non_claims]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RadicleResolutionCommand {
    Conflicts,
    Resolve,
}

impl RadicleResolutionCommand {
    fn as_str(self) -> &'static str {
        match self {
            Self::Conflicts => "conflicts",
            Self::Resolve => "resolve",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RadicleResolutionOptions {
    command: RadicleResolutionCommand,
    change: String,
    root: PathBuf,
    rad_home: PathBuf,
    repository_id: String,
    policy: PathBuf,
    show_head: Option<ChangeArtifactHeadSelection>,
    selections: Vec<ChangeArtifactHeadSelection>,
    lifecycle_status: Option<ChangeLifecycleStatus>,
    execute: bool,
}

pub(super) fn run_conflicts(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = parse_options(RadicleResolutionCommand::Conflicts, &args)?;
    let inspection =
        inspect_profile_status(&options.rad_home, &options.repository_id, &options.change)?;
    let policy = load_policy_file(options.policy.clone())?;
    let observed_policy_hash = cairn_core::canonical_json_hash(&policy.raw);
    let conflicts = inspect_change_cob_conflicts(&inspection.state)?;
    let shown_head = match &options.show_head {
        Some(selection) => match inspect_change_cob_conflict_value(
            &inspection.state,
            &selection.artifact,
            &selection.revision_id,
        ) {
            Ok(value) => Some(value),
            Err(error) => {
                return print_json(with_portable_receipt_hash(
                    &options.root,
                    failure_receipt(&options, error.to_string()),
                ));
            }
        },
        None => None,
    };
    let receipt = conflict_receipt(
        &options,
        &policy.name,
        &observed_policy_hash,
        &inspection,
        &conflicts,
        shown_head.map(serde_json::to_value).transpose()?,
    );
    print_json(with_portable_receipt_hash(&options.root, receipt))
}

pub(super) fn run_resolve(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = parse_options(RadicleResolutionCommand::Resolve, &args)?;
    let inspection =
        inspect_profile_status(&options.rad_home, &options.repository_id, &options.change)?;
    let policy = load_policy_file(options.policy.clone())?;
    let observed_policy_hash = cairn_core::canonical_json_hash(&policy.raw);
    let (conflicts, preview) =
        match build_resolution_plan(&options, &inspection, &observed_policy_hash) {
            Ok(value) => value,
            Err(error) => {
                return print_json(with_portable_receipt_hash(
                    &options.root,
                    failure_receipt(&options, error.to_string()),
                ));
            }
        };
    let mut blockers = Vec::new();
    let mut outcome = None;

    if options.execute {
        match RepositoryCheckoutLock::acquire(&options.root, &options.change) {
            Ok(repository_guard) => {
                let mut fatal_error = None;
                match recheck_resolution_inputs(&options, &inspection, &preview) {
                    Ok((current, current_plan)) => match execute_profile_resolution(
                        &options.rad_home,
                        &options.repository_id,
                        &current_plan,
                        &current,
                    ) {
                        Ok(resolved) => outcome = Some(resolved),
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
    let receipt = resolution_receipt(
        &options,
        &policy.name,
        &observed_policy_hash,
        &inspection,
        &conflicts,
        &preview,
        &blockers,
        outcome.as_ref(),
    )?;
    print_json(with_portable_receipt_hash(&options.root, receipt))
}

fn build_resolution_plan(
    options: &RadicleResolutionOptions,
    inspection: &ChangeStatusInspection,
    policy_hash: &str,
) -> Result<
    (
        ChangeCollaborationConflictInspection,
        ChangeCobResolutionPlan,
    ),
    Box<dyn Error>,
> {
    let conflicts = inspect_change_cob_conflicts(&inspection.state)?;
    let plan = plan_change_cob_resolution(
        &inspection.state,
        ChangeCollaborationResolutionSnapshot {
            change_id: options.change.clone(),
            policy_hash: policy_hash.to_string(),
            actor: inspection.authority.actor.clone(),
            is_delegate: inspection.authority.is_delegate,
            artifact_selections: options.selections.clone(),
            lifecycle_status: options.lifecycle_status,
        },
    )?;
    Ok((conflicts, plan))
}

fn recheck_resolution_inputs(
    options: &RadicleResolutionOptions,
    expected_inspection: &ChangeStatusInspection,
    expected_plan: &ChangeCobResolutionPlan,
) -> Result<(ChangeStatusInspection, ChangeCobResolutionPlan), Box<dyn Error>> {
    let current_policy = load_policy_file(options.policy.clone())?;
    let current_policy_hash = cairn_core::canonical_json_hash(&current_policy.raw);
    let current =
        inspect_profile_status(&options.rad_home, &options.repository_id, &options.change)?;
    let (_, current_plan) = build_resolution_plan(options, &current, &current_policy_hash)?;
    if &current != expected_inspection || &current_plan != expected_plan {
        return Err(BLOCKER_INPUT_DRIFT.into());
    }
    Ok((current, current_plan))
}

fn conflict_receipt(
    options: &RadicleResolutionOptions,
    policy_name: &str,
    observed_policy_hash: &str,
    inspection: &ChangeStatusInspection,
    conflicts: &ChangeCollaborationConflictInspection,
    shown_head: Option<Value>,
) -> Value {
    json!({
        "command": "collaboration conflicts",
        "backend": RADICLE_BACKEND_V1,
        "root": path_string(&options.root),
        "change": options.change,
        "workspace_id": inspection.repository_id,
        "base_revision": {
            "kind": "git_oid",
            "value": inspection.repository_head_oid,
        },
        "object_id": inspection.object_id,
        "actor": actor_observation(inspection, false),
        "policy": policy_name,
        "policy_hash": inspection.state.policy_hash,
        "observed_policy_hash": observed_policy_hash,
        "policy_current": inspection.state.policy_hash == observed_policy_hash,
        "state_hash": conflicts.state_hash,
        "conflicted": conflicts.has_conflicts(),
        "artifact_conflicts": conflicts.artifact_conflicts,
        "lifecycle_heads": conflicts.lifecycle_heads,
        "shown_head": shown_head,
        "blocked": false,
        "blockers": [],
        "mutated": false,
        "valid": true,
        "verdict": "PASS",
        "non_claims": non_claims(),
    })
}

#[allow(clippy::too_many_arguments)]
fn resolution_receipt(
    options: &RadicleResolutionOptions,
    policy_name: &str,
    observed_policy_hash: &str,
    inspection: &ChangeStatusInspection,
    conflicts: &ChangeCollaborationConflictInspection,
    plan: &ChangeCobResolutionPlan,
    blockers: &[String],
    outcome: Option<&ChangeResolutionOutcome>,
) -> Result<Value, Box<dyn Error>> {
    let blocked = !blockers.is_empty();
    Ok(json!({
        "command": "collaboration resolve",
        "backend": RADICLE_BACKEND_V1,
        "root": path_string(&options.root),
        "change": options.change,
        "workspace_id": inspection.repository_id,
        "base_revision": {
            "kind": "git_oid",
            "value": inspection.repository_head_oid,
        },
        "object_id": inspection.object_id,
        "actor": actor_observation(inspection, outcome.is_some()),
        "policy": policy_name,
        "policy_hash": inspection.state.policy_hash,
        "observed_policy_hash": observed_policy_hash,
        "policy_current": inspection.state.policy_hash == observed_policy_hash,
        "input_hash": plan.input_state_hash,
        "plan_hash": change_cob_resolution_plan_hash(plan)?,
        "operation_plan_id": plan.operation_id,
        "input_state_hash": plan.input_state_hash,
        "expected_content_root": plan.expected_content_root,
        "result_state_hash": outcome.map(|value| value.state_hash.as_str()),
        "result_content_root": outcome.map(|value| value.content_root.as_str()),
        "artifact_conflicts": conflicts.artifact_conflicts,
        "lifecycle_heads": conflicts.lifecycle_heads,
        "selected_artifacts": plan.selected_artifacts,
        "lifecycle_reset": plan.lifecycle_reset,
        "actions": resolution_actions(&plan.actions),
        "dry_run": !options.execute,
        "blocked": blocked,
        "blockers": blockers,
        "mutated": outcome.is_some(),
        "valid": !blocked,
        "verdict": if blocked { "FAIL" } else { "PASS" },
        "non_claims": non_claims(),
    }))
}

fn resolution_actions(actions: &[ChangeCobAction]) -> Vec<Value> {
    actions
        .iter()
        .filter_map(|action| match action {
            ChangeCobAction::SetArtifact {
                artifact,
                content,
                basis,
            } => Some(json!({
                "kind": "set_artifact",
                "artifact": artifact.as_str(),
                "basis": basis,
                "content_blake3": content
                    .as_ref()
                    .map(|value| cairn_core::blake3_hex(value.as_bytes())),
                "content_bytes": content.as_ref().map(String::len),
                "removed": content.is_none(),
            })),
            ChangeCobAction::SetLifecycle {
                status,
                subject_root,
                evidence,
                basis,
            } => Some(json!({
                "kind": "set_lifecycle",
                "status": lifecycle_status_name(*status),
                "subject_root": subject_root,
                "evidence": evidence,
                "basis": basis,
            })),
            ChangeCobAction::Init { .. } | ChangeCobAction::AttachEvidence { .. } => None,
        })
        .collect()
}

fn actor_observation(inspection: &ChangeStatusInspection, signed: bool) -> Value {
    json!({
        "id": inspection.authority.actor,
        "source": "radicle_profile_and_current_identity",
        "cryptographically_verified": signed,
        "currently_authorized": inspection.authority.is_delegate
            || inspection.authority.actor == inspection.state.creator,
        "authority_class": if inspection.authority.is_delegate {
            "current_delegate"
        } else {
            "signed_creator"
        },
        "canonical_identity_oid": inspection.authority.identity_oid,
    })
}

fn failure_receipt(options: &RadicleResolutionOptions, blocker: String) -> Value {
    json!({
        "command": format!("collaboration {}", options.command.as_str()),
        "backend": RADICLE_BACKEND_V1,
        "root": path_string(&options.root),
        "change": options.change,
        "dry_run": options.command == RadicleResolutionCommand::Resolve && !options.execute,
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

fn parse_options(
    command: RadicleResolutionCommand,
    args: &[String],
) -> Result<RadicleResolutionOptions, Box<dyn Error>> {
    let Some(change) = args.first() else {
        return Err(format!("radicle {} requires a change name", command.as_str()).into());
    };
    validate_change_name(change).map_err(|reason| format!("invalid change name: {reason}"))?;
    let mut root = None;
    let mut rad_home = None;
    let mut repository_id = None;
    let mut policy = None;
    let mut show_head = None;
    let mut selections = Vec::new();
    let mut lifecycle_status = None;
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
            "--show-head" if command == RadicleResolutionCommand::Conflicts => {
                let value =
                    read_unique_option_value(args, index, "--show-head", show_head.is_some())?;
                show_head = Some(parse_head_selection(&value)?);
                index = index.saturating_add(OPTION_TOKEN_WIDTH);
            }
            "--choose-head" if command == RadicleResolutionCommand::Resolve => {
                let value = read_unique_option_value(args, index, "--choose-head", false)?;
                selections.push(parse_head_selection(&value)?);
                index = index.saturating_add(OPTION_TOKEN_WIDTH);
            }
            "--lifecycle" if command == RadicleResolutionCommand::Resolve => {
                let value = read_unique_option_value(
                    args,
                    index,
                    "--lifecycle",
                    lifecycle_status.is_some(),
                )?;
                lifecycle_status = Some(parse_lifecycle_status(&value)?);
                index = index.saturating_add(OPTION_TOKEN_WIDTH);
            }
            "--execute" if command == RadicleResolutionCommand::Resolve => {
                if execute {
                    return Err("duplicate option: --execute".into());
                }
                execute = true;
                index = index.saturating_add(1);
            }
            unknown => {
                return Err(
                    format!("unknown radicle {} argument: {unknown}", command.as_str()).into(),
                )
            }
        }
    }
    Ok(RadicleResolutionOptions {
        command,
        change: change.clone(),
        root: root.unwrap_or_else(|| PathBuf::from(".")),
        rad_home: rad_home
            .ok_or_else(|| format!("radicle {} requires --rad-home PATH", command.as_str()))?,
        repository_id: repository_id
            .ok_or_else(|| format!("radicle {} requires --rid RID", command.as_str()))?,
        policy: policy.unwrap_or_else(|| PathBuf::from(default_policy_path())),
        show_head,
        selections,
        lifecycle_status,
        execute,
    })
}

fn parse_head_selection(value: &str) -> Result<ChangeArtifactHeadSelection, Box<dyn Error>> {
    let Some((artifact, revision_id)) = value.split_once('=') else {
        return Err("head selector must use ARTIFACT=REVISION".into());
    };
    if revision_id.is_empty() {
        return Err("head selector revision must not be empty".into());
    }
    if revision_id.len() > MAX_OPERATION_ID_BYTES {
        return Err(
            format!("head selector revision exceeds {MAX_OPERATION_ID_BYTES} bytes").into(),
        );
    }
    Ok(ChangeArtifactHeadSelection {
        artifact: ChangeArtifactId::parse(artifact.to_string())?,
        revision_id: revision_id.to_string(),
    })
}

fn parse_lifecycle_status(value: &str) -> Result<ChangeLifecycleStatus, Box<dyn Error>> {
    match value {
        "open" => Ok(ChangeLifecycleStatus::Open),
        _ => Err("conflict resolution supports only --lifecycle open".into()),
    }
}

fn lifecycle_status_name(status: ChangeLifecycleStatus) -> &'static str {
    match status {
        ChangeLifecycleStatus::Open => "open",
        ChangeLifecycleStatus::ReviewRequested => "review_requested",
        ChangeLifecycleStatus::Approved => "approved",
        ChangeLifecycleStatus::ArchiveRequested => "archive_requested",
        ChangeLifecycleStatus::Archived => "archived",
    }
}

fn non_claims() -> [&'static str; NON_CLAIM_COUNT] {
    [
        NON_CLAIM_AUTHORITY,
        NON_CLAIM_LOCAL,
        NON_CLAIM_MERGE,
        NON_CLAIM_LIFECYCLE,
        NON_CLAIM_TRANSPORT,
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use cairn_core::{
        apply_change_cob_operation, initialize_change_cob, ChangeCobInit, ChangeCobOperation,
        RADICLE_CHANGE_COB_SCHEMA_V1,
    };
    use onix_cairn_radicle::CurrentDelegateAuthority;

    use super::*;

    const CHANGE_ID: &str = "radicle-resolution-cli";
    const REPOSITORY_ID: &str = "rad:repository";
    const BASE_GIT_OID: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const IDENTITY_OID: &str = "cccccccccccccccccccccccccccccccccccccccc";
    const ROOT_OPERATION: &str = "1111111111111111111111111111111111111111";
    const LEFT_OPERATION: &str = "2222222222222222222222222222222222222222";
    const RIGHT_OPERATION: &str = "3333333333333333333333333333333333333333";
    const ACTOR: &str = "did:key:actor";
    const POLICY_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

    fn inspection() -> ChangeStatusInspection {
        let mut state = initialize_change_cob(ChangeCobOperation {
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
        let basis = BTreeSet::from([ROOT_OPERATION.to_string()]);
        for (operation_id, content) in
            [(LEFT_OPERATION, "# Left\n"), (RIGHT_OPERATION, "# Right\n")]
        {
            apply_change_cob_operation(
                &mut state,
                ChangeCobOperation {
                    id: operation_id.to_string(),
                    actor: ACTOR.to_string(),
                    is_delegate: true,
                    actions: vec![ChangeCobAction::SetArtifact {
                        artifact: ChangeArtifactId::proposal(),
                        content: Some(content.to_string()),
                        basis: basis.clone(),
                    }],
                },
            )
            .unwrap();
        }
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

    fn options(command: RadicleResolutionCommand) -> RadicleResolutionOptions {
        RadicleResolutionOptions {
            command,
            change: CHANGE_ID.to_string(),
            root: PathBuf::from("."),
            rad_home: PathBuf::from("/tmp/radicle"),
            repository_id: REPOSITORY_ID.to_string(),
            policy: PathBuf::from("policy.json"),
            show_head: None,
            selections: if command == RadicleResolutionCommand::Resolve {
                vec![ChangeArtifactHeadSelection {
                    artifact: ChangeArtifactId::proposal(),
                    revision_id: LEFT_OPERATION.to_string(),
                }]
            } else {
                Vec::new()
            },
            lifecycle_status: None,
            execute: false,
        }
    }

    // r[verify cairn.collaboration_resolution.receipts.modes]
    #[test]
    fn radicle_conflict_and_resolution_receipts_bind_heads_choices_and_non_claims(
    ) -> Result<(), Box<dyn Error>> {
        let inspection = inspection();
        let conflicts = inspect_change_cob_conflicts(&inspection.state)?;
        let conflict = conflict_receipt(
            &options(RadicleResolutionCommand::Conflicts),
            "default",
            POLICY_HASH,
            &inspection,
            &conflicts,
            None,
        );
        assert_eq!(conflict["conflicted"], true);
        assert_eq!(conflict["mutated"], false);

        let resolve_options = options(RadicleResolutionCommand::Resolve);
        let (_, plan) = build_resolution_plan(&resolve_options, &inspection, POLICY_HASH)?;
        let receipt = resolution_receipt(
            &resolve_options,
            "default",
            POLICY_HASH,
            &inspection,
            &conflicts,
            &plan,
            &[],
            None,
        )?;
        assert_eq!(
            receipt["selected_artifacts"][0]["revision_id"],
            LEFT_OPERATION
        );
        assert_eq!(receipt["dry_run"], true);
        assert_eq!(receipt["mutated"], false);
        assert!(receipt["non_claims"]
            .as_array()
            .is_some_and(|claims| claims.iter().any(|claim| claim == NON_CLAIM_MERGE)));
        Ok(())
    }

    #[test]
    fn radicle_resolution_arguments_require_explicit_context_and_safe_choices() {
        let args = [
            CHANGE_ID.to_string(),
            "--rad-home".to_string(),
            "/tmp/radicle".to_string(),
            "--rid".to_string(),
            REPOSITORY_ID.to_string(),
            "--choose-head".to_string(),
            format!("proposal={LEFT_OPERATION}"),
        ];
        let parsed = parse_options(RadicleResolutionCommand::Resolve, &args).unwrap();
        assert_eq!(parsed.selections.len(), 1);
        assert!(
            parse_options(RadicleResolutionCommand::Resolve, &[CHANGE_ID.to_string()]).is_err()
        );
        assert!(parse_options(
            RadicleResolutionCommand::Conflicts,
            &[
                CHANGE_ID.to_string(),
                "--rad-home".to_string(),
                "/tmp/radicle".to_string(),
                "--rid".to_string(),
                REPOSITORY_ID.to_string(),
                "--execute".to_string(),
            ],
        )
        .is_err());
        assert!(parse_head_selection("proposal").is_err());
        assert!(parse_lifecycle_status("approved").is_err());
    }
}
