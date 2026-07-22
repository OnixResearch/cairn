# Collaboration Sync Specification

## ADDED Requirements

### Requirement: Side-effect-free explicit preview

r[cairn.collaboration_sync.preview] The system MUST derive a deterministic Radicle known-peer pull preview from one explicit change ID, repository ID, Radicle home, control socket, peer node ID, peer protocol address, and bounded timeout policy without issuing a node-control command or modifying Radicle policy, storage, repository files, or working files.

#### Scenario: Preview binds exact local context

r[cairn.collaboration_sync.preview.context]
- GIVEN a valid explicit Radicle profile and either absent or valid local state for the requested change
- WHEN an operator previews collaboration sync
- THEN the preview reports the local profile actor, repository/change state summary, explicit peer, control socket, sorted connect/fetch/reload actions, and named deadlines
- AND no control-socket connection, peer connection, fetch, seed, follow, announce, fork, checkout, or write occurs

#### Scenario: Portable transport is unsupported

r[cairn.collaboration_sync.preview.portable]
- GIVEN the portable JSON collaboration backend
- WHEN an operator requests collaboration sync
- THEN the command rejects the unsupported transport capability before mutation
- AND it does not reinterpret a portable store as a network peer

### Requirement: Deterministic provider-specific planning

r[cairn.collaboration_sync.planning] The Radicle adapter MUST derive and evaluate synchronization plans as pure deterministic functions over explicit snapshots and observations while keeping profile, storage, control-socket, network, lock, and output effects in the imperative shell.

#### Scenario: Equivalent snapshots produce equivalent plans

r[cairn.collaboration_sync.planning.deterministic]
- GIVEN byte-equivalent validated sync snapshots
- WHEN Cairn derives known-peer pull plans
- THEN the plans and BLAKE3 plan identities are identical
- AND action order is connect, fetch, then typed reload

#### Scenario: Invalid observation cannot become success

r[cairn.collaboration_sync.planning.negative]
- GIVEN a stale local state, mismatched local node ID, failed connect/fetch, absent requested change, or absent exact peer change ref
- WHEN Cairn evaluates the execution observation
- THEN the result contains a deterministic blocker
- AND it cannot report successful synchronization

### Requirement: Serialized bounded Radicle execution

r[cairn.collaboration_sync.execution] On explicit execution, the system MUST acquire cooperating per-change Radicle locks, recheck the exact previewed local state, verify that the control socket node ID equals the explicit profile actor and differs from the peer, perform at most one non-persistent bounded connect and one bounded fetch, and release every acquired lock.

#### Scenario: One known peer is fetched

r[cairn.collaboration_sync.execution.success]
- GIVEN an unchanged preview, a running matching local node, a reachable explicit peer, and a valid repository
- WHEN the operator executes collaboration sync
- THEN Cairn performs one bounded non-persistent connect followed by one bounded fetch for the exact repository and peer
- AND it records the provider observations without seeding, following, announcing, pushing, or modifying working files

#### Scenario: Concurrent local operation blocks execution

r[cairn.collaboration_sync.execution.locked]
- GIVEN a cooperating import, update, resolution, checkout, or sync operation holds a relevant per-change lock
- WHEN another sync execution starts
- THEN it fails closed before node interaction
- AND reports the blocking lock while preserving deliberate stale-lock recovery

#### Scenario: Local state drift blocks node interaction

r[cairn.collaboration_sync.execution.stale]
- GIVEN local Radicle repository or change state differs from the preview
- WHEN execution rechecks under all cooperating locks
- THEN it reports context drift and does not connect or fetch

### Requirement: Exact typed peer evidence

r[cairn.collaboration_sync.integration] A successful sync MUST reload the named Cairn change through the typed Radicle adapter and verify an exact custom COB ref for the requested object under the requested peer namespace, including when the provider reports no changed refs.

#### Scenario: Bootstrap fetch materializes the requested change

r[cairn.collaboration_sync.integration.bootstrap]
- GIVEN the local profile lacks the repository or requested change and the explicit peer serves a valid signed change object
- WHEN sync execution succeeds
- THEN the local typed change reloads with the requested repository and change IDs
- AND the exact peer custom COB ref is present

#### Scenario: No-op fetch remains validated

r[cairn.collaboration_sync.integration.noop]
- GIVEN the local repository already contains the peer's current signed change refs
- WHEN sync executes again
- THEN zero changed refs is accepted only after exact peer-ref and typed-state validation
- AND the receipt reports a no-op without inventing a new state identity

