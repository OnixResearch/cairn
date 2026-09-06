# Evidence producer finish9-2746

## Scope

Worktree: `/home/brittonr/git/OnixResearch/.worktrees/cairn-drain-evidence-20260905`.
Branch: `drain/utility-evidence-20260905`.
No sync execution, archive execution, push, agent spawn, producer pin change, lockfile edit, or gate weakening occurred.

## Implementation

Closed the last consumer gap by adding a producer-owned developer-loop receipt path:

- `cairn check-evidence developer-loop --execute --root PATH --profile cairn-local --subject-hash HASH --out DIR`
- The command loads and admits the existing `cairn-local` developer-loop policy, runs only its reviewed argv grammar (`cargo fmt --all -- --check`, `cairn validate --root .`, `cairn traceability coverage --root .`) through Bounded Exec, measures bounded wall-clock duration in the CLI shell, publishes a durable no-replace `developer-loop-receipt.json`, and leaves the original `cairn developer-loop verify` validator as the consumer authority.
- The receipt records bounded stdout/stderr hashes and completion facts, not raw command output.
- `checks.x86_64-linux.rust-tests` now includes `pkgs.rustfmt` in `nativeCheckInputs` so the sandboxed producer replay can execute the admitted fmt grammar instead of depending on ambient host tooling.

Files changed:

- `crates/cairn-cli/src/check_evidence/developer_loop.rs`
- `crates/cairn-cli/src/check_evidence.rs`
- `crates/cairn-cli/src/support/usage.rs`
- `crates/cairn-cli/tests/check_evidence_local.rs`
- `docs/check-evidence-producer.md`
- `flake.nix`
- `.cairn/changes/produce-check-evidence-with-stack-adapters/tasks.md`

## Commands and exits

All command logs use prefix `finish9-2746` under `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`.
Cargo commands used `TMPDIR=/var/tmp env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c`.

| Command | Exit | Evidence |
|---|---:|---|
| `cargo test -p cairn-cli --test check_evidence_local real_developer_loop_profile_receipt_passes_the_original_validator -- --nocapture` | 0 | `finish9-2746-developer-loop-focused3.log`; earlier focused attempts exposed and fixed unformatted source and relative-root runner errors. |
| Live producer replay: `check-evidence source`, `check-evidence developer-loop --execute`, then original `developer-loop verify cairn-local` | 0 | `finish9-2746-loop-source3.json`, `finish9-2746-loop-producer3.json`, `finish9-2746-loop-verify3.json`; producer `valid: true`; original verifier `valid: true`, `verdict: PASS`, checks fmt/validate/traceability passed, duration 2 seconds. |
| Focused producer tests: `cargo test -p cairn-core check_evidence` and `cargo test -p cairn-cli --test check_evidence --test check_evidence_local --test check_evidence_saved --test check_evidence_stack --test check_evidence_nix` | 0 | `finish9-2746-producer-focused.log`; core check-evidence tests 26 passed; CLI producer tests 19 passed including the new developer-loop original-validator replay. |
| `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `./target/debug/cairn policy export --check`; `./target/debug/cairn validate --root .`; `./target/debug/cairn traceability coverage --root . --json` | 0 | `finish9-2746-verification-battery.log`, `finish9-2746-traceability.json`; traceability: 2313 requirements, 2313 referenced, missing/dangling empty, verdict `pass`. |
| `nix flake check "path:$PWD" --option min-free 0 -L` before adding `pkgs.rustfmt` to the rust-tests check environment | 1 | `finish9-2746-flake-check.log`; failed only in the new sandboxed developer-loop replay because the check environment could not run the fmt grammar. |
| `nix flake check "path:$PWD" --option min-free 0 -L` after adding `pkgs.rustfmt` | 0 | `finish9-2746-flake-check2.log`; full check finished with `all checks passed!`. |
| `./target/debug/cairn gate tasks produce-check-evidence-with-stack-adapters --root .` | 0 | `finish9-2746-gate-tasks.json`; tasks gate valid after marking all implementation tasks complete. |
| `./target/debug/cairn sync produce-check-evidence-with-stack-adapters --root .` | 0 | `finish9-2746-sync-preview.json`; dry run only, `blocked: false`, one `sync_delta_spec` action. |
| `./target/debug/cairn archive produce-check-evidence-with-stack-adapters --root .` | 0 | `finish9-2746-archive-preview.json`; dry run only, `blocked: false`, one `archive_change` action. |
| `./target/debug/cairn validate --root .` after task completion and previews | 0 | `finish9-2746-post-task-validate.json`; lifecycle remains valid. |

## Task state

- `consumers`: marked complete after the live positive `cairn-local` developer-loop replay produced a Cairn-owned `cairn.developer-loop-receipt.v1` and the original `cairn developer-loop verify` validator accepted it without manual result edits. The pinned Kiln replay remained covered by `finish7-2746-kiln-consumer-replay.log`.
- `verification`: marked complete after focused producer tests, fmt/clippy/policy/validate/traceability, full flake check, tasks gate, and unblocked sync/archive dry-run previews.

All 10/10 tasks are checked in `tasks.md`.

## Remaining gaps

No implementation gap remains in this worktree. The coordinator still owns sync execution, archive execution, integration, and push.
