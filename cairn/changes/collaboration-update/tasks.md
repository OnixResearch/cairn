# Tasks: Preview-First Collaboration Update

## Phase 1: Pure Update Planning

- [x] [serial] Add deterministic backend-neutral artifact update planning, explicit complete revision bases, no-op classification, and exact result identities. r[cairn.collaboration_update.plan]
- [x] [serial] Add positive and negative core tests for add/change/remove, no-op, authorization, stale policy/change identity, content and lifecycle conflicts, malformed artifacts, and protocol limits. r[cairn.collaboration_update.plan]

## Phase 2: Provider Execution

- [ ] [serial] Add preview-first portable store update with shared local-change locking, store locking, full input rechecks, validated prepared replacement, and exact reload verification. r[cairn.collaboration_update.portable_execution]
- [ ] [serial] Add preview-first Radicle signed update with current creator-or-delegate authority, signer possession, serialized state rechecks, typed reload, and expected-content verification. r[cairn.collaboration_update.radicle_execution]
- [ ] [serial] Add positive and negative provider tests for no-op, successful update, changed local/state/policy inputs, stale locks, unsafe stores, missing objects, and unauthorized actors. r[cairn.collaboration_update.portable_execution] r[cairn.collaboration_update.radicle_execution]

## Phase 3: CLI, Receipts, and Validation

- [ ] [serial] Expose `cairn collaboration update` for portable and Radicle backends while retaining the compatibility `cairn radicle update` entry point. r[cairn.collaboration_update.receipts]
- [ ] [serial] Emit deterministic backend-labeled update receipts with exact bases, artifact/content/state identities, authority observations, blockers, execution mode, mutation results, and explicit non-claims. r[cairn.collaboration_update.receipts] r[cairn.collaboration_update.non_claims]
- [ ] [serial] Update public docs and run focused, workspace, lifecycle, traceability, formatting, Clippy, purity, and Nix validation before sync/archive. r[cairn.collaboration_update.non_claims]
