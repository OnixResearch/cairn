# Radicle Two Peer Convergence Specification

## ADDED Requirements

### Requirement: Two-peer convergence rail retirement
r[cairn.radicle_two_peer_convergence.retired] Cairn MUST omit Radicle node and peer-convergence validation from maintained product gates under the COB-only provider boundary.

#### Scenario: Local format validation replaces runtime evidence
r[cairn.radicle_two_peer_convergence.retired.local_only]
- GIVEN the maintained source, test, and Nix validation surfaces
- WHEN collaboration compatibility is checked
- THEN Cairn MUST validate local signed COB format behavior without starting nodes or transferring refs between peers
- AND passing local checks MUST NOT be described as publication, synchronization, availability, or convergence evidence.

## REMOVED Requirements

### Requirement: Independent signed-reference transport
r[cairn.radicle_two_peer_convergence.transport] This obsolete transport requirement MUST be removed from maintained validation.

### Requirement: Explicit concurrent conflict convergence
r[cairn.radicle_two_peer_convergence.conflict] This obsolete peer-convergence requirement MUST be removed from maintained validation.

### Requirement: Resolution transfer convergence
r[cairn.radicle_two_peer_convergence.resolution] This obsolete resolution-transfer requirement MUST be removed from maintained validation.

### Requirement: Real loopback node evidence
r[cairn.radicle_two_peer_convergence.live] This obsolete live-node requirement MUST be removed from maintained validation.

### Requirement: Bounded negative paths
r[cairn.radicle_two_peer_convergence.blocked] This obsolete peer-failure requirement MUST be removed from maintained validation.

### Requirement: Claim boundary
r[cairn.radicle_two_peer_convergence.non_claims] This obsolete convergence-claim requirement MUST be removed from maintained validation.
