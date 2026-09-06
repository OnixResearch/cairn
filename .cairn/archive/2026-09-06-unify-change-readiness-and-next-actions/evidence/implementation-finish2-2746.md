# Readiness second pass 2746

Status: tested increment, not change completion.
Product commit: `5d24e9a` — Isolate readiness inputs to preserve independent validator decisions.
Owner: Cairn core and CLI maintainers.
The coordinator retains sync, archive, integration, and publication authority.

## Delivered capability

Selected metadata now has a bounded reader independent of task text and delta sources.
It uses the original pure metadata parser and retains the exact text identity.
Malformed metadata does not become an empty, satisfied dependency set.
Readable metadata and external summaries preserve their cross-repository decision after a task-text error.

The pure core selects gate artifacts and their transitive prerequisites from the admitted schema.
The shell can load a stage-local profile context after full-profile admission fails.
The selected profile and schema remain unchanged.
Unknown profiles do not receive a compatibility substitute.
Stage-local contexts supply gate observations only, not artifact, completion, or mutation authority.
Surviving gates bind their admitted source and profile identities.

Policy admission now follows independent completion-input and metadata reads.
The original completion validator requires the current policy hash.
Thus, policy admission failure cannot produce a completion verdict, even with readable contract and result files.
The report retains admitted contract, result, subject, and metadata identities instead.
It also binds the selected request, including evidence paths that it did not evaluate.
No null, default, or contract-declared policy substitutes for the unavailable policy.

Mutation comparison exposed another mismatch.
Original sync and archive previews consume a mutation gate only when their base plan is unblocked and policy requires that gate.
Readiness now uses that same condition in the pure core.
Profile blockers remain separate and enter after the base-plan decision.
An unavailable gate no longer disables an independently computable mutation observation that does not consume it.

The immediate outcome is more precise partial readiness.
The durable capability is stage-local source admission with maintained validator comparisons.
The tests compare complete reports across CLI and MCP and retain fixture-tree contents before and after each transport.
Tree snapshots include directories, file bytes, and link destinations.
These controls do not establish an atomic filesystem snapshot or truth of external evidence.

No producer pin, dependency manifest, lockfile, generated policy, gate scope, or lint allowance changed.
No memo, agent spawn, operator sync, archive, push, or worktree removal ran.

## Maintained comparison matrix

New shell controls live in `crates/cairn-cli/tests/readiness_isolation.rs`.
Additional direct controls live in `crates/cairn-cli/src/readiness_tests.rs` and the core readiness tests.

| Control | Comparison and retained facts |
|---|---|
| Unreadable design or task text | Compatibility plus `spec-driven`, `assurance`, `no-spec-delta`, and `outcome-machine`. Each intact fixture first compares all three original gates. After corruption, every surviving earlier gate retains its original verdict and exact issue code/message pairs. Completion and cross-repository decisions remain equal. |
| Missing, malformed, or invalid policy | Every policy-dependent stage remains unavailable. Admitted completion and metadata identities remain present. No completion verdict uses a substitute policy. Changed results and changed requested evidence paths change report identity. |
| Unknown explicit profile | Gates and mutation observations remain unavailable. Independently admitted completion and cross-repository decisions survive. |
| Malformed metadata | Completion survives. Cross-repository evaluation remains unavailable instead of reporting vacuous success. |
| Mutation policy and base plans | All four sync/archive gate-policy combinations, each with open and checked tasks. Readiness compares exact reasons and cross-repository decisions against original sync and archive previews. |
| Unreadable unrelated accepted source | Earlier stages and completion remain equal. Only mutation observations that consume the unavailable gate become unavailable. Other mutation observations remain equal. |
| Artifact dependency closure | Direct pure tests cover all shipped schemas and gate stages. A prerequisite edge changes the selected input set. Repeated selection remains deterministic and bounded. |

Each integration row compares CLI and MCP reports, including report identity and all actions.
Each transport retains the fixture tree and reports `mutation_authorized: false`.
Every action remains advisory and has no `--execute` argument.
The original preview comparison also retains the fixture tree.
These comparisons do not complete the missing action-contract acceptance cases.

## Commands and observed exits

Cargo commands used the required environment in this worktree:

```text
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c
```

Commands also used the PATH prefix from the assignment.
The private Pueue group was `finish2-2746-readiness`.
It used one worker initially, then two for independent Nix and Cargo verification.
All new logs reside under `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`.
All log names start with `finish2-2746-readiness-`.
These paths describe operator evidence, not product dependencies.

| Command | Exit and observation | Log suffix |
|---|---|---|
| `cargo test -p cairn-core readiness` before edits | 0, 35 matched tests passed | `baseline.log` |
| `cargo test -p cairn-core` | 0, 571 unit and 12 documentation tests passed | `core-final.log` |
| `cargo test -p cairn-cli --bin cairn --test readiness --test readiness_completion --test readiness_isolation --test lifecycle --test cross_repo_dependencies --test mcp_cli --test machine_cli` | 0, all selected suites passed | `shell-final.log` |
| `cargo test -p cairn-cli --bin cairn --test readiness_isolation` after the final request-binding change | 0, 115 binary unit tests and all three isolation tests passed | `request-binding.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` on frozen product source | 0 | `clippy-frozen.log` |
| `cargo fmt --all -- --check` on frozen product source | 0 | `fmt-frozen.log` |
| `./target/debug/cairn policy export --check` through the development shell | 0, policy fresh | `policy-frozen.log` |
| `./target/debug/cairn validate --root .` | 0, valid with no findings or issues | `validate-frozen.json` |
| `./target/debug/cairn traceability coverage --root . --json` | 0, pass, no missing or dangling markers | `traceability-frozen.json` |
| `cargo test --workspace --all-targets --all-features`, with `TMPDIR=/run/user/1555` | 0, frozen product source | `workspace-runtime-tmp.log` |
| `nix flake check "path:$PWD" --option min-free 0 -L` | 0, 25 flake checks | `flake-frozen.log` |

