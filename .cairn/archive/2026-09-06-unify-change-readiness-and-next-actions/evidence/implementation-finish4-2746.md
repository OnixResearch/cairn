# Readiness fourth pass 2746

Status: implementation evidence in progress.
Product base at start: `23a12aa`.
Private Pueue group: `finish4-2746-readiness`.
Persistent command logs: `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/finish4-2746-readiness-*.log`.

## Completed surfaces

- `core`: direct readiness tests cover schema-derived gate input selection, acceptance admission, semantic-review admission, outcome-machine admission, dependency closure, custom stages, and customized review topology. Stage-local profile reads compare against full-profile gate reports.
- `actions`: completion blockers now carry exact required-check command hashes into advisory prerequisites. Dependency-wait actions include the predecessor change identity. Declaration-wide saved-probe failures remain `unresolved`. Ready mutation stages use supported preview argv.
- `observation`, `negative`, and `transports`: the paired CLI/MCP isolation harness covers stage-local admission failures, safe-link and read-bound controls, policy/profile admission failures, completion-input isolation, cross-repository evidence isolation, mutation-gate isolation, and tree snapshots before and after both transports.
- `consumer`: maintained fixture rows compare original gate, completion, sync, archive, and cross-repository validators with readiness stages while preserving transport parity.
- `docs`: `docs/change-readiness.md` now records a bounded diagnostic-call study for basic gate/completion, stale/missing completion, malformed saved-probe declarations, and ready mutation previews. It makes no latency claim.

## Code changes

- `crates/cairn-core/src/verified/readiness.rs` adds prerequisite-bearing causes/blockers, producer-contract prerequisite extraction for completion diagnostics, predecessor-change prerequisite binding, and preview argv propagation from stage decisions.
- `crates/cairn-cli/src/readiness.rs` binds sync/archive preview argv to the selected root, policy, and repeated cross-repository evidence arguments without `--execute`.
- `crates/cairn-cli/tests/readiness_isolation.rs` refreshes fixture completion policy hashes when mutation-policy variants change, asserts supported preview argv, and retains tree snapshots.
- `crates/cairn-cli/tests/readiness_isolation/profile_admission.rs` customizes assurance review topology inside the existing stage-local admission matrix.
- `crates/cairn-core/src/verified/readiness/tests.rs` adds direct action tests for producer contracts, predecessor changes, and unresolved declaration-wide probe failures.

## Focused verification

Commands used the required Cargo environment with `TMPDIR=/var/tmp` and the assignment PATH plus `/run/current-system/sw/bin` because the printed Nix store path was absent on this host.

| Command | Exit | Log |
|---|---:|---|
| `cargo test -p cairn-core readiness` | 0 | `finish4-2746-readiness-focused.log` |
| `cargo test -p cairn-cli --bin cairn --test readiness --test readiness_completion --test readiness_isolation` | 0 | `finish4-2746-readiness-focused.log` |

## Task state

All ten tasks in `.cairn/changes/unify-change-readiness-and-next-actions/tasks.md` are marked complete after the implementation and focused controls above. Verification remains subject to the full frozen-source battery below.

## Full battery

The final battery ran after staging the frozen source, tasks, docs, and this evidence file so Nix flake checks saw the intended tree. Status file: `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/finish4-2746-readiness-battery.status`.

| Command | Exit | Log |
|---|---:|---|
| `cargo test --workspace --all-targets --all-features` | 0 | `finish4-2746-readiness-workspace.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | `finish4-2746-readiness-clippy.log` |
| `cargo fmt --all -- --check` | 0 | `finish4-2746-readiness-fmt.log` |
| `./target/debug/cairn policy export --check` | 0 | `finish4-2746-readiness-policy.log` |
| `./target/debug/cairn validate --root .` | 0 | `finish4-2746-readiness-validate.log` |
| `./target/debug/cairn traceability coverage --root . --json` | 0 | `finish4-2746-readiness-traceability.log` |
| `./target/debug/cairn sync unify-change-readiness-and-next-actions --root .` | 0 | `finish4-2746-readiness-sync-preview.log` |
| `./target/debug/cairn archive unify-change-readiness-and-next-actions --root .` | 0 | `finish4-2746-readiness-archive-preview.log` |
| `nix flake check "path:$PWD" --option min-free 0 -L` | 0 | `finish4-2746-readiness-flake.log` |

Validation reported a valid lifecycle with no findings. Traceability JSON reported no missing or dangling coverage. Sync and archive were preview-only; no `--execute`, sync, archive, push, agent spawn, or worktree removal ran.

The flake check retained the base warning-level `slotcar`/`octet-consumer-check` behavior from previous evidence. It completed all configured x86_64-linux checks and omitted incompatible aarch64-linux checks.

## Completion state

All change tasks are complete in this worktree. The coordinator still owns sync execution, archive execution, branch push, main integration, and worktree removal.
