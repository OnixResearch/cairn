## Phase 1: Implementation

- [x] [serial] r[cairn.ears_vocabulary] Create `cairn-core/src/verified/ears_property_vocabulary.rs`: 16-predicate vocabulary, `classify_ears_response`, `check_ears_vocabulary_coverage`, `EarsVocabularyReport` with `to_json()`, with 13 positive and negative tests.
- [x] [serial] r[cairn.ears_vocabulary] Register the module in `verified/mod.rs` and run validation: `cargo test -p cairn-core` (148 passed), `cargo fmt --check` (clean).

## Phase 2: Archive

- [x] [serial] r[cairn.ears_vocabulary] Verify and archive the change.
  - Evidence 2026-07-08: synced `ears-vocabulary` accepted spec, added source traceability refs for all new IDs, and passed policy export check, `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cairn validate`, `cairn traceability coverage --root . --json`, `cairn release-readiness`, and Slotcar.

## Verification Coverage

| Scenario | Evidence |
|---|---|
| `cargo test -p cairn-core` passes | 148 passed (135 original + 13 new) |
| `cargo fmt --check` clean | fmt check passes |
| `cairn validate` valid | valid: true, 0 issues |
| Classifies non-negative response | `ears_property_vocabulary::tests::classifies_non_negative_response` |
| Classifies monotone response | `ears_property_vocabulary::tests::classifies_monotone_response` |
| Classifies reject response | `ears_property_vocabulary::tests::classifies_reject_response` |
| Classifies preserves invariant | `ears_property_vocabulary::tests::classifies_preserves_invariant_response` |
| Classifies deadlock response | `ears_property_vocabulary::tests::classifies_deadlock_response` |
| Unresolvable response is recorded-only | `ears_property_vocabulary::tests::classifies_unresolvable_response_as_recorded_only` |
| Empty response is recorded-only | `ears_property_vocabulary::tests::classifies_empty_response_as_recorded_only` |
| Strict mode fails on unresolved | `ears_property_vocabulary::tests::strict_mode_fails_on_unresolved` |
| Strict mode passes when all resolved | `ears_property_vocabulary::tests::strict_mode_passes_when_all_resolved` |
| Non-strict accepts unresolved | `ears_property_vocabulary::tests::non_strict_mode_accepts_unresolved` |
| Empty resolutions pass | `ears_property_vocabulary::tests::empty_resolutions_pass` |
| Report includes boundary | `ears_property_vocabulary::tests::report_includes_boundary_non_claim` |
| Vocabulary predicates stable | `ears_property_vocabulary::tests::vocabulary_predicates_are_stable` |
