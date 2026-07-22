use std::{error::Error, path::PathBuf};

use cairn_core::canonical_json_hash;
use onix_cairn_radicle::{
    execute_profile_sync, inspect_profile_sync, plan_change_sync, ChangeSyncExecutionOutcome,
    ChangeSyncLocalState, ChangeSyncPlan, ChangeSyncSnapshot,
};
use serde_json::{json, Value};

use crate::support::{path_string, print_json, with_portable_receipt_hash};

use super::{read_unique_option_value, OPTION_TOKEN_WIDTH};

const RADICLE_BACKEND_V1: &str = "radicle.v1";
const SYNC_OPERATION_V1: &str = "known_peer_pull.v1";
const NON_CLAIM_PUBLICATION: &str =
    "known-peer pull does not publish, push, seed, follow, announce, or prove peer discovery";
const NON_CLAIM_CONVERGENCE: &str =
    "one observed fetch does not prove bidirectional state equality, availability, eventual convergence, or partition recovery";
const NON_CLAIM_SEMANTICS: &str =
    "typed reload does not resolve conflicts or prove semantic merge correctness, losing-head invalidity, lifecycle approval, evidence truth, working-tree materialization, or release eligibility";
const NON_CLAIM_AUTHORITY: &str =
    "recorded local and peer identities are bounded observations, not future authority or remote-trust leases";

// r[impl cairn.collaboration_sync.preview]
// r[impl cairn.collaboration_sync.receipts]
// r[impl cairn.collaboration_sync.non_claims]

#[derive(Debug, Clone, PartialEq, Eq)]
struct RadicleSyncOptions {
    change: String,
    root: PathBuf,
    rad_home: PathBuf,
    repository_id: String,
    control_socket: PathBuf,
    peer_id: String,
    peer_address: String,
    execute: bool,
}

pub(crate) fn run_sync(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = parse_sync_options(&args)?;
    let inspection =
        inspect_profile_sync(&options.rad_home, &options.repository_id, &options.change)?;
    let plan = plan_change_sync(ChangeSyncSnapshot {
        change_id: options.change.clone(),
        repository_id: inspection.repository_id,
        rad_home: path_string(&options.rad_home),
        control_socket: path_string(&options.control_socket),
        local_actor: inspection.local_actor,
        peer_id: options.peer_id.clone(),
        peer_address: options.peer_address.clone(),
        before: inspection.state,
    })?;
    let plan_hash = canonical_json_hash(&serde_json::to_value(&plan)?);
    let (outcome, blockers) = if options.execute {
        match execute_profile_sync(&options.rad_home, &plan) {
            Ok(outcome) => {
                let blockers = outcome.blockers.clone();
                (Some(outcome), blockers)
            }
            Err(error) => (None, vec![error.to_string()]),
        }
    } else {
        (None, Vec::new())
    };
    let receipt = sync_receipt(&options, &plan, &plan_hash, outcome.as_ref(), &blockers);
    print_json(with_portable_receipt_hash(&options.root, receipt))
}

fn parse_sync_options(args: &[String]) -> Result<RadicleSyncOptions, Box<dyn Error>> {
    let Some(change) = args.first() else {
        return Err("radicle sync requires a change name".into());
    };
    cairn_core::validate_change_name(change)
        .map_err(|reason| format!("invalid change name: {reason}"))?;
    let mut root = None;
    let mut rad_home = None;
    let mut repository_id = None;
    let mut control_socket = None;
    let mut peer_id = None;
    let mut peer_address = None;
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
            "--control-socket" => {
                control_socket = Some(
                    read_unique_option_value(
                        args,
                        index,
                        "--control-socket",
                        control_socket.is_some(),
                    )?
                    .into(),
                );
                index = index.saturating_add(OPTION_TOKEN_WIDTH);
            }
            "--peer-id" => {
                peer_id = Some(read_unique_option_value(
                    args,
                    index,
                    "--peer-id",
                    peer_id.is_some(),
                )?);
                index = index.saturating_add(OPTION_TOKEN_WIDTH);
            }
            "--peer-address" => {
                peer_address = Some(read_unique_option_value(
                    args,
                    index,
                    "--peer-address",
                    peer_address.is_some(),
                )?);
                index = index.saturating_add(OPTION_TOKEN_WIDTH);
            }
            "--execute" => {
                if execute {
                    return Err("duplicate option: --execute".into());
                }
                execute = true;
                index = index.saturating_add(1);
            }
            unknown => return Err(format!("unknown radicle sync argument: {unknown}").into()),
        }
    }
    Ok(RadicleSyncOptions {
        change: change.clone(),
        root: root.unwrap_or_else(|| PathBuf::from(".")),
        rad_home: rad_home.ok_or("radicle sync requires --rad-home PATH")?,
        repository_id: repository_id.ok_or("radicle sync requires --rid RID")?,
        control_socket: control_socket.ok_or("radicle sync requires --control-socket PATH")?,
        peer_id: peer_id.ok_or("radicle sync requires --peer-id NID")?,
        peer_address: peer_address.ok_or("radicle sync requires --peer-address HOST:PORT")?,
        execute,
    })
}

