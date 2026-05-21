use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use cairn_core::{
    build_archive_plan, build_openspec_migration_plan, build_sync_plan, canonical_json_hash,
    evaluate_gate_snapshot, GateStage, LifecycleLayout, Policy,
};
use serde_json::{json, Value};

use crate::agent_receipt::{
    agent_receipt_validation_receipt, is_blake3_hex_text, read_agent_receipt_file,
};
use crate::lifecycle::parse_gate_stage;
use crate::mcp::{mcp_call_tool, mcp_tool_catalog};
use crate::migration::collect_layout_migration_inputs;
use crate::native::{cairn_validation_receipt, change_status_json, repo_status_json};
use crate::plan_receipts::{
    archive_plan_json, gate_input_json, gate_report_json, migration_plan_json, sync_plan_json,
};
use crate::release_readiness::release_readiness_receipt;
use crate::{
    collect_files, default_policy_path, detect_layout, file_content_manifest, find_change,
    has_flag, load_policy_file, mutation_gate_blockers, path_string, portable_json_hash,
    portable_text, print_json, read_gate_snapshot, read_option, read_string_option, scan_repo,
    with_portable_receipt_hash,
};

mod coverage;
mod replay;
mod verify;

pub(crate) use coverage::{determinism_coverage_audit_receipt, determinism_surfaces_from_policy};
pub(crate) use replay::{
    determinism_replay_receipt, replay_cases_from_policy, replay_groups_from_policy, ReplayCase,
    ReplayGroup,
};
pub(crate) use verify::determinism_verify_replay_receipt;

pub(crate) fn run_determinism(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("determinism requires a subcommand: replay".into());
    };
    match subcommand.as_str() {
        "coverage-audit" => coverage::run_determinism_coverage_audit(iter.collect()),
        "replay" => replay::run_determinism_replay(iter.collect()),
        "verify-receipt" => verify::run_determinism_verify_receipt(iter.collect()),
        unknown => Err(format!("unknown determinism subcommand: {unknown}").into()),
    }
}
