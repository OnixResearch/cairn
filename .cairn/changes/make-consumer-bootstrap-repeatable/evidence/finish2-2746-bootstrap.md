# Bootstrap second-pass acceptance review

## Outcome and source identity

Cairn maintainers own the bootstrap shell, contracts, consumer fixtures, and operator guide.
This pass closes five implementation tasks. All requested command checks finished.
The verification handoff remains open only for coordinator-owned review of active-change sync/archive plans.
No active change was synchronized or archived. No branch was pushed.

Tested implementation commits:

- `6c728fb3240d75a5256a8a52faecd78a8135e8c4` — Reject root substitution before publishing a reviewed bootstrap plan.
- `6f18c986ba8a0bc2dcd9cd20f7e4c7154129e6ef` — Bind the public gate schema and receipt fixtures to policy selection.

The final Cargo battery ran on unchanged production, contract, fixture, and documentation bytes at `6f18c986ba8a0bc2dcd9cd20f7e4c7154129e6ef`.
Subsequent changes to this report and task checkboxes record evidence only.
No dependency, producer pin, lockfile, notice, lint allowance, or gate severity changed.

## Corrected behavior

The controlled post-lock callback exercises the production apply path. It is not a CLI option or environment-controlled bypass.
The callback substitutes a source bundle, destination bytes, destination symlink, or root after the lock exists and before the fresh plan.
The unchanged control publishes a complete installation and releases its lock.
Each substitution blocks publication and retains the lock. Neither complete metadata nor selection appears.
Unowned destination bytes and symlinks remain unchanged.

The root control exposed a real gap: a directory capability could outlive substitution of its selected pathname.
`bootstrap_fs::recheck_root` now reopens that path without following links and compares its device and inode with the original capability.
The shell checks this binding before and after plan preparation, including the preparation under lock.
The substituted root stays empty. The displaced root retains its lock without a prepared or complete installation.
These observations do not prove exclusion of unrelated writers with equal filesystem authority.

The CLI replay also reuses an actually issued valid token after three changes:

- An admissible source bundle has different serialized bytes. A fresh preview passes, but the old token yields `bootstrap.setup.stale_plan`.
- An export source changes. The old token yields `bootstrap.export.stale` before publication.
- The selected root is replaced with a new empty directory. The old token yields `bootstrap.setup.stale_plan`.

Restoring the original source and root restores the original plan hash. That exact token then publishes successfully.

## Recovery decision

The accepted publication scenario requires the system to block completion and preserve the last valid state **or an explicit recovery state**.
The design permits either unchanged-plan resume or a request for manual repair.
The initial shell chooses manual repair. Automatic resume is not necessary for this acceptance surface.

The existing real child-kill control reaches the production artifact/manifest boundary.
It retains artifact bytes, prepared state, and a lock without either final pointer.
The permission-failure control retains prepared bytes. A failed retry cannot erase that record.
The controlled durability-unknown result blocks selection after an artifact becomes visible.
The direct publication-contract test separately covers failed parent synchronization and unknown durability.
The shell uncertainty control is not a real injected kernel synchronization fault.

The expanded consumer replay calls recovery twice after an installed upgrade has explicit interrupted state.
Each result asserts `valid: false`, `blocked: true`, `mutated: false`, and `installation_receipt: false`.
The category is `bootstrap.recovery.manual_repair`. Prepared bytes, selection bytes, and complete metadata remain unchanged.
Normal validation and another setup plan remain blocked.
This proves the explicit-state alternative. It does not prove successful manual repair, rollback, saved-plan reconstruction, or synchronization retry.

## Task-by-task acceptance review

Requirement names below refer to the six requirements in `specs/consumer-bootstrap/spec.md`.

