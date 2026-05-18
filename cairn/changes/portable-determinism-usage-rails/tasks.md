# Tasks: Portable Determinism Usage Rails

## Phase 1: Spec and contracts

- [ ] [serial] r[cairn.supersede.portable_receipts] Define the portable receipt normalization contract and decide which existing receipt path fields are display-only versus hash material.
- [ ] [parallel] r[cairn.supersede.determinism_replay.cross_root] Add positive cross-root replay fixtures for status, validate, gate, sync, archive, migrate, release-readiness, and agent-receipt validation surfaces.
- [ ] [parallel] r[cairn.supersede.agent_run_receipts.nondeterminism.environment] Add negative fixtures for hidden environment/date/path entropy and deterministic issue codes.

## Phase 2: Implementation

- [ ] [depends:cairn.supersede.portable_receipts] r[cairn.supersede.portable_receipts.cross_root] Implement pure root-relative canonicalization for receipt hash inputs while preserving operator-facing display paths.
- [ ] [depends:cairn.supersede.determinism_replay] r[cairn.supersede.determinism_replay.repeated] Implement the replay oracle as focused tests or a `cairn determinism-check` CLI command with normalized JSON comparison.
- [ ] [depends:sync_archive.execute.mutation_manifest] r[sync_archive.execute.mutation_manifest] Add before/after BLAKE3 manifests for explicit sync/archive/migrate executions.
- [ ] [depends:cairn.supersede.mcp_reproducibility] r[cairn.supersede.mcp_reproducibility.describe] Add MCP `--describe` and common `--call` determinism fixtures.

## Phase 3: Verification and docs

- [ ] [depends:cairn.supersede.determinism_replay] r[cairn.supersede.determinism_replay.hidden_entropy] Run focused CLI regression tests proving repeated and cross-root receipts are stable and hidden entropy fails deterministically.
- [ ] [parallel] r[cairn.supersede.portable_receipts.display_paths] Document portable hash inputs, display path semantics, mutation manifests, and nondeterminism disclosure.
- [ ] [serial] r[cairn.supersede.determinism_replay] Run Cairn validate, Tracey coverage, cargo tests, and relevant Nix checks before sync/archive.
