use std::{error::Error, path::PathBuf};

use crate::support::default_policy_path;

const OPTION_WIDTH: usize = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AgentWorkflowCommand {
    List,
    Show,
    Plan,
    Check,
    Install,
    InspectCopy,
}

impl AgentWorkflowCommand {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Show => "show",
            Self::Plan => "plan",
            Self::Check => "check",
            Self::Install => "install",
            Self::InspectCopy => "inspect-copy",
        }
    }

    fn requires_destination(self) -> bool {
        matches!(self, Self::Plan | Self::Check | Self::Install)
    }
}

#[derive(Debug)]
pub(super) struct AgentWorkflowOptions {
    pub(super) command: AgentWorkflowCommand,
    pub(super) workflow: Option<String>,
    pub(super) root: Option<PathBuf>,
    pub(super) policy: PathBuf,
    pub(super) adapter: Option<String>,
    pub(super) destination: Option<String>,
    pub(super) profile: Option<String>,
    pub(super) guide: Option<String>,
    pub(super) manifest: Option<String>,
    pub(super) manifest_identity: Option<String>,
    pub(super) execute: bool,
    pub(super) plan_hash: Option<String>,
}

pub(super) fn parse_agent_workflow_options(
    args: &[String],
) -> Result<AgentWorkflowOptions, Box<dyn Error>> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err("agent workflow requires list, show, plan, check, install, or inspect-copy".into());
    };
    let command = match command {
        "list" => AgentWorkflowCommand::List,
        "show" => AgentWorkflowCommand::Show,
        "plan" => AgentWorkflowCommand::Plan,
        "check" => AgentWorkflowCommand::Check,
        "install" => AgentWorkflowCommand::Install,
        "inspect-copy" => AgentWorkflowCommand::InspectCopy,
        _ => return Err(format!("unknown agent workflow command: {command}").into()),
    };
    let mut workflow = None;
    let mut root = None;
    let mut policy = None;
    let mut adapter = None;
    let mut destination = None;
    let mut profile = None;
    let mut guide = None;
    let mut manifest = None;
    let mut manifest_identity = None;
    let mut execute = false;
    let mut plan_hash = None;
    let mut index = 1;
    if command == AgentWorkflowCommand::Show
        && args.get(index).is_some_and(|value| !value.starts_with('-'))
    {
        workflow = args.get(index).cloned();
        index += 1;
    }
    while index < args.len() {
        match args[index].as_str() {
            "--root" => set_path(&mut root, args, index, "--root")?,
            "--policy" => set_path(&mut policy, args, index, "--policy")?,
            "--adapter" => set_string(&mut adapter, args, index, "--adapter")?,
            "--destination" => set_string(&mut destination, args, index, "--destination")?,
            "--profile" => set_string(&mut profile, args, index, "--profile")?,
            "--guide" => set_string(&mut guide, args, index, "--guide")?,
            "--manifest" => set_string(&mut manifest, args, index, "--manifest")?,
            "--manifest-identity" => {
                set_string(&mut manifest_identity, args, index, "--manifest-identity")?
            }
            "--workflow" if command == AgentWorkflowCommand::InspectCopy => {
                set_string(&mut workflow, args, index, "--workflow")?
            }
            "--plan-hash" => set_string(&mut plan_hash, args, index, "--plan-hash")?,
            "--execute" if !execute => {
                execute = true;
                index += 1;
                continue;
            }
            "--execute" => return Err("--execute must not be repeated".into()),
            unknown => return Err(format!("unknown agent workflow option: {unknown}").into()),
        }
        index += OPTION_WIDTH;
    }
    validate_options(
        command,
        workflow.as_deref(),
        root.as_ref(),
        adapter.as_deref(),
        destination.as_deref(),
        profile.as_deref(),
        guide.as_deref(),
        manifest.as_deref(),
        manifest_identity.as_deref(),
        execute,
        plan_hash.as_deref(),
    )?;
    Ok(AgentWorkflowOptions {
        command,
        workflow,
        root,
        policy: policy.unwrap_or_else(|| PathBuf::from(default_policy_path())),
        adapter,
        destination,
        profile,
        guide,
        manifest,
        manifest_identity,
        execute,
        plan_hash,
    })
}