fn sync_receipt(
    options: &RadicleSyncOptions,
    plan: &ChangeSyncPlan,
    plan_hash: &str,
    outcome: Option<&ChangeSyncExecutionOutcome>,
    blockers: &[String],
) -> Value {
    let blocked = !blockers.is_empty();
    let success = options.execute && outcome.is_some_and(|result| result.success) && !blocked;
    let after = outcome.and_then(|result| result.after.as_ref());
    let storage_changed = if options.execute {
        outcome.and_then(|result| result.storage_changed)
    } else {
        Some(false)
    };
    json!({
        "command": "radicle sync",
        "collaboration_command": "collaboration sync",
        "backend": RADICLE_BACKEND_V1,
        "operation": SYNC_OPERATION_V1,
        "root": path_string(&options.root),
        "change": plan.change_id,
        "repository_id": plan.repository_id,
        "rad_home": plan.rad_home,
        "control_socket": plan.control_socket,
        "local_actor": plan.local_actor,
        "peer": {
            "id": plan.peer_id,
            "address": plan.peer_address,
        },
        "actions": plan.actions,
        "before": plan.before,
        "after": after,
        "conflicts_observed": after.is_some_and(ChangeSyncLocalState::has_conflicts),
        "plan_hash": plan_hash,
        "dry_run": !options.execute,
        "execution_requested": options.execute,
        "executed": outcome.is_some(),
        "network_attempted": outcome.is_some_and(|result| result.network_attempted),
        "local_node_id": outcome.and_then(|result| result.local_node_id.as_deref()),
        "connect_status": outcome.map(|result| result.connect_status.as_str()),
        "fetch_status": outcome.map(|result| result.fetch_status.as_str()),
        "ref_updates": outcome.map_or(&[][..], |result| result.ref_updates.as_slice()),
        "namespaces": outcome.map_or(&[][..], |result| result.namespaces.as_slice()),
        "peer_change_ref_present": outcome.and_then(|result| result.peer_change_ref_present),
        "storage_changed": storage_changed,
        "noop": outcome.is_some_and(|result| result.noop),
        "blocked": blocked,
        "blockers": blockers,
        "mutated": storage_changed,
        "valid": if options.execute { success } else { !blocked },
        "verdict": if blocked { "FAIL" } else { "PASS" },
        "non_claims": [
            NON_CLAIM_PUBLICATION,
            NON_CLAIM_CONVERGENCE,
            NON_CLAIM_SEMANTICS,
            NON_CLAIM_AUTHORITY,
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHANGE_ID: &str = "collaboration-sync";
    const REPOSITORY_ID: &str = "rad:z3gqcJUoA1n9HaHKufZs5FCSGazv5";
    const LOCAL_ACTOR: &str = "z6MknSLrJoTcukLrE435hVNQT4JUhbvWLX4kUzqkEStBU8Vi";
    const PEER_ID: &str = "z6MksFqXN3Yhqk8pTJdUGLwATkRfQvwZXPqR2qMEhbS9wzpT";
    const PEER_ADDRESS: &str = "127.0.0.1:8776";
    const RAD_HOME: &str = "/tmp/cairn-sync";
    const CONTROL_SOCKET: &str = "/tmp/cairn-sync/node/control.sock";

    fn args() -> Vec<String> {
        vec![
            CHANGE_ID.to_string(),
            "--rad-home".to_string(),
            RAD_HOME.to_string(),
            "--rid".to_string(),
            REPOSITORY_ID.to_string(),
            "--control-socket".to_string(),
            CONTROL_SOCKET.to_string(),
            "--peer-id".to_string(),
            PEER_ID.to_string(),
            "--peer-address".to_string(),
            PEER_ADDRESS.to_string(),
        ]
    }

    fn options(execute: bool) -> RadicleSyncOptions {
        RadicleSyncOptions {
            change: CHANGE_ID.to_string(),
            root: PathBuf::from("."),
            rad_home: PathBuf::from(RAD_HOME),
            repository_id: REPOSITORY_ID.to_string(),
            control_socket: PathBuf::from(CONTROL_SOCKET),
            peer_id: PEER_ID.to_string(),
            peer_address: PEER_ADDRESS.to_string(),
            execute,
        }
    }

    fn plan() -> Result<ChangeSyncPlan, Box<dyn Error>> {
        Ok(plan_change_sync(ChangeSyncSnapshot {
            change_id: CHANGE_ID.to_string(),
            repository_id: REPOSITORY_ID.to_string(),
            rad_home: RAD_HOME.to_string(),
            control_socket: CONTROL_SOCKET.to_string(),
            local_actor: LOCAL_ACTOR.to_string(),
            peer_id: PEER_ID.to_string(),
            peer_address: PEER_ADDRESS.to_string(),
            before: ChangeSyncLocalState::RepositoryMissing,
        })?)
    }

    // r[verify cairn.collaboration_sync.preview.context]
    // r[verify cairn.collaboration_sync.blocked.socket]
    #[test]
    fn sync_arguments_require_every_explicit_provider_value() -> Result<(), Box<dyn Error>> {
        let parsed = parse_sync_options(&args())?;
        assert_eq!(parsed.change, CHANGE_ID);
        assert!(!parsed.execute);
        for required in [
            "--rad-home",
            "--rid",
            "--control-socket",
            "--peer-id",
            "--peer-address",
        ] {
            let mut incomplete = args();
            let index = incomplete
                .iter()
                .position(|argument| argument == required)
                .ok_or("required fixture option is missing")?;
            incomplete.drain(index..=index.saturating_add(1));
            assert!(
                parse_sync_options(&incomplete).is_err(),
                "accepted missing {required}"
            );
        }
        let mut duplicate = args();
        duplicate.extend(["--peer-id".to_string(), PEER_ID.to_string()]);
        assert!(parse_sync_options(&duplicate).is_err());
        let mut unknown = args();
        unknown.push("--discover".to_string());
        assert!(parse_sync_options(&unknown).is_err());
        Ok(())
    }

    // r[verify cairn.collaboration_sync.receipts.modes]
    // r[verify cairn.collaboration_sync.non_claims.local]
    #[test]
    fn preview_receipt_has_no_provider_result_and_bounds_claims() -> Result<(), Box<dyn Error>> {
        let plan = plan()?;
        let plan_hash = canonical_json_hash(&serde_json::to_value(&plan)?);
        let receipt = sync_receipt(&options(false), &plan, &plan_hash, None, &[]);
        assert_eq!(receipt["dry_run"], true);
        assert_eq!(receipt["executed"], false);
        assert_eq!(receipt["network_attempted"], false);
        assert_eq!(receipt["before"]["status"], "repository_missing");
        assert!(receipt["after"].is_null());
        assert_eq!(receipt["operation"], SYNC_OPERATION_V1);
        assert!(receipt["non_claims"]
            .as_array()
            .is_some_and(|claims| claims.iter().any(|claim| {
                claim
                    .as_str()
                    .is_some_and(|text| text.contains("eventual convergence"))
            })));
        Ok(())
    }

    #[test]
    fn blocked_execution_receipt_does_not_claim_mutation() -> Result<(), Box<dyn Error>> {
        let plan = plan()?;
        let plan_hash = canonical_json_hash(&serde_json::to_value(&plan)?);
        let blockers = vec!["peer connection failed: unavailable".to_string()];
        let receipt = sync_receipt(&options(true), &plan, &plan_hash, None, &blockers);
        assert_eq!(receipt["blocked"], true);
        assert_eq!(receipt["valid"], false);
        assert!(receipt["mutated"].is_null());
        assert_eq!(receipt["verdict"], "FAIL");
        Ok(())
    }
}
