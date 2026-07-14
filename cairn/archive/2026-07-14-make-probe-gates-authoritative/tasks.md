## Phase 1: Authoritative gate core

- [x] [serial] Define normalized external gate-evidence observations and a pure finalizer that derives one ordered issue set, `valid`, and `verdict`. r[cairn.probe_execution_boundary.authoritative_gate_composition]
- [x] [serial] Add core assertions and negative tests that make contradictory `valid`/`verdict` outcomes unrepresentable. r[cairn.probe_execution_boundary.authoritative_gate_composition]

## Phase 2: Shell observation and enforcement

- [x] [serial] Read and bound saved probe declarations/receipts in the CLI shell, then convert all read, parse, hash, policy, and validation results into normalized observations. r[cairn.probe_execution_boundary.authoritative_evidence_binding]
- [x] [serial] Route CLI gate evaluation through the final composed issue set, including inline-probe denial and saved-probe findings. r[cairn.probe_execution_boundary.authoritative_gate_composition]
- [x] [serial] Route sync, archive, and mutation-capable MCP preflight through the same authoritative composed gate and refuse every blocked mutation without changing files. r[cairn.probe_execution_boundary.authoritative_mutation_block]
- [x] [serial] Extend policy-owned receipt/hash schemas and gate input material with bounded probe evidence identities and deterministic issue codes. r[cairn.probe_execution_boundary.authoritative_evidence_binding]
- [x] [serial] Normalize probe outcomes and make only current explicit `passed` results eligible while preserving every non-pass status as a blocker. r[cairn.probe_execution_boundary.authoritative_status]
- [x] [serial] Normalize safe actionable probe diagnostics with stable IDs, hashes, bounded source spans, remediation, reproduction argv, artifact refs, severity, and non-claims. r[cairn.probe_execution_boundary.authoritative_diagnostics]

## Phase 3: Verification and documentation

- [x] [parallel] Add positive fixtures for current valid saved receipts and stable cross-root hashes. r[cairn.probe_execution_boundary.authoritative_tests]
- [x] [parallel] Add negative fixtures for missing, unreadable, malformed, oversized, tampered, stale-policy, command/requirement-mismatched, every non-pass status, unsupported status, unsafe diagnostics, and denied inline execution. r[cairn.probe_execution_boundary.authoritative_tests]
- [x] [serial] Add sync/archive no-mutation regressions for every non-passing saved-probe class. r[cairn.probe_execution_boundary.authoritative_mutation_block]
- [x] [serial] Document authoritative saved-evidence gating, hash inputs, privacy boundaries, and non-claims without enabling inline execution. r[cairn.probe_execution_boundary.authoritative_boundary]
- [x] [serial] Run focused core/CLI tests, policy freshness, Cairn validation and stage gates, traceability coverage, Slotcar, and release-readiness checks. r[cairn.probe_execution_boundary.authoritative_tests]
