# Round-three worker evidence

Status: all required check commands passed on the final observed source, including the repeat flake check. Verification remains open for coordinator reconciliation of concurrent accepted-spec changes. This report records this worker's commands, not commands from other concurrent sessions.

## Source and concurrency

- Assigned worktree: `/home/brittonr/git/OnixResearch/.worktrees/cairn-drain-guidance-20260906`.
- Starting revision: `881e537` on `drain/utility-guidance-20260906`.
- Private Pueue group: `guide3-2746`.
- This worker did not run memo, spawn agents, change producer pins, execute sync or archive, or push.
- Another session committed `6d4ceca` while this worker's temporary unsafe-omission source mutant was active. That commit captured the mutant condition.
- This worker restored the original conservative condition and committed `59aeaf9` (`Keep verification guidance advisory across transports`). This commit changes one line only. The implementation files entered the concurrent `6d4ceca` commit.
- The first commit attempt, task 1527, stopped at `git diff --exit-code` because the index contained the mutant. Task 1528 staged the restored file and committed it.
- An untracked `.cairn/specs/change-verification-guidance/` directory appeared during this worker's commands. This worker did not create, edit, stage, or remove it.
- The task checkboxes for adapters, interfaces, controls, mutation, and consumer changed to complete in the concurrent commit. This worker did not edit those checkboxes. Verification remains open.

The concurrent `guide3-2746-adapters-and-interfaces.md` report overstates Valence support. The current implementation does not admit Valence span facts. It rejects present Valence evidence and preserves declared-link fallback when that evidence is absent.

## Implemented and tested scope

- Added pure structural admission through the existing pinned Octet translator. Source identity, repository, target, features, producer revision, and canonical receipt hash are explicit bindings.
- Kept Octet observations completion-ineligible. Observations never narrow the check selection. The outer source binding remains a declaration, not producer authenticity.
- Returned `declared_links_only` and `valence_span_contract_unavailable`. Present unsupported Valence evidence blocks admission instead of disappearing from the result.
- Added exact `argv` and argument objects for producer-plan requests. The human-readable command is display-only. The producer contract, profiles, and applicability paths must occur together.
- Added full CLI/MCP result equality checks, exact producer arguments with spaces and shell punctuation, input snapshot preservation, and rejection of partial plans and execution flags.
- Exercised deletion, rename, copied marker, unlinked source, missing edge coverage, missing check mapping, stale index, cycle, macro, generated source, target, and feature controls through both transports.
- Rebuilt both trace indexes from complete current Cairn CLI source bytes in an explicit one-file projection. The after comparison appends a comment in memory. The test registry is explicit and bounded.
- The fresh-source replay selects `policy-fresh` and `workspace-tests`, reports `subject_changed` for prior whole-subject evidence, and leaves full gates authoritative.

## Producer contract limit

Octet translation retains revision `890f424cd7b1b2015f39c0eb5747f1bd1e576028` and its existing `octet-acceptance-receipt/v1` contract.

A read-only search at Valence revision `e40c76b4d2070a29636e00c85c0dff93f03dba2f` found no `valence.evidence-admission.v1` occurrence. The exact command was:

```text
git -C /home/brittonr/git/OnixResearch/valence grep -n 'valence.evidence-admission.v1' e40c76b4d2070a29636e00c85c0dff93f03dba2f --
```

Task 1449 exited 1 with empty output. Cairn's existing adoption consumer recognizes that schema for identity and roles, but it has no span contract. The pinned stack-sidecar translator is a different contract. This worker did not substitute it for span admission.

Positive Valence span admission remains unavailable. Completion in a declared-link fallback scope must not become a claim that this stronger path exists.

## Mutation evidence

The retained `guide3-2746-unsafe-omission.patch` reverses the uncertainty condition in the actual pure selection implementation. The mutant adds broader defaults only when uncertainty is empty.

Task 1516 compiled this mutant and ran:

```text
cargo test -p cairn-core verification_impact
```

Exit: **101**. Result: **16 passed, 4 failed**. Failures:

- `missing_mapping_and_unlinked_source_select_the_broader_default`
- `missing_dependency_coverage_selects_broader_default`
- `macro_and_generated_sources_expand_to_default_work`
- `linked_edit_selects_declared_and_mandatory_checks`

The first three failures assert that the broader default check remains selected. The fourth rejects the incorrect extra selection for a fully linked case.

The integration suite also catches explicit report mutants that remove the broader default or the mandatory check. Its positive preservation assertion runs before either negative control. These expected assertion failures do not indicate a failed integration suite.

The source mutant was restored before the full workspace battery started. The first flake snapshot also contains the correct condition. Its source was inspected at `/nix/store/5gh29cvs7l14xg2mam175hl5gnjdyyk1-source/crates/cairn-core/src/verified/verification_impact.rs`.

## Command environment and logs

All shell commands used the shared required PATH prefix and `TMPDIR=/var/tmp`.
Cargo commands used:

```text
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c ...
```

Logs are under `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/` with prefix `guide3-2746`.