| Task | Whole acceptance surface and controls | State |
|---|---|---|
| baseline | Existing init, guidance, and migration tests; missing-policy failure and explicit-policy success in `consumer_bootstrap_controls`; first-pass evidence | Complete, unchanged |
| contracts | Nickel/Rust accepted cohort, declaration, ownership, upgrade, and recovery fixtures; rejected bounds, escapes, and malformed identities; contract/core evidence | Complete, unchanged |
| core | Direct deterministic admission, ownership, compatibility, plan identity, generation, and recovery tests; final core suite passes | Complete, unchanged |
| reuse | Exact published Cap Root, Nickel Export, and Durable File Publication identities; source-pin negative controls and direct consumer conformance; dependency-admission evidence | Complete, unchanged |
| shell | Publication and boundary scenarios: read-only preview, exact token, locked fresh preparation, root rebinding, capability-relative publication, synchronized read-back, retained interrupted state, manual-only recovery controls | Closed |
| guidance | Guidance and policy scenarios: declared project-local composition, selected-policy validation, current manifest findings, changed-policy upgrade, missing/modified declaration controls, no ambient HOME discovery | Closed |
| negative-shell | Publication, guidance, and boundary scenarios: valid-token drift, post-lock substitutions, modified/missing/unmanaged/symlinked guidance, permission failure, killed publisher, durability uncertainty, and failed retry/repeated recovery | Closed |
| consumer | Conformance scenarios: `env_clear`, isolated HOME, empty root, exact built CLI identity, setup/check/doctor/validation, declared changed-policy upgrade, retained old generation and consumer-owned Nickel file; exact JSON receipts retained | Closed |
| docs | Policy, guidance, and boundary scenarios: checked command procedure, bundle example and executable fixture, precedence, explicit upgrades, manual recovery, migration limits, bounds, and project-local guidance in `docs/consumer-bootstrap-preview.md` | Closed |
| verification | Focused and complete Cargo battery, strict Clippy, policy freshness, lifecycle validation, traceability, final Nix verdict, and reviewed coordinator handoff | All command checks pass; open only for coordinator-owned sync/archive plan review |

The new unmanaged-guidance control asserts the manual-repair diagnostic and unchanged unowned bytes.
Modified guidance asserts `bootstrap.state.modified_managed` and unchanged modified bytes.
Missing and symlinked guidance assert `bootstrap.state.missing_or_unsafe_managed` and preservation of that state.
These assertions supplement the existing generated-workflow classification tests. Process status alone is not acceptance evidence.

## Full-suite failures and repairs

The early full-feature baseline passed `evidence_registry`, then failed the validation golden.
Bootstrap had added `policy_selection` and a receipt hash without updating that golden.
The refreshed golden retains the full exact-object assertion.

The next frozen full-feature run again passed `evidence_registry`.
It failed two public gate-schema assertions because the Nickel schema did not declare `policy_selection`.
The first complete Nix build reproduced those same two failures in `checks.x86_64-linux.rust-tests`.
The Rust test builder returned 101. Its enclosing builder reported 1. Pueue history disappeared before the outer CLI exit was captured.
No passing Nix result is inferred from that run.

The repair adds the actual field to `cairn-policy/default.ncl` and regenerates policy JSON through `cairn policy export`.
It refreshes the policy-bound agent fixtures, fixture-only lifecycle preview goldens, and both exact Preserves hash expectations.
Fixture preview generation uses no `--execute` and does not operate on this active change.
The updated canonical policy hash is `c8276321e0e82c524e09ad9357fa48a9d38ac8590263ee6f79c51a328dc6037d`.
The exact generated-policy byte hash is `7a67802377005c18ebded42899e7f564f5d87087b6c14ca70464b35886f340a9`.
The legacy agent fixture remains non-authoritative. Refreshing its identities does not turn it into completion evidence.

The recorded `evidence_registry/failure_cases.rs:189` failure did not recur in any of the three full-feature runs that reached it.
The final frozen run passed that target in 37.38 seconds.
No assertion was removed, skipped, or relaxed. This pass does not establish the cause of the older failure or classify it as order-dependent.

## Environment failures and reproduction order

A later focused replay failed while creating replay scratch files. A direct replay then exposed `Disk quota exceeded` before Nix could start the CLI.
`df` showed `/tmp` at 100% byte and inode use. `/var/tmp` had free bytes and inodes.
The passing final battery uses `TMPDIR=/var/tmp`. No other session's temporary files were removed.

An overlapping Nix retry exited 1 before evaluation because a Cargo object disappeared while Nix copied `target/debug/deps`.
A `path:$PWD` flake snapshot includes the ignored `target/` tree in this worktree.
The final Nix run therefore starts only after the Cargo battery finishes. It has no command timeout.

Cargo reproduction prefix:

```text
export TMPDIR=/var/tmp
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c
```

The command records also prepend the explicit host PATH from the shared assignment.
The inner development shell exports `TMPDIR=/var/tmp` for the test processes.

## Observed final Cargo and lifecycle verdicts

