## Phase 1: Boundary and model

- [x] [serial] Replace the umbrella Radicle dependency with direct COB/crypto/OID/Git primitives and add a deterministic dependency audit. r[cairn.radicle_cob_only.dependencies]
- [x] [serial] Add bounded immutable delegate authority to new root state while preserving empty-authority legacy semantic hashes. r[cairn.radicle_cob_only.authority]
- [x] [serial] Implement and test the explicit Git COB store, action codec, signed create/read/update path, and fail-closed ref validation. r[cairn.radicle_cob_only.store]

## Phase 2: Product shell

- [x] [serial] Replace profile/RID commands with preview-first `radicle-cob.v1` and `cairn cob` commands over explicit Git repository and keystore paths. r[cairn.radicle_cob_only.cli]
- [x] [serial] Remove collaboration sync, Radicle node/profile/identity behavior, live-node fixtures, and transport/convergence gates. r[cairn.radicle_cob_only.no_runtime]
- [x] [serial] Update operator documentation, usage text, workflow guidance, receipts, and non-claims to the COB-only boundary. r[cairn.radicle_cob_only.docs]

## Phase 3: Verification

- [x] [serial] Add positive and negative direct-COB integration tests for import, status, update, conflicts, resolution, checkout, signer/authority failure, malformed refs, stale previews, and exact reload. r[cairn.radicle_cob_only.validation]
- [ ] [serial] Run focused, workspace, lifecycle, traceability, dependency-audit, formatting, Clippy, release-readiness, self-dogfood, and full flake checks before sync/archive. r[cairn.radicle_cob_only.validation]
