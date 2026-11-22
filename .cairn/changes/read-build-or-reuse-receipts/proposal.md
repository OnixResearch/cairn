# Read build-or-reuse realization receipts

## Why

Mantle's route `cache-inputs-build-roots-v2` may reuse a root it built and signed
earlier instead of building it again. Its receipt v2 records each selected root
with a disposition (`built` or `reused`), the builder signer fingerprint and,
for a reused root, a link to the original `built` receipt. Cairn's convergence
inspection currently understands only receipt v1 and never reads the root's
disposition, so it cannot tell a fresh build from an earlier one and cannot
check the reuse claim.

## What Changes

- `cairn-policy/assurance-targets.ncl` admits both realization receipt schema
  generations and the version 2 root dispositions `built` and `reused`; the
  generated JSON export is regenerated.
- `convergence inspect` accepts a version 2 realization receipt and checks the
  root entry, its disposition, its builder signer role and its fingerprint
  against a new case-level `trusted_builder_keys` list.
- A `reused` root must supply the case's `original_realization_receipt`. Cairn
  re-checks the recorded link: the original parses as version 2, records the
  linked digest, and carries a matching `built` root with the same NAR hash and
  signer fingerprint. A failure is the new `convergence.reuse_link_invalid`.
- A `current` row reports `bound.root_disposition` and, for a reused root,
  `bound.original_receipt_blake3`.
- Positive and negative fixtures under `fixtures/convergence/pilot-v2/`, unit
  tests in the convergence core and CLI coverage over the retained pilot.
- `docs/convergence.md` and the policy non-claims state that Cairn checks
  recorded fingerprints and the recorded link and does not verify signature
  bytes.

## Impact

- **Files**: convergence core evaluator and unit tests, convergence CLI input
  reader, policy source and generated export, convergence fixtures and CLI
  tests, `docs/convergence.md`.
- **Compatibility**: version 1 receipts keep their existing binding and states;
  a version 1 receipt on `cache-only-preserve-v1` additionally reports the
  derived `preserved` disposition. The case's new fields are optional.
- **Assurance**: advisory only. The receipt remains neither release
  eligibility nor proof of correctness, and no gate outcome changes.

## Out of Scope

- Verifying signature bytes or that a signature exists.
- Mantle's route implementation, receipt production and builder keys.
- Kiln's pinned version 1 receipt reader.
- Reuse from a cache, and key rotation by rebuild-compare-co-sign.
