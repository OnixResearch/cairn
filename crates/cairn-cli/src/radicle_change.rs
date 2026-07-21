pub(crate) mod status_checkout;
pub(crate) mod update;

use std::{
    collections::BTreeMap,
    error::Error,
    io,
    path::{Path, PathBuf},
};

use cairn_core::{
    canonical_json_hash, change_cob_import_content_root, plan_change_cob_import,
    validate_change_name, ChangeCobError, ChangeCobImportSnapshot, ChangeCobInit, MAX_ARTIFACTS,
    MAX_ARTIFACT_BYTES, RADICLE_CHANGE_COB_TYPE_NAME,
};
use onix_cairn_radicle::{
    execute_profile_import, inspect_profile_import, ChangeImportError, ChangeImportInspection,
    ChangeImportOutcome,
};
use serde_json::{json, Value};

use crate::support::{
    default_policy_path, load_policy_file, path_string, print_json, with_portable_receipt_hash,
    CapabilityRoot, CapabilityRootKind,
};

const OPTION_TOKEN_WIDTH: usize = 2;
const IMPORT_NON_CLAIM_LOCAL: &str =
    "local typed COB creation does not prove network publication, peer availability, or remote trust";
const IMPORT_NON_CLAIM_LIFECYCLE: &str =
    "signed import does not prove gate validity, evidence truth, dependency readiness, archive execution, or release eligibility";
const IMPORT_NON_CLAIM_AUTHORITY: &str =
    "the recorded canonical identity proves only point-in-time authority and is not a future authorization lease";

// r[impl cairn.radicle_import_cli.command]
// r[impl cairn.radicle_import_cli.receipt]
// r[impl cairn.radicle_import_cli.non_claims]

#[derive(Debug, Clone, PartialEq, Eq)]
struct RadicleImportOptions {
    change: String,
    root: PathBuf,
    rad_home: PathBuf,
    repository_id: String,
    policy: PathBuf,
    execute: bool,
}

pub(crate) fn run_radicle(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let Some(subcommand) = args.first() else {
        return Err("radicle requires a subcommand: import, status, checkout, or update".into());
    };
    match subcommand.as_str() {
        "import" => run_radicle_import(args.into_iter().skip(1).collect()),
        "status" => status_checkout::run_status(args.into_iter().skip(1).collect()),
        "checkout" => status_checkout::run_checkout(args.into_iter().skip(1).collect()),
        "update" => update::run_update(args.into_iter().skip(1).collect()),
        unknown => Err(format!("unknown radicle subcommand: {unknown}").into()),
    }
}

fn run_radicle_import(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = parse_import_options(&args)?;
    let inspection =
        inspect_profile_import(&options.rad_home, &options.repository_id, &options.change)?;
    let policy = load_policy_file(options.policy.clone())?;
    let policy_hash = canonical_json_hash(&policy.raw);
    let snapshot =
        read_active_change_snapshot(&options.root, &options.change, &inspection, &policy_hash)?;
    let init = plan_change_cob_import(snapshot)?;
    let content_root = change_cob_import_content_root(&init, &inspection.authority.actor)?;
    let mut blockers = import_precondition_blockers(&inspection);
    let mut outcome = None;

    if options.execute && blockers.is_empty() {
        let current_policy = load_policy_file(options.policy.clone())?;
        let current_policy_hash = canonical_json_hash(&current_policy.raw);
        let current_snapshot = read_active_change_snapshot(
            &options.root,
            &options.change,
            &inspection,
            &current_policy_hash,
        )?;
        let current_init = plan_change_cob_import(current_snapshot)?;
        if current_init != init {
            blockers.push("active change or policy input changed before execution".to_string());
        } else {
            match execute_profile_import(
                &options.rad_home,
                &options.repository_id,
                current_init,
                &inspection,
                &content_root,
            ) {
                Ok(created) => outcome = Some(created),
                Err(error) if is_pre_mutation_blocker(&error) => blockers.push(error.to_string()),
                Err(error) => return Err(error.into()),
            }
        }
    }

    let receipt = import_receipt(
        &options,
        &inspection,
        &policy.name,
        &init,
        &content_root,
        &blockers,
        outcome.as_ref(),
    )?;
    print_json(with_portable_receipt_hash(&options.root, receipt))
}

