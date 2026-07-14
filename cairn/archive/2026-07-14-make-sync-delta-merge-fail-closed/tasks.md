## Phase 1: Typed delta and identity core

- [x] [serial] Define pure accepted-requirement and delta-operation inventories with stable requirement IDs, operation kinds, block identities, and deterministic diagnostics. r[cairn.sync_archive.delta_merge.preflight]
- [x] [serial] Parse explicit ADDED/MODIFIED/REMOVED sections while ignoring fenced examples and rejecting empty, malformed, ambiguous, duplicate, or full-spec-only inputs. r[cairn.sync_archive.delta_merge.preflight]
- [x] [serial] Replace heading-keyed merge behavior with requirement-ID-keyed target resolution and duplicate accepted-ID detection. r[cairn.sync_archive.delta_merge.identity]
- [x] [serial] Implement deterministic add/modify/remove target-cardinality, renamed-heading, conflict, and already-applied semantics. r[cairn.sync_archive.delta_merge.operations]

## Phase 2: Plan, receipt, and shell enforcement

- [x] [serial] Load bounded delta and accepted pre-state snapshots in the shell and compute every semantic merge result before the first write. r[cairn.sync_archive.delta_merge.mutation_truth]
- [x] [serial] Block sync dry-run and execution on any delta/identity/target diagnostic and preserve every destination byte on blocked execution. r[cairn.sync_archive.delta_merge.mutation_truth]
- [x] [serial] Bind source content, accepted pre-state, normalized operations, expected results, and policy identity into root-portable input/plan/receipt hashes. r[cairn.sync_archive.delta_merge.receipt_binding]
- [x] [serial] Derive `mutated` and mutation-manifest entries from actual byte changes, including deterministic already-applied no-op receipts. r[cairn.sync_archive.delta_merge.mutation_truth]
- [x] [serial] Update policy-owned schemas and operator diagnostics for operation summaries, conflicts, already-applied states, and repair guidance. r[cairn.sync_archive.delta_merge.receipt_binding]

## Phase 3: Verification and documentation

- [x] [parallel] Add positive tests for add, modify, remove, heading rename, new accepted spec creation, multiple specs, and idempotent replay. r[cairn.sync_archive.delta_merge.tests]
- [x] [parallel] Add negative tests for full-spec-only input, empty sections, missing/multiple IDs, duplicate operation IDs, duplicate accepted IDs, unknown modify targets, conflicting adds, malformed removals, and fenced fake headings. r[cairn.sync_archive.delta_merge.tests]
- [x] [serial] Add CLI regressions proving blocked/no-op executions do not write and changed source or pre-state bytes change the plan identity. r[cairn.sync_archive.delta_merge.tests]
- [x] [serial] Replace the documented manual post-sync search workaround with fail-closed delta repair and verification guidance. r[cairn.sync_archive.delta_merge.preflight]
- [x] [serial] Run focused core/CLI tests, policy freshness, Cairn validation and stage gates, traceability coverage, Slotcar, and release-readiness checks. r[cairn.sync_archive.delta_merge.tests]
