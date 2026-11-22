# Realization receipt generations

## ADDED Requirements

### Requirement: Admitted receipt generations and root dispositions

r[cairn.convergence.realization.policy_admission] Cairn MUST admit the Mantle realization receipt schema generations `mantle-foreign-realization-receipt-v1` and `mantle-foreign-realization-receipt-v2` through the assurance-targets policy and its generated export, and MUST admit only the version 2 root dispositions `built` and `reused`.

The case MUST accept an optional `trusted_builder_keys` array of builder public-key fingerprints beside `trusted_signers`, and an optional per-contract `original_realization_receipt` path beside `realization_receipt`. A receipt whose schema identifier the policy does not admit MUST be rejected before evaluation, and a version 2 root disposition the policy does not admit MUST leave the row not `current`.

#### Scenario: Both generations are admitted

- GIVEN a case naming a version 1 or version 2 realization receipt whose schema the policy export admits
- WHEN the command runs
- THEN the receipt MUST be read within its bound and evaluated
- AND a schema outside the admitted list MUST be rejected with `convergence.unknown_schema`.

#### Scenario: Policy drift changes the decision

- GIVEN a version 2 root whose disposition the policy does not admit
- WHEN the row is evaluated
- THEN it MUST report `convergence.realization_incomplete`
- AND it MUST NOT be reported `current`.

### Requirement: Root disposition and builder signer

r[cairn.convergence.realization.root_disposition] For a version 2 receipt Cairn MUST bind the `roots` entry for the handoff root's node id and the consumed output path, MUST require its `signer.role` to be `builder` and its `signer.public_key_blake3` to be one of the case's `trusted_builder_keys`, and MUST report the admitted disposition as `bound.root_disposition`.

A builder fingerprint absent from the trusted list MUST leave the row not `current` for `built` and `reused` roots alike. A version 1 receipt MUST keep its existing binding and MUST NOT be re-read as version 2; on `cache-only-preserve-v1` it MUST report the derived disposition `preserved`, on `cache-inputs-build-roots-v1` the derived disposition `built`, and on any other route no disposition.

#### Scenario: A built version 2 root is current

- GIVEN a complete version 2 receipt whose selected root has disposition `built`, a builder signer and a trusted fingerprint
- WHEN the remaining bindings and Octet's decision hold
- THEN the row MUST be reported `current`
- AND `bound.root_disposition` MUST be `built` and `bound.original_receipt_blake3` MUST be null.

#### Scenario: An untrusted builder fingerprint is never current

- GIVEN a version 2 root whose signer fingerprint is absent from the case's trusted builder keys, or whose role is not `builder`
- WHEN the row is evaluated
- THEN it MUST report `convergence.untrusted_signer`
- AND it MUST NOT be reported `current`.

#### Scenario: A preserved version 1 receipt reports preserved

- GIVEN a version 1 receipt on `cache-only-preserve-v1`
- WHEN the row is current
- THEN `bound.root_disposition` MUST be `preserved`
- AND the version 1 binding and states MUST otherwise be unchanged.

### Requirement: Reuse link

r[cairn.convergence.realization.reuse_link] A version 2 root with disposition `reused` MUST require the case's `original_realization_receipt` and MUST be reported `current` only when that file parses as version 2, records a `receipt_blake3` equal to the root's `reused.original_receipt_blake3`, and carries a root entry for the same derivation, output and path with disposition `built`, the same NAR hash and the same signer fingerprint.

Any failure of those checks MUST report `convergence.reuse_link_invalid` and leave the row not `current`, and a `reused` root whose Octet decision is missing or failing MUST NOT be `current`. A current reused row MUST report `bound.original_receipt_blake3`. Cairn checks recorded identities only; it does not verify signature bytes or recompute either receipt's digest.

#### Scenario: A linked reused root is current

- GIVEN a reused version 2 root and the case's original `built` receipt with the linked digest, NAR hash and signer fingerprint
- WHEN the remaining bindings and Octet's decision hold
- THEN the row MUST be reported `current`
- AND `bound.root_disposition` MUST be `reused` and `bound.original_receipt_blake3` MUST be the linked digest.

#### Scenario: A broken link is never current

- GIVEN a reused root with a missing original, another recorded digest, another NAR hash or signer fingerprint, an original disposition other than `built`, or a failing Octet decision
- WHEN the row is evaluated
- THEN a link failure MUST report `convergence.reuse_link_invalid` and a failing decision its decision code
- AND the row MUST NOT be reported `current`.
