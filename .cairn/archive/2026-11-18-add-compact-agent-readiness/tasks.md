# Compact agent readiness tasks

## Contract and implementation

- [x] [serial] [task:baseline] Capture current full readiness, human summary, machine, and MCP positive and negative baselines first. r[cairn.agent_readiness.conformance]
- [x] [serial] [task:contracts] [after:baseline] Define typed Nickel view, budget, required-field, omission, and detail-selector contracts with named bounds. r[cairn.agent_readiness.view] r[cairn.agent_readiness.decisions] r[cairn.agent_readiness.detail]
- [x] [serial] [task:projection] [after:contracts] Implement pure compact selection that preserves stage states, validity, authority, complete actions, and original receipt identity. r[cairn.agent_readiness.decisions] r[cairn.agent_readiness.actions]
- [x] [serial] [task:transport] [after:projection] Add strict CLI and MCP agent-view options with unchanged full and machine-envelope behavior. r[cairn.agent_readiness.view]
- [x] [serial] [task:detail] [after:transport] Add bounded detail retrieval with fresh report observation, exact selectors, and expected-receipt admission. r[cairn.agent_readiness.detail]

## Acceptance and handoff

- [x] [serial] [task:positive] [after:detail] Test full-to-compact parity, exact action arguments, current detail reads, deterministic ordering, and complete omission metadata. r[cairn.agent_readiness.decisions] r[cairn.agent_readiness.actions] r[cairn.agent_readiness.detail]
- [x] [serial] [task:negative] [after:positive] Test insufficient budgets, stale detail, unknown selectors, blocked reports, unsafe paths, truncated-action denial, and zero mutation. r[cairn.agent_readiness.decisions] r[cairn.agent_readiness.detail] r[cairn.agent_readiness.conformance]
- [x] [serial] [task:benchmark] [after:negative] Record representation sizes for the full and agent views over the declared fixtures, keeping token cost explicitly unavailable. r[cairn.agent_readiness.conformance]
- [x] [serial] [task:docs] [after:benchmark] Document view semantics, exact detail requests, omitted counts, transport distinctions, and fresh-preflight requirements. r[cairn.agent_readiness.view] r[cairn.agent_readiness.detail]
- [x] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.agent_readiness.conformance]
- [x] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.agent_readiness.conformance]

Executed evidence is retained under `evidence/implementation/`:
`representation-measurement.json` records the complete standalone JSON sizes for
the same fixture (`fixtures/basic`, change `demo`): full 13225 bytes versus agent
2431 bytes with `max_entries=0`, and states `tokens.state = "unavailable"`.

Rails exercised on this branch: 44 core readiness tests, 6 CLI readiness tests, the
readiness `machine_cli` and `mcp_cli` suites, `cargo fmt --check`,
`cairn policy export --check`, `cairn validate --root .` (0 issues) and
`cairn traceability coverage --root .` (2377/2377).

Scoped limitations, stated rather than waived: no live agent-host campaign was
run, so token cost and model-visible exposure remain unavailable and the amended
`cairn.agent_readiness.conformance` requirement claims representation parity only.
The repository-wide strict Octet catalog is a separately tracked decision and is
not claimed here.