The successful full workspace replay ran from `2026-09-06T02:41:41-04:00` to `2026-09-06T02:43:29-04:00`.
Git status was empty before and after that run.
Product source remained at `5d24e9a` throughout the final battery and all environment replays.
The all-feature binary ran 116 unit tests per binary.
Pueue task `3177` recorded `workspace=0`.
Pueue task `3128` recorded `flake=0`.

### Failed attempts and environment isolation

An intermediate compile failed because `ChangeMetadata` does not implement `Serialize`.
The fix retained typed metadata and a separate source hash. It did not add a serialization contract.
Log: `isolation-first.log`.

An intermediate policy-failure control rejected a substituted null-policy hash.
The original completion validator correctly reported policy drift.
The implementation now reports unavailable completion with admitted input identities instead.
Log: `isolation-second.log`. Subsequent isolation controls passed.

The first frozen workspace run exited 101 in `evidence_export_links_runtime_roles`.
Its export receipt reported artifact-copy and artifact-write failures.
Log: `workspace-frozen.log`.
The subsequent Nix-shell replay did not reach Cargo:

```text
error: creating temporary file '/tmp/nix-shell.x8tuqM': Disk quota exceeded
```

A native file tool also returned `EDQUOT`.
`df` reported `/tmp` at 251 GiB used, zero available bytes, and zero available inodes.
The Git filesystem had 220 GiB available.
These observations support an environment diagnosis, not a source-regression claim.

With a worktree-local `TMPDIR`, the isolated runtime export passed without source changes.
Log: `runtime-private-tmp.log`.
That full workspace attempt stopped in a bounded-tree special-file fixture with `Error: Io`.
The long temporary path is unsuitable for its Unix-socket fixture.
Log: `workspace-private-tmp.log`.

The final replay used the existing user runtime directory as `TMPDIR`.
It had 19 GiB available and a short path.
The complete workspace suite then passed without product changes or test exclusions.
No global temporary files were removed.

### Nix observations and strict acceptance boundary

The flake log retained the base's `checks.x86_64-linux.slotcar` name.
That check built `octet-consumer-check` and emitted warnings.
The successful warning-level check is not strict Octet acceptance.
No allowances or gate changes hid those warnings.

The log also recorded a missing host `ssh` executable during source-reference lookup.
Nix used the cached reference and completed the checks through the configured builder.
The self-dogfood legacy-layout rejection was its expected negative control.

The 25 check names were:

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

## Task state and exact next work

All eight open tasks remain open. `tasks.md` is unchanged.
This pass narrowed the first task's gap but did not establish its entire acceptance surface.
No task closed from process success, transport parity alone, or this report.

| Task | Remaining gap and next action |
|---|---|
| `core` | Complete stage-local admission controls for acceptance-only, review-only, and outcome-machine inputs. Audit fixed stage predicates in `load_profile_observations` against all admitted schema arrangements, including customized prerequisite and review topology. Replace those predicates with schema-derived requirements where equivalence does not hold. The shipped-profile design/task matrix is not proof for every admitted arrangement. |
| `actions` | Bind exact producer contracts and predecessor change identities in prerequisites. Complete supported preview argv with selected root, policy, and evidence arguments. Retain declaration-wide probe causes as explicitly unresolved unless a reviewed remediation rule exists. |
| `observation` | The requested task/metadata split and shipped-profile isolation now pass. Complete the remaining stage-local safe-link, read-bound, review, acceptance, and outcome-machine admission controls. Preserve each surviving decision and its admitted identity. |
| `transports` | Extend the same paired transport harness to the remaining admission matrix and completed action contracts. Current tests compare all actions but do not prove missing action semantics. |
| `negative` | Add the remaining admission controls through both transports with complete tree snapshots. Include customized schema arrangements, not only shipped profiles. |
| `consumer` | Mutation preview reason and cross-repository comparisons now pass for policy/base-plan combinations. Complete original-validator comparisons for the remaining admission and action cases. The new rows do not replace the complete maintained consumer matrix. |
| `docs` | Measure diagnostic calls for known repairs against the legacy command sequence. The existing three-gate/one-report data point remains incomplete. No latency or complete repair-study claim follows from this pass. |
| `verification` | The full battery passed on frozen product source after environment isolation. Repeat it after the remaining product work. Strict Octet acceptance and later reviewed sync/archive plans remain outside this increment. |

The next implementation step remains `core`, then `actions`, in task order.
The remaining implementation work does not require permission to weaken validators or restrict the existing profile registry.
Drain presentation remains outside this increment.
The coordinator retains later lifecycle execution and publication.
