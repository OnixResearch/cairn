# Verification (2026-11-19, this clone, base 96559148)

- `cargo test -p cairn-core --lib selected_work workflow_obligations`: 27 passed after the not-selected expectation update (selected_work 10, workflow_obligations 17 including 2 new consumer-fact tests).
- `cargo test -p cairn-core --test selected_work_consumer`: 3 passed.
- `cargo test -p cairn-cli --test selected_work --test workflow_obligations --test readiness --test mcp_cli`: 11, 6, 6, 30 passed.
- `cargo fmt --all --check`: clean. `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `nix run path:$PWD#cairn -- policy export --check`: `policy fresh`.
- Dev-binary `cairn validate --root .`: valid, 0 issues; proposal/design/tasks gates valid; `cairn sync --execute` merged both deltas; `cairn traceability coverage --root .`: 2477/2477.
- Nickel: `contracts/selected-work.ncl` typechecks; positive fixture exports; negative fixture is refused on `state`; generated bounds are fresh.
- `cargo test --workspace --no-fail-fast`: 11 failures in `release_receipts` (1), `lifecycle` (1), `lifecycle_mutations` (1), `buoy_comparison` (1) and `check_evidence_local` (7). The same 11 fail identically on a pristine `git archive` of 96559148 with the same commands; they are stale goldens/receipt fixtures, the active buoy change and an `Error: Profile` environment failure, not this change.
- `cargo test -p cairn-cli --test workflow_obligations` failed on main because its expected command classes omitted `readiness`; the expectation was aligned with the shipped policy.
- `nix flake check path:$PWD -L --builders '' --keep-going`: exit 1. The new `selected-work-contracts` check built. Failures: `readiness-policy-fresh`, `targeted-context-policy-fresh`, `incremental-observations-policy-fresh` and `workflow-readiness-policy-fresh` (Nickel "infinite recursion" in policy files this change does not touch, e.g. `targeted-context.ncl:19 contracts = { Policy = Policy }`), and `rust-tests` (first failure `buoy_comparison`, one of the pre-existing workspace failures).
- After the separate repair (renamed self-referencing Nickel bindings, `DetailKind` diagnostic match, refreshed hash-only goldens and preserves hash) and the new `dependencies` cohort domain: `nix build` of `readiness`, `targeted-context`, `incremental-observations`, `workflow-readiness` policy-fresh checks and `selected-work-contracts`: exit 0. `cargo fmt --check` and Clippy: exit 0. `cargo test --workspace --no-fail-fast`: 1851 passed, 10 failed: `buoy_comparison` (1, active buoy change), `check_evidence_local` (7) and `check_evidence_nix` (2). The check-evidence harnesses copy the ambient `PATH` (4910 bytes in this dev shell) into a profile bounded at 4096 bytes; with a 1119-byte `PATH` `check_evidence_local` passes 8/8.
- Harness repair: the check-evidence fixtures now pass a `PATH` built from the directories of the tools they run (`cargo`, `rustc`, `cc`, `ld`; or `nix`), deduplicated in tool order, and fail with an explicit message above the 4096-byte bound. Product admission is unchanged. In the repo dev shell with its 5353-byte `PATH`: `check_evidence_local` 8 passed, `check_evidence_nix` 2 passed.

## Final gates (2026-11-19, repo dev shell, `PATH` 5353 bytes)

- `cargo fmt --all --check`: exit 0.
- `cargo clippy --workspace --all-targets -- -D warnings`: exit 0.
- `cargo test --workspace --no-fail-fast`: exit 101; 1860 passed, 1 failed: `buoy_comparison::comparison_identity_binds_source_mapping_fixture_and_policy`.
- Pre-existing, owned by the active `add-buoy-traceability-comparison` change: for the `schema_version_drift` case the native trace index now reports `stale_item`, while the comparator and test declare `unsupported` (issue `comparison.native_execution_mismatch`). It fails identically on a clean export of 96559148: `git archive 96559148 | tar -x -C /tmp/cto-clean && cd /tmp/cto-clean && TMPDIR=/tmp CARGO_TARGET_DIR=<separate-target> RUSTC_WRAPPER= nix develop path:/tmp/cto-clean --builders '' -c cargo test -p cairn-cli --test buoy_comparison` (exit 101; 3 passed, 1 failed; same message). Owner decision: integrate with this failure recorded as pre-existing.

## Rebased onto upstream main (2026-11-19)

Upstream `main` moved to 585f50d9 and then 3dc654d3 (assurance targets, convergence inspect, reviewed-specification binding). Both rebases applied without conflicts; the generated policy hash did not change, so no golden or receipt refresh was needed.

- On 585f50d9: full `nix flake check` 40 checks, 38 passed; `rust-tests` failed only on `buoy_comparison`, and `tracey-coverage` failed on the upstream references below.
- On 3dc654d3 (repo dev shell): `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` exit 0; `cargo test --workspace --no-fail-fast` 1930 passed, 10 failed; packaged `policy export --check` fresh and `validate` valid; `traceability coverage` exit 1.
- Pre-existing on upstream, owned by its active assurance-targets and convergence change: 12 dangling requirement references (`cairn.assurance_targets.{contract,evidence_job,projection,row_binding}`, `cairn.convergence.{acceptance,authority,completeness,determinism,inputs,inspection,realization,states}`) fail `traceability coverage`, and through it nine tests (`release_readiness` x6, both evidence-export tests, the developer-loop receipt). With `buoy_comparison` these are the same 10 failures a clean `git archive 585f50d9` export shows with the same commands; none of this change's references dangle. The repair commit additionally fixes eight tests that fail on that export (the PATH-length and stale-hash cases).

No live Kiln campaign or Lattice execution was run.
