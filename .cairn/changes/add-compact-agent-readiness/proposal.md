# Add compact agent readiness

## Why

Cairn already exposes unified readiness, full JSON, a machine envelope, and a human summary.
At `cb0be66e0e992c0f4070956bb9d451d8a8fe7573`, the human summary omits action arguments and exact blocker details.
Agents must choose between a large receipt and a view without enough information for a reliable next step.

## What Changes

- Add an explicit compact JSON view of the existing readiness report through CLI and MCP.
- Keep transport completion, report validity, lifecycle classification, stage states, and mutation authority distinct.
- Return exact advisory action arguments, prerequisites, blocker codes, source bindings, and full-report identity.
- Add bounded identity-checked detail retrieval for omitted diagnostics and stage decisions.
- Preserve the existing full JSON, human summary, and receipt identity contracts.

## Impact

Current consumers: direct CLI and MCP agents that diagnose one active change.
Immediate outcome: one small machine response supports the next action without hidden blockers.
Durable contribution: a versioned readiness projection that other Cairn consumers can reuse.
Maintenance owner: Cairn lifecycle-core and CLI/MCP maintainers.

Reuse `ReadinessReport`, existing action derivation, and the machine transport contract.
The core owns deterministic projection decisions. Shell adapters own observation, serialization, and detail retrieval.
Expected changes include readiness policy, contracts, core projection helpers, CLI/MCP options, and parity fixtures.

## Repeatability evidence

Compare compact and full views for passing, blocked, unavailable, and inconsistent reports.
Retain counts and continuation state when output bounds omit entries.
Reject stale detail requests, unknown views, unsafe selectors, and budgets too small for required fields.
Use the common benchmark to compare complete diagnostic tasks, not response bytes alone.

## Out of Scope

- Another readiness engine, new gate semantics, or automatic repair
- Automatic execution, sync, archive, or approval
- Lossy summaries that hide lifecycle failures
- Treating a receipt reference as authenticity or future mutation permission

Dependency: `measure-agent-lifecycle-token-cost`. Implementation remains open.
