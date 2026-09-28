# Compact agent readiness tasks

## Contract and implementation

- [ ] [serial] [task:baseline] Capture current full readiness, human summary, machine, and MCP positive and negative baselines before core changes. r[cairn.agent_readiness.conformance]
- [ ] [serial] [task:contracts] [after:baseline] Define typed Nickel view, budget, required-field, omission, and detail-selector contracts with named bounds. r[cairn.agent_readiness.view] r[cairn.agent_readiness.decisions] r[cairn.agent_readiness.detail]
- [ ] [serial] [task:projection] [after:contracts] Implement pure compact selection that preserves stage states, validity, authority, complete actions, and original receipt identity. r[cairn.agent_readiness.decisions] r[cairn.agent_readiness.actions]
- [ ] [serial] [task:transport] [after:projection] Add strict CLI and MCP agent-view options with unchanged full and machine-envelope behavior. r[cairn.agent_readiness.view]
- [ ] [serial] [task:detail] [after:transport] Add bounded detail retrieval with fresh report observation, exact selectors, and expected-receipt admission. r[cairn.agent_readiness.detail]

## Acceptance and handoff

- [ ] [serial] [task:positive] [after:detail] Test full-to-compact parity, exact action arguments, current detail reads, deterministic ordering, and complete omission metadata. r[cairn.agent_readiness.decisions] r[cairn.agent_readiness.actions] r[cairn.agent_readiness.detail]
- [ ] [serial] [task:negative] [after:positive] Test insufficient budgets, stale detail, unknown selectors, blocked reports, unsafe paths, truncated-action denial, and zero mutation. r[cairn.agent_readiness.decisions] r[cairn.agent_readiness.detail] r[cairn.agent_readiness.conformance]
- [ ] [serial] [task:benchmark] [after:negative] Record paired complete diagnostic-loop tokens and oracle outcomes, including every follow-up detail call. r[cairn.agent_readiness.conformance]
- [ ] [serial] [task:docs] [after:benchmark] Document view semantics, exact detail requests, omitted counts, transport distinctions, and fresh-preflight requirements. r[cairn.agent_readiness.view] r[cairn.agent_readiness.detail]
- [ ] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.agent_readiness.conformance]
- [ ] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.agent_readiness.conformance]

All tasks remain open. Save implementation evidence under `evidence/implementation/`.
Planning receipts do not satisfy implementation tasks or authorize archive execution.
The shared verification and measurement rules are in `docs/agent-efficiency-roadmap.md`.
