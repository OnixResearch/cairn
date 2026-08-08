## Why

Cairn finalizes a gate report before the CLI shell appends saved-probe findings. A denied inline probe request or a missing, stale, malformed, or tampered saved probe can therefore produce a contradictory receipt with `valid=false`, a non-empty `issues` array, and `verdict="PASS"`.

The mutation guard has a more serious split: `cairn gate tasks` validates saved probe evidence, but sync/archive pre-mutation gating calls only the core snapshot evaluator. A change with a missing required probe receipt can fail the visible tasks gate and still be synced with `--execute`. The gate `input_hash` also omits probe declarations and saved receipt identities, so a passing receipt does not bind the evidence that contributed to the decision.

## What Changes

- Compose lifecycle-artifact findings and shell-observed saved-probe findings into one deterministic gate evaluation before deriving `valid` and `verdict`. r[cairn.probe_execution_boundary.authoritative_gate_composition]
- Use that same composed evaluation for `gate tasks`, sync, archive, and mutation-capable MCP paths so failed probe evidence blocks mutation. r[cairn.probe_execution_boundary.authoritative_mutation_block]
- Bind normalized probe declaration and saved-receipt identities into gate input/receipt hashes without embedding raw probe output, secrets, or host-specific paths. r[cairn.probe_execution_boundary.authoritative_evidence_binding]
- Count only an explicit current passing probe result as satisfying required evidence; blocked, skipped, deferred, timed-out, dry-run, cancelled, cleanup-failed, and infrastructure-failed results remain non-passing. r[cairn.probe_execution_boundary.authoritative_status]
- Emit bounded actionable probe diagnostics with stable IDs, result status, evidence hashes, repository-relative source spans when known, remediation, exact reproduction argv, safe artifact refs, and non-claims. r[cairn.probe_execution_boundary.authoritative_diagnostics]
- Preserve the functional-core / imperative-shell boundary and the default prohibition on inline probe execution. r[cairn.probe_execution_boundary.authoritative_boundary]
- Add positive and negative regression fixtures for coherent outcomes, valid saved evidence, missing evidence, malformed evidence, stale policy, tampering, and no-mutation behavior. r[cairn.probe_execution_boundary.authoritative_tests]

## Impact

- **Files**: gate evaluation DTOs and finalization in `cairn-core`; probe observation, gate input hashing, sync/archive guards, MCP adapters, policy receipt schemas, and operator docs in `cairn-cli`/`cairn-policy`.
- **Testing**: focused core and CLI tests proving one final outcome, pass-only status handling, actionable safe diagnostics, evidence-bound hashes, and identical probe enforcement on inspection and mutation paths.
