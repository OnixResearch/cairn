# Readiness continuation 2384

Status: tested increment, not change completion.
Product commit: `6c48ce6` — Bound readiness inputs and retain independent decisions.
Maintenance owner: Cairn lifecycle-core and CLI maintainers.
The coordinator retains sync, archive, integration, and publication authority.

## Preserved work

This pass preserved all existing commits and the unfinished source edits present at startup.
Commit `6c48ce6` includes those edits, their regression controls, and the new source-admission increment.
The previous `implementation-next-pass.md` report remains unchanged as historical evidence.
No producer pin, dependency manifest, lockfile, generated producer schema, vendored source, or gate allowance changed.
No memo, agent spawn, sync, archive, push, or worktree removal ran.
All previously unchecked tasks remain unchecked.

## Delivered capability

- Lifecycle scanning and merge preflight use a shared source-reader implementation with explicit per-reader budgets.
- The reader admits at most 1 MiB per file, 16 MiB total bytes, 16,384 directory entries, and 64 relative path components.
- Directory traversal and file admission reject symbolic links. Reads require regular files and enforce the limit during the read.
- Policy loading admits at most 16 MiB. It rejects linked parents and leaves, including a symbolic link followed by `..`.
- Explicit policy paths with ordinary parent components remain compatible. The loader resolves each preceding directory without following links.
- Workflow admission text now uses bounded no-follow reads instead of a metadata check followed by an unbounded read.
- Mutation observations bind the exact bytes supplied to specification validation, including accepted specifications outside merge targets.
- A semantically inert accepted-spec edit changes the validation input hash and readiness receipt identity.
- Source or profile admission failure preserves the independent completion decision when policy loading succeeds.
- The pure core distinguishes missing merge admission from a valid empty merge. Missing merge admission does not discard archive observations.
- Saved-probe causes name the admitted receipt path. The core binds each target to an exact original validator code and message.
- Distinct producer targets survive equal diagnostics. An unmatched supplied cause makes the observation unavailable.
- Existing probe schemas, gate decisions, producer bindings, and non-claims remain intact. Unsupported producer commands remain absent from `argv`.

The durable contribution is reusable bounded source admission and a more precise readiness projection.
Tests exercise actual reads, exact limits, invalid UTF-8, linked ancestors, dangling destinations, independent failures, and direct-value causal admission.
These controls establish bounded fixture observations, not an atomic filesystem snapshot or arbitrary-race protection.

## Commands and observed verdicts

All Cargo commands used this prefix from the readiness worktree:

```text
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c
```

The private Pueue group was `continue-2384-readiness`.
Logs reside under `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/`.
Every new log has the prefix `continue-2384-readiness`. Older logs were not overwritten.
These paths are operator evidence locations, not runtime dependencies.

| Command after the prefix | Observed verdict | Log suffix after `continue-2384-readiness-` |
|---|---|---|
| `cargo test -p cairn-core readiness` before edits | 33 passed | `baseline-core.log` |
| `cargo test -p cairn-cli --test readiness --bin cairn` before edits | 99 unit and 4 transport tests passed | `baseline-shell.log` |
| `cargo test -p cairn-cli --test lifecycle --test cross_repo_dependencies --test mcp_cli --test machine_cli` baseline regression | Passed | `baseline-regression.log` |
| `cargo clippy -p cairn-core -p cairn-cli --all-targets -- -D warnings` baseline | Passed | `baseline-clippy.log` |
| `cargo test -p cairn-core` final source | 570 unit and 12 documentation tests passed | `core-complete.log` |
| `cargo test -p cairn-cli --bin cairn --test readiness --test readiness_completion --test lifecycle --test cross_repo_dependencies --test mcp_cli --test machine_cli` final source | 109 unit, 4 readiness, 1 completion, 57 lifecycle, 5 dependency, 13 MCP, and 13 machine tests passed | `shell-complete.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` final source | Passed, no allowances added | `clippy-complete.log` |
| `cargo fmt --all --check` final source | Passed | `fmt-complete.log` |
| `./target/debug/cairn policy export --check` | Policy fresh | `policy.log` |

Additional current-source CLI observations:

- `./target/debug/cairn validate --root .` returned no findings or issues. See `continue-2384-readiness-validation.json`.
- `./target/debug/cairn traceability coverage --root . --json` returned `valid: false` and `verdict: fail`. See `continue-2384-readiness-traceability.json`.
- `git diff --check` passed before the product commit. Dependency and lockfile diffs were empty.

### Failed runs retained

The first source-reader test replay rejected existing explicit policy paths containing `..`.
The repair retained parent-path compatibility without following symbolic links.
Positive parent-path and negative link-cancellation tests passed in the final replay.
See `continue-2384-readiness-source-first.log` and `continue-2384-readiness-source-second.log`.

`cargo test --workspace` stopped at `evidence_export_materializes_complete_portable_chain`.
The export payload reported `export.bundle_verification_failed` and `export.manifest_verification_failed`.
Its bundle verifier reported `release_readiness.invalid`. Release readiness failed its traceability check.
The current traceability receipt identifies these six dangling requirements:

- `cairn.unified_readiness.actions`
- `cairn.unified_readiness.boundary`
- `cairn.unified_readiness.conformance`
- `cairn.unified_readiness.decisions`
- `cairn.unified_readiness.snapshot`
- `cairn.unified_readiness.transport`

They remain in the active delta, outside the accepted-spec traceability scope.
The coordinator owns any later reviewed sync. This pass did not bypass or weaken traceability.
See `continue-2384-readiness-workspace.log`. This run preceded the final direct-value merge-admission test and is not a passing workspace receipt.
Final core, focused shell, strict Clippy, and formatting replays include that last change.

## Exact remaining gaps

1. The report has per-reader limits, not one aggregate I/O budget shared across every loader and both observation passes.
2. Source, policy, and profile loading still have shared failure boundaries. Completion survives source/profile failure, but the complete independent-stage matrix remains open.
3. Dependency, external-evidence, or gate-observation admission errors can still make both mutation stages unavailable. Gate-disabled policy cases need independent admission controls.
4. Repeated observations detect differences in the represented inputs. They do not establish an atomic snapshot or prevent arbitrary concurrent filesystem mutation.
5. Probe targets are exact for admitted receipt paths. Declaration-wide failures and unsupported producer commands still require reviewed causal adapters.
6. Producer-specific `argv` and producer-contract prerequisite identities remain incomplete. Scope hashes are not a substitute for those contracts.
7. Drain presentation adoption, the full consumer comparison, and the complete diagnostic-call study remain open.
8. Workspace acceptance remains blocked by the six dangling readiness requirement IDs. Later test targets after the failed workspace target did not execute in that run.
9. No full Nix verification or Octet acceptance is claimed. The coordinator owns the separate Octet repair.

At the product commit, all startup source edits and all new implementation edits were committed.
The only subsequent work is this evidence report. No unfinished source edit is intentionally left outside version control.
This report and successful process exits do not establish producer payload success, executed coverage beyond the listed tests, or change completion.
