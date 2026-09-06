# Bootstrap publication handoff

## Outcome, owner, and commit

Implementation commit: `1499c654402c839ac46e5779141c01d144cf487c` — Publish reviewed bootstrap generations with recoverable local state.

Cairn maintainers own the bootstrap shell, generated-guidance composition, operator guide, and consumer fixtures.
The durable contribution is a repeatable local publication path over the already admitted Cap Root, Nickel Export, and Durable File Publication dependencies.
No dependency, producer pin, Cargo lockfile, Nix lockfile, or notice changed.

This pass did not finish the change. No checkbox changed. The six open tasks remain open rather than claiming whole-task acceptance from partial or failed verification.
There was no sync, archive, push, agent spawn, or worktree removal.

## Implemented behavior

- `setup plan` and non-executing `setup apply` remain read-only.
- Apply requires `--execute` and the exact `plan_hash`. It binds source observations, the root device/inode, prior selection, desired metadata, and generated-workflow plan.
- Apply acquires a project-local exclusive lock and rebuilds the plan before publication.
- Bounded no-follow capability reads and the pinned publication mechanism enforce the file boundary.
- Prepared state precedes artifact publication. The complete manifest and selection pointer follow the artifacts.
- Publication requires parent synchronization and exact read-back. Errors retain prepared state or the lock.
- A changed selected policy requires a new generation path. Existing generations and consumer-owned files remain untouched.
- `setup check` checks current managed identities and generated-guidance freshness.
- `setup recover` is diagnostic/manual only. It retains interrupted state and reports a blocked manual-repair category. It does not resume, roll back, delete, or retry synchronization.
- The existing generated-workflow renderer, observer, and ownership planner supply guidance files and freshness findings. Bootstrap owns their durable publication.
- Doctor checks explicitly declared project-local guidance. Missing, modified, or unmanaged declarations cannot conceal the obligation. No home-directory guidance discovery occurs.
- The operator guide documents setup, precedence, upgrade generations, manual recovery, boundaries, and unchanged low-level init behavior.

## Observed controls

The final consumer replay uses `env_clear`, an isolated HOME, an empty consumer root, and an explicit source root.
It checks actual JSON fields rather than process success alone.
It installs policy and guidance, validates through the project declaration, changes the policy in a declared upgrade, and validates again.
The upgrade publishes a new policy generation and refreshes generated guidance while retaining the old policy bytes and a consumer-owned Nickel file.

Negative controls cover invalid execution tokens, lock contention, unmanaged destinations, stale exports, modified managed policy, missing/modified/symlinked guidance, missing/modified guidance declarations, and retained prepared state.
A child test reaches the production artifact/manifest boundary and is killed by its parent. The artifact, prepared record, and lock remain; neither final pointer exists.
A real directory-permission failure leaves prepared state without a complete selection. A failed retry preserves that record.
The uncertainty control injects the producer's unknown-durability result at the executor's post-artifact boundary. It is a controlled shell failure, not a real kernel synchronization fault.
The existing direct producer conformance test separately covers failed parent synchronization and `CommittedDurabilityUnknown`.

## Commands and exits

Cargo commands used:

```text
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c
```

The host PATH lost `env` and `nix` during this pass. Later commands prepend existing Nix-store coreutils and Nix binaries.
One test command therefore exited before Cargo started; it is not test evidence.

