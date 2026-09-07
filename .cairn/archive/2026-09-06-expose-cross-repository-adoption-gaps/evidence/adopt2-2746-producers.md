# adopt2-2746 producer-admission evidence

## Scope

Completed task `producers` only. No sync, archive, push, sibling scans, producer execution, or dependency pin changes were performed.

## Producer contract study

Inspected the workspace root source at immutable revision `22515fb7bc01d378312053d389d4bbdd55637a50` with `git show` from the dirty primary workspace without editing it.

Reviewed producer contracts:

- `tools/stack-deps/src/model.rs`: `onixresearch.stack-dependency-catalog.v2`, `onixresearch.stack-dependency-receipt.v1`, `onixresearch.stack-dependency-reconciliation.v2`, dependency states, audit receipt fields, and required non-claims.
- `tools/stack-deps/src/core.rs`: full audit and reconciliation validation boundaries, source identity matching, exact revision checks, no mutation evidence in audit receipts, and required non-claims.
- `tools/stack-deps/src/health_model.rs`: `onixresearch.stack-health-receipt.v1`, conformance result status vocabulary, checked receipt fields, and health non-claims.
- `tools/stack-deps/src/health_core.rs`: stack-doctor receipt validation, required claims, checked-scope count, result identity checks, and conformance status vocabulary.
- `tools/stack-deps/src/bin/stack-doctor.rs`: stack-doctor command contract (`profile-check`, `receipt-check`, `observe`) and preview-free observation boundary. Cairn producer admission remains saved-input only and does not run the tool.

## Implementation

Changed `crates/cairn-core/src/verified/adoption_report.rs`:

- Added source-revision and schema constants for the exact workspace producer revision named in `docs/cairn-utility-program.md`.
- Added `AdoptionProducerCohort`, `AdoptionProducerAdmission`, and `StackDepsContractInput` pure-core contracts.
- Added `admit_stack_deps_contracts` for explicit saved catalog + full audit receipt + reconciliation receipt admission.
- Added `admit_stack_doctor_health` for explicit saved stack-health receipt admission.
- Added direct positive and negative controls for:
  - exact stack-deps schema/source/non-claim admission;
  - old consumer pin preserved as `consumer_pin: gap`;
  - unsupported/stale stack-deps schema or reconciliation hash rejection;
  - stack-doctor pass/fail conformance preserved as integration states;
  - stack-doctor claim drift and unsupported status rejection.

Changed `cairn-policy/adoption-report.ncl` and positive/negative fixtures so the policy cohort records the producer-owned schema names and non-claim vocabulary found at revision `22515fb7bc01d378312053d389d4bbdd55637a50`.

Changed `.cairn/changes/expose-cross-repository-adoption-gaps/tasks.md`: marked only `producers` complete after the focused controls and task gate passed.

## Commands and exits

All commands used the required PATH prefix, `TMPDIR=/var/tmp`, and private Pueue group `adopt2-2746`.

- `git status --short && git rev-parse --short HEAD && git log --oneline -5` (pueue 536): exit 0; starting worktree clean at `80b50da`.
- `git -C /home/brittonr/git/OnixResearch status --short && git -C /home/brittonr/git/OnixResearch rev-parse HEAD ...` (pueue 538): exit 0; primary workspace was dirty, so it was read-only only.
- Producer source inspections with `git show 22515fb7...:tools/stack-deps/...` (pueue 540, 541, 542, 544): exit 0.
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c cargo test -p cairn-core adoption_report -- --nocapture` (pueue 570, log `adopt2-2746-producers-core-test.log`): exit 0; 10 adoption report tests passed.
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c cargo fmt --all -- --check && ... cargo clippy -p cairn-core --all-targets -- -D warnings` (pueue 576, log `adopt2-2746-producers-fmt-clippy.log`): exit 0.
- `./target/debug/cairn policy export --check` (pueue 561): exit 127 because `./target/debug/cairn` was not built in this target directory.
- `nix run path:$PWD#cairn -- policy export --check` (pueue 572, log `adopt2-2746-producers-policy.log`): exit 0; policy fresh.
- `nix run path:$PWD#cairn -- validate --root . && nix run path:$PWD#cairn -- gate tasks expose-cross-repository-adoption-gaps --root .` (pueue 575, log `adopt2-2746-producers-validate-gate.log`): exit 0; lifecycle valid and task gate passed with 4 done / 6 todo, next ready task `evidence`.
- `nix run path:$PWD#cairn -- traceability coverage --root . --json` (pueue 581, log `adopt2-2746-producers-traceability.json`): exit 1; new Rust `r[...]` markers referenced delta-spec IDs that are not accepted until sync.
- Removed those pre-sync Rust markers without changing logic; retained the task evidence in this file instead.
- `cargo fmt --all -- --check && cargo test -p cairn-core adoption_report -- --nocapture` (pueue 589, log `adopt2-2746-producers-post-trace-repair.log`): exit 0; 10 adoption report tests passed.
- `nix run path:$PWD#cairn -- traceability coverage --root . --json` (pueue 591, log `adopt2-2746-producers-traceability-repaired.json`): exit 0; no missing or dangling markers.
- `cargo clippy -p cairn-core --all-targets -- -D warnings` (pueue 592, log `adopt2-2746-producers-clippy-final.log`): exit 0.

Non-blocking environment noise observed in Nix runs: `warning: unknown experimental feature 'wasm-builtin'` and failed SSH ref refresh for `OnixResearch/onix-artifact`; Nix used the cached ref and completed.

## Remaining gaps

Open tasks, in order:

1. `evidence`: compose Cairn lifecycle/completion/registry and applicable Valence admission without promoting upstream or catalog authority.
2. `interfaces`: add bounded CLI and MCP report queries with parity/coverage controls.
3. `controls`: old-pin, missing-check, stale-policy, conflicting-revision, unsupported-schema, cycle, and partial-scope controls beyond the producer-admission subset.
4. `consumers`: replay saved Cairn-to-Octet and Octet-to-Lattice cohorts from a fresh directory without sibling checkouts or services.
5. `docs`: publish producer recipe, authority map, supported versions, diagnostic actions, and bounded-stage interpretation.
6. `verification`: full battery, sync dry-run, and archive dry-run. Coordinator owns sync/archive/push.
