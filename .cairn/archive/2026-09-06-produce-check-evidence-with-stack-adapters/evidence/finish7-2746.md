# Evidence producer finish7-2746

## Scope

Worktree: `/home/brittonr/git/OnixResearch/.worktrees/cairn-drain-evidence-20260905`.
Starting tip: `317b79a`.
No sync, archive execution, push, agent spawn, producer pin change, lockfile edit, gate weakening, or Nix store isolation attempt occurred.

## Commands and exits

All command logs use prefix `finish7-2746` under `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`.
Cargo commands used `TMPDIR=/var/tmp env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c`.

| Command | Exit | Evidence |
|---|---:|---|
| `cargo test --workspace --all-targets --all-features` | 0 | Pueue task 658; copied to `finish7-2746-workspace-all-features.log`; completed the previously missing full workspace/all-features run. |
| `cargo test -p cairn-core check_evidence` plus `cargo test -p cairn-cli --test check_evidence --test check_evidence_local --test check_evidence_saved --test check_evidence_stack --test check_evidence_nix` | 0 | `finish7-2746-controls-focused.log` |
| Kiln fixture replay: helper export from pinned `5d99f4e9bb954743443e340baece31256b4c53f1`, `cmp` against checked fixture, helper tests, and Cairn import test `original_kiln_fixture_export_crosses_the_real_import_boundary` | 0 | `finish7-2746-kiln-consumer-replay.log` |
| Developer-loop validator replay and producer gap audit | first command log exit 1 due a grep probe mismatch after the validator passed; replayed clean | `finish7-2746-developer-loop-gap.log`, `finish7-2746-developer-loop-gap2.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | `finish7-2746-verification-battery.log` |
| `cargo fmt --all -- --check` | 0 | same |
| `./target/debug/cairn policy export --check` | 0 | same; policy fresh |
| `./target/debug/cairn validate --root .` | 0 | same; validation valid |
| `./target/debug/cairn traceability coverage --root . --json` | 0 | same; 2307 requirements, 2307 referenced, no missing or dangling, verdict `pass` |
| `nix flake check "path:$PWD" --option min-free 0 -L` | 1 | same; failed only at `checks.x86_64-linux.rust-tests`, test `real_nix_success_is_not_an_admitted_payload_or_test_coverage` observed `exited_nonzero` instead of expected `exited_zero` inside the Nix check derivation |
| `./target/debug/cairn sync produce-check-evidence-with-stack-adapters --root .` | 0 | same; dry-run only |
| `./target/debug/cairn archive produce-check-evidence-with-stack-adapters --root .` | 0 | same; dry-run only, blocked because two tasks remain todo |

The flake check evaluated the check set through at least these names before the failing build: `slotcar`, `check-producer-source-pins`, `bootstrap-source-pin`, `artifact-auth-source-pin`, `nickel-cohort-exact`, `nickel-toolchain-fixtures`, `runtime-generated-policy-only`, `policy-fresh`, `readiness-policy-fresh`, `authenticated-policy-profiles-fresh`, `policy-stale-detects-drift`, `packaged-policy-export-runtime`, `nickel-export-cutover`, `preserves-corpus-fresh`, `preserves-corpus-stale-detects-drift`, `gate-contract`, `gate-contract-wasm`, `gate-contract-nickel`, `rustfmt`, `rust-tests`, `radicle-cob-format-compatibility`, `cairn-validate`, `tracey-coverage`, `trace-index-live-proof`, `rename-classification`, `self-dogfood`, and `clippy`.

## Failure-cohort matrix

| Cohort | Local run coverage | Saved-import coverage | Publication / verify coverage | Verdict |
|---|---|---|---|---|
| denied | `real_cargo_passes_original_completion_and_denial_never_spawns`; `explicit_execution_and_import_never_use_a_fallback`; core `authorization_requires_exact_plan_and_explicit_execute` | `explicit_execution_and_import_never_use_a_fallback` rejects unsupported import execution fallback | denied run creates no attempt or artifact | Covered |
| empty-test | `real_empty_failed_payload_timeout_and_flood_never_complete`; core `zero_exit_is_not_empty_test_payload_cache_or_build_success`; cargo grammar `complete_nonempty_summary_and_negative_grammar_controls` | saved producers return `completion_eligible: false` and no completion rows | `rehashed_nonpassing_observations_cannot_hide_behind_passing_rows` rejects zero executed tests behind passing rows | Covered |
| failed-payload | `real_empty_failed_payload_timeout_and_flood_never_complete`; core `zero_exit_is_not_empty_test_payload_cache_or_build_success`; `nix_check_is_only_a_build_obligation` | `native_mantle_admission_preserves_status_and_rejects_substitution`; `native_octet_json_translation_is_bounded_redacted_and_never_strict_acceptance`; `original_mantle_fixture_intake_keeps_cache_and_failure_meaning` | `publication_rejects_mixed_attempts_and_failed_or_reordered_rows`; `rehashed_nonpassing_observations_cannot_hide_behind_passing_rows` | Covered |
| skipped | core `zero_exit_is_not_empty_test_payload_cache_or_build_success`; Lattice `original_producer_positive_skipped_and_negative_cohorts_agree`; Kiln `native_kiln_validator_preserves_success_skipped_and_pending_without_coverage` | `published_lattice_inputs_reach_the_shell_without_test_promotion`; `original_kiln_fixture_export_crosses_the_real_import_boundary`; `valence_and_kiln_json_have_native_meaning_but_no_execution_coverage` | `rehashed_nonpassing_observations_cannot_hide_behind_passing_rows` rejects native `skipped` behind a passing row | Covered |
| timed-out | `real_empty_failed_payload_timeout_and_flood_never_complete`; core `every_process_failure_stays_nonpassing_in_original_consumer` | Mantle saved statuses include `timed_out` in `native_mantle_admission_preserves_status_and_rejects_substitution` | `rehashed_nonpassing_observations_cannot_hide_behind_passing_rows`; `publication_rejects_mixed_attempts_and_failed_or_reordered_rows` | Covered |
| cancelled | `real_cancellation_and_source_drift_withhold_success`; core `every_process_failure_stays_nonpassing_in_original_consumer` | Mantle saved statuses include `cancelled`; Lattice/Kiln native controls preserve cancelled/skipped/pending without coverage | non-passing rows withhold manifest; interruption keeps `incomplete.json` | Covered |
| flooded | `real_empty_failed_payload_timeout_and_flood_never_complete`; core `every_process_failure_stays_nonpassing_in_original_consumer` maps output overflow to `infrastructure_error` | saved import malformed/raw secret controls reject without echoing raw payloads | final manifest absent after infrastructure error | Covered |
| stale | `real_cancellation_and_source_drift_withhold_success`; `unavailable_source_preserves_executed_facts_without_completion`; core `stale_and_substituted_observations_are_rejected` | Lattice `stale_execution`, Kiln `stale_attempt`, Valence/Octet identity drift tests, Mantle request identity substitutions | `only_complete_current_readback_requests_a_final_manifest`; `final_manifest_requires_exact_complete_artifacts_and_explicit_recovery` | Covered |
| substituted | core `invalid_contracts_never_create_plans`, `bounds_environment_grammar_and_duplicates_fail_closed`, `target_features_and_environment_change_review_identity`, `stale_and_substituted_observations_are_rejected`; CLI `plan_is_current_policy_bound_read_only_and_strict` | `kiln_substitution_and_native_boundaries_remain_blockers`; `request_profile_identity_and_order_substitutions_fail_closed`; `substitutions_and_nonclaims_fail_closed`; saved Mantle/Octet substitution tests | changed/missing artifacts and rehashed non-passing observations reject in manifest tests and `check-evidence verify` | Covered |
| teardown-failure | `real_owned_descendant_teardown_and_escaped_pipe_uncertainty`; core `every_process_failure_stays_nonpassing_in_original_consumer` and `zero_exit_is_not_empty_test_payload_cache_or_build_success` | saved `unknown` Mantle outcome remains native but completion-ineligible | `rehashed_nonpassing_observations_cannot_hide_behind_passing_rows` rejects `teardown_complete=false`; manifest withheld | Covered |

The matrix is an audit over the currently checked implementation. No task was marked complete from a planning gate.

## Consumer replay status

The fresh Cairn local consumer replay is covered by `real_cargo_passes_original_completion_and_denial_never_spawns`: `check-evidence run` produces local Cargo evidence, original `agent-receipt complete` accepts it, `check-evidence verify` admits the final manifest, and the unchanged developer-loop validator rejects those Cargo rows with `developer_loop.receipt.schema` rather than reinterpreting them as loop facts.

The pinned Kiln replay used the original fixture source at `5d99f4e9bb954743443e340baece31256b4c53f1`. The helper regenerated `target/finish7-2746-kiln-producer-replay.json`, `cmp` matched `fixtures/check-evidence/kiln-producer-fixtures.json`, helper tests ran the original validator, and Cairn's real import boundary accepted the generated fixture while keeping `completion_eligible: false`.

The positive `cairn-local` developer-loop replay remains a producer-owned gap. The admitted producer contract currently allows only `cargo_test` and `nix_check` profile kinds, Rust has only `CheckKind::{CargoTest,NixCheck}`, local payload admission only trusts Cargo test summaries, and `check-evidence run` emits completion rows instead of `cairn.developer-loop-receipt.v1` with `fmt`, `validate`, `traceability`, and total duration facts. The existing `developer_loop` tests prove the original validator accepts a correctly shaped receipt and rejects stale subjects, but that receipt is hand-built test input, not generated producer evidence.

## Task state

`controls` is covered by the matrix above and the focused control replay.
`consumers` remains open only for the positive generated `cairn-local` developer-loop receipt gap.
`verification` remains open because `nix flake check` exits 1, and archive preview remains blocked by the two open tasks.

## Remaining exact gaps

1. Add a producer-owned loop receipt path or admitted profile family that generates `cairn.developer-loop-receipt.v1` with fmt, lifecycle validation, traceability, and bounded duration facts without hand-authored result rows.
2. Fix the Nix-check fixture so `checks.x86_64-linux.rust-tests` observes the success Nix payload as `exited_zero` inside the Nix check derivation. Do not reattempt the reverted local-store isolation approach.
3. Re-run `nix flake check "path:$PWD" --option min-free 0 -L` after that repair, then replay dry-run sync/archive previews.
