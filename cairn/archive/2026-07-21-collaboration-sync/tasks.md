## Phase 1: Contract and Pure Adapter Logic

- [x] [serial] Define preview, execution, exact peer evidence, conflict, blocker, receipt, and non-claim requirements before implementation. r[cairn.collaboration_sync.preview]
- [x] [serial] Implement deterministic Radicle sync planning and observation evaluation with positive and negative tests. r[cairn.collaboration_sync.planning]

## Phase 2: Provider Shell and CLI

- [x] [serial] Implement explicit profile/storage inspection, cooperating locks, node identity verification, bounded connect/fetch, exact peer-ref verification, and typed reload. r[cairn.collaboration_sync.execution]
- [x] [serial] Add preview-first backend-neutral CLI routing plus compatibility `radicle sync`, strict arguments, deterministic receipts, and portable rejection. r[cairn.collaboration_sync.receipts]

## Phase 3: Integration and Evidence

- [x] [serial] Extend real two-node coverage through the public sync path for bootstrap, no-op, conflict preservation, stale preview, wrong socket, unavailable peer, unseeded local policy, and missing peer-change evidence. r[cairn.collaboration_sync.integration]
- [x] [serial] Document the exact pull semantics, operational inputs, conflict behavior, and bounded non-claims. r[cairn.collaboration_sync.non_claims]
- [x] [serial] Run focused, workspace, lifecycle, traceability, formatting, Clippy, purity, live-node Nix, full flake, release-readiness, and self-dogfood validation before sync/archive. r[cairn.collaboration_sync.validation]

## Verification Evidence

- Pre-change `onix-cairn-radicle` and `cairn-cli` test suites passed before adapter or CLI edits.
- Pure sync planning/evaluation tests cover deterministic action order, malformed/self peers, relative sockets, stale state, socket/profile substitution, missing signed namespace/ref evidence, sorted receipts, no-op classification, Radicle-home substitution, and complete lock release.
- The public `cairn collaboration sync` binary passed the dedicated real two-node gate for read-only preview, unseeded-policy rejection, bootstrap, validated no-op, wrong socket, unavailable peer, missing peer custom ref, stale-plan rejection, explicit conflict preservation, and resolution fetch.
- After the full flake exposed a three-second control-socket handshake timeout, the named connect/fetch windows were increased to ten/fifteen seconds; the actual live command scenario then passed fifty consecutive repetitions and the full Nix gate.
- Full workspace tests, formatting, and Clippy with warnings denied passed after implementation.
- Policy freshness, Cairn validation, proposal/design/tasks gates, and scoped verified-core preservation passed; this slice changes no `crates/cairn-core/src/verified` files.
- Traceability coverage passed with `1717/1717` accepted requirement and scenario IDs referenced.
- Release-readiness, the dedicated Radicle check, self-dogfood, and `nix flake check path:$PWD -L` passed on `x86_64-linux`; `aarch64-linux` was omitted as incompatible.
