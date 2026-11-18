mod execute;
mod observe;
mod options;
mod receipt;

use std::{error::Error, path::Path};

use cairn_core::{
    admit_generated_agent_workflow_policy, canonical_json_hash, find_entry_guide_profile,
    find_generated_workflow_descriptor, inspect_copied_generated_workflow_guide,
    plan_generated_agent_workflow_install, render_generated_agent_workflows,
    safe_generated_destination_path, validate_generated_workflow_manifest,
    AdmittedGeneratedWorkflowCatalog, GeneratedWorkflowManifest, Policy,
    GENERATED_AGENT_WORKFLOW_ENTRY_ID,
};

use crate::support::{load_policy_file, print_json, CapabilityRoot, CapabilityRootKind};

// r[impl cairn.generated_agent_workflows.commands]
// r[impl cairn.generated_agent_workflows.commands.read]
// r[impl cairn.generated_agent_workflows.commands.destination]
// r[impl cairn.generated_agent_workflows.boundary]
// r[impl cairn.generated_agent_workflows.boundary.nonclaim]
// r[impl cairn.generated_agent_workflows.boundary.effects]
// r[impl cairn.generated_agent_workflows.compatibility]
use self::{
    execute::{execute_install_plan, AgentWorkflowLock},
    observe::{has_symlink_component, observe_generated_workflow_destination},
    options::{parse_agent_workflow_options, AgentWorkflowCommand, AgentWorkflowOptions},
    receipt::{
        catalog_receipt, copied_guide_receipt, descriptor_receipt, entry_receipt,
        install_plan_receipt,
    },
};

pub(crate) fn run_agent(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.first().map(String::as_str) != Some("workflow") {
        return Err("agent requires the workflow command family".into());
    }
    let options = parse_agent_workflow_options(&args[1..])?;
    let (policy, catalog, policy_hash) = load_catalog(&options.policy)?;
    match options.command {
        AgentWorkflowCommand::List => print_json(catalog_receipt(&catalog, &policy_hash)),
        AgentWorkflowCommand::Show => run_show(&options, &catalog, &policy_hash),
        AgentWorkflowCommand::Plan | AgentWorkflowCommand::Check => {
            run_install_preview(&options, &catalog, &policy_hash)
        }
        AgentWorkflowCommand::Install => run_install(&options, &policy, &catalog, &policy_hash),
        AgentWorkflowCommand::InspectCopy => run_inspect_copy(&options, &catalog),
    }
}

fn run_show(
    options: &AgentWorkflowOptions,
    catalog: &AdmittedGeneratedWorkflowCatalog,
    policy_hash: &str,
) -> Result<(), Box<dyn Error>> {
    let workflow = options
        .workflow
        .as_deref()
        .ok_or("workflow ID is unavailable")?;
    if workflow != GENERATED_AGENT_WORKFLOW_ENTRY_ID {
        find_generated_workflow_descriptor(catalog, workflow)?;
        return print_json(descriptor_receipt(catalog, policy_hash, workflow));
    }
    let adapter = options.adapter.as_deref().ok_or("adapter is unavailable")?;
    if let Some(profile) = options.profile.as_deref() {
        find_entry_guide_profile(catalog, profile)?;
    }
    let files = render_generated_agent_workflows(catalog, adapter).map_err(workflow_issues)?;
    let file = files
        .iter()
        .find(|file| file.workflow_id == GENERATED_AGENT_WORKFLOW_ENTRY_ID)
        .ok_or("rendered entry guide is unavailable")?;
    print_json(entry_receipt(
        catalog,
        policy_hash,
        adapter,
        options.profile.as_deref(),
        file,
    ))
}

fn run_inspect_copy(
    options: &AgentWorkflowOptions,
    catalog: &AdmittedGeneratedWorkflowCatalog,
) -> Result<(), Box<dyn Error>> {
    let root_path = options.root.as_deref().ok_or("root is unavailable")?;
    let guide = options.guide.as_deref().ok_or("guide is unavailable")?;
    let manifest_path = options
        .manifest
        .as_deref()
        .ok_or("manifest is unavailable")?;
    let max_path_bytes = catalog.policy.bounds.max_path_bytes;
    if !safe_generated_destination_path(guide, max_path_bytes)
        || !safe_generated_destination_path(manifest_path, max_path_bytes)
    {
        return Err("guide and manifest must be safe project-relative paths".into());
    }
    let root = CapabilityRoot::open(CapabilityRootKind::Repository, root_path)?;
    if has_symlink_component(&root, Path::new(guide))?
        || has_symlink_component(&root, Path::new(manifest_path))?
    {
        return Err("inspect-copy rejects symlinked guide or manifest paths".into());
    }
    let max_file_bytes = u64::try_from(catalog.policy.bounds.max_file_bytes)?;
    let guide_contents = String::from_utf8(root.read_bounded(Path::new(guide), max_file_bytes)?)?;
    let manifest = match root.read_bounded(Path::new(manifest_path), max_file_bytes) {
        Ok(bytes) => serde_json::from_slice::<GeneratedWorkflowManifest>(&bytes)
            .ok()
            .filter(validate_generated_workflow_manifest),
        Err(_) => None,
    };
    let workflow = options
        .workflow
        .as_deref()
        .unwrap_or(GENERATED_AGENT_WORKFLOW_ENTRY_ID);
    let inspection = inspect_copied_generated_workflow_guide(
        &guide_contents,
        workflow,
        manifest.as_ref(),
        options.manifest_identity.as_deref(),
    );
    print_json(copied_guide_receipt(catalog, &inspection))
}

