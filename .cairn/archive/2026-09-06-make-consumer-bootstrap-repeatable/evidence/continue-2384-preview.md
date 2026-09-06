# Bootstrap preview increment

## Outcome and durable capability

This increment adds `setup plan --root PATH --source-root PATH --bundle RELATIVE_PATH`.
It admits a read-only artifact preview from explicit bounded sources. It does not install files.
Cairn maintainers own the source-bundle contract, preview adapter, and consumer tests.
The maintained consumer replay runs from an empty root with isolated home and data paths.

The preview uses the published Nickel Export core to rebuild and compare a saved manifest.
Cap Root admits relative locators. Capability-relative readers reject symlinks, non-files, excessive files, and ambiguous absence.
The source map binds every read. Repeated reads of one locator must retain the same identity.
The running CLI, policy, export manifest, evaluator descriptor, source, workflow bytes, destinations, and desired bytes are bound independently.
The pure core checks source mappings, reserved destinations, cohort compatibility, selected profile, and ownership actions.

The result reports `installation_receipt: false` and `execution_supported: false`.
Its artifact plan hash is not an apply token or root-authority binding.
The producer schema and non-claim remain unchanged. Saved observations are not proof of evaluator execution.
Workflow bytes are identity-bound only; generated-guidance compatibility remains unchecked.
No new dependency or producer pin was added in this increment.

## Compatibility repair

The first workspace replay exposed an explicit-policy path regression in `agent_guidance`.
The path contained `crates/cairn-cli/../../cairn-policy/...` and failed with `bootstrap.path.unsafe_root`.
The repair permits parent navigation for explicitly selected external policies through a bounded stack of opened directory capabilities.
Every preceding component is opened without following links. Lexical cancellation cannot hide a symlink.
Project-relative locators and bootstrap roots still reject parent escapes.
Positive parent-navigation and negative symlink-cancellation tests pass. The eight agent-guidance tests pass after repair.

## Commands and observed verdicts

Cargo commands used this prefix throughout:

```text
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c
```

| Command | Observed verdict |
|---|---|
| Baseline `cargo test -p cairn-core consumer_bootstrap` | 10 passed |
| Baseline `cargo test -p cairn-cli --test bootstrap_project_policy --test bootstrap_publication_contract --test consumer_bootstrap_controls` | 6 passed |
| Final `cargo test -p cairn-cli --test bootstrap_preview --test bootstrap_project_policy --test bootstrap_publication_contract --test consumer_bootstrap_controls --test agent_guidance` | 15 passed |
| Final `cargo test -p cairn-cli --bin cairn bootstrap` | 2 passed |
| Final `cargo test -p cairn-core consumer_bootstrap` | 11 passed |
| `cargo test -p cairn-cli --test determinism` with no concurrent product edits | 7 passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo fmt --all --check` | Passed |
| `git diff --check` | Passed |

Nickel acceptance and rejection cases cover the new source bundle through the consumer control test.
The preview test checks payload validity, exact identities, byte-for-byte repeatability, no mutation, and explicit execution denial.
Negative cases reject tool drift, unknown profiles, stale source bytes, unowned destinations, prepared state, symlinks, and malformed options.
Source tests reject changed repeated reads and exhausted read budgets.

Two `cargo test --workspace` attempts did not pass completely:

1. The first stopped at the explicit-policy regression described above.
2. The second passed that regression but stopped at `determinism_replay_oracle_is_stable_and_non_mutating`, where repeated receipt hashes differed.

Product files changed during the second replay. A later quiescent replay passed all seven determinism tests.
This targeted result does not establish that the rest of the workspace suite passed. A quiescent full workspace rerun remains required.

Logs are in the coordinator checkout's `.pi/cairn-rest-1295/` directory.
Every new log uses the prefix `continue-2384-make-consumer-bootstrap-repeatable-`.
Key suffixes are `final-tests.log`, `final-replay-clippy.log`, `final-replay-format.log`, `determinism-quiescent.log`, `workspace-tests.log`, and `workspace-repaired.log`.

## Exact remaining gaps

- No setup apply, upgrade, installation check, or recovery command is implemented. The preview explicitly rejects execution.
- No installation lock, source/root grant recheck, generation staging, or complete-selection publication adapter is connected.
- Durable File Publication remains covered by direct consumer conformance tests, not by an installation transaction.
- No bootstrap shell receipt covers interrupted installation, unknown durability, failed recovery, or preservation of an old complete generation.
- Generated-workflow planning, installation, and freshness are not composed with bootstrap or doctor.
- CLI validation, doctor, and gates use project policy selection. Other command families and MCP still use their existing loaders.
- No clean-root setup-to-validation or upgrade-to-validation installation replay exists.
- No final full workspace pass, Nix flake check, lifecycle gate pass, or full Octet acceptance is claimed.

All incomplete shell, guidance, negative-shell, consumer, documentation, and verification tasks remain unchecked.
The preview guide documents current behavior only. It does not complete the full setup documentation task.
Sync, archive, push, worktree removal, and Octet repair remain coordinator-owned.
