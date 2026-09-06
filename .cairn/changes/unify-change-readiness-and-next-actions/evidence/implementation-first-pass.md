# Readiness implementation first pass

Status: partial implementation, not change completion.
Owner: Cairn lifecycle-core and CLI maintainers.
The coordinator retains integration, sync, archive, and publication authority.

## Delivered capability

The additive `readiness` command and `cairn_readiness` MCP tool share a pure report.
The report separates artifact, three gates, completion, sync, and archive decisions.
Known causes select advisory repairs. Unknown causes have unresolved actions without invented commands.
Stale review controls select the changed artifact and current BLAKE3 binding.
Invalid authoritative completion remains blocked. Its class label does not grant validity.
The shell reads explicit completion inputs and does not execute checks or mutation commands.

## Observed checks

All Cargo commands used `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop --option min-free 0 -c`.

| Command after that prefix | Observed result |
|---|---|
| `cargo test -p cairn-core artifact_workflow` before core edits | 13 passed |
| `cargo test -p cairn-core readiness` | 31 passed, including five new direct-value tests |
| `cargo test -p cairn-cli --test readiness --test readiness_completion` | Four passed |
| `cargo test -p cairn-cli --test mcp_cli --test machine_cli --test agent_completion` | 31 passed |
| `cargo clippy -p cairn-core -p cairn-cli --all-targets -- -D warnings` | Passed after repair of the new completion-input tuple type |
| `cargo fmt --check` | Passed |

Nickel positive fixture export passed. The negative fixture rejected `authority: mutation_grant`.
The generated readiness policy came from `nickel export --format json --field policy cairn-policy/readiness.ncl`.
The initial positive fixture exposed a recursive-field name collision. The committed contract uses `StageKind` and passed its replay.

The CLI fixture compares three direct gate calls with one readiness call.
Gate verdicts and issue counts agree. CLI and MCP semantic reports agree exactly.
No latency or general productivity improvement was measured.

Local persistent logs are under `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`.
Relevant names are `readiness-baseline-core.log`, `readiness-core.log`, `readiness-regression.log`, `readiness-final-focused.log`, and `readiness-final-shell.log`.
These log paths locate operator evidence. They are not runtime dependencies.

## Exact remaining work

- Add explicit cross-repository evidence admission and transport parity. The current shell supplies no external evidence and preserves resulting blockers.
- Complete failed-probe, task-order, dependency, and stale-review CLI/MCP acceptance controls. Existing regression tests pass, but do not replace the unified-report matrix.
- Audit no-follow and aggregate read limits in the reused scan and saved-probe loaders. Completion-file controls alone do not establish this boundary.
- Preserve observations independently when one loader fails. The current fallback reports unavailable stages and a bounded error class, but discards successful partial observations.
- Bind and recheck every external observation cohort. The current source/profile reread is not an atomic filesystem snapshot.
- Complete exact producer/prerequisite actions and action deduplication. Preview arguments are available. Other actions retain structured targets and bindings without executable argv.
- Integrate readiness-policy freshness and Nickel fixtures into maintained repository-wide checks.
- Finish the full consumer comparison and any reviewed drain presentation adoption.
- Run policy freshness, workspace tests, traceability, Cairn gates, Octet, and full Nix verification on final source.

The worktree predates canonical Octet integration and retains existing Nix naming.
No pins, locks, lint defaults, gate scopes, or suppression budgets changed.
The coordinator repairs Octet in its separate worktree. This pass does not claim Octet acceptance.
The verification task remains open. No sync or archive operation ran.
