# Workflow obligation tasks

## Contract and implementation

- [ ] [serial] [task:policy] Author the standalone Nickel obligation policy with native predicate references, observation model, `verify` pilot, bounded `verify.bounded-exception` example, bounds and non-claims; export the whole root to `cairn-policy/generated/workflow-obligations.json`. r[cairn.workflow_obligations.policy]
- [ ] [serial] [task:model] [after:policy] Add versioned `deny_unknown_fields` DTOs and admission against the admitted generated workflow catalog, checking exact workflow, intent, policy, catalog, command-class, predicate, node, alternative, temporal and exception references. r[cairn.workflow_obligations.policy]
- [ ] [serial] [task:categories] [after:model] Model action preconditions, expected observations, progress facts and completion separately; record agent success statements, activity counts and checked boxes without using them as evidence. r[cairn.workflow_obligations.evaluation]
- [ ] [serial] [task:expression] [after:categories] Add finite `all_of`, `any_of` with one complete selected alternative and conditional applicability, with deterministic node explanations, satisfied/remaining sets and non-successful blocked or abandoned terminals. r[cairn.workflow_obligations.evaluation]
- [ ] [serial] [task:temporal] [after:expression] Add the bounded named-event ordering extension with declared trace, observer, clock, horizon, completeness and span; missing, partial and untrusted observations stay unknown or unsatisfied. r[cairn.workflow_obligations.temporal]
- [ ] [serial] [task:exception] [after:temporal] Reference native policy-exception identity, request revision, policy identity, scope rule and one compensating control; require current native applicability and control evidence and preserve the original unsatisfied condition. r[cairn.workflow_obligations.exception]
- [ ] [serial] [task:render] [after:exception] Derive guidance and the predicate from one admitted definition, with layout-only presentations and adapter-invariance checks; append converted `verify` guidance while leaving uncoverted workflows byte-identical. r[cairn.workflow_obligations.guidance] r[cairn.workflow_obligations.adapter_invariance] r[cairn.workflow_obligations.legacy_compatibility]
- [ ] [serial] [task:export] [after:render] Export the bound predicate, obligation summaries, alternatives and evidence requirements with `cairn obligation list|explain|export`; add no transport defaults or executor, clock service or monitoring. r[cairn.workflow_obligations.export]

## Acceptance and handoff

- [ ] [serial] [task:positive] [after:export] Test the shipped policy admission, exact binding, complete native evidence proof, selected complete alternative, conditional applicability, temporal ordering, exception acceptance and consumer export. r[cairn.workflow_obligations.evaluation] r[cairn.workflow_obligations.temporal] r[cairn.workflow_obligations.exception] r[cairn.workflow_obligations.export]
- [ ] [serial] [task:negative] [after:positive] Test unknown or stale references, missing selection, partial selected branches, stale/out-of-scope evidence, activity and checked-box claims, missing or partial temporal evidence, out-of-horizon and untrusted observers, missing control evidence, revocation, expiry, revision/policy/scope/control changes and blocked or abandoned terminals. r[cairn.workflow_obligations.evaluation] r[cairn.workflow_obligations.temporal] r[cairn.workflow_obligations.exception]
- [ ] [serial] [task:invariance] [after:negative] Confirm all guidance adapters retain identical obligation IDs, native evidence revisions and command classes, and that presentation-only edits cannot change the predicate or authority. r[cairn.workflow_obligations.adapter_invariance]
- [ ] [serial] [task:compatibility] [after:invariance] Confirm workflows without obligations keep their exact legacy contents, BLAKE3 and descriptor identity, and that existing workflow, gate, readiness and policy defaults are unchanged. r[cairn.workflow_obligations.legacy_compatibility]
- [ ] [serial] [task:policy-freshness] [after:compatibility] Regenerate `cairn-policy/generated/workflow-obligations.json` from the Nickel root and confirm freshness in the integrated tree. r[cairn.workflow_obligations.policy]
- [ ] [serial] [task:verify] [after:policy-freshness] Run focused core and CLI tests, policy freshness, formatting, workspace tests, Clippy, validation and traceability. Retain current-source evidence.
- [ ] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews only after implementation evidence passes.

Implementation and acceptance tasks remain open. Save evidence under `evidence/`.
Planning receipts do not satisfy implementation tasks or authorize archive execution.
