# Current entry guidance tasks

## Contract and implementation

- [x] [serial] [task:baseline] Capture generator, installation, freshness, adapter, and declared host-loaded guidance baselines with positive and negative fixtures. r[cairn.agent_entry_guidance.conformance]
- [x] [serial] [task:contracts] [after:baseline] Define typed entry descriptors, command-catalog identity, stage routes, mandatory boundaries, and named text bounds. r[cairn.agent_entry_guidance.generation] r[cairn.agent_entry_guidance.scope]
- [x] [serial] [task:render] [after:contracts] Extend the existing generator with compact current entry guides and separate selected-stage instruction routes. r[cairn.agent_entry_guidance.generation] r[cairn.agent_entry_guidance.scope]
- [x] [serial] [task:install] [after:render] Extend existing project-local manifests, preview plans, guarded installation, and freshness checks for entry-guide selection. r[cairn.agent_entry_guidance.install]
- [x] [serial] [task:examples] [after:install] Add maintained command-example fixtures and explicit report-only inspection of identified copied guidance. r[cairn.agent_entry_guidance.examples]

## Acceptance and handoff

- [x] [serial] [task:positive] [after:examples] Test current layouts, supported profiles, exact stage routes, reviewed installation, and semantic parity across supported adapters. r[cairn.agent_entry_guidance.generation] r[cairn.agent_entry_guidance.install] r[cairn.agent_entry_guidance.examples]
- [x] [serial] [task:negative] [after:positive] Test stale catalogs, unknown profiles, small budgets, modified files, unsafe destinations, races, and interrupted-installation recovery. r[cairn.agent_entry_guidance.scope] r[cairn.agent_entry_guidance.install]
- [x] [serial] [task:benchmark] [after:negative] Record the representation size of the rendered entry guide per adapter, keeping token cost and host-loaded exposure explicitly unavailable. r[cairn.agent_entry_guidance.conformance]
- [x] [serial] [task:docs] [after:benchmark] Update repository workflow source, example links, host support, explicit copied-guide inspection, and no-global-write guidance. r[cairn.agent_entry_guidance.examples] r[cairn.agent_entry_guidance.install]
- [x] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.agent_entry_guidance.conformance]
- [x] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.agent_entry_guidance.conformance]

Executed evidence: `generated_agent_workflows` 16/16, `agent_guidance` 7/7,
`readiness_completion` 1/1, `readiness` 6/6 and `readiness_drain` 6/6 pass.
`cairn validate --root .` reports 0 issues and
`cairn traceability coverage --root .` is 2382/2382.

## Work required to reach this state

Merging the archived readiness-adoption branch introduced a test that had never
run. Satisfying it required real implementation:

1. A `readiness` read-only command class in
   `generated_agent_workflow_policy.command_classes`; without it, admission
   rejects the descriptor with `unknown command class: readiness`.
2. Apply, verify, drain and archive descriptors gained the `readiness` class, a
   readiness entry in `required_evidence`, exact readiness and
   workflow-instruction routes in `immediate_action`, and a shared non-claim that
   a descriptor requests readiness as an input route without promoting an
   unavailable observation to authority. `verify` also declares its completion
   inputs.
3. `fixtures/agent-completion/{contract,results}.json` were refreshed to the new
   `policy_hash`; without this, authoritative completion reported
   `verification_contract.policy_drift` and blocked the completion stage.
4. `observe_task_order` relativized the change path under the capability root.
   It previously joined the scan's absolute `change.path` onto the root, so every
   per-change `tasks.md` read was refused and `task_ordering.available` was false
   for any drain with a change present. The ordering read also now uses its own
   bounded budget so aggregate readiness consumption cannot starve it.

Scoped limitations: no live host campaign was run, so token cost and the content a
host actually loads remain unavailable; only representation sizes are recorded.
Generated guidance grants no authority, and the repository-wide strict Octet
catalog is a separately tracked decision.
