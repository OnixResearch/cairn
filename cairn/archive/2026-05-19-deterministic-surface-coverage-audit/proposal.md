# Change: Deterministic Surface Coverage Audit

## Why

Cairn now has deterministic replay, receipt verification, evidence bundles, and evidence manifests, but confidence still depends on whether every public deterministic receipt-producing surface is represented in the replay policy. Agents and CI need a first-class self-audit that compares declared deterministic surfaces against policy replay coverage and fails when a public surface is untracked.

Without this audit, a future CLI or MCP receipt surface could ship with stable-looking output but no repeated/cross-root replay coverage. That would silently reduce determinism confidence.

## What Changes

- Add a policy-owned `determinism_surface_inventory` listing public deterministic receipt surfaces and required replay coverage.
- Add `cairn determinism coverage-audit --root .` to compare the inventory with `determinism_replay_policy` cases/groups.
- Emit a deterministic pass/fail receipt with covered/missing surfaces, replay cases, replay groups, current policy hash, and receipt hash.
- Document the audit as the next confidence rail after replay/verify/bundle/manifest generation.
- Add tests for passing coverage, missing replay coverage, and stable/secret-safe failure output.

## Non-Goals

- Proving all possible workspace behavior is deterministic.
- Running the replay oracle itself; this audit checks coverage metadata.
- Auditing non-receipt-producing or intentionally interactive commands.
- Cryptographic provenance or network transparency validation.
