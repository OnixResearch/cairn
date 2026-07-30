# Validation: Versioned CLI machine contract

## Baseline

The pre-change workspace test suite passed in pueue task `1041`.

The baseline preserved default JSON results, stderr failures, nonzero statuses, nested receipt hashes, MCP framing, validation, and release-readiness behavior.

## Focused evidence

The focused core policy and envelope suites passed.

The focused CLI suite passed with positive and negative cases. It covered success, blocked receipts, nested nonzero status, repeatability, lookup failure, privacy, invalid policy, incompatible output, and bounded capture.

The default CLI and MCP compatibility tests passed without machine framing.

The policy export produced a deterministic checked projection. Runtime admission rejected unknown classes, duplicate codes, unsafe targets, malformed ordering, invalid status mappings, and invalid bounds.

The focused purity scan found no shell effects in these files:

- `crates/cairn-core/src/verified/cli_machine.rs`
- `crates/cairn-core/src/verified/policy/machine.rs`

## Receipt refresh

The machine policy changed the generated policy identity.

Cairn refreshed the agent completion fixtures, agent replay linkage, Preserves hash goldens, and lifecycle plan goldens. Focused agent, evidence, release-receipt, and lifecycle tests passed after the refresh.

## Lifecycle evidence

Proposal, design, and tasks gates passed before accepted-spec sync.

Accepted-spec sync completed for `cairn/specs/cli-machine-contract/spec.md`.

Traceability passed after every new accepted requirement and scenario had a source or test marker.

## Claim boundary

A machine envelope proves controlled transport and serialization only.

It does not prove command correctness, nested evidence truth, mutation safety, host integrity, lifecycle readiness, or release eligibility.

Host termination, process kill, memory exhaustion, pre-boundary panic, broken stdout, and failure before Cairn starts remain outside the envelope guarantee.
