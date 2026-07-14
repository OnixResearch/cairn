## Design

1. Add a `StateMachineReceipt` core model for lifecycle transition evidence. Fields include model artifact ref, transition relation profile, state/event vocabulary, property labels, checker identity, result status, diagnostics, input hashes, output hash, and non-claims.
2. Treat TLA+ and tla2 outputs as optional external witnesses. The core validates receipt shape, identities, freshness, status vocabulary, and claim boundaries only.
3. Keep model parsing, model-check execution, simulation, code generation, filesystem reads, stdout/stderr, and JSON presentation in the shell or external tools.
4. Bind checked properties to named lifecycle concerns such as gate transition admissibility, sync/archive dry-run-before-execute, evidence export/verify sequencing, and release-readiness blocker transitions.
5. Add a read-only CLI validation path for already-produced state-machine receipts.
6. Add positive fixtures for a passing lifecycle FSM receipt and a bounded simulation receipt; add negative fixtures for stale model hash, unknown checker, missing property label, unchecked status, role promotion, and missing non-claims.

## Non-Goals

- No bundled TLA+ parser/checker or dependency on tla2.
- No claim that a model-check receipt proves Cairn implementation correctness, repository correctness, release eligibility, or absence of all lifecycle bugs.
- No replacement for existing Cairn validation, gates, replay, or release-readiness checks.