# Radicle Two Peer Convergence Specification

## Purpose

Defines the `radicle-two-peer-convergence` capability.

## Requirements

### Requirement: Two-peer convergence rail retirement
r[cairn.radicle_two_peer_convergence.retired] Cairn MUST omit Radicle node and peer-convergence validation from maintained product gates under the COB-only provider boundary.

#### Scenario: Local format validation replaces runtime evidence
r[cairn.radicle_two_peer_convergence.retired.local_only]
- GIVEN the maintained source, test, and Nix validation surfaces
- WHEN collaboration compatibility is checked
- THEN Cairn MUST validate local signed COB format behavior without starting nodes or transferring refs between peers
- AND passing local checks MUST NOT be described as publication, synchronization, availability, or convergence evidence.
