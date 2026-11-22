# Design

## Context

Mantle's receipt v1 has no field that distinguishes a root it built from one it
merely preserved, and the existing Cairn binding reads only per-unit
`execution_class` (never) and the output `target_path` and `nar_sha256`.
Receipt v2 adds a `roots` array whose entries carry the disposition, the builder
signer fingerprint and the reuse link. The reuse claim is only meaningful if
Cairn re-checks it; otherwise a replayed or edited receipt could relabel any
present root as `reused`.

## Decisions

### Decision: Policy owns the admitted generations and dispositions

**Choice:** `schemas.mantle_realization_receipts` lists the admitted receipt
schemas (`v1` and `v2`), and `mantle.root_dispositions` lists `built` and
`reused`. The generated JSON is regenerated with the repository's Nickel export.

**Rationale:** The closed vocabularies belong with the other admitted foreign
schema identifiers. The CLI admits either receipt generation by that list, and
the pure evaluator admits a version 2 disposition only from the policy list. A
new generation or disposition is therefore a reviewed policy change, not a
silent parser widening.

### Decision: A disposition is derived for version 1, read for version 2

**Choice:** For a version 1 receipt Cairn derives `preserved` from
`cache-only-preserve-v1` and `built` from `cache-inputs-build-roots-v1`; every
other route, including none, reports null. For version 2 it reports the root
entry's admitted disposition.

**Rationale:** Version 1 receipts must keep their bytes and states. Their route
already tells Cairn which meaning applies: the preserve route did not build or
execute the root, and the version 1 build route always refused a present root.
Null on an unclassified route avoids inventing a claim.

### Decision: Check recorded identities, never signature bytes

**Choice:** A version 2 root is admitted only with `signer.role == "builder"`
and a `signer.public_key_blake3` the case lists in `trusted_builder_keys`.
A `reused` root additionally requires the case's `original_realization_receipt`
and a pure link check over the recorded identities.

**Rationale:** Cairn has no public key material for these records and its
existing signer check is name-based. Verifying signature bytes would need a new
key-distribution contract outside this change. Re-checking the link makes the
reuse claim falsifiable against the supplied original: a changed digest, NAR
hash, disposition, signer fingerprint, derivation, output or path refuses the
row.

### Decision: Fail closed with one stable link code

**Choice:** Every reuse-link failure reports `convergence.reuse_link_invalid`
and leaves the row not `current`; an untrusted builder fingerprint reports
`convergence.untrusted_signer`; a disposition outside the policy or a
`built`/`reused` object that contradicts the disposition reports
`convergence.realization_incomplete`.

**Rationale:** One code for the link keeps the failure attributable to the
claim Cairn cannot substantiate, while reusing the existing signer and
incomplete codes avoids growing the vocabulary without need.

## Risks / Trade-offs

- The case now trusts builder fingerprints explicitly; a case that omits
  `trusted_builder_keys` makes every version 2 row not `current`, which is the
  intended fail-closed behavior but requires updating operators.
- A reused receipt linked to an original with internally inconsistent fields
  beyond the checked set is not detected. Cairn checks the listed identities
  only and does not recompute either receipt's digest.
- Version 1 receipts on the ordinary route still report no disposition, because
  that route can mix preserved and substituted outputs.
