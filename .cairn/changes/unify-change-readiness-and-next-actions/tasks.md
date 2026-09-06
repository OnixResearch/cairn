# Unified readiness tasks

## Contract and core

- [x] [serial] [task:baseline] Record workflow, gate, drain, completion, and machine baselines with successful and blocked fixture results. r[cairn.unified_readiness.conformance]
- [x] [serial] [task:contracts] [after:baseline] Define typed Nickel snapshot, stage, blocker, action, scope, and named-bound contracts with positive and negative fixtures. r[cairn.unified_readiness.snapshot] r[cairn.unified_readiness.actions]
- [ ] [serial] [task:core] [after:contracts] Compose existing validators in a pure readiness model without merging artifact, completion, sync, or archive authority. r[cairn.unified_readiness.decisions] r[cairn.unified_readiness.boundary]
- [ ] [serial] [task:actions] [after:core] Implement causal action derivation with stable identities, prerequisite binding, exact targets, and unresolved-remediation handling. r[cairn.unified_readiness.actions]

## Shell and parity

- [ ] [serial] [task:observation] [after:actions] Compose bounded observation adapters with source, policy, evidence, and infrastructure-error binding. r[cairn.unified_readiness.snapshot] r[cairn.unified_readiness.boundary]
- [ ] [serial] [task:transports] [after:observation] Add equivalent CLI, MCP, and concise human projections without changing legacy machine or command receipts. r[cairn.unified_readiness.transport]
- [ ] [serial] [task:negative] [after:transports] Add stale-review, failed-probe, missing-result, dependency, task-order, transport, and zero-effect failure controls. r[cairn.unified_readiness.decisions] r[cairn.unified_readiness.boundary] r[cairn.unified_readiness.conformance]

## Consumer evidence and handoff

- [ ] [serial] [task:consumer] [after:negative] Compare original validators and both transports across the maintained positive and negative consumer fixtures. r[cairn.unified_readiness.transport] r[cairn.unified_readiness.conformance]
- [ ] [serial] [task:docs] [after:consumer] Document stage distinctions and typed repair actions with a bounded diagnostic-call comparison. r[cairn.unified_readiness.actions] r[cairn.unified_readiness.transport]
- [ ] [serial] [task:verification] [after:docs] Run focused and full checks, retain current-source receipts, and prepare reviewed sync and archive plans. r[cairn.unified_readiness.conformance]

The baseline task has current positive and negative evidence in `evidence/resume/`.
The contract task has positive and negative Nickel fixtures and direct bound tests.
The first implementation adds a pure report and CLI/MCP adapters. Its acceptance matrix and observation audit remain incomplete.
Open tasks remain open until their complete acceptance criteria pass. Planning gates do not complete these tasks.
Archive execution is not a prerequisite of any task. Use the normal lifecycle after implementation evidence passes.
