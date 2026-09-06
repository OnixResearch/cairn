# Readiness next pass

Status: tested increments, not change completion.
Owner: Cairn lifecycle-core and CLI maintainers.
The coordinator retains sync, archive, integration, and publication authority.
Commits `0cffeb8`, `be2b4b7`, and `3612013` remain in the branch history.

## Delivered capability

- Known equivalent repairs share one action across stages. The action retains every causal blocker ID.
- Unknown diagnostics with different messages remain separate. Action ordering uses canonical identities.
- Review refresh targets use the actual profile API's change-relative artifact paths. The report converts these paths to repository-relative targets.
- Bounded capability reads open each parent with no-follow semantics. They reject leaf links and require regular files.
- Completion files share a 16 MiB input budget. Direct controls accept the exact limit and reject one additional byte.
- Saved probe observation admits at most 64 probes. Declaration documents and receipts have 64 KiB limits.
- Probe validation and evidence binding consume the same receipt bytes. A dangling declaration link fails closed.
- Readiness admits explicit cross-repository summary files through CLI and MCP. It reuses the existing summary parser and dependency validator.
- Cross-repository admission allows at most 64 repository-relative files, each at most 64 KiB. It performs no bundle discovery or fetch.
- Completion and mutation-observation failures preserve successful artifact and gate decisions.
- Observation gathering runs twice. The comparison covers source, policy, profile, probes, completion, dependency graph, external summaries, and emitted mutation decisions.
- The maintained Nix `readiness-policy-fresh` check validates exported bytes, positive and negative contract fixtures, and stale-policy rejection.

The shell uses the existing `cap-fs-ext` version 4.0.2 from the lockfile as a direct dependency.
No existing dependency version, flake pin, lint default, or gate scope changed.
No vendored source changed.

## Executable evidence

Cargo commands used this prefix:

```text
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c
```

| Command after the prefix | Observed result |
|---|---|
| `cargo test -p cairn-core readiness` | 33 passed |
| `cargo test -p cairn-cli --bin cairn` | 99 passed |
| `cargo test -p cairn-cli --test lifecycle --test cross_repo_dependencies --test mcp_cli --test machine_cli` | 57 lifecycle, 5 dependency, 13 MCP, and 13 machine tests passed |
| `cargo clippy -p cairn-core -p cairn-cli --all-targets -- -D warnings` | Passed |
| `./target/debug/cairn policy export --check` | `policy fresh: cairn-policy/generated/cairn-policy.json` |

The focused readiness and completion integration tests passed before the final argument-type control.
The final replay log records that control and the final formatting check.

This command also passed:

```text
nix build path:$PWD#checks.x86_64-linux.readiness-policy-fresh --option min-free 0 --no-link -L
```

The consumer controls compare direct gate decisions with CLI and MCP readiness reports.
They cover passing and failed probes, missing receipts, tampered receipts, stale policy, task-order cycles, and stale assurance reviews.
Local dependency controls retain the exact upstream target and clear the wait action after the fixture moves upstream to archive.
Cross-repository controls cover explicit passing evidence, absent evidence, failed status, unsafe paths, directories, oversized files, and excessive file counts.
Inspection preserves the saved probe receipt bytes and does not create missing receipts.
These are bounded fixture observations, not whole-system correctness or release evidence.

An initial test run failed with `No space left on device` before compilation.
Only this worktree's `target/debug/incremental` cache was removed. The replay disabled incremental compilation and passed.

Persistent logs are under `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`:

- `readiness-next-policy-core.log`
- `readiness-next-shell.log`
- `readiness-next-final.log`
- `readiness-next-transport-final.log`

These are operator log locations, not runtime dependencies.

## Exact remaining gaps

1. `support/scan.rs` still uses unbounded traversal and text reads. Its full no-follow and aggregate-read contract remains open.
2. Workflow admission-text, policy, and sync-merge loaders still need bounded no-follow conversion. The readiness report's post-load size check cannot replace admission limits.
3. Accepted-spec contents outside merge targets still lack exact cohort binding. Repeated decisions do not prove unchanged input bytes.
4. Source or profile load failures still discard otherwise independent observations. Mutation loader failures also combine sync and archive unavailability.
5. Cohort drift controls exercise an explicit observation callback. They do not establish an atomic filesystem snapshot or protection against arbitrary external mutation.
6. Producer-specific action arguments and prerequisite identities remain incomplete. Some probe failures still produce unresolved actions. Generic gate targets are not exact producer targets.
7. Drain presentation adoption and the complete consumer comparison remain open.
8. Workspace tests, final traceability, Cairn lifecycle gates, Octet acceptance, and full Nix verification remain outstanding.

All previously unchecked tasks remain unchecked. No sync, archive, push, or worktree removal ran.
The evidence consists of executable controls and their observed outputs. This note and structural planning gates do not establish implementation completion.
