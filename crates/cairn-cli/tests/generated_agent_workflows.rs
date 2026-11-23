use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use serde_json::Value;

const ADAPTER_SKILLS: &str = "agent-skills";
const SKILLS_DESTINATION: &str = ".agents/skills";
const EXPECTED_WORKFLOW_COUNT: usize = 6;
const EXPECTED_GENERATED_FILE_COUNT: usize = 7;
const MODIFIED_CONTENT: &str = "operator-owned content\n";
const UPDATE_GOAL_SUFFIX: &str = " Use the reviewed update policy.";

// r[verify cairn.generated_agent_workflows.descriptors]
// r[verify cairn.generated_agent_workflows.descriptors.valid]
// r[verify cairn.generated_agent_workflows.descriptors.invalid]
// r[verify cairn.generated_agent_workflows.catalog]
// r[verify cairn.generated_agent_workflows.catalog.list]
// r[verify cairn.generated_agent_workflows.catalog.contracts]
// r[verify cairn.generated_agent_workflows.explore]
// r[verify cairn.generated_agent_workflows.explore.valid]
// r[verify cairn.generated_agent_workflows.explore.invalid]
// r[verify cairn.generated_agent_workflows.adapters]
// r[verify cairn.generated_agent_workflows.adapters.valid]
// r[verify cairn.generated_agent_workflows.adapters.authority]
// r[verify cairn.generated_agent_workflows.adapters.unknown]
// r[verify cairn.generated_agent_workflows.commands]
// r[verify cairn.generated_agent_workflows.commands.read]
// r[verify cairn.generated_agent_workflows.commands.destination]
// r[verify cairn.generated_agent_workflows.manifest]
// r[verify cairn.generated_agent_workflows.manifest.plan]
// r[verify cairn.generated_agent_workflows.manifest.freshness]
// r[verify cairn.generated_agent_workflows.install]
// r[verify cairn.generated_agent_workflows.install.fresh]
// r[verify cairn.generated_agent_workflows.install.conflict]
// r[verify cairn.generated_agent_workflows.install.obsolete]
// r[verify cairn.generated_agent_workflows.boundary]
// r[verify cairn.generated_agent_workflows.boundary.nonclaim]
// r[verify cairn.generated_agent_workflows.boundary.effects]
// r[verify cairn.generated_agent_workflows.functional_boundary]
// r[verify cairn.generated_agent_workflows.functional_boundary.pure]
// r[verify cairn.generated_agent_workflows.functional_boundary.shell]
// r[verify cairn.generated_agent_workflows.compatibility]
// r[verify cairn.generated_agent_workflows.compatibility.existing]
// r[verify cairn.generated_agent_workflows.verification]
// r[verify cairn.generated_agent_workflows.verification.suites]
// r[verify cairn.generated_agent_workflows.docs]
// r[verify cairn.generated_agent_workflows.docs.install]
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repository root")
        .to_path_buf()
}

fn policy_path() -> PathBuf {
    repo_root().join("cairn-policy/generated/cairn-policy.json")
}

fn test_root(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!(
        "cairn-generated-agent-workflows-{name}-{}",
        std::process::id()
    ));
    match fs::remove_dir_all(&root) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    fs::create_dir_all(root.join(".cairn"))?;
    fs::create_dir_all(root.join(".cairn/specs"))?;
    fs::create_dir_all(root.join(".cairn/changes"))?;
    fs::create_dir_all(root.join(".cairn/archive"))?;
    Ok(root)
}

fn cairn() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
    command
        .current_dir(repo_root())
        .env_remove("CAIRN_GLOBAL_REGISTRY");
    command
}

fn run(args: &[&str]) -> Result<(Output, Option<Value>), Box<dyn Error>> {
    let output = cairn().args(args).output()?;
    let value = if output.stdout.is_empty() {
        None
    } else {
        Some(serde_json::from_slice(&output.stdout)?)
    };
    Ok((output, value))
}

fn destination_args(root: &Path, command: &str, adapter: &str, destination: &str) -> Vec<String> {
    vec![
        "agent".to_string(),
        "workflow".to_string(),
        command.to_string(),
        "--root".to_string(),
        root.display().to_string(),
        "--policy".to_string(),
        policy_path().display().to_string(),
        "--adapter".to_string(),
        adapter.to_string(),
        "--destination".to_string(),
        destination.to_string(),
    ]
}

fn run_owned(args: &[String]) -> Result<(Output, Option<Value>), Box<dyn Error>> {
    let borrowed = args.iter().map(String::as_str).collect::<Vec<_>>();
    run(&borrowed)
}

