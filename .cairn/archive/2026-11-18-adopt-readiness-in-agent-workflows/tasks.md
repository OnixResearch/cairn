# Readiness-backed workflow tasks

## Contract and implementation

- [x] [serial] [task:baseline] Capture legacy drain, direct readiness, original validators, and generated guidance across supported positive and negative profiles. r[cairn.agent_workflow_readiness.conformance]
- [x] [serial] [task:contracts] [after:baseline] Define the typed per-change evidence manifest, new drain view, aggregate budgets, and exact guidance routes. r[cairn.agent_workflow_readiness.compatibility] r[cairn.agent_workflow_readiness.guidance]
- [x] [serial] [task:composition] [after:contracts] Implement pure shared-observation composition with existing dependency order, ready-task order, and unresolved-action behavior. r[cairn.agent_workflow_readiness.shared] r[cairn.agent_workflow_readiness.order]
- [x] [serial] [task:adapter] [after:composition] Add explicit CLI and MCP readiness-backed drain adapters with shared bounded observation and unchanged legacy receipts. r[cairn.agent_workflow_readiness.shared] r[cairn.agent_workflow_readiness.compatibility]
- [x] [serial] [task:guidance] [after:adapter] Update apply, verify, drain, and archive descriptors to request readiness and exact artifact instructions without new authority. r[cairn.agent_workflow_readiness.guidance]

## Acceptance and handoff

- [x] [serial] [task:positive] [after:guidance] Compare stage decisions, exact next actions, profile behavior, and dependency order against direct validators and readiness. r[cairn.agent_workflow_readiness.shared] r[cairn.agent_workflow_readiness.order]
- [x] [serial] [task:negative] [after:positive] Test missing results, stale subjects, failed probes, malformed manifests, cycles, aggregate bounds, and independent stage-failure preservation. r[cairn.agent_workflow_readiness.compatibility] r[cairn.agent_workflow_readiness.conformance]
- [x] [serial] [task:benchmark] [after:negative] Record the representation size of the legacy and readiness drain views over the maintained fixtures, keeping token cost and model-visible exposure explicitly unavailable. r[cairn.agent_workflow_readiness.conformance]
- [x] [serial] [task:docs] [after:benchmark] Document new-view selection, legacy differences, per-change inputs, exact instruction routes, and unchanged mutation preflight. r[cairn.agent_workflow_readiness.compatibility] r[cairn.agent_workflow_readiness.guidance]
- [x] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.agent_workflow_readiness.conformance]
- [x] [serial] [task:handoff] [after:verify] Prepare reviewed sync previews for both changed spec domains and an archive preview after implementation evidence passes. r[cairn.agent_workflow_readiness.conformance]

Executed evidence: `readiness_drain` (legacy byte compatibility, direct-route
aliasing, readiness view composition, dependency order, unavailable evidence,
stale subject, malformed manifest, aggregate-budget exhaustion, cycle rejection,
guidance-route/schema agreement), `generated_agent_workflows`, the pre-existing
`readiness`, `readiness_completion` and `readiness_isolation` suites, and 54 core
readiness tests. `cairn validate --root .` reports 0 issues, `cairn policy export
--check` is fresh, and `cairn traceability coverage --root .` is 2382/2382.

Two defects were found and fixed during acceptance rather than waived:

1. The branch had never compiled. Merging the archived readiness dependency
   exposed five errors: a function-shape collision in `readiness.rs`, a
   `Value`/`ReadinessReport` return mismatch in `fresh_readiness_report`, a
   private `collect_archived_change_ids_with_budget`, a `&Box<dyn Error>`
   coercion, and flake derivations that needed both dependency checks.
2. The branch had regenerated the **main** Nickel policy, which changed
   `policy_hash` and made the completion stage `blocked` for fixtures pinning the
   previous hash. The workflow readiness policy is read from the standalone
   `cairn-policy/generated/workflow-readiness.json`, so those main-policy edits
   were unnecessary; reverting them restored `completion: ready` and avoided a
   repository-wide fixture-hash cascade.

Scoped limitations: no live agent campaign was run, so token cost and
model-visible exposure remain unavailable; the benchmark row records representation
sizes only. The repository-wide strict Octet catalog is a separately tracked
decision and is not claimed here.
