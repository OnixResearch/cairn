# Readiness finish pass 2746

Status: tested increment, not change completion.
Product commit: `c5facecac0368640f722c18031a56f84ef35331e` — Share readiness budgets to preserve bounded partial observations.
Owner: Cairn lifecycle-core and CLI maintainers.
The coordinator retains sync, archive, integration, and publication authority.

## Delivered capability

The shell now shares one explicit budget across all readiness readers and both observation passes.
The request admits at most 64 MiB of file bytes and 65,536 directory entries. Each reader retains its smaller local limits.
The budget covers policy, lifecycle sources, workflow artifacts, reviews, probes, completion inputs, dependencies, external summaries, and mutation observations.
Failed reads charge the bytes already read. The budget has no global or thread-local state.

Selected-change loading no longer depends on unrelated lifecycle sources.
An unrelated source failure preserves the selected artifact and gate decisions, while the dependency scan remains unavailable.
Stage-local loading preserves earlier gates when a later artifact is unreadable. Each retained gate binds its admitted source bytes.
This path requires an available profile or the existing metadata-selected compatibility profile. It never substitutes compatibility for an unknown or explicit profile.
Completion remains independent when policy loading succeeds. Available cross-repository decisions survive profile or dependency-scan failure.
Mutation-gate admission errors affect only stages whose policy requires that gate. If neither stage requires it, the shell skips that observation.

Declaration-wide probe failures now identify the exact declaration document. Their codes and messages still match the original gate issues.
Unsupported remediation remains unresolved, with no invented producer command.
The completion fixture now compares the original validator with CLI and MCP for passing, stale-subject, and missing-result inputs.
The symlink control also compares both transports and retains the task-file bytes.

The immediate outcome is bounded inspection with more precise partial observations.
The durable contribution is an explicit shared shell budget and reusable stage-local source admission.
Direct controls accept the exact measured two-pass byte budget and reject a budget one byte smaller.
The measured fixture includes policy, lifecycle, completion, explicit external evidence, and saved probe declarations.
Additional controls prove shared reader limits, directory-entry accounting, independent earlier gates, and gate-policy isolation.
These observations do not establish an atomic filesystem snapshot or protection against arbitrary external mutation.

No dependency manifest, lockfile, producer pin, vendored source, generated policy, gate scope, or lint allowance changed.
No memo, agent spawn, operator sync, archive, push, or worktree removal ran. Planning gates did not run as completion evidence.

## Commands and observed verdicts

All Cargo commands used this prefix in the readiness worktree:

```text
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c
```

The private Pueue group is `finish-2746-readiness`.
Logs reside under `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`.
Every new log has the prefix `finish-2746-readiness-`. Older logs remain unchanged.
These paths locate operator evidence and are not product dependencies.