| Task | Command or log suffix | Observed result |
|---|---|---|
| 1446 | Initial status and log | Clean at `881e537` |
| 1447 | `cargo test -p cairn-core verification_impact`, `baseline.log` | Exit 0, 19 passed |
| 1449 | Pinned Valence schema search, `valence-contract-search.log` | Exit 1, no matches |
| 1480 | Format, focused core, CLI integration, `focused-1.log` | Exit 0, 20 core and 5 integration tests passed |
| 1497 | Focused core/CLI, Clippy, fmt, Nickel fixtures, `focused-2.log` | Exit 0 |
| 1508 | `nix flake check "path:$PWD" --option min-free 0 -L`, `flake.log` | Exit 1, traceability failure |
| 1516 | Actual source mutant, `unsafe-omission.log` | Expected exit 101, 4 failures |
| 1527 | Commit preflight | Exit 1, caught staged mutant |
| 1528 | Stage restored condition and commit | Exit 0, `59aeaf9` |
| 1510 | Full frozen-command battery, `battery-exits.log` | All eight commands exited 0; archive preview remains blocked |
| 1549 | Independent live fmt/policy/validate/traceability and previews, `live-exits.log` | All six commands exited 0 |
| 1561 | Repeat flake on the changed live tree, `flake-live.log` | Exit 0, all 26 flake checks passed |
| 1595 | Compile isolated consumer capture driver, `capture.log` | Exit 1, mismatched serde_json build artifact |
| 1596 | Compile with matching serde_json artifact and capture replay, `capture-2.log` | Exit 0, exact comparison and CLI/MCP reports retained |
| 1565 | `git apply --check` for the retained mutant, diff check, status | Exit 0; patch applies to restored source |

Task 1497 ran these checks in order:

```text
cargo fmt --all
cargo test -p cairn-core verification_impact
cargo test -p cairn-cli --test verification_impact -- --nocapture
cargo test -p cairn-cli verification_impact
cargo clippy -p cairn-core -p cairn-cli --all-targets -- -D warnings
cargo fmt --all -- --check
```

It also exported every positive verification-impact Nickel fixture and required rejection of every negative fixture. The new structural fallback and substituted producer-cohort fixtures passed those controls.

## Full battery

Task 1510 runs each required command independently. Its shell process status is not an aggregate gate verdict. The per-command exits in `battery-exits.log` are authoritative.

```text
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
./target/debug/cairn policy export --check
./target/debug/cairn validate --root .
./target/debug/cairn traceability coverage --root . --json
./target/debug/cairn sync derive-change-verification-guidance --root .
./target/debug/cairn archive derive-change-verification-guidance --root .
```

The last two commands are dry-run previews. Neither includes `--execute`.

Task 1510 completed at 21:28:14 EDT. Every listed command exited 0. The restored core passed all 644 tests, including all 20 verification-impact tests. Both CLI binary unit suites passed 128 tests each. All five verification-impact integration tests passed. Workspace Clippy finished without warnings. Final traceability has no missing or dangling IDs. The archive preview still reports one incomplete verification task despite process exit 0.

### First flake result

The first flake run evaluated 26 checks. Its check attribute is still `slotcar`, with derivation name `octet-consumer-check`. It emitted warning-level Tiger Style diagnostics. This is not strict full-catalog acceptance.

The decisive failure is `checks.x86_64-linux.tracey-coverage`. Its receipt has `missing: []` and these six dangling requirement IDs:

- `cairn.verification_impact.applicability`
- `cairn.verification_impact.conformance`
- `cairn.verification_impact.inputs`
- `cairn.verification_impact.interfaces`
- `cairn.verification_impact.selection`
- `cairn.verification_impact.uncertainty`

The snapshot preceded the concurrent accepted-spec directory. This worker did not repair the failure through sync execution or manual accepted-spec edits. Source mutation did not cause this failure.

Nix also emitted missing-SSH/cache fallback messages, unknown `wasm-builtin` warnings, and store hard-link limit warnings. Nix continued past those messages.

### Live-tree checks after the concurrent accepted-spec addition

Task 1549 ran fmt, policy freshness, validation, traceability, and both dry-run previews while the workspace test suite continued. All six commands exited 0. These results do not replace the remaining workspace, Clippy, and flake results.

- Validation: no issues, 1 active change.
- Traceability: `missing: []`, `dangling: []`.
- Sync preview: `blocked: false`, `dry_run: true`, `mutated: false`. The destination already contains the same content, so its merge preflight reports `changed: false`.
- Archive preview: `blocked: true`, `dry_run: true`, `mutated: false`. The reason is one incomplete verification task. Command exit 0 means the preview ran, not that archive is allowed.

These live results include the accepted-spec directory created by the other session. They are not results for the earlier flake snapshot. Task 1561 repeats the full flake check against the current live tree without lifecycle execution. Its immutable source is `/nix/store/h1847cdg6vw2l2y770dz8rgagsggy9c5-source`. This worker inspected that source and confirmed both the restored uncertainty condition and the accepted guidance specification.

Another session cleared finished Pueue history during this run. The durable log files and recorded per-command exits remain available. Running task 1561 remained present.

## Remaining handoff

1. Resolve ownership of concurrent worktree changes before further commits or lifecycle operations.
2. Preserve the restored uncertainty condition from `59aeaf9`. Do not use `6d4ceca` alone.
3. Retain the declared-link fallback limit. Positive Valence span admission needs a reviewed compatible published contract and an authorized immutable revision.
4. Inspect all per-command battery exits and dry-run receipts. Rerun the complete battery on one frozen final tree after the coordinator resolves the accepted-spec changes.
5. The final capture now retains `guide3-2746-consumer-input.json`, `guide3-2746-consumer-cli.json`, and `guide3-2746-consumer-mcp.json` in this evidence directory. An isolated Rust driver included the existing integration-test helpers and linked the already-tested core. It reproduced complete CLI/MCP equality, the two required checks, stale prior evidence, and no execution. The first compile selected an incompatible cached serde_json artifact; task 1596 used the matching artifact and passed. No production or test source changed for capture.
6. All required check commands now pass, including the repeat flake check. Keep the distinction between this observed tree and the earlier failing snapshot. Verification remains open for the coordinator to reconcile the untracked accepted-spec directory and its ownership before final closure. Only the coordinator can execute sync, archive, or push.
