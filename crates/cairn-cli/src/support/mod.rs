use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

// r[verify cli_maintainability.main_compat_release_followup.behavior_preserved]
// r[verify cli_maintainability.main_compat_release_followup.behavior_preserved.focused_checks]
// r[verify cli_maintainability.main_compat_release_followup.behavior_preserved.repository_checks]
use cairn_core::{
    archive_dir, blake3_hex, build_repo_status, canonical_json_hash, changes_dir,
    evaluate_gate_snapshot, merge_delta_spec, parse_policy_json, specs_dir, validate_spec_sources,
    ChangeSource, ChangeStatus, GateArtifact, GateSnapshot, GateStage, LifecycleLayout, Policy,
    RepoStatus, SpecSource, SyncPlan,
};
use serde_json::{json, Value};

mod args;
mod layout;
mod mutation;
mod paths;
mod receipts;
mod scan;
mod usage;

pub(crate) use args::{has_flag, read_option, read_string_option};
pub(crate) use layout::{default_policy_path, detect_layout, load_policy_file};
pub(crate) use mutation::{
    archive_mutation_manifest, execute_archive_plan, execute_sync_plan, sync_mutation_manifest,
    sync_mutation_paths,
};
pub(crate) use paths::{
    collect_files, file_content_manifest, path_string, portable_json_hash, portable_text,
    safe_relative_path, tree_content_paths,
};
pub(crate) use receipts::{print_json, with_portable_receipt_hash, with_receipt_hash};
pub(crate) use scan::{
    collect_spec_sources, collect_validation_spec_sources, find_change, find_spec_source,
    mutation_gate_blockers, read_gate_snapshot, scan_repo,
};
pub(crate) use usage::{print_usage, usage_text};

// r[impl cli_maintainability.support_helper_submodule_split.focused_modules]
// r[impl cli_maintainability.support_helper_submodule_split.focused_modules.visible_boundaries]
// r[impl cli_maintainability.support_helper_submodule_split.focused_modules.narrow_bridges]
// r[verify cli_maintainability.support_helper_submodule_split.behavior_preserved]
// r[verify cli_maintainability.support_helper_submodule_split.behavior_preserved.focused_checks]
// r[verify cli_maintainability.support_helper_submodule_split.behavior_preserved.repository_checks]
