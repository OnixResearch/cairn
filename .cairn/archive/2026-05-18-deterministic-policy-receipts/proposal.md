## Why

Cairn is becoming the deterministic successor to OpenSpec, but its next hardening step needs an explicit contract for three recurring design choices: Nickel should own typed policy/configuration, BLAKE3 should identify evidence and provenance, and lifecycle gates should be mandatory, reproducible decision points. Without an accepted spec, these choices can drift into ad hoc CLI behavior or unreviewable conventions.

## What Changes

- Add `r[cairn.supersede.nickel_contracts]` to require broader Nickel-authored contracts for lifecycle policy, gate schemas, task marker policy, bounded exemptions, and sync/archive policy.
- Add `r[cairn.supersede.blake3_receipts]` to require BLAKE3 identifiers for canonical policy, gate inputs, plans, migration manifests, and emitted receipts.
- Add `r[cairn.supersede.deterministic_gates]` to require deterministic mandatory gates for proposal/design/tasks, pre-sync, pre-archive, migration, policy freshness, and traceability.

## Impact

- **Files**: `cairn-policy/`, `crates/cairn-core/`, `crates/cairn-cli/`, `tools/`, fixture repos, CLI goldens, Nix checks, Tracey coverage checks.
- **Testing**: Nickel export freshness checks, focused core unit tests for canonical hashing/gate decisions, CLI golden receipts for hashes and blockers, fixture-backed sync/archive/migration tests, and `nix flake check`.
- **Non-goals**: Do not invoke Nickel at runtime, do not hash nondeterministic logs, do not add network/NLP gates, and do not weaken the FCIS boundary.
