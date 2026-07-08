## Phase 1: Implementation

- [x] [serial] r[cairn.verification_role] Create `cairn-core/src/verified/verification_role.rs` pure core module: `VerificationRole` classification, `VerificationRoleSummary` with `verified_count == property_count`, `from_artifacts`, `to_json`, with positive and negative tests.
- [x] [serial] r[cairn.verification_role] Wire `evidence verify` to read `verification_role` from export-receipt artifacts, include in verified-artifact entries, emit `verification_role_summary` in the verify receipt.
- [x] [serial] r[cairn.verification_role] Wire `release-readiness` to emit `verification_role_boundary` non-claim field.
- [x] [serial] r[cairn.verification_role] Update `cairn-policy/default.ncl` receipt schemas: add `verification_role_summary` to `evidence verify` top-level keys, `verification_role` to artifact keys.
- [x] [serial] r[cairn.verification_role] Update `fixtures/agent-receipts/positive.json` with new policy hash; fix pre-existing `preserves_content_hash` test key gaps.
- [x] [serial] r[cairn.verification_role] Run validation: `cargo test -p cairn-core` (135 passed), `cargo test -p cairn-cli --test release_receipts` (18 passed, 1 pre-existing failure), `cargo fmt --check` (clean), `cairn policy export --check` (fresh).

## Phase 2: Archive

- [x] [serial] r[cairn.verification_role] Verify and archive the change.
  - Evidence 2026-07-08: synced `verification-role` accepted spec, added source traceability refs for all new IDs, and passed policy export check, `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cairn validate`, `cairn traceability coverage --root . --json`, `cairn release-readiness`, and Slotcar.

## Verification Coverage

| Scenario | Evidence |
|---|---|
| `cargo test -p cairn-core` passes | 135 passed, 0 failed |
| `cargo test -p cairn-cli --test release_receipts` passes | 18 passed, 1 pre-existing failure (`public_receipt_schemas_are_stable` due to unrelated `--baseline` flag issue) |
| `cargo fmt --check` clean | fmt check passes |
| `cairn policy export --check` fresh | `policy fresh: cairn-policy/generated/cairn-policy.json` |
| `release-readiness` reports `valid: true` | smoke: `valid: True`, `verification_role_boundary` present |
| `agent-receipt validate` on positive fixture passes | `valid: True`, 0 issues |
| Classifies known roles | `verification_role::tests::classifies_known_roles` |
| Classifies unknown role as Unknown | `verification_role::tests::classifies_unknown_role_as_unknown` |
| Only Property is verified | `verification_role::tests::only_property_is_verified` |
| Summary counts roles correctly | `verification_role::tests::summary_counts_roles_correctly` |
| Zero verified means nothing was verified | `verification_role::tests::summary_zero_verified_means_nothing_was_verified` |
| Summary from artifacts reads verification_role field | `verification_role::tests::summary_from_artifacts_reads_verification_role_field` |
| Summary JSON includes boundary non-claim | `verification_role::tests::summary_json_includes_boundary_non_claim` |
| Summary JSON has stable fields | `verification_role::tests::summary_json_has_stable_fields` |