fn parse_import_options(args: &[String]) -> Result<RadicleImportOptions, Box<dyn Error>> {
    let Some(change) = args.first() else {
        return Err("radicle import requires a change name".into());
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
            unknown => return Err(format!("unknown radicle import argument: {unknown}").into()),
        }
    }
    let rad_home = rad_home.ok_or("radicle import requires --rad-home PATH")?;
    let repository_id = repository_id.ok_or("radicle import requires --rid RID")?;
    Ok(RadicleImportOptions {
        change: change.clone(),
        root: root.unwrap_or_else(|| PathBuf::from(".")),
        rad_home,
        repository_id,
        policy: policy.unwrap_or_else(|| PathBuf::from(default_policy_path())),
        execute,
    })
}

fn read_unique_option_value(
    args: &[String],
    index: usize,
    name: &str,
    already_present: bool,
) -> Result<String, Box<dyn Error>> {
    if already_present {
        return Err(format!("duplicate option: {name}").into());
    }
    let value_index = index.saturating_add(1);
    let Some(value) = args.get(value_index) else {
        return Err(format!("{name} requires a value").into());
    };
    if value.starts_with("--") {
        return Err(format!("{name} requires a value").into());
    }
    Ok(value.clone())
}

fn read_active_change_snapshot(
    root: &Path,
    change: &str,
    inspection: &ChangeImportInspection,
    policy_hash: &str,
) -> Result<ChangeCobImportSnapshot, Box<dyn Error>> {
    validate_change_name(change).map_err(|reason| format!("invalid change name: {reason}"))?;
    let capability = CapabilityRoot::open(CapabilityRootKind::Repository, root)?;
    let change_dir = PathBuf::from("cairn/changes").join(change);
    let change_metadata = capability.symlink_metadata(&change_dir).map_err(|error| {
        format!(
            "active native change {} is unavailable: {error}",
            change_dir.display()
        )
    })?;
    if change_metadata.file_type().is_symlink() || !change_metadata.is_dir() {
        return Err(format!(
            "active native change path is not a safe directory: {}",
            change_dir.display()
        )
        .into());
    }
    let proposal = read_optional_import_artifact(&capability, &change_dir.join("proposal.md"))?;
    let design = read_optional_import_artifact(&capability, &change_dir.join("design.md"))?;
    let tasks = read_optional_import_artifact(&capability, &change_dir.join("tasks.md"))?;
    let metadata = read_optional_import_artifact(&capability, &change_dir.join("metadata.json"))?;
    let delta_specs = read_delta_specs(&capability, &change_dir.join("specs"))?;
    Ok(ChangeCobImportSnapshot {
        change_id: change.to_string(),
        repository_id: inspection.repository_id.clone(),
        base_git_oid: inspection.base_git_oid.clone(),
        policy_hash: policy_hash.to_string(),
        proposal,
        design,
        tasks,
        metadata,
        delta_specs,
    })
}

fn read_optional_import_artifact(
    capability: &CapabilityRoot,
    relative: &Path,
) -> Result<Option<String>, Box<dyn Error>> {
    let metadata = match capability.symlink_metadata(relative) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!(
            "import artifact is not a safe regular file: {}",
            relative.display()
        )
        .into());
    }
    let max_bytes = u64::try_from(MAX_ARTIFACT_BYTES)?;
    let bytes = capability.read_bounded(relative, max_bytes)?;
    Ok(Some(String::from_utf8(bytes).map_err(|error| {
        format!(
            "import artifact is not UTF-8 at {}: {error}",
            relative.display()
        )
    })?))
}