fn run_install_preview(
    options: &AgentWorkflowOptions,
    catalog: &AdmittedGeneratedWorkflowCatalog,
    policy_hash: &str,
) -> Result<(), Box<dyn Error>> {
    let (root_path, adapter, destination) = destination_options(options)?;
    let root = CapabilityRoot::open(CapabilityRootKind::Repository, root_path)?;
    let snapshot = observe_generated_workflow_destination(&root, catalog, adapter, destination)?;
    let plan = plan_generated_agent_workflow_install(
        catalog,
        adapter,
        destination,
        policy_hash,
        &snapshot,
    )
    .map_err(workflow_issues)?;
    print_json(install_plan_receipt(
        options.command.name(),
        &plan,
        false,
        false,
    ))
}

fn run_install(
    options: &AgentWorkflowOptions,
    _policy: &Policy,
    catalog: &AdmittedGeneratedWorkflowCatalog,
    policy_hash: &str,
) -> Result<(), Box<dyn Error>> {
    let (root_path, adapter, destination) = destination_options(options)?;
    let root = CapabilityRoot::open(CapabilityRootKind::Repository, root_path)?;
    let snapshot = observe_generated_workflow_destination(&root, catalog, adapter, destination)?;
    let preview = plan_generated_agent_workflow_install(
        catalog,
        adapter,
        destination,
        policy_hash,
        &snapshot,
    )
    .map_err(workflow_issues)?;
    if !options.execute {
        return print_json(install_plan_receipt("install", &preview, false, false));
    }
    require_expected_plan(options, &preview.plan_hash)?;
    if preview.blocked() {
        return Err(format!(
            "generated workflow install is blocked: {:?}",
            preview.blockers
        )
        .into());
    }
    let _lock = AgentWorkflowLock::acquire(&root)?;
    let (_, current_catalog, current_policy_hash) = load_catalog(&options.policy)?;
    let current_snapshot =
        observe_generated_workflow_destination(&root, &current_catalog, adapter, destination)?;
    let current = plan_generated_agent_workflow_install(
        &current_catalog,
        adapter,
        destination,
        &current_policy_hash,
        &current_snapshot,
    )
    .map_err(workflow_issues)?;
    require_expected_plan(options, &current.plan_hash)?;
    if current.blocked() {
        return Err(format!(
            "generated workflow install is blocked: {:?}",
            current.blockers
        )
        .into());
    }
    let mutated = execute_install_plan(&root, &current)?;
    print_json(install_plan_receipt("install", &current, true, mutated))
}

fn load_catalog(
    policy_path: &Path,
) -> Result<(Policy, AdmittedGeneratedWorkflowCatalog, String), Box<dyn Error>> {
    let policy = load_policy_file(policy_path.to_path_buf())?;
    let policy_hash = canonical_json_hash(&policy.raw);
    let catalog = admit_generated_agent_workflow_policy(&policy.generated_agent_workflow_policy)
        .map_err(|issues| format!("generated workflow policy is invalid: {issues:?}"))?;
    Ok((policy, catalog, policy_hash))
}

fn destination_options(
    options: &AgentWorkflowOptions,
) -> Result<(&Path, &str, &str), Box<dyn Error>> {
    let root = options.root.as_deref().ok_or("root is unavailable")?;
    let adapter = options.adapter.as_deref().ok_or("adapter is unavailable")?;
    let destination = options
        .destination
        .as_deref()
        .ok_or("destination is unavailable")?;
    Ok((root, adapter, destination))
}

// Bootstrap composes the same renderer, observer, and ownership classifier as
// the standalone installer. Bootstrap owns durable publication of these writes.
pub(crate) fn bootstrap_plan(
    root_path: &Path,
    policy: &Policy,
    adapter: &str,
    destination: &str,
) -> Result<cairn_core::GeneratedWorkflowInstallPlan, Box<dyn Error>> {
    let catalog = admit_generated_agent_workflow_policy(&policy.generated_agent_workflow_policy)
        .map_err(workflow_issues)?;
    let root = CapabilityRoot::open(CapabilityRootKind::Repository, root_path)?;
    let snapshot = observe_generated_workflow_destination(&root, &catalog, adapter, destination)?;
    Ok(plan_generated_agent_workflow_install(
        &catalog,
        adapter,
        destination,
        &canonical_json_hash(&policy.raw),
        &snapshot,
    )
    .map_err(workflow_issues)?)
}

fn workflow_issues(issues: Vec<cairn_core::GeneratedWorkflowIssue>) -> String {
    format!("generated workflow operation failed: {issues:?}")
}

fn require_expected_plan(
    options: &AgentWorkflowOptions,
    observed_plan_hash: &str,
) -> Result<(), Box<dyn Error>> {
    let expected = options
        .plan_hash
        .as_deref()
        .ok_or("execute requires a plan hash")?;
    if expected != observed_plan_hash {
        return Err(format!(
            "{}: expected {expected}, observed {observed_plan_hash}",
            cairn_core::GENERATED_AGENT_WORKFLOW_ISSUE_PLAN_STALE
        )
        .into());
    }
    Ok(())
}
