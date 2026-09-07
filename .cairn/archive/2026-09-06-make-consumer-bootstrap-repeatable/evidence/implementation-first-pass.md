# Bootstrap first implementation increment

## Scope and owner

Cairn maintainers own the new consumer-bootstrap core and its fixtures.
The immediate result is executable admission, ownership, plan-identity, and recovery logic with paired tests.
The durable contribution is a pure boundary for the later installation shell.
This increment does not deliver installed-consumer setup.

The baseline revision is `95dbaa883054bf6ed87612d57ed26b9ab55919dc`.
The worktree retains its existing Slotcar Nix names and checks.
Those names predate canonical Octet integration. This increment does not claim a passing canonical Octet gate.

## Product files

- `crates/cairn-core/src/verified/consumer_bootstrap.rs` owns decisions over supplied facts.
- `crates/cairn-core/src/verified/consumer_bootstrap/tests.rs` covers admission and rejection.
- `contracts/consumer-bootstrap.ncl` owns typed input shapes and named bounds.
- `fixtures/consumer-bootstrap/` contains accepted and rejected Nickel inputs.
- `crates/cairn-cli/tests/consumer_bootstrap_controls.rs` retains installed-consumer controls and Nickel/Rust conformance.

The core distinguishes missing policy selections from invalid selections.
An invalid explicit selection cannot fall back to project or default policy.
Managed ownership requires prior ownership facts, not only matching desired bytes.
Plans bind the cohort, prior complete manifest, locators, payload identities, sizes, and observed destination facts.
Recovery distinguishes resumable bytes, unknown synchronization, complete prepared artifacts, and manual repair.
The complete-manifest decision requests publication. It does not report completed installation.

## Dependency inspection and exact blockers

The existing Cargo and Nix files both pin Nickel Export to `257fafc1c746f1faf156207043a4c826bfb16d49`.
`crates/cairn-cli/src/nickel_export_boundary.rs` already uses its export observation and receipt contracts.
No new Nickel Export implementation or source copy enters this increment.

The exact Cap Root source at `8991e2572578c16e590c2f1c0915db6c46c5d9cb` was inspected through the existing Cargo Git object cache.
Its `cap-root-core` API exposes `RootRelativePath::new`, `AdmissionMode::NonEmptyRelative`, and `RootRelativePath::new_leaf`.
Its published repository identity is `rad:z3cWBCK7VPbEmbL5MAvKJWKjCE745`.

The existing Cargo checkout for Durable File Publication resolves to `951c27f59003cea9bfdb40ed4d89653d50fada1f`.
Its published repository identity is `rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM`.
Its shell exposes `publish_one_file`, `PublicationIo`, `StageNameSource`, and `LinuxDirectory::from_open_directory`.
It explicitly retains `CommittedDurabilityUnknown` after a visible rename with failed parent synchronization.
It does not supply locks, installation transactions, or recovery. Cairn must own those operations.

Neither Cap Root nor Durable File Publication is an admitted Cairn Cargo or Nix dependency in this worktree.
The worker assignment prohibits changes to pins and lockfiles.
Consequently, their concrete adapters cannot be added under the current assignment.
This is a dependency-admission blocker, not evidence that either producer API is absent.
Cached source inspection is not fresh published-source acquisition or consumer conformance evidence.

The inspected publication package uses `MIT OR Apache-2.0`.
No producer source was copied, modified, or relicensed. Existing notices remain unchanged.
The OnixResearch owner grant permits reviewed reuse but does not establish contract conformance.

## Reproduction and observations

All Cargo commands used this prefix from the assigned worktree:

```text
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop --option min-free 0 -c
```

| Command after the prefix | Observation |
|---|---|
| `cargo test -p cairn-cli --bin cairn native` | Build passed, but the filter selected zero tests. This is not baseline behavior evidence. |
| `cargo test -p cairn-cli --test generated_agent_workflows --test lifecycle` | Before implementation, 9 guidance tests and 56 lifecycle tests passed. These include init, policy errors, upgrades, symlinks, stale plans, and migration controls. |
| `cargo test -p cairn-core consumer_bootstrap` | All 9 new pure-core tests passed. |
| `cargo test -p cairn-cli --test consumer_bootstrap_controls` | Both controls passed, including Nickel/Rust conformance and the missing-policy/explicit-policy pair. |
| `cargo test -p cairn-core` | All 570 unit tests and 12 documentation tests passed. |
| `cargo clippy -p cairn-core --all-targets -- -D warnings` | Passed after the narrow test correction. No lint exemptions were added. |
| `cargo fmt --all` | Completed. No unrelated source files changed. |

`git diff --check` passed. The new core source has no matches for the documented filesystem, environment, process, clock, or output smoke-check patterns.

The consumer control runs the built CLI outside its source directory with isolated home and data paths.
Low-level init creates native directories without a policy.
Validation then fails without a policy and succeeds with an explicit checked policy.
An invalid explicit policy fails even when a valid default policy is available.
This test preserves baseline semantics. It does not exercise an installed project declaration.

The Nickel test exports the accepted fixture and decodes it into the Rust cohort contract.
The rejected fixture exceeds the named artifact-count bound and produces a contract error.
Early contract-authoring mistakes were corrected before this passing replay.
The first strict Clippy run found one unnecessary test clone. The test now uses `std::slice::from_ref` without a lint exemption.

Persistent command logs use the `bootstrap-` prefix in the coordinator-requested directory:
`/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`.

## Remaining work

1. Authorize a coordinated Cargo/Nix admission of the exact reviewed Cap Root and Durable File Publication revisions.
2. Acquire the published sources and retain adapter conformance tests and required notices.
3. Implement checked project declarations and root-relative policy loading through those admitted boundaries.
4. Add preview-first setup commands, locks, rechecks, publication, saved recovery state, and explicit recovery.
5. Compose the existing generated-workflow installer and doctor findings.
6. Reproduce real setup and upgrade execution from fresh consumer roots.
7. Run shell fault tests, policy freshness, strict CLI checks, lifecycle gates, and full Nix checks.

No setup command, project policy resolver, publication adapter, or doctor integration is claimed in this increment.
No task is completed by a planning receipt. Sync, archive, and integration remain with the coordinator.