#### Scenario: Concurrent heads remain explicit

r[cairn.collaboration_sync.integration.conflict]
- GIVEN local and remote authorized operations create concurrent artifact heads
- WHEN the known-peer fetch succeeds
- THEN typed reload reports every resulting conflict deterministically
- AND sync neither chooses a head nor invokes resolution

### Requirement: Fail-closed negative paths

r[cairn.collaboration_sync.blocked] The system MUST fail closed for malformed or duplicate arguments, invalid or self peer identities, invalid addresses or repository IDs, unavailable or wrong-profile control sockets, unavailable peers, fetch failures, stale local context, lock contention, invalid signed operations, missing typed change state, and missing exact peer change refs.

#### Scenario: Wrong control socket is rejected

r[cairn.collaboration_sync.blocked.socket]
- GIVEN a control socket belongs to a node other than the explicit profile actor
- WHEN sync execution checks the local node identity
- THEN it reports the mismatch before contacting the peer
- AND no fetch occurs

#### Scenario: Unavailable peer is bounded

r[cairn.collaboration_sync.blocked.unavailable]
- GIVEN an explicit peer address that cannot establish the expected peer session
- WHEN sync execution attempts the connection
- THEN it returns a blocker within the named deadline
- AND it does not fall back to discovery or another peer

#### Scenario: Missing peer change evidence is rejected

r[cairn.collaboration_sync.blocked.missing_ref]
- GIVEN fetch returns but the requested peer namespace lacks the exact Cairn change object ref
- WHEN Cairn evaluates the post-fetch state
- THEN synchronization is blocked
- AND unrelated repository or ref updates cannot satisfy the command

### Requirement: Deterministic bounded receipts

r[cairn.collaboration_sync.receipts] Preview and execution MUST emit deterministic backend-labeled receipts containing command/version identity, execution mode, explicit local/peer context, sorted actions, fixed deadlines, pre/post typed state, BLAKE3 plan identity, sorted ref updates and namespaces, exact peer-ref evidence, conflicts, blockers, node/fetch observations, storage-change/no-op classification, and bounded non-claims.

#### Scenario: Preview and execution remain distinguishable

r[cairn.collaboration_sync.receipts.modes]
- GIVEN equivalent sync inputs
- WHEN preview and execution receipts are emitted
- THEN preview reports `dry_run=true`, `executed=false`, and no provider result
- AND execution reports observed provider steps and post-state without claiming a precomputed remote result

#### Scenario: Ref evidence is deterministic

r[cairn.collaboration_sync.receipts.sorted]
- GIVEN provider ref updates or namespaces in different iteration orders
- WHEN Cairn emits receipts
- THEN entries are sorted into identical order before receipt hashing
- AND equivalent evidence produces an identical portable receipt hash

### Requirement: Explicit claim boundary

r[cairn.collaboration_sync.non_claims] Sync receipts and documentation MUST state that a successful known-peer pull proves only the recorded local profile, control socket, peer/address, repository/change, signed namespaces, ref updates, typed reload, versions, and deadlines, not publication, discovery, push, bidirectional equality, availability, eventual convergence, conflict correctness, remote trust, lifecycle approval, evidence truth, working-tree materialization, or release eligibility.

#### Scenario: Success remains a local observation

r[cairn.collaboration_sync.non_claims.local]
- GIVEN a successful fetch and typed reload
- WHEN the receipt is reviewed
- THEN it does not claim the peer retained equivalent state after the fetch
- AND it does not claim future availability, partition recovery, or convergence

### Requirement: Reproducible validation

r[cairn.collaboration_sync.validation] The implementation MUST pass positive and negative pure tests, CLI tests, actual two-node command integration, workspace checks, lifecycle and traceability gates, the dedicated live-node Nix gate, release-readiness, self-dogfood, and the full supported-system flake check before archive.

#### Scenario: Actual command path is exercised

r[cairn.collaboration_sync.validation.live]
- GIVEN two independent Nix-pinned Radicle node profiles
- WHEN the dedicated live gate invokes the public collaboration sync preview and execution path
- THEN bootstrap, no-op, conflict preservation, stale preview, wrong socket, unavailable peer, and missing peer evidence are checked
- AND ordinary workspace tests remain independent of ambient daemon availability
