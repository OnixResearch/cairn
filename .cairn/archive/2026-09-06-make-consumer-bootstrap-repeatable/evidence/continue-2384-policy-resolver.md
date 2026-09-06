# Project policy resolver increment

This increment preserves the existing bootstrap commits and completes the inherited resolver edits for validation, doctor, and CLI gates.
Cairn maintainers own this resolver and its tests.

## Observed behavior

- Explicit policy selection has precedence. Invalid, missing, repeated, and unsupported attached option values fail without fallback.
- Project selection resolves beneath the selected root, not the working directory.
- The declaration binds the complete-selection bytes, cohort, policy locator, and exact policy bytes.
- Bounded capability-relative reads reject symlinks, non-files, parent escapes, and excessive files.
- Only a proven absent path permits the legacy default. A failed open after an observed file does not permit fallback.
- Validation, doctor, and CLI gate output identify the policy source and exact BLAKE3 identity.
- Complete-selection metadata is not an installation receipt. No resolver check proves publication or durability.

The inherited direct `cap-fs-ext` dependency uses the already locked 4.0.2 package. Existing producer pins remain unchanged.
Cargo generated its dependency edge. No Nix lockfile change was needed or made.

## Commands and results

Cargo commands used `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c`.

- Baseline: `cargo test -p cairn-cli --test bootstrap_project_policy --test bootstrap_publication_contract --test consumer_bootstrap_controls`: 6 tests passed.
- Baseline: `cargo test -p cairn-core consumer_bootstrap`: see the separate core-baseline log.
- `cargo test -p cairn-cli --bin cairn bootstrap`: one bounded filesystem test passed.
- Final focused integration command, identical to the CLI baseline: 6 tests passed.
- `cargo clippy -p cairn-core -p cairn-cli --all-targets -- -D warnings`: passed.
- `cargo fmt --all --check`: passed.
- `git diff --check`: passed.

The Nickel control now checks accepted and rejected complete-selection records as well as the existing cohort, declaration, and ownership fixtures.
The gate replay checks policy resolution only. Its process status is not recorded as a passing gate payload.
Validation and doctor acceptance assertions inspect their `valid` payload fields.

Logs are retained outside the worktree under `.pi/cairn-rest-1295/` in the coordinator's Cairn checkout.
Their prefix is `continue-2384-make-consumer-bootstrap-repeatable-`.
The focused final log is `resolver-final.log`; earlier filtered test output is not full integration evidence.

## Remaining scope

Setup planning, installation, locked rechecks, publication, recovery, and generated-guidance composition remain unfinished.
Other command families and MCP still use their existing explicit/default loaders; they do not yet all use project selection.
No shell task is checked off. This increment does not claim full Octet acceptance, installation completion, or release readiness.
