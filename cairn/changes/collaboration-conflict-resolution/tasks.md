# Tasks: Explicit Collaboration Conflict Resolution

## Phase 1: Pure Inspection and Planning

- [ ] [serial] Add deterministic conflict-head inspection and bounded exact single-head disclosure for artifact and lifecycle conflicts. r[cairn.collaboration_resolution.inspection]
- [ ] [serial] Add complete history-preserving resolution planning, explicit lifecycle reset, deterministic operation/result identities, and stale-plan application. r[cairn.collaboration_resolution.plan]
- [ ] [serial] Add positive and negative core tests for complete selection, exact bases, lifecycle reset, unauthorized/stale/malformed inputs, no-conflict input, and plan drift. r[cairn.collaboration_resolution.inspection] r[cairn.collaboration_resolution.plan]

## Phase 2: Provider Execution

- [ ] [serial] Add preview-first portable resolution with provider/repository locking, full input rechecks, validated prepared replacement, and exact reload verification. r[cairn.collaboration_resolution.portable_execution]
- [ ] [serial] Add preview-first signed Radicle resolution with current identity authority, signer possession, serialized state rechecks, typed reload, and exact result verification. r[cairn.collaboration_resolution.radicle_execution]
- [ ] [serial] Add positive and negative provider tests for inspection, preview, successful resolution, drift, locks, unsafe stores, missing objects, and unauthorized actors. r[cairn.collaboration_resolution.portable_execution] r[cairn.collaboration_resolution.radicle_execution]

## Phase 3: CLI, Receipts, and Validation

- [ ] [serial] Expose backend-neutral `collaboration conflicts|resolve` plus compatibility `radicle conflicts|resolve` with strict explicit selectors. r[cairn.collaboration_resolution.receipts]
- [ ] [serial] Emit deterministic backend-labeled inspection/resolution receipts with head metadata, choices, complete bases, identities, authority observations, blockers, execution mode, mutation result, and bounded non-claims. r[cairn.collaboration_resolution.receipts] r[cairn.collaboration_resolution.non_claims]
- [ ] [serial] Update public docs and run focused, workspace, lifecycle, traceability, formatting, Clippy, purity, and Nix validation before sync/archive. r[cairn.collaboration_resolution.non_claims]