| Command after the Cargo prefix, unless stated otherwise | Observed result |
|---|---|
| Initial CLI bootstrap baseline | Passed: 7 tests |
| Initial `cargo test -p cairn-core consumer_bootstrap` | Passed: 11 tests |
| Final `cargo test -p cairn-core consumer_bootstrap` | Exit 0; 12 tests passed |
| `cargo test -p cairn-cli --bin cairn bootstrap` | Exit 0; six tests, including the child-control entry point |
| Focused preview, project-policy, publication-contract, consumer-controls, generated-workflows targets | Exit 0; 17 tests passed |
| Final `cargo test -p cairn-cli --test bootstrap_preview -- --nocapture` | Exit 0; two tests passed, with retained consumer receipts |
| Final `cargo clippy --workspace --all-targets -- -D warnings` | Exit 0; no lint allowances added |
| Final `cargo fmt --all -- --check` | Exit 0 |
| `./target/debug/cairn policy export --check` outside the dev shell | Exit 1: `Not a directory (os error 20)` |
| Same policy export command inside the dev shell | Exit 0 |
| `./target/debug/cairn validate --root .` | Exit 0; valid payload |
| `./target/debug/cairn traceability coverage --root . --json` | Exit 0; valid payload, missing and dangling arrays empty |
| `git diff --check` | Exit 0 |
| First `cargo test --workspace --all-targets --all-features` | Exit 101: new fixture expected `stale_source`, but existing vocabulary is `stale-source`; corrected |
| Second full workspace/all-target/all-feature replay | Exit 101 at `evidence_registry/failure_cases.rs:189`, expecting `evidence_import.undeclared_bundle_file` |
| Quiescent `cargo test -p cairn-cli --test evidence_registry` | Exit 0; one test passed. This default-feature replay is not full-suite acceptance |

The missing guidance traceability entry is resolved by implementation markers on real doctor/planner composition, not a placeholder.

## Nix observations

Exact `nix flake check "path:$PWD" --option min-free 0 -L` attempts exited 102 when source changed during evaluation.
One subsequent attempt evaluated all 31 checks and began remote builds, but became obsolete after a fixture correction and was explicitly killed.
The final frozen-source retry used a 180-second operator timeout and exited 124. It did not finish evaluation/build acceptance.

The base exposes the check name `slotcar`, whose evaluated derivation is `octet-consumer-check`.
Other observed names include `bootstrap-source-pin`, `artifact-auth-source-pin`, `nickel-cohort-exact`, `nickel-toolchain-fixtures`, `runtime-generated-policy-only`, `policy-fresh`, `authenticated-policy-profiles-fresh`, `policy-stale-detects-drift`, `packaged-policy-export-runtime`, `nickel-export-cutover`, `preserves-corpus-fresh`, `preserves-corpus-stale-detects-drift`, `gate-contract`, `gate-contract-wasm`, `gate-contract-nickel`, `rustfmt`, `rust-tests`, `radicle-cob-format-compatibility`, `cairn-validate`, `tracey-coverage`, `trace-index-live-proof`, `rename-classification`, `self-dogfood`, and `clippy`.
No passing Slotcar/Octet verdict, strict acceptance, or full Nix result is claimed.

## Persistent logs

Logs are in `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`.
All names below start with `finish-2746-bootstrap-`:

- `baseline.log`, `core-baseline.log`
- `final-core.log`, `final-unit.log`, `final-focused.log`
- `consumer-receipts.log`, `declaration-final.log` (latest exact consumer receipt lines)
- `declaration-clippy.log`, `format-final.log`, `policy-final.log`
- `validate-final.json`, `traceability-final.json`
- `workspace.log`, `workspace-final.log`, `evidence-isolated.log`
- `flake.log`, `flake-quiescent.log`, `flake-final.log`, `flake-frozen.log`

Earlier failures remain preserved. No older log was overwritten.

## Exact remaining work

1. Review whether diagnostic-only manual recovery covers the intended shell acceptance. If resumable recovery is required, connect saved-plan reconstruction and fresh artifact/synchronization observations to the existing pure recovery transitions. Current shell recovery deliberately performs no recovery mutation.
2. Strengthen the stale-plan controls with a previously issued valid token after source/root drift and a controlled lock-time substitution. Current controls include invalid-token denial, source drift, no-follow rejection, and lock contention, not every race schedule.
3. Run the complete workspace/all-target/all-feature suite on frozen final source. Investigate the exact evidence-registry assertion if it repeats. Do not replace that acceptance with the passing isolated default-feature replay.
4. Run the full Nix check without the short handoff timeout, preserving the base's actual check names and warning/error levels.
5. Review the recorded acceptance surfaces before changing task checkboxes. Then run the package gates and coordinator-owned completion workflow.

The fresh consumer path and changed-policy upgrade now run successfully. Full change completion and release eligibility remain unclaimed.
