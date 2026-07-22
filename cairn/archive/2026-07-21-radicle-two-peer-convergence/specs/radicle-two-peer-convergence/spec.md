# Radicle Two-Peer Convergence Specification

## ADDED Requirements

### Requirement: Independent signed-reference transport

r[cairn.radicle_two_peer_convergence.transport] Cairn MUST exercise change COB transfer between profiles with distinct home paths, storage paths, signers, public keys, and Radicle namespaces through Radicle fetch behavior rather than direct filesystem, object, or reference copying.

#### Scenario: Imported change transfers and reloads

r[cairn.radicle_two_peer_convergence.transport.success]

- **GIVEN** two independent Radicle profiles that are current delegates of one repository
- **WHEN** the first peer imports a typed Cairn change COB and the second peer fetches the first peer's signed namespace
- **THEN** the second peer MUST load the same object ID, exact typed state, state hash, content root, creator, policy hash, and artifact bytes

#### Scenario: Shared storage is rejected as evidence

r[cairn.radicle_two_peer_convergence.transport.independent]

- **GIVEN** a proposed convergence fixture whose peers share a home, signer, storage path, or repository handle
- **WHEN** the fixture validates its peer identities
- **THEN** validation MUST reject the fixture before classifying any observation as two-peer evidence

### Requirement: Explicit concurrent conflict convergence

r[cairn.radicle_two_peer_convergence.conflict] Starting from the same imported root, Cairn MUST create independently signed authorized updates in each peer namespace, transfer both namespaces, and require both peers to report the same complete sorted conflict heads and deterministic state identity.

#### Scenario: Concurrent artifact updates converge to one visible conflict

r[cairn.radicle_two_peer_convergence.conflict.success]

- **GIVEN** both current delegates have fetched the same conflict-free change root
- **WHEN** each peer updates the same artifact without first fetching the other's update and then both signed namespaces are exchanged
- **THEN** both peers MUST report exactly the same two current revision heads, actors, values, complete bases, conflict metadata, and state hash

#### Scenario: One-way or stale exchange is not convergence

r[cairn.radicle_two_peer_convergence.conflict.stale]

- **GIVEN** concurrent peer updates exist
- **WHEN** only one signed namespace or a stale pre-update namespace is available to a peer
- **THEN** the harness MUST reject any claim that both peers have converged on the complete conflict

### Requirement: Resolution transfer convergence

r[cairn.radicle_two_peer_convergence.resolution] Cairn MUST resolve the observed conflict through one explicit current-head choice whose new revision basis supersedes every competing head, transfer that signed resolution, and require both peers to reload identical conflict-free state and content identities.

#### Scenario: One complete-basis resolution converges

r[cairn.radicle_two_peer_convergence.resolution.success]

- **GIVEN** both peers report the same complete conflict
- **WHEN** one current delegate explicitly selects one head, signs the complete-basis resolution, and the other peer fetches that namespace
- **THEN** both peers MUST report no remaining conflict and MUST have identical object ID, state hash, content root, selected artifact bytes, operation history, and resolution bases

#### Scenario: Partial or stale resolution cannot establish convergence

r[cairn.radicle_two_peer_convergence.resolution.blocked]

- **GIVEN** a resolution plan that omits a competing head or was derived before the complete conflict exchange
- **WHEN** execution or convergence validation is attempted
- **THEN** the operation MUST fail closed and MUST NOT be reported as converged

### Requirement: Real loopback node evidence

r[cairn.radicle_two_peer_convergence.live] A dedicated Nix check MUST start two actual pinned `radicle-node` processes with independent profiles, use bounded control-socket readiness and explicit loopback connect/fetch operations, exercise the custom Cairn COB transfer path, and terminate and reap both processes on success or failure.

#### Scenario: Live nodes transport the same convergence scenario

r[cairn.radicle_two_peer_convergence.live.success]

- **GIVEN** two running loopback Radicle nodes with distinct node IDs, homes, sockets, listeners, and storage
- **WHEN** they explicitly connect and perform bounded fetches for import, concurrent updates, and resolution
- **THEN** the dedicated check MUST observe the same typed import, conflict, and final convergence invariants as the deterministic transport rail

#### Scenario: Live dependency is explicit

r[cairn.radicle_two_peer_convergence.live.explicit]

- **GIVEN** ordinary Cargo tests do not guarantee a `radicle-node` executable
- **WHEN** the workspace test suite runs without the named live gate
- **THEN** the live test MUST remain explicitly ignored rather than silently pass based on ambient PATH, and the named Nix check MUST execute it with the pinned daemon available

### Requirement: Bounded negative paths

r[cairn.radicle_two_peer_convergence.blocked] The harness MUST reject unauthorized peer mutation, unavailable peers, missing change objects, stale or incomplete transfer, invalid fetch results, readiness deadline expiration, and state mismatch without direct fallback copying or leaked node processes.

#### Scenario: Unavailable peer fails within a deadline

r[cairn.radicle_two_peer_convergence.blocked.unavailable]

- **GIVEN** a peer address or control socket that is not serving the requested repository
- **WHEN** connect or fetch is attempted
- **THEN** the attempt MUST return a bounded failure, preserve the destination's prior state, and reap every child process owned by the harness

#### Scenario: Non-delegate cannot create a convergent update

r[cairn.radicle_two_peer_convergence.blocked.authority]

- **GIVEN** a third distinct signer that is not a current repository delegate and is not the change creator
- **WHEN** that signer attempts to update or resolve the transferred Cairn change
- **THEN** current identity authority checks MUST reject mutation before it can contribute a signed state classified as convergent

### Requirement: Claim boundary

r[cairn.radicle_two_peer_convergence.non_claims] Passing two-peer evidence MUST be scoped to the exact profiles, daemon versions, repository, operations, loopback transport, deadlines, and observations in the recorded run and MUST NOT be promoted into proof of public-network publication, discovery, arbitrary-peer trust, availability, eventual convergence, partition recovery, release eligibility, or a supported `collaboration sync` command.

#### Scenario: Receipt and documentation remain bounded

r[cairn.radicle_two_peer_convergence.non_claims.local]

- **GIVEN** both deterministic and live-node convergence rails pass
- **WHEN** Cairn documents or records the result
- **THEN** it MUST identify the tested transport and versions and preserve explicit non-claims for untested network, trust, availability, synchronization, and release properties
