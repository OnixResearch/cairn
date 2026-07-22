## Phase 1: Lifecycle and fixture contracts

- [x] [serial] Define bounded two-peer transport, semantic convergence, live-node, negative-path, and non-claim requirements before implementation. r[cairn.radicle_two_peer_convergence.transport]
- [x] [serial] Add shared fixture logic for two distinct profiles, current delegate authority, exact state comparison, and deterministic cleanup. r[cairn.radicle_two_peer_convergence.transport]

## Phase 2: Deterministic signed-reference convergence

- [x] [serial] Add an always-on independent-storage Radicle fetch test for import transfer and exact typed reload. r[cairn.radicle_two_peer_convergence.transport]
- [x] [serial] Create independently signed concurrent updates, exchange both namespaces, and verify identical explicit conflict heads and state identities. r[cairn.radicle_two_peer_convergence.conflict]
- [x] [serial] Resolve once with complete bases, transfer the resolution, and verify identical conflict-free state/content identities. r[cairn.radicle_two_peer_convergence.resolution]
- [x] [serial] Add negative tests for unauthorized mutation, stale or incomplete exchange, missing objects, and invalid transfer outcomes. r[cairn.radicle_two_peer_convergence.blocked]

## Phase 3: Live-node Nix evidence

- [x] [serial] Add bounded two-process `radicle-node` loopback orchestration with OS-assigned ports, explicit connect/fetch, captured diagnostics, and guaranteed teardown. r[cairn.radicle_two_peer_convergence.live]
- [x] [serial] Add a named flake check that supplies the pinned daemon and executes the ignored live-node test explicitly. r[cairn.radicle_two_peer_convergence.live]
- [x] [serial] Prove unavailable peers and readiness/fetch timeout paths fail within bounds without leaking processes or mutating peer state. r[cairn.radicle_two_peer_convergence.blocked]

## Phase 4: Documentation and release evidence

- [x] [serial] Document the two transport rails, observed convergence claim, version boundary, and remaining network/synchronization non-claims. r[cairn.radicle_two_peer_convergence.non_claims]
- [ ] [serial] Run focused, workspace, lifecycle, traceability, formatting, Clippy, purity, live-node Nix, full flake, and release-readiness validation before sync/archive. r[cairn.radicle_two_peer_convergence.non_claims]
