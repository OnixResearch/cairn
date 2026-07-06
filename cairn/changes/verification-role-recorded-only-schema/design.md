## Design: c-earchin recorded_only discipline for cairn receipts

### Functional core / imperative shell boundary

`verification_role.rs` is pure deterministic core: no I/O, no filesystem, no
process, no async. It classifies role strings and summarizes artifact arrays
in-memory. The CLI shell (`evidence/verify.rs`, `release_readiness.rs`) reads
artifacts from disk and passes them to the core.

### VerificationRole classification

```
"property"      -> Property      (counts as verified)
"recorded_only" -> RecordedOnly  (records structure only, proves nothing)
"boundary"      -> Boundary      (non-claim text artifact)
anything else   -> Unknown       (counted separately, not verified)
```

The key invariant: `verified_count == property_count`. A "0 verified" receipt
means "nothing was verified," not "all good." This prevents the vacuous-pass
misread.

### Integration points

- `evidence verify`: reads `verification_role` from each artifact in the export
  receipt, includes it in the verified-artifact entries, and emits
  `verification_role_summary` in the final receipt.
- `release-readiness`: emits `verification_role_boundary` non-claim text
  documenting the discipline. Does not count artifacts itself (that is
  `evidence verify`'s job); the boundary field ensures consumers know the rule.

### Policy schema

The Nickel policy (`cairn-policy/default.ncl`) owns the receipt schema. Updated:
- `evidence verify` `top_level_keys`: added `verification_role_summary`
- `evidence verify` `artifact_keys`: added `verification_role`

### Pre-existing fixes

While updating the schema, fixed two pre-existing issues:
- `preserves_content_hash` was in the `agent-receipt preserves-hash` receipt but
  not in the test's expected keys.
- The `agent-receipt verify-bundle` policy schema already had
  `preserves_content_hash`; the `agent-receipt validate` schema correctly does
  NOT include it (the validate receipt does not carry that field).

### What is explicitly NOT provided

- Enforcement of "only `property` counts as verified" in `release-readiness`
  gating. That lands in Phase 2/3 when the role is bound to a
  provenance-validated receipt and the EARS property vocabulary gate dispatches
  to trellis property predicates.
- A hard dependency on valence-core. The role strings (`property`,
  `recorded_only`, `boundary`) are recognized as plain strings; valence-core's
  `VerificationRole` enum serializes to exactly those strings, so when valence
  artifacts flow through cairn in Phase 2, the roles will be recognized
  automatically.
