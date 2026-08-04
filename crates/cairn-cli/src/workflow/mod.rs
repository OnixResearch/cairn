use std::{error::Error, path::Path};

use cairn_core::{
    build_workflow_instructions, canonical_json_hash, changes_dir, derive_workflow_status,
    validate_change_name, LifecycleLayout, Policy, WorkflowSchemaPolicy,
};
use serde_json::Value;

use crate::support::{detect_layout, load_policy_file, path_string, print_json};

mod observe;
mod options;
mod receipts;

use observe::observe_workflow_artifacts;
use options::{parse_workflow_request, WorkflowRequestKind};
use receipts::{instructions_receipt, status_receipt, workflow_input_hash, WorkflowReceiptContext};

// r[impl cairn.artifact_workflow_contract.cli]
pub(crate) fn run_workflow(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let request = parse_workflow_request(&args)?;
    let value = match &request.kind {
        WorkflowRequestKind::Status { change } => workflow_status_value(
            &request.root,
            change,
            request.schema.as_deref(),
            &request.policy,
        )?,
        WorkflowRequestKind::Instructions { change, artifact } => workflow_instructions_value(
            &request.root,
            change,
            artifact,
            request.schema.as_deref(),
            &request.policy,
        )?,
    };
    print_json(value)
}

pub(crate) fn workflow_status_value(
    root: &Path,
    change: &str,
    schema_selector: Option<&str>,
    policy_path: &Path,
) -> Result<Value, Box<dyn Error>> {
    let context = workflow_context(root, change, schema_selector, policy_path)?;
    status_receipt(&context.receipt_context(), &context.report)
}

pub(crate) fn workflow_instructions_value(
    root: &Path,
    change: &str,
    artifact: &str,
    schema_selector: Option<&str>,
    policy_path: &Path,
) -> Result<Value, Box<dyn Error>> {
    let context = workflow_context(root, change, schema_selector, policy_path)?;
    let instructions = build_workflow_instructions(
        &context.schema,
        &context.report,
        artifact,
        &context.change_path,
    )?;
    instructions_receipt(&context.receipt_context(), &instructions)
}

struct WorkflowContext<'a> {
    root: &'a Path,
    layout: LifecycleLayout,
    change: &'a str,
    policy: Policy,
    schema: WorkflowSchemaPolicy,
    policy_hash: String,
    input_hash: String,
    change_path: String,
    report: cairn_core::WorkflowStatusReport,
}

impl WorkflowContext<'_> {
    fn receipt_context(&self) -> WorkflowReceiptContext<'_> {
        WorkflowReceiptContext {
            root: self.root,
            layout: self.layout,
            change: self.change,
            policy_name: &self.policy.name,
            policy_hash: &self.policy_hash,
            input_hash: &self.input_hash,
            schema: &self.schema,
        }
    }
}

fn workflow_context<'a>(
    root: &'a Path,
    change: &'a str,
    schema_selector: Option<&str>,
    policy_path: &Path,
) -> Result<WorkflowContext<'a>, Box<dyn Error>> {
    validate_change_name(change)?;
    let selected = detect_layout(root);
    if selected.layout != LifecycleLayout::Native {
        return Err(
            "cairn.workflow.layout_unsupported: workflow commands require native cairn layout"
                .into(),
        );
    }
    let policy = load_policy_file(policy_path.to_path_buf())?;
    let schema_index = select_schema_index(&policy, schema_selector)?;
    let schema = policy
        .workflow_schemas
        .get(schema_index)
        .cloned()
        .ok_or("cairn.workflow.schema_unknown: selected workflow schema disappeared")?;
    let observations = observe_workflow_artifacts(root, selected.layout, change, &schema, &policy)?;
    let input_hash = workflow_input_hash(change, &schema, &observations)?;
    let report = derive_workflow_status(&schema, &observations)?;
    let policy_hash = canonical_json_hash(&policy.raw);
    let change_path = path_string(Path::new(&changes_dir(selected.layout)).join(change));
    Ok(WorkflowContext {
        root,
        layout: selected.layout,
        change,
        policy,
        schema,
        policy_hash,
        input_hash,
        change_path,
        report,
    })
}

