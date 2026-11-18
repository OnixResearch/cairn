# Implementation notes: adopt readiness in agent workflows

Status: source and test text implemented in branch `change/adopt-readiness-workflows-20261117`.
No build, test, formatter, Clippy, validation, traceability, Nix, or lifecycle command was executed while producing these bytes.
Every task checkbox in `tasks.md` intentionally remains open until the parent runs the repository rails.

## Implemented surfaces

### Nickel-owned declaration

- Source: `cairn-policy/workflow-readiness.ncl`
- Exported root field: `policy`
- Generated artifact: `cairn-policy/generated/workflow-readiness.json`
- Regeneration commands (parent):

  ```bash
  nickel export --format json --field policy cairn-policy/workflow-readiness.ncl > cairn-policy/generated/workflow-readiness.json
  cairn policy export --output cairn-policy/generated/cairn-policy.json
  ```

- `cairn-policy/default.ncl` changed the `apply`, `verify`, `drain`, and `archive` descriptor `required_evidence`, `command_classes`, and `immediate_action`, added the read command class `readiness`, and added the shared readiness constraint and non-claim.
- `cairn-policy/generated/cairn-policy.json` was updated mechanically to the same values with the canonical export formatting (sorted keys, two-space indent, trailing newline). The Nickel export check remains authoritative.
- Fixtures: `cairn-policy/fixtures/workflow-readiness-positive.ncl`, `cairn-policy/fixtures/workflow-readiness-negative.ncl`.
- Nix check: `checks.workflow-readiness-policy-fresh` in `flake.nix`.

### Core API

`crates/cairn-core/src/verified/readiness/workflow.rs`, re-exported from `crate::verified::readiness`:

- `workflow_readiness_policy() -> Result<&'static WorkflowReadinessPolicy, &'static str>`
- `admit_workflow_readiness_policy(&WorkflowReadinessPolicy) -> Result<(), &'static str>`
- `select_drain_readiness(&[DrainReadinessChange<'_>], &ChangeDependencyGraph, &WorkflowReadinessPolicy) -> Result<DrainReadinessSelection, &'static str>`
- Types: `WorkflowReadinessPolicy`, `WorkflowReadinessInstruction`, `WorkflowReadinessBounds`, `DrainReadinessChange`, `DrainTaskOrdering`, `DrainReadinessSelection`, `DrainChangeSelection`, `DrainSelectedAction`, `DrainNextTask`, `DrainInstructionRoute`.
- Error codes: `WORKFLOW_READINESS_POLICY_INVALID`, `WORKFLOW_READINESS_POLICY_BOUNDS`, `WORKFLOW_READINESS_BOUND`.
- Behavior: dependency order first, then declared ready-task order, then the checked action-kind order. Preview actions are withheld for ordering-blocked changes. The task route is `cairn workflow instructions tasks --change CHANGE` with the MCP equivalent `cairn_workflow_instructions`.

### Shell API

`crates/cairn-cli/src/drain_plan.rs`:

- `run_drain_plan(Vec<String>)` for `cairn drain-plan [--root] [--policy] [--cross-repo-evidence]... [--view legacy|readiness] [--evidence-manifest FILE]`.
- `readiness_drain_value(&Value, command: &str) -> Result<Value, Box<dyn Error>>` for MCP dispatch.
- `crate::readiness::readiness_report_with_budget(root, policy_path, change, input, &ObservationBudget)` is the shared per-change observation entry point.
- Manifest admission uses `serde` `deny_unknown_fields` plus bounds from the generated policy. Stable codes: `workflow_readiness.manifest.{unavailable,invalid,schema,bound,unknown_change,duplicate_change,partial_completion,unsafe_path,invalid_subject,duplicate_evidence}`.
- `crates/cairn-cli/src/mcp/read_tools.rs::mcp_drain_plan(input, command)` dispatches the unchanged legacy receipt or the readiness view. The MCP call passes `"mcp cairn_drain_plan"`; the direct CLI passes `"cairn drain-plan"`.
- `crates/cairn-cli/src/mcp/schema.rs` adds `view` (enum from the generated policy) and `evidence_manifest`, keeps `additionalProperties: false`, and rejects `evidence_manifest` outside the readiness view and `cross_repo_evidence` inside it.
- Registered `drain-plan` in `main.rs`, `machine.rs`, and the usage text.

### Tests

- `crates/cairn-core/src/verified/readiness/workflow/tests.rs`: policy admission, dependency order, blocked predecessors, task route, unavailable observations, action ordering, truncation, bound rejection, graph-issue blocking.
- `crates/cairn-cli/src/drain_plan_tests.rs`: shared-budget exhaustion and exact reproduction, manifest admission negatives and positives, direct readiness parity, path safety.
- `crates/cairn-cli/tests/readiness_drain.rs`: legacy byte compatibility, direct CLI alias, readiness composition, dependency ordering, unavailable evidence, manifest negatives through the CLI.
- `crates/cairn-cli/tests/generated_agent_workflows.rs`: guidance route and MCP schema agreement for `apply`, `verify`, `drain`, and `archive`.

## Deliberately unchanged

- The legacy drain receipt fields and hashes for MCP requests without `view`.
- Gate, completion, sync, archive, and cross-repository validators.
- Mutation preflight and `--execute` authority.
- The readiness adapter's report schema and per-change identity.
- No scheduler, discovery, inference from task checkboxes, or default view switch.

## Open verification

- Policy freshness for both generated artifacts.
- All focused and workspace tests, Clippy, formatting, validation, traceability, and Nix checks.
- Benchmark receipts for `[task:benchmark]`.
- Sync previews and the archive preview for `[task:handoff]`.
