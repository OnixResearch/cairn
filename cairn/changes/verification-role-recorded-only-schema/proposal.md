## Why

The local trust stack needs a `verification_role` discipline so that "0 verified"
cannot be misread as "all good." This is the c-earchin `recorded_only` discipline
applied to cairn receipts: every evidence artifact carries a role
(`property` | `recorded_only` | `boundary`), and only `property` artifacts count
as verified. Valence's Evidence IR v1 (Phase 0) already emits the schema; cairn
now consumes it in `evidence verify` and `release-readiness`.

## What Changes

- Add `cairn-core/src/verified/verification_role.rs`: pure core module with
  `VerificationRole` classification (`Property` | `RecordedOnly` | `Boundary` |
  `Unknown`), `VerificationRoleSummary` (counts by role, `verified_count` =
  `property_count` only), `from_artifacts` (reads `verification_role` from JSON
  artifact arrays), and `to_json` (renders summary with boundary non-claim).
- Wire `evidence verify` to read `verification_role` from each export-receipt
  artifact, include it in the verified-artifact entries, and emit a
  `verification_role_summary` in the verify receipt.
- Wire `release-readiness` to emit a `verification_role_boundary` non-claim
  field documenting the discipline.
- Update `cairn-policy/default.ncl` receipt schemas: add `verification_role_summary`
  to `evidence verify` top-level keys and `verification_role` to artifact keys.
- Update `fixtures/agent-receipts/positive.json` with the new policy hash
  (cascading from the policy schema update).
- Fix pre-existing test failures: `preserves_content_hash` was in the
  `agent-receipt preserves-hash` receipt but not in the expected test keys;
  `preserves_content_hash` was missing from the `agent-receipt verify-bundle`
  policy schema. Both are now fixed.

## Impact

- **Files**: `crates/cairn-core/src/verified/verification_role.rs` (new),
  `crates/cairn-core/src/verified/mod.rs`, `crates/cairn-cli/src/evidence/mod.rs`,
  `crates/cairn-cli/src/evidence/verify.rs`, `crates/cairn-cli/src/release_readiness.rs`,
  `crates/cairn-cli/tests/release_receipts/agent_replay.rs`,
  `crates/cairn-cli/tests/release_receipts/release_readiness.rs`,
  `crates/cairn-core/src/verified/agent_receipt/mod.rs`,
  `cairn-policy/default.ncl`, `cairn-policy/generated/cairn-policy.json`,
  `fixtures/agent-receipts/positive.json`.
- **Testing**: `cargo test -p cairn-core` (135 passed), `cargo test -p cairn-cli --test release_receipts` (18 passed, 1 pre-existing failure in `public_receipt_schemas_are_stable` due to an unrelated `--baseline` flag issue), `cargo fmt --check` (clean), `cairn policy export --check` (fresh).
- **Non-claims**: `verification_role_summary` counts only `property` artifacts as verified; `recorded_only` and `boundary` artifacts prove nothing. This is an epistemic-honesty rail, not a correctness proof.