fn select_schema_index(
    policy: &Policy,
    schema_selector: Option<&str>,
) -> Result<usize, Box<dyn Error>> {
    match schema_selector {
        Some(selector) => policy
            .workflow_schemas
            .iter()
            .position(|schema| schema.id == selector)
            .ok_or_else(|| {
                let known = policy
                    .workflow_schemas
                    .iter()
                    .map(|schema| schema.id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "cairn.workflow.schema_unknown: unknown schema {selector}; known schemas: {known}"
                )
                .into()
            }),
        None => {
            if policy.workflow_schemas.is_empty() {
                Err("cairn.workflow.schema_unknown: policy has no workflow schemas".into())
            } else {
                Ok(0)
            }
        }
    }
}

pub(crate) fn add_mcp_transport(mut value: Value) -> Value {
    if let Some(object) = value.as_object_mut() {
        object.remove("receipt_hash");
        object.remove("portable_receipt_hash");
        object.insert(
            "mcp".to_string(),
            serde_json::json!({"read_only": true, "destructive": false}),
        );
    }
    value
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    const TEST_DIRECTORY_PREFIX: &str = "cairn-workflow-shell";
    const VALID_SPEC: &str = "# Demo Delta\n\n## ADDED Requirements\n\n### Requirement: Demo\n\nr[demo.workflow] Cairn MUST keep this fixture deterministic.\n\n#### Scenario: Demo passes\n\nr[demo.workflow.pass]\n- GIVEN a valid fixture\n- WHEN validation runs\n- THEN the fixture MUST pass.\n";
    const VALID_PROPOSAL: &str = "## Why\n\nA deterministic test is required.\n\n## What Changes\n\n- Add a test.\n\n## Impact\n\n- Tests change.\n";
    const VALID_DESIGN: &str = "# Design\n\n## Decision\n\nUse a deterministic fixture.\n";
    const VALID_TASKS: &str = "# Tasks\n\n- [ ] [serial] Add the fixture. r[demo.workflow]\n";

    fn test_root(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{TEST_DIRECTORY_PREFIX}-{name}-{}",
            std::process::id()
        ))
    }

    fn policy_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../cairn-policy/generated/cairn-policy.json")
    }

    fn write_fixture(root: &Path) -> Result<(), Box<dyn Error>> {
        let change = root.join("cairn/changes/demo");
        fs::create_dir_all(change.join("specs/demo"))?;
        fs::create_dir_all(root.join("cairn/specs"))?;
        fs::create_dir_all(root.join("cairn/archive"))?;
        fs::write(change.join("proposal.md"), VALID_PROPOSAL)?;
        fs::write(change.join("design.md"), VALID_DESIGN)?;
        fs::write(change.join("tasks.md"), VALID_TASKS)?;
        fs::write(change.join("specs/demo/spec.md"), VALID_SPEC)?;
        Ok(())
    }

    fn remove_fixture(root: &Path) -> Result<(), Box<dyn Error>> {
        match fs::remove_dir_all(root) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    #[test]
    fn status_and_instructions_are_deterministic_and_read_only() -> Result<(), Box<dyn Error>> {
        // r[verify cairn.artifact_workflow_contract.cli.inspect]
        // r[verify cairn.artifact_workflow_contract.instructions.ready]
        let root = test_root("positive");
        remove_fixture(&root)?;
        write_fixture(&root)?;
        let tasks_before = fs::read(root.join("cairn/changes/demo/tasks.md"))?;
        let first = workflow_status_value(&root, "demo", None, &policy_path())?;
        let second = workflow_status_value(&root, "demo", None, &policy_path())?;
        assert_eq!(first, second);
        assert_eq!(first["valid"], true);
        assert_eq!(first["artifacts"][0]["state"], "done");
        let instructions =
            workflow_instructions_value(&root, "demo", "tasks", None, &policy_path())?;
        assert_eq!(instructions["artifact"]["artifact_id"], "tasks");
        assert_eq!(instructions["read_only"], true);
        assert_eq!(
            fs::read(root.join("cairn/changes/demo/tasks.md"))?,
            tasks_before
        );
        remove_fixture(&root)?;
        Ok(())
    }

    #[test]
    fn cli_and_mcp_return_equivalent_facts_and_reject_unknown_fields() -> Result<(), Box<dyn Error>>
    {
        // r[verify cairn.artifact_workflow_contract.mcp.parity]
        // r[verify cairn.artifact_workflow_contract.mcp.invalid]
        let root = test_root("mcp");
        remove_fixture(&root)?;
        write_fixture(&root)?;
        let policy = policy_path();
        let direct = workflow_status_value(&root, "demo", None, &policy)?;
        let mcp = crate::mcp::mcp_call_tool(
            "cairn_workflow_status",
            &serde_json::json!({
                "root": root,
                "change": "demo",
                "policy": policy,
            }),
            false,
        )?;
        let structured = &mcp["structuredContent"];
        assert_eq!(structured["artifacts"], direct["artifacts"]);
        assert_eq!(structured["input_hash"], direct["input_hash"]);
        assert_eq!(structured["mcp"]["read_only"], true);
        assert!(crate::mcp::mcp_call_tool(
            "cairn_workflow_status",
            &serde_json::json!({
                "root": root,
                "change": "demo",
                "unknown": "rejected",
            }),
            false,
        )
        .is_err());
        remove_fixture(&root)?;
        Ok(())
    }

    #[test]
    fn existing_lifecycle_receipt_fields_remain_available() -> Result<(), Box<dyn Error>> {
        // r[verify cairn.artifact_workflow_contract.compatibility]
        // r[verify cairn.artifact_workflow_contract.compatibility.existing]
        let root = test_root("compatibility");
        remove_fixture(&root)?;
        write_fixture(&root)?;
        let policy = policy_path();
        let cases = [
            (
                "cairn_status",
                serde_json::json!({"root": root}),
                &["root", "layout", "specs", "changes", "receipt_hash"][..],
            ),
            (
                "cairn_gate",
                serde_json::json!({
                    "root": root,
                    "change": "demo",
                    "stage": "tasks",
                    "policy": policy,
                }),
                &[
                    "change",
                    "stage",
                    "verdict",
                    "issues",
                    "policy_hash",
                    "input_hash",
                    "receipt_hash",
                ][..],
            ),
            (
                "cairn_sync_plan",
                serde_json::json!({
                    "root": root,
                    "change": "demo",
                    "policy": policy,
                }),
                &[
                    "actions",
                    "blocked",
                    "change",
                    "dry_run",
                    "input_hash",
                    "plan_hash",
                    "receipt_hash",
                ][..],
            ),
            (
                "cairn_archive_plan",
                serde_json::json!({
                    "root": root,
                    "change": "demo",
                    "policy": policy,
                }),
                &[
                    "actions",
                    "blocked",
                    "change",
                    "dry_run",
                    "input_hash",
                    "plan_hash",
                    "receipt_hash",
                ][..],
            ),
        ];
        for (tool, input, required_fields) in cases {
            let output = crate::mcp::mcp_call_tool(tool, &input, false)?;
            let structured = &output["structuredContent"];
            for field in required_fields {
                assert!(structured.get(field).is_some(), "{tool} lost {field}");
            }
            assert!(structured.get("workflow_schema").is_none());
        }
        remove_fixture(&root)?;
        Ok(())
    }

    #[test]
    fn workflow_guide_states_commands_and_authority_boundaries() -> Result<(), Box<dyn Error>> {
        // r[verify cairn.artifact_workflow_contract.docs]
        // r[verify cairn.artifact_workflow_contract.docs.boundary]
        // r[verify cairn.artifact_workflow_contract.boundary.nonclaim]
        let guide = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/artifact-workflow.md"),
        )?;
        assert!(guide.contains("cairn workflow status CHANGE --root PATH"));
        assert!(guide.contains("cairn workflow instructions ARTIFACT --change CHANGE --root PATH"));
        assert!(guide.contains("Workflow output proves only the selected policy graph"));
        assert!(guide.contains("release eligibility"));
        assert!(guide.contains("proposal, design, and tasks gates as separate authority checks"));
        Ok(())
    }

    #[test]
    fn malformed_artifact_and_unknown_selectors_fail_closed() -> Result<(), Box<dyn Error>> {
        // r[verify cairn.artifact_workflow_contract.cli.invalid]
        // r[verify cairn.artifact_workflow_contract.status.invalid]
        let root = test_root("negative");
        remove_fixture(&root)?;
        write_fixture(&root)?;
        fs::write(
            root.join("cairn/changes/demo/specs/demo/spec.md"),
            "CAIRN_PLACEHOLDER",
        )?;
        let status = workflow_status_value(&root, "demo", None, &policy_path())?;
        assert_eq!(status["artifacts"][1]["state"], "blocked");
        assert!(workflow_status_value(&root, "demo", Some("unknown"), &policy_path()).is_err());
        let error = workflow_instructions_value(&root, "demo", "unknown", None, &policy_path())
            .expect_err("unknown artifact fails");
        assert!(error
            .to_string()
            .contains(cairn_core::WORKFLOW_ISSUE_ARTIFACT_UNKNOWN));
        remove_fixture(&root)?;
        Ok(())
    }
}