| Command after the Cargo prefix, unless stated otherwise | Exit and observed result |
|---|---|
| `cargo test -p cairn-core` | 0; 573 unit tests and 12 documentation tests passed |
| CLI bootstrap unit filter | 0; seven tests, including the controlled locked recheck and child-control entry point |
| Focused CLI targets listed below | 0; all 127 tests passed, with semantic consumer receipts |
| `cargo test --workspace --all-targets --all-features` | 0; complete run, no filtered acceptance replacement |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0; no allowances added |
| `cargo fmt --all -- --check` | 0 |
| `./target/debug/cairn policy export --check` | 0; policy fresh |
| `./target/debug/cairn validate --root .` | 0; valid payload |
| `./target/debug/cairn traceability coverage --root . --json` | 0; `valid: true`, missing and dangling arrays empty |
| Direct CLI proposal/design/tasks gates | 0 each; planning conformance only |
| `nix flake check "path:$PWD" --option min-free 0 -L` | 0; completed without a command timeout, with warning-level Octet findings |

After the evidence and checkbox update, direct validation, traceability, the tasks gate, and `git diff --check` each returned 0.
The updated traceability payload remains valid with empty missing and dangling arrays.

Focused targets: `bootstrap_preview`, `bootstrap_project_policy`, `bootstrap_publication_contract`, `consumer_bootstrap_controls`, `generated_agent_workflows`, `lifecycle`, `lifecycle_mutations`, `release_receipts`, and `agent_completion`.

## Nix check names and scope

The exact final command is `nix flake check "path:$PWD" --option min-free 0 -L`, with `TMPDIR=/var/tmp`.
The final run is Pueue task `3169`, after the successful Cargo battery task `3166`.
Its source snapshot is `/nix/store/cmkjna62brv1m77gvlbcrf18v9cwpqid-source`.
It completed at 02:57:05 on 2026-09-06 after 9 minutes 18 seconds, with outer exit 0.
The independent `flake-quiescent.exit` record contains `flake_exit=0`.
This run evaluated all 25 attributes below and printed `running 24 flake checks` after reuse of cached results.

The flake exposes these 25 check attributes. Cached results affect the runner's printed build count, not this attribute list.

```text
artifact-auth-source-pin
authenticated-policy-profiles-fresh
bootstrap-source-pin
cairn-validate
clippy
gate-contract
gate-contract-nickel
gate-contract-wasm
nickel-cohort-exact
nickel-export-cutover
nickel-toolchain-fixtures
packaged-policy-export-runtime
policy-fresh
policy-stale-detects-drift
preserves-corpus-fresh
preserves-corpus-stale-detects-drift
radicle-cob-format-compatibility
rename-classification
runtime-generated-policy-only
rust-tests
rustfmt
self-dogfood
slotcar
trace-index-live-proof
tracey-coverage
```

The base's `slotcar` attribute evaluates to an `octet-consumer-check` derivation.
Both completed runs emitted warning-level Octet findings and finished that checker.
The final log includes source-file length warnings, among other findings.
This is not strict Octet acceptance or a zero-finding claim. Strict Clippy is recorded separately.
The flake and producer pins remain unchanged.

## Retained records and coordinator handoff

All command logs and exact consumer JSON receipt lines are retained under:
`/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`.
Names below have the prefix `finish2-2746-bootstrap-`:

- `workspace-baseline.log`, `workspace-final.log`: preserved product failures and successful evidence-registry controls.
- `unit.log`, `focused.log`, `schema-tests.log`, `focused-final.log`: focused controls and preserved intermediate failures.
- `schema-export.log`, `schema-fixtures.log`: policy and fixture regeneration observations.
- `core-final.log`, `focused-quiescent.log`, `workspace-quiescent.log`: final passing core, consumer, and full-feature evidence.
- `quiescent-exits.log`: exact final Cargo and diagnostic exits.
- `clippy-quiescent.log`, `fmt-quiescent.log`, `policy-quiescent.log`.
- `validate-quiescent.json`, `traceability-quiescent.json`, `gate-quiescent-{proposal,design,tasks}.json`.
- `validate-handoff.json`, `traceability-handoff.json`, `tasks-handoff.json`: checks after the evidence/checkbox update.
- `flake.log`: first completed Nix build, including warning-level findings and the two schema failures.
- `flake-final.log`, `flake-final.exit`: preserved disappearing-object failure, exit 1.
- `flake-quiescent.log`, `flake-quiescent.exit`: final serialized Nix run and its exact outer exit.
- `check-names.json`: evaluated check attribute list.

The coordinator owns review of this evidence and fresh active-change sync/archive plans.
Those plans must retain the six added requirements and the non-claims above.
No active-change plan, archive execution, integration, or release eligibility is claimed here.
The worker prohibition on active-change sync/archive commands leaves this named handoff obligation with the coordinator.
The coordinator must obtain and review fresh plans before closing the final task. No implementation or full-test gap remains in this report's declared scope.
