# Change readiness

`cairn readiness CHANGE` reports separate artifact, proposal, design, task, completion, sync, and archive states.
The command does not execute checks or mutate lifecycle files.

```text
cairn readiness demo --root fixtures/basic
cairn readiness demo --root fixtures/basic --format summary
cairn --machine readiness demo --root fixtures/basic
cairn mcp --call cairn_readiness '{"change":"demo","root":"fixtures/basic"}'
```

JSON is the default format. CLI and MCP return the same semantic report and receipt identity.
The machine envelope retains its existing transport contract.
`ok: true` means transport success. It does not mean lifecycle readiness.

## States and authority

- `ready`: the selected observation passed its existing validator.
- `blocked`: an observed prerequisite or validator rejected progress.
- `unavailable`: the observation is absent, unreadable, unsafe, inconsistent, or outside the supported scope.
- `not_applicable`: the supplied observation explicitly excludes this stage. The current shell does not infer this state for completion.

`valid` describes report consistency. `classification` describes the combined lifecycle state.
A consistent report can be blocked or unavailable.
An identity conflict makes every stage unavailable and sets `valid: false`.
Original decisions remain in `stages[].decision`. The stage projection can additionally block mutation previews when completion is unavailable or invalid.

Completion requires explicit saved inputs:

```text
cairn readiness CHANGE --root REPO --contract contract.json --results results.json --subject-hash HASH
```

Both paths are relative to the selected root. The two files share a 16 MiB limit.
The bounded reader rejects symlinks in every relative path component and requires a regular file.
The subject hash is an explicit caller observation, not an automatically measured source-code identity.
The existing completion validator checks the selected policy, subject, contract, and result bindings.
`classification: authoritative` inside a completion decision does not override `valid: false` or `verdict: FAIL`.
Without completion inputs, completion remains unavailable. Artifact presence and checked tasks cannot substitute for execution evidence.

## Actions

Actions have stable causes, targets, input prerequisites, expected observations, and an `advisory` authority class.
Known rules select artifact repair, evidence refresh, or dependency wait.
Unknown diagnostics produce `unresolved` actions with no invented command.
Ready mutation stages offer existing preview commands only. These commands have no `--execute` argument.
Actual mutation still requires a separate existing command and fresh preflight.
Known repairs with equal rule, target, and input bindings share one action. Each original blocker keeps its identity.
Unknown diagnostics with different messages remain separate.
Saved-probe blockers identify the admitted receipt path rather than the task document.
The core accepts these targets only when their codes and messages match the original validator issues.
Two producers with the same diagnostic retain their separate targets. Unsupported producer commands remain absent from `argv`.

The report binds the lifecycle source, selected policy, supplied evidence, and readiness policy with BLAKE3 identities.
The model admits seven stages, at most 512 blockers, and targets of at most 4096 bytes.
Its semantic observation limit is 16 MiB. Existing loaders retain their own limits.

## Reproduction and limits

The maintained tests are `crates/cairn-core/src/verified/readiness/tests.rs`, `crates/cairn-cli/tests/readiness.rs`, and `readiness_completion.rs`.
The basic fixture compares three direct gate calls with one readiness call. All three gate verdicts and issue counts agree.
This is a bounded command-count comparison, not a latency or general productivity claim.

This first implementation does not yet replace drain presentation. Legacy receipts remain unchanged.
Explicit cross-repository evidence uses repeated `--cross-repo-evidence FILE` options or the MCP `cross_repo_evidence` array.
This surface admits repository-relative summary files only. It does not discover bundles or fetch remote evidence.
It uses the existing `cairn.cross-repo-receipt-summary.v1` admission contract.
The limits are 64 files and 64 KiB per file. Unsafe, missing, oversized, or malformed files make mutation observations unavailable.

Completion and mutation-observation failures retain successful artifact and gate observations. Error details use bounded classes, not raw infrastructure text.
The shell repeats observation gathering and rejects differences in the bound cohort. This does not provide an atomic filesystem snapshot.
Lifecycle scans and merge preflight use no-follow source readers. Each reader admits at most 1 MiB per file and 16 MiB in total.
Each traversal admits at most 16,384 entries and 64 relative path components. These are per-reader budgets, not one shared report-wide budget.
Policy loading admits at most 16 MiB and rejects linked parents and leaves. Explicit parent components remain supported without concealing symbolic links.
Workflow admission text uses its existing size bound during the read, not only before it.

Mutation observations bind the exact validation-source bytes, including accepted specifications outside merge targets.
Source and profile admission failures retain an independently evaluated completion decision when policy loading succeeds.
A merge-only admission failure marks sync unavailable without discarding the archive decision.

The full report-wide I/O budget, complete independent stage loading, producer-specific commands, and drain adoption remain open.
Repeated reads detect observed differences but do not protect against arbitrary concurrent filesystem changes. These gaps prevent change completion.

Saved probe observation admits at most 64 declarations and 64 KiB per declaration document or receipt.
It reads each receipt once for validation and binding. The read uses the no-follow bounded reader.
The maintained Nix `readiness-policy-fresh` check covers generated policy, contract fixtures, and stale-policy rejection.

Cairn lifecycle-core maintainers own the report. CLI maintainers own observation and transport adapters.
Readiness does not prove evidence truth, implementation correctness, release eligibility, or future mutation authority.

<!-- r[impl cairn.unified_readiness.transport] r[impl cairn.unified_readiness.actions] -->