fn read_delta_specs(
    capability: &CapabilityRoot,
    specs_dir: &Path,
) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let metadata = match capability.symlink_metadata(specs_dir) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(BTreeMap::new()),
        Err(error) => return Err(error.into()),
    };
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(format!(
            "change specs path is not a safe directory: {}",
            specs_dir.display()
        )
        .into());
    }
    let mut entries = capability.entries_bounded(specs_dir, MAX_ARTIFACTS)?;
    entries.sort_by(|left, right| left.file_name().cmp(right.file_name()));
    let mut delta_specs = BTreeMap::new();
    for entry in entries {
        if delta_specs.len() >= MAX_ARTIFACTS {
            return Err(format!(
                "change specs exceed the import artifact limit of {MAX_ARTIFACTS}"
            )
            .into());
        }
        if !entry.file_type().is_dir() {
            return Err(format!(
                "unsupported entry in change specs directory: {}",
                specs_dir.join(entry.file_name()).display()
            )
            .into());
        }
        let spec_id = entry
            .file_name()
            .to_str()
            .ok_or("delta-spec directory name is not UTF-8")?
            .to_string();
        let spec_path = specs_dir.join(entry.file_name()).join("spec.md");
        let content = read_optional_import_artifact(capability, &spec_path)?
            .ok_or_else(|| format!("delta-spec file is missing: {}", spec_path.display()))?;
        delta_specs.insert(spec_id, content);
    }
    Ok(delta_specs)
}

fn import_precondition_blockers(inspection: &ChangeImportInspection) -> Vec<String> {
    let mut blockers = Vec::new();
    if !inspection.authority.is_delegate {
        blockers.push(format!(
            "actor {} is not a delegate in current identity {}",
            inspection.authority.actor, inspection.authority.identity_oid
        ));
    }
    if let Some(object_id) = &inspection.existing_object_id {
        blockers.push(format!("change already has Radicle object {object_id}"));
    }
    blockers
}

fn is_pre_mutation_blocker(error: &ChangeImportError) -> bool {
    matches!(
        error,
        ChangeImportError::CurrentDelegateRequired { .. }
            | ChangeImportError::ConcurrentImport { .. }
            | ChangeImportError::ExistingChangeObject { .. }
            | ChangeImportError::DuplicateChangeObjects { .. }
            | ChangeImportError::ContextDrift { .. }
            | ChangeImportError::RepositoryMismatch { .. }
            | ChangeImportError::BaseGitObjectMismatch { .. }
            | ChangeImportError::SignerMismatch { .. }
            | ChangeImportError::ContentRootDrift { .. }
    )
}