| Command after the prefix | Observed result | Log suffix |
|---|---|---|
| `cargo test -p cairn-core readiness` before edits | Exit 0, 35 matched tests passed | `baseline-core.log` |
| `cargo test -p cairn-core` on final product source | Exit 0, 570 unit and 12 documentation tests passed | `core-final.log` |
| `cargo test -p cairn-cli --bin cairn --test readiness --test readiness_completion --test lifecycle --test cross_repo_dependencies --test mcp_cli --test machine_cli` | Exit 0, 114 unit, 4 readiness, 1 completion, 57 lifecycle, 5 dependency, 13 MCP, and 13 machine tests passed | `shell-final.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | Exit 0, no allowances added | `clippy-final.log` |
| `cargo fmt --all -- --check` | Exit 0 | `fmt-final.log` |
| `cargo test --workspace --all-targets --all-features` | Exit 0, quiescent worktree | `workspace-quiescent.log` |

The quiescent workspace run covered product commit `c5facec` from 01:50:39 to 01:53:09 on 2026-09-06, offset `-04:00`.
Git status was empty before and after that run. This agent made no worktree edits during the run.
The final all-feature binary ran 115 unit tests per binary. The focused default-feature binary ran 114.
The previously blocked `evidence_export_materializes_complete_portable_chain` test passed.

Additional current-source checks:

- `./target/debug/cairn validate --root .`: exit 0, `valid: true`, no findings or issues. Log: `validate-final.json`.
- `./target/debug/cairn traceability coverage --root . --json`: exit 0, `valid: true`, `verdict: pass`, 2,295 requirements and references, no missing or dangling markers. Log: `traceability-final.json`.
- `./target/debug/cairn policy export --check` through the development shell: exit 0, `policy fresh`. Log: `policy-devshell.log`.
- `git diff --check` passed before the product commit.

### Host command-path failures

Two intermediate commands did not reach Cargo: `sh: line 1: env: command not found` (exit 127).
Their logs are `isolation-first.log` and `focused.log`. They are not failed Rust fixtures.
A direct host policy check returned exit 1 with `error: Not a directory (os error 20)` in `policy-final.log`.
The development-shell replay passed without source changes.

Subsequent commands prepended these existing tool directories to PATH:

```text
/nix/store/3qgy8q2j64v2m9jy3a5jmssacbblhd4r-coreutils-9.11/bin
/nix/store/352895cylim71n9r04wxphkp350m7cd5-nix-2.36.0/bin
```

This pass did not repair or change the host profile. The tool paths are local execution evidence, not runtime dependencies.

### Full Nix battery

Command:

```text
nix flake check "path:$PWD" --option min-free 0 -L
```

Log: `flake-final.log`. Pueue task: `2948`.
The command completed with exit 0. The full Nix battery passed on product commit `c5facec`.
The log reports `running 26 flake checks` and retains the base's `checks.x86_64-linux.slotcar` name.
That attribute built `octet-consumer-check` and emitted warnings. This warning-level result is not strict Octet acceptance.
The self-dogfood check printed its expected legacy-layout rejection and still passed. It was not a failed product test.

The logged check attributes are:

```text
slotcar
artifact-auth-source-pin
nickel-cohort-exact
nickel-toolchain-fixtures
runtime-generated-policy-only
policy-fresh
readiness-policy-fresh
authenticated-policy-profiles-fresh
policy-stale-detects-drift
packaged-policy-export-runtime
nickel-export-cutover
preserves-corpus-fresh
preserves-corpus-stale-detects-drift
gate-contract
gate-contract-wasm
gate-contract-nickel
rustfmt
rust-tests
radicle-cob-format-compatibility
cairn-validate
tracey-coverage
trace-index-live-proof
rename-classification
self-dogfood
clippy
```

## Task state and exact remaining gaps

All eight previously open tasks remain open. `tasks.md` is unchanged.
No task closed from a planning gate, a prior increment, or this report.

| Task | Remaining acceptance gap and next action |
|---|---|
| `core` | Complete the independent-stage acceptance matrix for failed explicit profiles and policy admission. Preserve independently computable decisions without substituting an unadmitted profile or policy. |
| `actions` | Bind exact producer contracts and predecessor change identities in prerequisites. Complete supported preview argv, including selected input arguments. Add reviewed remediation for remaining unsupported causes. Declaration-wide failures have exact targets but remain unresolved without a reviewed rule. |
| `observation` | Policy admission still precedes other loading. Explicit-profile admission can still discard earlier gate decisions. Selected-source admission still combines task text and cross-repository metadata, so a task read failure can discard independently readable dependency facts. Split those input boundaries and compare each surviving decision. |
| `transports` | Extend CLI/MCP parity to the complete independent-stage failure matrix and completed action contracts. Existing shared serialization and normal-fixture parity do not close these missing acceptance cases. |
| `negative` | Add the explicit-profile, policy-failure, and selected-source/dependency isolation controls, with exact identities and zero-effect assertions through both transports. Existing probe, review, task-order, and completion controls remain passing. |
| `consumer` | Finish the maintained comparison matrix for all original validators and both transports, including mutation-stage observations and the remaining failures. The completion matrix increment is not the whole consumer task. |
| `docs` | Complete the bounded diagnostic-call comparison for known repairs. The existing three-gate/one-report fixture comparison is not a complete repair study or a latency claim. |
| `verification` | The full battery passed, including the quiescent workspace run. Implementation acceptance and strict Octet acceptance remain incomplete. Repeat the battery after the remaining product work. The coordinator owns later reviewed sync/archive plans. |

Drain presentation remains deliberately outside this increment, as the design permits before compatibility acceptance.
Passing unchanged legacy drain tests does not establish compatibility for replacing their presentation with the shared model.
Cairn maintainers can revisit adoption after the complete unified compatibility matrix passes.

The final evidence commit records this handoff only. Successful process exits, marker coverage, and this note do not establish change completion or release eligibility.
