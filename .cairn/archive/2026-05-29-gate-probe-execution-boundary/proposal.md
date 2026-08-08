# Gate Probe Execution Boundary

## Why

Cairn gates are currently pure artifact checks: they read lifecycle content and policy, evaluate a deterministic snapshot, and emit a receipt. That is a good trust boundary. As probes become first-class evidence, Cairn needs a clear rule for command execution so gates do not silently run arbitrary commands while still being able to prove that declared probes were executed and passed.

Cairn should split probe execution from gate evaluation. A probe runner may execute policy-authorized commands and emit deterministic probe receipts. Gates should validate declarations and saved probe receipts by default. Any gate mode that executes probes inline must be explicit, policy-enabled, bounded, and receipt-producing.

## What Changes

- Preserve the default gate contract: `cairn gate proposal|design|tasks` MUST NOT execute probe commands, shell commands, network calls, or arbitrary external tools while evaluating a stage. r[cairn.probe_execution_boundary.default_gate_no_execution]
- Add an explicit probe execution surface that runs selected probes, records bounded command/evidence metadata, evaluates assertions, and emits deterministic receipts. r[cairn.probe_execution_boundary.explicit_probe_run]
- Teach gates to validate saved probe receipts against declared probe IDs, requirement IDs, bundle hashes, policy hashes, command identities, assertion outcomes, and coverage classes. r[cairn.probe_execution_boundary.gates_verify_receipts]
- Define an optional inline `--run-probes` gate mode that is disabled by default and allowed only when policy, capability declarations, budgets, and redaction rules permit it. r[cairn.probe_execution_boundary.inline_probe_mode]
- Add fail-closed stale, missing, malformed, tampered, or insufficient probe evidence diagnostics. r[cairn.probe_execution_boundary.fail_closed]

## Impact

This keeps Cairn gates deterministic and reviewable while giving behavioral requirements executable proof hooks. It also avoids turning lifecycle gates into hidden command runners. CI can run probes first, store receipts, then run gates that verify the receipts.