fn import_receipt(
    options: &RadicleImportOptions,
    inspection: &ChangeImportInspection,
    policy_name: &str,
    init: &ChangeCobInit,
    content_root: &str,
    blockers: &[String],
    outcome: Option<&ChangeImportOutcome>,
) -> Result<Value, Box<dyn Error>> {
    let artifacts = init
        .artifacts
        .iter()
        .map(|(artifact, content)| {
            Ok(json!({
                "id": artifact.as_str(),
                "path": artifact.relative_path()?,
                "blake3": cairn_core::blake3_hex(content.as_bytes()),
                "bytes": content.len(),
            }))
        })
        .collect::<Result<Vec<_>, ChangeCobError>>()?;
    let input_hash = canonical_json_hash(&serde_json::to_value(init)?);
    let plan_hash = canonical_json_hash(&json!({
        "type_name": RADICLE_CHANGE_COB_TYPE_NAME,
        "change": options.change,
        "repository_id": inspection.repository_id,
        "actor": inspection.authority.actor,
        "identity_oid": inspection.authority.identity_oid,
        "base_git_oid": inspection.base_git_oid,
        "policy": policy_name,
        "policy_hash": init.policy_hash,
        "input_hash": input_hash,
        "content_root": content_root,
    }));
    let blocked = !blockers.is_empty();
    Ok(json!({
        "command": "radicle import",
        "root": path_string(&options.root),
        "change": options.change,
        "type_name": RADICLE_CHANGE_COB_TYPE_NAME,
        "repository_id": inspection.repository_id,
        "actor": inspection.authority.actor,
        "current_identity_oid": inspection.authority.identity_oid,
        "currently_authorized": inspection.authority.is_delegate,
        "base_git_oid": inspection.base_git_oid,
        "policy": policy_name,
        "policy_hash": init.policy_hash,
        "artifacts": artifacts,
        "artifact_count": init.artifacts.len(),
        "input_hash": input_hash,
        "plan_hash": plan_hash,
        "content_root": content_root,
        "dry_run": !options.execute,
        "blocked": blocked,
        "blockers": blockers,
        "mutated": outcome.is_some(),
        "object_id": outcome.map(|created| created.object_id.clone()),
        "state_hash": outcome.map(|created| created.state_hash.clone()),
        "valid": !blocked,
        "verdict": if blocked { "FAIL" } else { "PASS" },
        "non_claims": [
            IMPORT_NON_CLAIM_LOCAL,
            IMPORT_NON_CLAIM_LIFECYCLE,
            IMPORT_NON_CLAIM_AUTHORITY,
        ],
    }))
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, fs};

    use super::*;

    const TEST_DIRECTORY_PREFIX: &str = "cairn-radicle-import-cli";
    const REPOSITORY_ID: &str = "rad:z3example";
    const BASE_GIT_OID: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const POLICY_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const ACTOR: &str = "z6Mkexample";
    const IDENTITY_OID: &str = "cccccccccccccccccccccccccccccccccccccccc";

    fn test_directory(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{TEST_DIRECTORY_PREFIX}-{name}-{}",
            std::process::id()
        ))
    }

    fn remove_test_directory(path: &Path) -> Result<(), Box<dyn Error>> {
        match fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    fn inspection() -> ChangeImportInspection {
        ChangeImportInspection {
            authority: onix_cairn_radicle::CurrentDelegateAuthority {
                actor: ACTOR.to_string(),
                identity_oid: IDENTITY_OID.to_string(),
                is_delegate: true,
            },
            repository_id: REPOSITORY_ID.to_string(),
            base_git_oid: BASE_GIT_OID.to_string(),
            existing_object_id: None,
        }
    }

    // r[verify cairn.radicle_import_cli.command.preview]
    // r[verify cairn.radicle_import_cli.command.explicit_context]
    #[test]
    fn import_arguments_require_explicit_radicle_context() {
        let parsed = parse_import_options(&[
            "demo".to_string(),
            "--rad-home".to_string(),
            "/tmp/radicle".to_string(),
            "--rid".to_string(),
            REPOSITORY_ID.to_string(),
        ])
        .unwrap();
        assert_eq!(parsed.change, "demo");
        assert!(!parsed.execute);
        assert!(parse_import_options(&["demo".to_string()]).is_err());
        assert!(parse_import_options(&[
            "demo".to_string(),
            "--rad-home".to_string(),
            "/tmp/radicle".to_string(),
        ])
        .is_err());
        assert!(parse_import_options(&[
            "demo".to_string(),
            "--rad-home".to_string(),
            "/tmp/radicle".to_string(),
            "--rid".to_string(),
            REPOSITORY_ID.to_string(),
            "--unknown".to_string(),
        ])
        .is_err());
    }

    // r[verify cairn.radicle_import_cli.receipt]
    // r[verify cairn.radicle_import_cli.receipt.blocked]
    // r[verify cairn.radicle_import_cli.non_claims.local_only]
    #[test]
    fn import_receipt_binds_preview_and_blocked_non_claims() -> Result<(), Box<dyn Error>> {
        let options = RadicleImportOptions {
            change: "demo".to_string(),
            root: PathBuf::from("."),
            rad_home: PathBuf::from("/tmp/radicle"),
            repository_id: REPOSITORY_ID.to_string(),
            policy: PathBuf::from(default_policy_path()),
            execute: false,
        };
        let init = plan_change_cob_import(ChangeCobImportSnapshot {
            change_id: options.change.clone(),
            repository_id: REPOSITORY_ID.to_string(),
            base_git_oid: BASE_GIT_OID.to_string(),
            policy_hash: POLICY_HASH.to_string(),
            proposal: Some("# Proposal\n".to_string()),
            design: None,
            tasks: None,
            metadata: None,
            delta_specs: BTreeMap::new(),
        })?;
        let allowed = inspection();
        let content_root = change_cob_import_content_root(&init, ACTOR)?;
        let receipt = with_portable_receipt_hash(
            &options.root,
            import_receipt(
                &options,
                &allowed,
                "cairn-default",
                &init,
                &content_root,
                &[],
                None,
            )?,
        );
        assert_eq!(receipt.get("valid").and_then(Value::as_bool), Some(true));
        assert_eq!(
            receipt
                .get("receipt_hash")
                .and_then(Value::as_str)
                .map(str::len),
            Some(cairn_core::BLAKE3_HEX_LENGTH)
        );
        assert!(receipt
            .get("non_claims")
            .and_then(Value::as_array)
            .is_some_and(|claims| claims.iter().any(|claim| claim == IMPORT_NON_CLAIM_LOCAL)));

        let mut denied = allowed;
        denied.authority.is_delegate = false;
        let blockers = import_precondition_blockers(&denied);
        let blocked = import_receipt(
            &options,
            &denied,
            "cairn-default",
            &init,
            &content_root,
            &blockers,
            None,
        )?;
        assert_eq!(blocked.get("blocked").and_then(Value::as_bool), Some(true));
        assert_eq!(blocked.get("mutated").and_then(Value::as_bool), Some(false));
        Ok(())
    }

    // r[verify cairn.radicle_import_cli.lossless_plan.exact]
    #[test]
    fn active_change_snapshot_reads_exact_supported_bytes() -> Result<(), Box<dyn Error>> {
        let root = test_directory("exact");
        remove_test_directory(&root)?;
        let change_dir = root.join("cairn/changes/demo");
        fs::create_dir_all(change_dir.join("specs/demo"))?;
        fs::write(change_dir.join("proposal.md"), "# Proposal\n\nExact.  \n")?;
        fs::write(change_dir.join("design.md"), "# Design\n")?;
        fs::write(change_dir.join("tasks.md"), "# Tasks\n")?;
        fs::write(
            change_dir.join("metadata.json"),
            "{\n  \"depends_on\": []\n}\n",
        )?;
        fs::write(change_dir.join("specs/demo/spec.md"), "# Delta\n")?;

        let snapshot = read_active_change_snapshot(&root, "demo", &inspection(), POLICY_HASH)?;
        assert_eq!(
            snapshot.proposal.as_deref(),
            Some("# Proposal\n\nExact.  \n")
        );
        assert_eq!(
            snapshot.delta_specs.get("demo").map(String::as_str),
            Some("# Delta\n")
        );
        let init = plan_change_cob_import(snapshot)?;
        assert_eq!(
            init.artifacts.len(),
            init.artifacts.keys().collect::<BTreeSet<_>>().len()
        );

        fs::remove_dir_all(root)?;
        Ok(())
    }

    // r[verify cairn.radicle_import_cli.receipt.blocked]
    #[test]
    fn active_change_snapshot_rejects_unsupported_spec_entries() -> Result<(), Box<dyn Error>> {
        let root = test_directory("unsupported");
        remove_test_directory(&root)?;
        let change_dir = root.join("cairn/changes/demo");
        fs::create_dir_all(change_dir.join("specs"))?;
        fs::write(change_dir.join("specs/loose.md"), "not a delta spec\n")?;

        let result = read_active_change_snapshot(&root, "demo", &inspection(), POLICY_HASH);
        assert!(result.is_err());

        fs::remove_dir_all(root)?;
        Ok(())
    }
}