fn install(root: &Path, adapter: &str, destination: &str) -> Result<Value, Box<dyn Error>> {
    let preview_args = destination_args(root, "install", adapter, destination);
    let (preview_output, preview) = run_owned(&preview_args)?;
    assert!(preview_output.status.success());
    let preview = preview.ok_or("missing preview")?;
    let plan_hash = preview["plan_hash"].as_str().ok_or("missing plan hash")?;
    let mut execute_args = preview_args;
    execute_args.extend([
        "--execute".to_string(),
        "--plan-hash".to_string(),
        plan_hash.to_string(),
    ]);
    let (output, receipt) = run_owned(&execute_args)?;
    if !output.status.success() {
        return Err(format!(
            "install failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    receipt.ok_or_else(|| "missing install receipt".into())
}

#[test]
fn lists_descriptors_and_plans_each_adapter_without_writes() -> Result<(), Box<dyn Error>> {
    let policy = policy_path().display().to_string();
    let (list_output, list) = run(&["agent", "workflow", "list", "--policy", &policy])?;
    assert!(list_output.status.success());
    let list = list.ok_or("missing list receipt")?;
    assert_eq!(
        list["workflows"].as_array().map(Vec::len),
        Some(EXPECTED_WORKFLOW_COUNT)
    );
    assert_eq!(list["adapters"].as_array().map(Vec::len), Some(3));
    let (show_output, show) = run(&["agent", "workflow", "show", "explore", "--policy", &policy])?;
    assert!(show_output.status.success());
    let show = show.ok_or("missing show receipt")?;
    assert_eq!(show["workflow"]["mutation_class"], "read_only");
    assert!(show["workflow"]["constraints"]
        .as_array()
        .is_some_and(|values| values.iter().any(|value| value
            .as_str()
            .is_some_and(|text| text.contains("Do not create a change")))));

    for (adapter, destination, expected_suffix) in [
        ("agent-skills", ".agents/skills", "/SKILL.md"),
        ("claude", ".claude/commands", ".md"),
        ("codex", ".codex/prompts", ".md"),
    ] {
        let root = test_root(adapter)?;
        let args = destination_args(&root, "plan", adapter, destination);
        let (output, receipt) = run_owned(&args)?;
        assert!(output.status.success());
        let receipt = receipt.ok_or("missing plan receipt")?;
        assert_eq!(receipt["dry_run"], true);
        assert_eq!(receipt["mutated"], false);
        assert!(receipt["files"].as_array().is_some_and(|files| {
            files.len() == EXPECTED_GENERATED_FILE_COUNT
                && files.iter().all(|file| {
                    file["relative_path"]
                        .as_str()
                        .is_some_and(|path| path.ends_with(expected_suffix))
                })
        }));
        assert!(!root.join(destination).exists());
    }
    Ok(())
}

#[test]
fn machine_mode_preserves_catalog_receipt_identity() -> Result<(), Box<dyn Error>> {
    let policy = policy_path().display().to_string();
    let (default_output, default_receipt) =
        run(&["agent", "workflow", "list", "--policy", &policy])?;
    assert!(default_output.status.success());
    let default_receipt = default_receipt.ok_or("default catalog receipt")?;
    let (machine_output, machine) = run(&[
        "--machine",
        "agent",
        "workflow",
        "list",
        "--policy",
        &policy,
    ])?;
    assert!(machine_output.status.success());
    let machine = machine.ok_or("machine envelope")?;
    assert_eq!(machine["ok"], true);
    assert_eq!(
        machine["result"]["receipt_hash"],
        default_receipt["receipt_hash"]
    );
    Ok(())
}

#[test]
// r[verify cairn.agent_entry_guidance.install]
fn installs_and_checks_manifest_owned_files() -> Result<(), Box<dyn Error>> {
    let root = test_root("install-remove")?;
    let installed = install(&root, ADAPTER_SKILLS, SKILLS_DESTINATION)?;
    assert_eq!(installed["executed"], true);
    assert_eq!(installed["mutated"], true);
    let files = installed["files"].as_array().ok_or("missing files")?;
    assert_eq!(files.len(), EXPECTED_GENERATED_FILE_COUNT);
    for file in files {
        let path = file["relative_path"].as_str().ok_or("missing path")?;
        let contents = fs::read_to_string(root.join(path))?;
        assert!(contents.starts_with("---\nname: cairn-"));
        assert!(contents.contains("## Non-claims"));
    }

    let check_args = destination_args(&root, "check", ADAPTER_SKILLS, SKILLS_DESTINATION);
    let (check_output, check) = run_owned(&check_args)?;
    assert!(check_output.status.success());
    assert_eq!(check.ok_or("missing check")?["current"], true);

    for file in files {
        let path = file["relative_path"].as_str().ok_or("missing path")?;
        assert!(root.join(path).exists());
    }
    Ok(())
}

#[test]
fn managed_policy_update_replaces_only_stale_generated_files() -> Result<(), Box<dyn Error>> {
    let root = test_root("managed-update")?;
    let installed = install(&root, "codex", ".codex/prompts")?;
    let files = installed["files"].as_array().ok_or("installed files")?;
    let unchanged_path = files
        .iter()
        .find(|file| file["workflow_id"] == "verify")
        .and_then(|file| file["relative_path"].as_str())
        .ok_or("unchanged path")?;
    let unchanged_before = fs::read(root.join(unchanged_path))?;

    let mut policy: Value = serde_json::from_slice(&fs::read(policy_path())?)?;
    let descriptors = policy["generated_agent_workflow_policy"]["descriptors"]
        .as_array_mut()
        .ok_or("descriptors")?;
    let apply = descriptors
        .iter_mut()
        .find(|descriptor| descriptor["id"] == "apply")
        .ok_or("apply descriptor")?;
    let goal = apply["goal"].as_str().ok_or("apply goal")?.to_string();
    apply["goal"] = Value::String(format!("{goal}{UPDATE_GOAL_SUFFIX}"));
    let updated_policy_path = root.join("updated-policy.json");
    fs::write(&updated_policy_path, serde_json::to_vec_pretty(&policy)?)?;

    let mut preview_args = destination_args(&root, "install", "codex", ".codex/prompts");
    let policy_index = preview_args
        .iter()
        .position(|value| value == "--policy")
        .ok_or("policy option")?;
    preview_args[policy_index + 1] = updated_policy_path.display().to_string();
    let (preview_output, preview) = run_owned(&preview_args)?;
    assert!(preview_output.status.success());
    let preview = preview.ok_or("updated preview")?;
    assert!(preview["files"].as_array().is_some_and(|files| files
        .iter()
        .any(|file| file["workflow_id"] == "apply" && file["freshness"] == "stale-source")));
    let mut execute_args = preview_args;
    execute_args.extend([
        "--execute".to_string(),
        "--plan-hash".to_string(),
        preview["plan_hash"]
            .as_str()
            .ok_or("updated plan")?
            .to_string(),
    ]);
    let (execute_output, receipt) = run_owned(&execute_args)?;
    assert!(
        execute_output.status.success(),
        "{}",
        String::from_utf8_lossy(&execute_output.stderr)
    );
    let receipt = receipt.ok_or("updated receipt")?;
    let apply_path = receipt["files"]
        .as_array()
        .and_then(|files| files.iter().find(|file| file["workflow_id"] == "apply"))
        .and_then(|file| file["relative_path"].as_str())
        .ok_or("updated apply path")?;
    assert!(fs::read_to_string(root.join(apply_path))?.contains(UPDATE_GOAL_SUFFIX));
    assert_eq!(fs::read(root.join(unchanged_path))?, unchanged_before);
    Ok(())
}

#[test]
fn stale_plan_and_modified_managed_file_fail_without_replacement() -> Result<(), Box<dyn Error>> {
    let root = test_root("stale-modified")?;
    let preview_args = destination_args(&root, "install", "claude", ".claude/commands");
    let (preview_output, preview) = run_owned(&preview_args)?;
    assert!(preview_output.status.success());
    let preview = preview.ok_or("missing preview")?;
    let first_path = preview["files"][0]["relative_path"]
        .as_str()
        .ok_or("missing path")?;
    fs::create_dir_all(root.join(first_path).parent().ok_or("missing parent")?)?;
    fs::write(root.join(first_path), MODIFIED_CONTENT)?;
    let mut stale_args = preview_args.clone();
    stale_args.extend([
        "--execute".to_string(),
        "--plan-hash".to_string(),
        preview["plan_hash"]
            .as_str()
            .ok_or("missing plan")?
            .to_string(),
    ]);
    let (stale_output, _) = run_owned(&stale_args)?;
    assert!(!stale_output.status.success());
    assert!(String::from_utf8_lossy(&stale_output.stderr).contains("generated_workflow.plan.stale"));
    assert_eq!(fs::read_to_string(root.join(first_path))?, MODIFIED_CONTENT);
    assert_eq!(fs::read_dir(root.join(".claude/commands"))?.count(), 1);

    fs::remove_dir_all(root.join(".claude"))?;
    let installed = install(&root, "claude", ".claude/commands")?;
    let managed_path = installed["files"][0]["relative_path"]
        .as_str()
        .ok_or("missing managed path")?;
    fs::write(root.join(managed_path), MODIFIED_CONTENT)?;
    let (blocked_output, blocked) = run_owned(&preview_args)?;
    assert!(blocked_output.status.success());
    let blocked = blocked.ok_or("missing blocked plan")?;
    assert_eq!(blocked["blocked"], true);
    assert!(blocked["files"]
        .as_array()
        .is_some_and(|files| files.iter().any(|file| file["freshness"] == "modified")));
    assert_eq!(
        fs::read_to_string(root.join(managed_path))?,
        MODIFIED_CONTENT
    );
    Ok(())
}

#[test]
fn rejects_unsafe_destination_unknown_adapter_and_ambient_home_path() -> Result<(), Box<dyn Error>>
{
    let root = test_root("unsafe")?;
    for (adapter, destination) in [
        ("agent-skills", "../home"),
        ("agent-skills", "~/.agents/skills"),
        ("remote-package", ".agents/skills"),
    ] {
        let args = destination_args(&root, "plan", adapter, destination);
        let (output, receipt) = run_owned(&args)?;
        assert!(!output.status.success());
        assert!(receipt.is_none());
    }
    assert_eq!(fs::read_dir(&root)?.count(), 1);
    Ok(())
}

#[cfg(unix)]
#[test]
fn symlink_destination_is_blocked_without_outside_writes() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;

    let root = test_root("symlink")?;
    let outside = test_root("symlink-outside")?;
    symlink(&outside, root.join(".agents"))?;
    let args = destination_args(&root, "install", ADAPTER_SKILLS, SKILLS_DESTINATION);
    let (output, preview) = run_owned(&args)?;
    assert!(output.status.success());
    let preview = preview.ok_or("missing preview")?;
    assert_eq!(preview["blocked"], true);
    assert!(preview["files"].as_array().is_some_and(|files| files
        .iter()
        .all(|file| file["freshness"] == "symlink-conflict")));
    assert_eq!(fs::read_dir(&outside)?.count(), 1);
    Ok(())
}

#[test]
fn maintained_hermes_source_remains_byte_compatible() {
    // The reviewed Octet rename changes the maintained source baseline.
    // Generated workflow operations must still preserve those exact bytes.
    const EXPECTED_HERMES_BLAKE3: &str =
        "079a6dabe0b1894058e1c16ad277ce15e83b0b55821cbe9f452bb413c48f5045";
    let bytes = include_bytes!("../../../docs/hermes/cairn-workflow-skill.md");
    assert_eq!(
        blake3::hash(bytes).to_hex().as_str(),
        EXPECTED_HERMES_BLAKE3
    );
}

#[test]
fn plan_receipts_are_checkout_independent_and_do_not_expose_roots() -> Result<(), Box<dyn Error>> {
    let first = test_root("portable-first")?;
    let second = test_root("portable-second")?;
    let first_args = destination_args(&first, "plan", "codex", ".codex/prompts");
    let second_args = destination_args(&second, "plan", "codex", ".codex/prompts");
    let (_, first_receipt) = run_owned(&first_args)?;
    let (_, second_receipt) = run_owned(&second_args)?;
    let first_receipt = first_receipt.ok_or("missing first receipt")?;
    let second_receipt = second_receipt.ok_or("missing second receipt")?;
    assert_eq!(first_receipt["plan_hash"], second_receipt["plan_hash"]);
    assert_eq!(
        first_receipt["receipt_hash"],
        second_receipt["receipt_hash"]
    );
    let serialized = serde_json::to_string(&first_receipt)?;
    assert!(!serialized.contains(&first.display().to_string()));
    assert!(!serialized.contains("HOME"));
    assert!(!serialized.contains("raw_prompt"));
    Ok(())
}

#[test]
// r[verify cairn.agent_entry_guidance.generation]
// r[verify cairn.agent_entry_guidance.conformance]
fn entry_guide_show_renders_routes_for_every_adapter_and_refuses_unknown_selections(
) -> Result<(), Box<dyn Error>> {
    let policy = policy_path().display().to_string();
    let expected_routes = [
        "cairn status --root PATH",
        "cairn change list --root PATH",
        "cairn workflow status <change> --root PATH",
        "cairn context --root PATH",
        "cairn doctor --root PATH",
        "cairn validate --root PATH",
        "cairn gate tasks <change> --root PATH",
        "cairn workflow instructions <artifact> --change <change> --root PATH",
        "cairn change create <name> --profile <profile> --root PATH",
    ];
    for adapter in ["agent-skills", "claude", "codex"] {
        let (output, receipt) = run(&[
            "agent",
            "workflow",
            "show",
            "entry",
            "--policy",
            &policy,
            "--adapter",
            adapter,
        ])?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let receipt = receipt.ok_or("missing entry receipt")?;
        let contents = receipt["file"]["contents"]
            .as_str()
            .ok_or("missing entry contents")?;
        assert!(contents.contains(".cairn/"));
        assert!(!contents.contains("cairn/changes"));
        for route in expected_routes {
            assert!(contents.contains(route), "{adapter} misses {route}");
        }
        for profile in [
            "spec-driven",
            "assurance",
            "no-spec-delta",
            "outcome-machine",
        ] {
            assert!(contents.contains(profile));
        }
        assert!(receipt["command_catalog_identity"].is_string());
        assert_eq!(receipt["workflow"], "entry");
    }

    let (unknown_adapter_output, _) = run(&[
        "agent",
        "workflow",
        "show",
        "entry",
        "--policy",
        &policy,
        "--adapter",
        "remote-agent",
    ])?;
    assert!(!unknown_adapter_output.status.success());
    let (unknown_profile_output, _) = run(&[
        "agent",
        "workflow",
        "show",
        "entry",
        "--policy",
        &policy,
        "--adapter",
        "codex",
        "--profile",
        "remote-profile",
    ])?;
    assert!(!unknown_profile_output.status.success());
    let (non_entry_selection, _) = run(&[
        "agent",
        "workflow",
        "show",
        "explore",
        "--policy",
        &policy,
        "--adapter",
        "codex",
    ])?;
    assert!(!non_entry_selection.status.success());
    Ok(())
}

#[test]
// r[verify cairn.agent_entry_guidance.examples]
fn inspect_copy_reports_identity_bound_state_without_writing() -> Result<(), Box<dyn Error>> {
    let root = test_root("inspect-copy")?;
    let installed = install(&root, "claude", ".claude/commands")?;
    let entry_path = installed["files"]
        .as_array()
        .and_then(|files| files.iter().find(|file| file["workflow_id"] == "entry"))
        .and_then(|file| file["relative_path"].as_str())
        .ok_or("installed entry")?;
    let manifest_path = installed["manifest"]["relative_path"]
        .as_str()
        .ok_or("manifest path")?;
    let identity = installed["manifest"]["identity"]
        .as_str()
        .ok_or("manifest identity")?;
    let copied_guide = root.join("copied-entry.md");
    let copied_manifest = root.join("copied-manifest.json");
    fs::copy(root.join(entry_path), &copied_guide)?;
    fs::copy(root.join(manifest_path), &copied_manifest)?;
    let policy = policy_path().display().to_string();
    let inspect = |guide: &Path, manifest: &Path, identity: Option<&str>| {
        let mut args = vec![
            "agent".to_string(),
            "workflow".to_string(),
            "inspect-copy".to_string(),
            "--root".to_string(),
            root.display().to_string(),
            "--policy".to_string(),
            policy.clone(),
            "--guide".to_string(),
            guide
                .strip_prefix(&root)
                .unwrap_or(guide)
                .display()
                .to_string(),
            "--manifest".to_string(),
            manifest
                .strip_prefix(&root)
                .unwrap_or(manifest)
                .display()
                .to_string(),
        ];
        if let Some(identity) = identity {
            args.extend(["--manifest-identity".to_string(), identity.to_string()]);
        }
        run_owned(&args)
    };
    let (output, receipt) = inspect(&copied_guide, &copied_manifest, Some(identity))?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt = receipt.ok_or("inspection receipt")?;
    assert_eq!(receipt["report_only"], true);
    assert_eq!(receipt["mutated"], false);
    assert_eq!(receipt["inspection"]["classification"], "current");
    assert_eq!(receipt["inspection"]["native_layout_current"], true);

    fs::write(&copied_guide, MODIFIED_CONTENT)?;
    let (_, modified) = inspect(&copied_guide, &copied_manifest, Some(identity))?;
    assert_eq!(
        modified.ok_or("modified receipt")?["inspection"]["classification"],
        "modified"
    );

    fs::write(&copied_guide, "Read cairn/changes before starting.\n")?;
    let (_, legacy) = inspect(&copied_guide, &copied_manifest, Some(identity))?;
    let legacy = legacy.ok_or("legacy receipt")?;
    assert_eq!(legacy["inspection"]["native_layout_current"], false);
    assert!(legacy["inspection"]["issues"]
        .as_array()
        .is_some_and(|issues| issues
            .iter()
            .any(|issue| issue["code"] == "generated_workflow.guide.layout.stale")));

    let wrong = "a".repeat(64);
    let (_, wrong_identity) = inspect(&copied_guide, &copied_manifest, Some(wrong.as_str()))?;
    assert_eq!(
        wrong_identity.ok_or("wrong identity receipt")?["inspection"]["classification"],
        "unclassified"
    );
    let (_, no_identity) = inspect(&copied_guide, &copied_manifest, None)?;
    assert_eq!(
        no_identity.ok_or("no identity receipt")?["inspection"]["classification"],
        "unclassified"
    );
    assert_eq!(
        fs::read_to_string(&copied_guide)?,
        "Read cairn/changes before starting.\n"
    );
    assert!(copied_manifest.exists());
    Ok(())
}

#[test]
// r[verify cairn.agent_entry_guidance.scope]
fn modified_entry_guide_blocks_and_preserves_bytes() -> Result<(), Box<dyn Error>> {
    let root = test_root("modified-entry")?;
    let installed = install(&root, "claude", ".claude/commands")?;
    let entry_path = installed["files"]
        .as_array()
        .and_then(|files| files.iter().find(|file| file["workflow_id"] == "entry"))
        .and_then(|file| file["relative_path"].as_str())
        .ok_or("installed entry")?;
    let preview_args = destination_args(&root, "install", "claude", ".claude/commands");
    let (preview_output, preview) = run_owned(&preview_args)?;
    assert!(preview_output.status.success());
    let plan_hash = preview
        .ok_or("preview")?
        .get("plan_hash")
        .and_then(Value::as_str)
        .ok_or("plan hash")?
        .to_string();
    fs::write(root.join(entry_path), MODIFIED_CONTENT)?;
    let mut execute_args = preview_args.clone();
    execute_args.extend([
        "--execute".to_string(),
        "--plan-hash".to_string(),
        plan_hash,
    ]);
    let (execute_output, _) = run_owned(&execute_args)?;
    assert!(!execute_output.status.success());
    assert!(
        String::from_utf8_lossy(&execute_output.stderr).contains("generated_workflow.plan.stale")
    );
    assert_eq!(fs::read_to_string(root.join(entry_path))?, MODIFIED_CONTENT);

    let (check_output, check) = run_owned(&destination_args(
        &root,
        "check",
        "claude",
        ".claude/commands",
    ))?;
    assert!(check_output.status.success());
    let check = check.ok_or("check receipt")?;
    assert_eq!(check["blocked"], true);
    assert!(check["files"].as_array().is_some_and(|files| files
        .iter()
        .any(|file| file["workflow_id"] == "entry" && file["freshness"] == "modified")));
    assert_eq!(fs::read_to_string(root.join(entry_path))?, MODIFIED_CONTENT);
    Ok(())
}

#[test]
fn maintained_guidance_examples_match_current_command_contracts() -> Result<(), Box<dyn Error>> {
    use cairn_core::contains_legacy_layout_reference;
    use std::collections::BTreeSet;

    let policy_text = fs::read_to_string(policy_path())?;
    let policy = cairn_core::parse_policy_json(&policy_text)?;
    let catalog =
        cairn_core::admit_generated_agent_workflow_policy(&policy.generated_agent_workflow_policy)
            .map_err(|issues| format!("policy admission: {issues:?}"))?;
    let fixture: Value = serde_json::from_slice(&fs::read(
        repo_root().join("fixtures/generated-agent-guidance/examples.json"),
    )?)?;
    let routes = catalog
        .policy
        .entry
        .discovery_routes
        .iter()
        .chain(catalog.policy.entry.readiness_routes.iter())
        .cloned()
        .chain([
            catalog.policy.entry.stage_route.clone(),
            catalog.policy.entry.profile_route.clone(),
        ])
        .collect::<BTreeSet<String>>();
    let examples = fixture["examples"].as_array().ok_or("examples array")?;
    let mut accepted = 0;
    let mut covered_routes = BTreeSet::new();
    for example in examples {
        let expect = example["expect"].as_str().ok_or("example expectation")?;
        let expect_accepted = expect == "accepted";
        let matched = match example["kind"].as_str().ok_or("example kind")? {
            "command" => {
                let id = example["command_id"].as_str().ok_or("command id")?;
                let invocation = example["invocation"].as_str().ok_or("invocation")?;
                let command = catalog
                    .policy
                    .command_catalog
                    .iter()
                    .find(|command| command.id == id);
                let matches = command.is_some_and(|command| command.invocation == invocation);
                if matches {
                    assert!(routes.contains(id), "example {id} is not an entry route");
                    covered_routes.insert(id.to_string());
                }
                matches
            }
            "command_identity" => {
                let invocation = example["invocation"].as_str().ok_or("invocation")?;
                catalog
                    .policy
                    .command_catalog
                    .iter()
                    .any(|command| command.invocation == invocation)
            }
            "layout" => {
                let text = example["text"].as_str().ok_or("layout text")?;
                !contains_legacy_layout_reference(text)
            }
            "profile" => {
                let profile = example["profile"].as_str().ok_or("profile")?;
                catalog
                    .policy
                    .entry
                    .profiles
                    .iter()
                    .any(|value| value == profile)
            }
            "adapter" => {
                let adapter = example["adapter"].as_str().ok_or("adapter")?;
                catalog
                    .policy
                    .adapters
                    .iter()
                    .any(|value| value.id == adapter)
            }
            other => return Err(format!("unknown example kind: {other}").into()),
        };
        assert_eq!(
            matched,
            expect_accepted,
            "example {} did not match its expected outcome",
            example["id"].as_str().unwrap_or("<unknown>")
        );
        if expect_accepted {
            accepted += 1;
        }
    }
    assert!(accepted > 0);
    assert_eq!(covered_routes, routes);
    Ok(())
}

#[test]
fn command_catalog_drift_updates_only_the_stale_entry_guide() -> Result<(), Box<dyn Error>> {
    let root = test_root("catalog-drift")?;
    let installed = install(&root, "codex", ".codex/prompts")?;
    let entry_path = installed["files"]
        .as_array()
        .and_then(|files| files.iter().find(|file| file["workflow_id"] == "entry"))
        .and_then(|file| file["relative_path"].as_str())
        .ok_or("installed entry")?
        .to_string();
    let unchanged_path = installed["files"]
        .as_array()
        .and_then(|files| files.iter().find(|file| file["workflow_id"] == "verify"))
        .and_then(|file| file["relative_path"].as_str())
        .ok_or("installed verify")?
        .to_string();
    let entry_before = fs::read(root.join(&entry_path))?;
    let verify_before = fs::read(root.join(&unchanged_path))?;

    let mut policy: Value = serde_json::from_slice(&fs::read(policy_path())?)?;
    let commands = policy["generated_agent_workflow_policy"]["command_catalog"]
        .as_array_mut()
        .ok_or("command catalog")?;
    commands
        .iter_mut()
        .find(|command| command["id"] == "status")
        .ok_or("status command")?["invocation"] =
        Value::String("cairn status --root . --format json".to_string());
    let updated_policy_path = root.join("updated-policy.json");
    fs::write(&updated_policy_path, serde_json::to_vec_pretty(&policy)?)?;

    let mut preview_args = destination_args(&root, "install", "codex", ".codex/prompts");
    let policy_index = preview_args
        .iter()
        .position(|value| value == "--policy")
        .ok_or("policy option")?;
    preview_args[policy_index + 1] = updated_policy_path.display().to_string();
    let (preview_output, preview) = run_owned(&preview_args)?;
    assert!(preview_output.status.success());
    let preview = preview.ok_or("drift preview")?;
    assert!(preview["files"].as_array().is_some_and(|files| files
        .iter()
        .any(|file| file["workflow_id"] == "entry" && file["freshness"] == "stale-source")));
    assert_eq!(preview["manifest"]["freshness"], "stale-source");
    assert_eq!(fs::read(root.join(&entry_path))?, entry_before);

    let mut execute_args = preview_args;
    execute_args.extend([
        "--execute".to_string(),
        "--plan-hash".to_string(),
        preview["plan_hash"]
            .as_str()
            .ok_or("drift plan")?
            .to_string(),
    ]);
    let (execute_output, receipt) = run_owned(&execute_args)?;
    assert!(
        execute_output.status.success(),
        "{}",
        String::from_utf8_lossy(&execute_output.stderr)
    );
    let receipt = receipt.ok_or("drift receipt")?;
    assert_eq!(receipt["executed"], true);
    let entry_after = fs::read(root.join(&entry_path))?;
    assert_ne!(entry_after, entry_before);
    assert!(String::from_utf8_lossy(&entry_after).contains("cairn status --root . --format json"));
    assert_eq!(fs::read(root.join(&unchanged_path))?, verify_before);
    Ok(())
}

#[test]
fn invalid_entry_budget_and_legacy_layout_fail_cli_admission() -> Result<(), Box<dyn Error>> {
    let root = test_root("entry-admission")?;
    let source: Value = serde_json::from_slice(&fs::read(policy_path())?)?;
    for name in ["budget", "layout"] {
        let mut policy = source.clone();
        match name {
            "budget" => {
                policy["generated_agent_workflow_policy"]["entry"]["max_bytes"] =
                    serde_json::json!(64);
            }
            "layout" => {
                policy["generated_agent_workflow_policy"]["entry"]["layout"] =
                    serde_json::json!("cairn");
            }
            _ => unreachable!("bounded test mutation"),
        }
        let path = root.join(format!("invalid-{name}.json"));
        fs::write(&path, serde_json::to_vec_pretty(&policy)?)?;
        let path_arg = path.display().to_string();
        let (output, _) = run(&["agent", "workflow", "list", "--policy", &path_arg])?;
        assert!(!output.status.success(), "{name} must fail admission");
    }
    Ok(())
}

// r[verify cairn.agent_workflow_readiness.guidance]
#[test]
fn readiness_guidance_routes_match_the_current_cli_and_mcp_schemas() -> Result<(), Box<dyn Error>> {
    let policy = policy_path().display().to_string();
    let (catalog_output, catalog) = run(&["mcp", "--describe"])?;
    assert!(catalog_output.status.success());
    let catalog = catalog.ok_or("missing catalog")?;
    let tools = catalog["tools"].as_array().ok_or("tools")?;
    let drain = tools
        .iter()
        .find(|tool| tool["name"] == "cairn_drain_plan")
        .ok_or("drain tool")?;
    let views = drain["inputSchema"]["properties"]["view"]["enum"]
        .as_array()
        .ok_or("drain views")?;
    assert!(views.iter().any(|view| view == "readiness"));
    assert!(views.iter().any(|view| view == "legacy"));
    let instructions = tools
        .iter()
        .find(|tool| tool["name"] == "cairn_workflow_instructions")
        .ok_or("workflow instructions tool")?;
    for required in ["change", "artifact"] {
        assert!(instructions["inputSchema"]["required"]
            .as_array()
            .is_some_and(|values| values.iter().any(|value| value == required)));
    }
    let readiness = tools
        .iter()
        .find(|tool| tool["name"] == "cairn_readiness")
        .ok_or("readiness tool")?;
    assert!(readiness["inputSchema"]["required"]
        .as_array()
        .is_some_and(|values| values.iter().any(|value| value == "change")));

    let expectation = [
        (
            "apply",
            "cairn readiness CHANGE",
            "cairn workflow instructions ARTIFACT --change CHANGE",
        ),
        (
            "verify",
            "cairn readiness CHANGE",
            "cairn workflow instructions ARTIFACT --change CHANGE",
        ),
        (
            "archive",
            "cairn readiness CHANGE",
            "preview its sync and archive plans",
        ),
        (
            "drain",
            "cairn drain-plan --view readiness",
            "cairn workflow instructions ARTIFACT --change CHANGE",
        ),
    ];
    for (workflow, readiness_route, artifact_route) in expectation {
        let (output, show) = run(&["agent", "workflow", "show", workflow, "--policy", &policy])?;
        assert!(output.status.success());
        let show = show.ok_or("missing workflow")?;
        let descriptor = &show["workflow"];
        assert_eq!(descriptor["id"], workflow);
        let action = descriptor["immediate_action"]
            .as_str()
            .ok_or("immediate action")?;
        assert!(action.contains(readiness_route), "{workflow}: {action}");
        assert!(action.contains(artifact_route), "{workflow}: {action}");
        assert!(!action.contains("--execute"), "{workflow}: {action}");
        assert!(descriptor["command_classes"]
            .as_array()
            .is_some_and(|classes| classes.iter().any(|class| class == "readiness")));
        assert!(descriptor["required_evidence"]
            .as_array()
            .is_some_and(|evidence| evidence.iter().any(|entry| entry
                .as_str()
                .is_some_and(|text| text.contains("readiness")))));
        assert!(descriptor["non_claims"].as_array().is_some_and(|claims| {
            claims.iter().any(|claim| {
                claim
                    .as_str()
                    .is_some_and(|text| text.contains("never promotes an unavailable observation"))
            })
        }));
    }
    let (output, show) = run(&["agent", "workflow", "show", "verify", "--policy", &policy])?;
    assert!(output.status.success());
    let verify = show.ok_or("missing verify")?;
    assert!(verify["workflow"]["required_evidence"]
        .as_array()
        .is_some_and(|evidence| evidence.iter().any(|entry| entry
            .as_str()
            .is_some_and(|text| text.contains("completion inputs")))));
    Ok(())
}

// r[verify cairn.generated_agent_workflows.command_routes]
#[test]
fn consumer_policies_load_without_a_workflow_text_scan() -> Result<(), Box<dyn Error>> {
    // The 9082e1ae catalog (readiness in the gate class, no drain-plan
    // entry) and a workflow naming a command outside its classes both load:
    // policy admission does not read workflow text. Cairn's own tests check
    // the shipped policy's command routes instead.
    let root = test_root("command-routes")?;
    let mut policy: Value = serde_json::from_slice(&fs::read(policy_path())?)?;
    let workflow = &mut policy["generated_agent_workflow_policy"];
    let commands = workflow["command_catalog"]
        .as_array_mut()
        .ok_or("command catalog")?;
    commands
        .iter_mut()
        .find(|command| command["id"] == "readiness")
        .ok_or("readiness command")?["command_class"] = Value::from("gate");
    commands.retain(|command| command["id"] != "drain-plan-readiness");
    workflow["descriptors"]
        .as_array_mut()
        .ok_or("descriptors")?
        .iter_mut()
        .find(|descriptor| descriptor["id"] == "explore")
        .ok_or("explore")?["immediate_action"] =
        Value::from("Run **cairn change create NAME**, then cairn bogus.");
    let path = root.join("previous-default.json");
    fs::write(&path, serde_json::to_vec_pretty(&policy)?)?;
    let path_arg = path.display().to_string();
    let (output, listed) = run(&["agent", "workflow", "list", "--policy", &path_arg])?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(listed.is_some());
    Ok(())
}
