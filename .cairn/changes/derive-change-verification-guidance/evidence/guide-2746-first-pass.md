# guide-2746 first implementation pass evidence

## Scope

Worktree: `/home/brittonr/git/OnixResearch/.worktrees/cairn-drain-guidance-20260906`
Branch: `drain/utility-guidance-20260906`
Base already contained prerequisites for unified readiness and check-evidence producer.
No sync, archive, push, or worktree removal was performed.

## Commit

- `6554aa2 Add pure verification impact guidance`.

## Implemented

- Added `crates/cairn-core/src/verified/verification_impact.rs` as a pure advisory core.
  - Admits before/after source manifests, asserted changed paths, content-bound trace indexes, and repository check maps.
  - Derives added, deleted, modified, and rename deltas from admitted identities.
  - Requires the asserted path set to match the derived delta exactly.
  - Unions before and after trace links, so deletions and moved/copied markers remain visible.
  - Selects requirement checks through declared mappings and dependency edges.
  - Preserves mandatory checks in every valid selection.
  - Expands to default checks under unknown coverage, missing mapping, unlinked source, macro opacity, generated source, and incomplete coverage.
  - Blocks stale trace/source manifest comparisons, invalid source scope drift, unknown checks, unsafe paths, duplicate paths, and requirement-edge cycles.
  - Emits deterministic selected checks, affected requirements, causal paths, uncertainty records, full required checks, non-claims, input hash, and semantic hash.
- Exposed the module through `crates/cairn-core/src/verified/mod.rs` and public core exports in `crates/cairn-core/src/lib.rs`.
- Added `contracts/verification-impact.ncl` with named bounds and typed contracts for source comparison, check map, scope, uncertainty, and explanation shapes.
- Added Nickel fixtures:
  - Positive: `fixtures/verification-impact/positive/source-comparison.ncl`
  - Negative: `fixtures/verification-impact/negative/unsafe-path.ncl`
  - Negative: `fixtures/verification-impact/negative/unsorted-features.ncl`

## Task state changes

- Marked `baseline` complete after recording current trace-index, requirement-registry, developer-loop, and applicability controls.
- Marked `contracts` complete after Nickel contracts and positive/negative fixtures passed.
- Marked `core` complete after pure core implementation and direct positive/negative controls passed.
- Left `evidence`, `adapters`, `interfaces`, `controls`, `mutation`, `consumer`, and `verification` open.

## Commands and results

All commands used the required PATH prefix and `TMPDIR=/var/tmp`. Cargo commands used `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c ...`.

| Pueue task | Command summary | Exit | Result |
|---|---:|---:|---|
| 927 | `git status --short --branch && git log --oneline -5` | 0 | Worktree on `drain/utility-guidance-20260906`, ahead 1 at `811771b`. |
| 943 | attempted direct `nickel` fixture check | 0 | Invalid evidence: host lacked `nickel`; shell continued to final `echo`. Retained here as a discarded command, not acceptance evidence. |
| 945 | `nix develop ... nickel export ...` | nonzero | Failed on invalid escape in new `contracts/verification-impact.ncl`; fixed. |
| 947 | `nix develop ... nickel export ...` | nonzero | Failed on invalid `std.contract.Array` syntax; fixed. |
| 951 | `nix develop ... nickel export ...` | nonzero | Failed on remaining `std.contract.Array` syntax; fixed. |
| 953 | `nix develop ... nickel export positive; negative fixtures must fail` | 0 | `nickel-fixtures-pass`. |
| 954 | `cargo test -p cairn-core verification_impact -- --nocapture` | 0 | Initial 10 verification-impact tests passed. |
| 959 | attempted multi-filter cargo baseline test | nonzero | Invalid cargo invocation; no acceptance evidence. |
| 964 | `cargo test -p cairn-core content_bound_trace_index -- --nocapture` | 0 | 8 trace-index baseline tests passed, including valid/stale source and copied-marker freshness. |
| 970 | attempted multi-filter cargo baseline test | nonzero | Invalid cargo invocation; no acceptance evidence. |
| 974 | `cargo test -p cairn-core requirement_registry`; `generated_policy_admits_developer_loop_profiles`; `stale_and_substituted_observations_are_rejected` | 0 | 11 requirement-registry tests, 1 developer-loop policy test, and 1 check-evidence applicability stale/substitution test passed. |
| 977 | `cargo fmt --all -- --check` | nonzero | Reported formatting diffs; fixed with `cargo fmt --all`. |
| 979 | `cargo fmt --all` | 0 | Formatting applied. |
| 980 | `cargo test -p cairn-core verification_impact -- --nocapture` | 0 | 10 tests passed after formatting. |
| 984 | `cargo clippy -p cairn-core --all-targets -- -D warnings` | 0 | Focused core Clippy passed. |
| 988 | `cargo fmt --all -- --check` | 0 | Formatting check passed. |
| 995 | `cargo fmt --all; cargo test -p cairn-core verification_impact; cargo clippy -p cairn-core --all-targets -- -D warnings; cargo fmt --all -- --check` | 0 | Final focused battery passed with 13 verification-impact tests. |
| 997 | `git diff --check && git status --short` | 0 | No whitespace errors; showed expected modified/untracked files. |
| 1001 | `nix run path:$PWD#cairn -- validate --root .` | 0 | Native lifecycle validation passed with `valid: true`, 3 completed tasks and 7 open tasks. |
| 1010 | `git add ... && git diff --cached --check && git commit -m "Add pure verification impact guidance"` | 0 | Created commit `db37a6f`; no staged whitespace errors. |
| 1011 | `git status --short --branch && git log --oneline -3` | 0 | Worktree clean; branch ahead 2 with `db37a6f` on top of `811771b`. |
| 1012 | `git add evidence && git commit --amend --no-edit && git status --short --branch && git log --oneline -3` | 0 | Amended implementation commit to `6554aa2`; worktree clean after amend. |

Observed non-blocking environment warning: `warning: unknown experimental feature 'wasm-builtin'` during Nix commands. Task 1001 also emitted transient SSH/remote-builder cache warnings before local validation succeeded.

## Remaining gaps / next actions

1. `evidence`: compose current applicability validators into the impact report so a changed final subject never grants whole-subject evidence reuse. The core currently records the non-claim but does not yet call the applicability validator.
2. `adapters`: admit compatible Octet structural and Valence span evidence, and retain declared-link fallback when stronger contracts are unavailable.
3. `interfaces`: add CLI and MCP read-only guidance surfaces through unified readiness actions and exact check-evidence producer plan requests without execution.
4. `controls`: add the full shell/interface control battery for deletion, rename, copied marker, missing edge, stale index, cycle, macro, generated source, target, and feature cases. Core controls exist for these classes, but task acceptance needs whole-surface coverage after interfaces/adapters exist.
5. `mutation`: retain an explicit unsafe-omission mutant that fails acceptance, plus required-check-preservation positives.
6. `consumer`: replay fresh Cairn source comparisons and document selected checks, refresh needs, fallback scope, and outstanding full gates.
7. `verification`: run the final full battery, prepare reviewed sync/archive plans, and record exact receipts. Do not execute sync/archive in this subagent pass.