#[allow(clippy::too_many_arguments)]
fn validate_options(
    command: AgentWorkflowCommand,
    workflow: Option<&str>,
    root: Option<&PathBuf>,
    adapter: Option<&str>,
    destination: Option<&str>,
    profile: Option<&str>,
    guide: Option<&str>,
    manifest: Option<&str>,
    manifest_identity: Option<&str>,
    execute: bool,
    plan_hash: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    if command == AgentWorkflowCommand::Show && workflow.is_none_or(str::is_empty) {
        return Err("agent workflow show requires a workflow ID".into());
    }
    if command.requires_destination()
        && (root.is_none() || adapter.is_none() || destination.is_none())
    {
        return Err(format!(
            "agent workflow {} requires --root, --adapter, and --destination",
            command.name()
        )
        .into());
    }
    if command == AgentWorkflowCommand::Show {
        if root.is_some()
            || destination.is_some()
            || guide.is_some()
            || manifest.is_some()
            || manifest_identity.is_some()
        {
            return Err(
                "show accepts only --policy, --adapter, and --profile for the entry workflow"
                    .into(),
            );
        }
        let entry = workflow == Some(cairn_core::GENERATED_AGENT_WORKFLOW_ENTRY_ID);
        if entry && adapter.is_none() {
            return Err("agent workflow show entry requires --adapter".into());
        }
        if !entry && (adapter.is_some() || profile.is_some()) {
            return Err("--adapter and --profile are available only for the entry workflow".into());
        }
    }
    if command == AgentWorkflowCommand::List
        && (root.is_some()
            || adapter.is_some()
            || destination.is_some()
            || profile.is_some()
            || guide.is_some()
            || manifest.is_some()
            || manifest_identity.is_some()
            || workflow.is_some())
    {
        return Err("list accepts only --policy".into());
    }
    if command == AgentWorkflowCommand::InspectCopy {
        if root.is_none() || guide.is_none() || manifest.is_none() {
            return Err(
                "agent workflow inspect-copy requires --root, --guide, and --manifest".into(),
            );
        }
        if adapter.is_some() || destination.is_some() || profile.is_some() {
            return Err(
                "inspect-copy accepts only --policy, --root, --guide, --manifest, --manifest-identity, and --workflow".into(),
            );
        }
    }
    if matches!(
        command,
        AgentWorkflowCommand::Plan | AgentWorkflowCommand::Check | AgentWorkflowCommand::Install
    ) && (profile.is_some()
        || guide.is_some()
        || manifest.is_some()
        || manifest_identity.is_some()
        || workflow.is_some())
    {
        return Err(format!(
            "agent workflow {} accepts only destination options",
            command.name()
        )
        .into());
    }
    if execute && command != AgentWorkflowCommand::Install {
        return Err("--execute is available only for install".into());
    }
    if execute && plan_hash.is_none() {
        return Err("--execute requires --plan-hash from a current preview".into());
    }
    if !execute && plan_hash.is_some() {
        return Err("--plan-hash requires --execute".into());
    }
    if command == AgentWorkflowCommand::InspectCopy && execute {
        return Err("inspect-copy is report-only".into());
    }
    Ok(())
}

fn set_string(
    target: &mut Option<String>,
    args: &[String],
    index: usize,
    name: &'static str,
) -> Result<(), Box<dyn Error>> {
    if target.is_some() {
        return Err(format!("{name} must not be repeated").into());
    }
    let value = args
        .get(index + 1)
        .filter(|value| !value.is_empty() && !value.starts_with("--"))
        .ok_or_else(|| format!("{name} requires a value"))?;
    *target = Some(value.clone());
    Ok(())
}

fn set_path(
    target: &mut Option<PathBuf>,
    args: &[String],
    index: usize,
    name: &'static str,
) -> Result<(), Box<dyn Error>> {
    if target.is_some() {
        return Err(format!("{name} must not be repeated").into());
    }
    let mut value = None;
    set_string(&mut value, args, index, name)?;
    *target = value.map(PathBuf::from);
    Ok(())
}
