## Phase 1: Contract and Pure Adapter Logic

- [ ] [serial] Define preview, execution, exact peer evidence, conflict, blocker, receipt, and non-claim requirements before implementation. r[cairn.collaboration_sync.preview]
- [ ] [serial] Implement deterministic Radicle sync planning and observation evaluation with positive and negative tests. r[cairn.collaboration_sync.planning]

## Phase 2: Provider Shell and CLI

- [ ] [serial] Implement explicit profile/storage inspection, cooperating locks, node identity verification, bounded connect/fetch, exact peer-ref verification, and typed reload. r[cairn.collaboration_sync.execution]
- [ ] [serial] Add preview-first backend-neutral CLI routing plus compatibility `radicle sync`, strict arguments, deterministic receipts, and portable rejection. r[cairn.collaboration_sync.receipts]

## Phase 3: Integration and Evidence

- [ ] [serial] Extend real two-node coverage through the public sync path for bootstrap, no-op, conflict preservation, stale preview, wrong socket, unavailable peer, and missing peer-change evidence. r[cairn.collaboration_sync.integration]
- [ ] [serial] Document the exact pull semantics, operational inputs, conflict behavior, and bounded non-claims. r[cairn.collaboration_sync.non_claims]
- [ ] [serial] Run focused, workspace, lifecycle, traceability, formatting, Clippy, purity, live-node Nix, full flake, release-readiness, and self-dogfood validation before sync/archive. r[cairn.collaboration_sync.validation]
