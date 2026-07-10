## Why

Cairn lifecycle behavior is already state-heavy: proposal/design/tasks gates, sync/archive planning, validation, release-readiness, and evidence export all have explicit transitions and blockers. The tla2 repository is a useful reference for TLA+ parsing/checking/simulation/codegen workflows and for treating state-machine evidence as bounded receipts rather than correctness claims.

Reference source: `https://github.com/joweeba/tla2`.

## What Changes

- Define a Cairn-owned state-machine receipt profile for lifecycle transition models and optional external model-check outputs.
- Record model artifact identity, transition relation profile, checked property labels, checker identity, result status, diagnostics, and non-claims.
- Add positive/negative fixtures for lifecycle FSM receipts and fail-closed handling of stale, unknown, unchecked, or overclaiming model evidence.
- Keep TLA+ tooling optional and external; Cairn validates receipts and claim boundaries, not arbitrary temporal logic semantics.

## Impact

- **Surfaces**: lifecycle orchestration specs, release evidence receipts, determinism/replay docs, optional external witness adapters.
- **Non-claims**: No embedded TLA+ checker, no proof of source-code correctness, no lifecycle eligibility from a model-check receipt alone.
- **Validation**: pure receipt validation, positive/negative fixtures, CLI receipt smoke, Cairn gates/validate.