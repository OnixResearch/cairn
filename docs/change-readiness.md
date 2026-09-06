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

Both paths are relative to the selected root. Each file has a 16 MiB limit.
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

The report binds the lifecycle source, selected policy, supplied evidence, and readiness policy with BLAKE3 identities.
The model admits seven stages, at most 512 blockers, and targets of at most 4096 bytes.
Its semantic observation limit is 16 MiB. Existing loaders retain their own limits.

## Reproduction and limits

The maintained tests are `crates/cairn-core/src/verified/readiness/tests.rs`, `crates/cairn-cli/tests/readiness.rs`, and `readiness_completion.rs`.
The basic fixture compares three direct gate calls with one readiness call. All three gate verdicts and issue counts agree.
This is a bounded command-count comparison, not a latency or general productivity claim.

This first implementation does not yet replace drain presentation. Legacy receipts remain unchanged.
The shell evaluates cross-repository declarations without an evidence-input option, so required external evidence remains blocked.
The full stale-review, failed-probe, dependency, and task-order consumer matrix remains incomplete.
The existing scan and probe loaders also need a complete no-follow and aggregate-read-bound audit before this change can close.
A load failure currently produces an unavailable report without its original infrastructure diagnostic.

Cairn lifecycle-core maintainers own the report. CLI maintainers own observation and transport adapters.
Readiness does not prove evidence truth, implementation correctness, release eligibility, or future mutation authority.

<!-- r[impl cairn.unified_readiness.transport] r[impl cairn.unified_readiness.actions] -->
