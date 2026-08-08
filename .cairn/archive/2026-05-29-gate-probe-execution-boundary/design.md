# Design: Gate Probe Execution Boundary

## Context

Current Cairn gates are intentionally narrow. `cairn gate proposal|design|tasks` loads policy and lifecycle artifacts in the CLI, passes a pure snapshot into `cairn-core`, and emits deterministic JSON. The gate path does not spawn commands.

First-class probes need command execution, but putting command execution directly into default gates would weaken the trust boundary. A gate could become slow, flaky, side-effecting, or environment-dependent without the operator asking for execution. The safer design is a two-step evidence chain:

```text
cairn probe run ... -> probe receipt
cairn gate tasks ... -> validates lifecycle + probe receipt evidence
```

## Execution model

### Default gates remain non-executing

Default gates continue to evaluate only already-loaded data:

- policy data;
- lifecycle source or generated lifecycle bundle;
- declared probe metadata;
- saved probe receipt metadata when configured;
- file/content hashes of explicitly declared local evidence paths.

Default gates must not run shell commands, probe commands, network requests, package managers, Nickel evaluation, Steel scripts, or arbitrary host tools. They can reject missing or stale evidence but not create it.

### Probe runner owns command execution

Add an explicit probe execution command family, for example:

```bash
cairn probe run --root . --probe archive-blocked-no-mutation --out target/probes/archive-blocked-no-mutation.json
cairn probe run --root . --change typed-lifecycle-probes --out-dir target/probes/
```

The probe runner is the imperative shell. It may execute only structured probe commands authorized by policy. It records:

- probe ID and requirement IDs;
- declared coverage class;
- command identity and bounded argv;
- selected fixture refs and hashes;
- exit status class;
- stdout/stderr size and BLAKE3 hashes, not raw unbounded logs;
- artifact paths and BLAKE3 hashes;
- assertion results;
- lifecycle bundle hash;
- policy hash;
- receipt hash.

Execution should avoid ambient authority. Filesystem, process, network, secret access, mutation, and external tool access must be capability-declared and policy-authorized. Initial probes should use local CLI commands only.

### Gates verify receipts

Gate evaluation consumes saved probe receipts as evidence. The pure core should check:

- receipt schema version is supported;
- receipt hash is correct;
- probe ID exists in declared lifecycle data;
- requirement IDs match declared coverage;
- command identity matches declared probe command;
- assertion summary passed;
- coverage class matches task/gate expectations;
- lifecycle bundle hash matches current generated bundle;
- policy hash matches current policy when policy participates;
- artifact hashes still match declared evidence files if paths are included.

This makes probe evidence replayable and lets CI cache expensive probe execution while gates remain deterministic.

### Optional inline probe mode

An inline gate mode can be added later:

```bash
cairn gate tasks my-change --run-probes
```

This mode is not the default. It must be enabled by policy and must reuse the same probe runner and receipt validation path. Inline execution should emit both probe receipts and the gate receipt, or include stable links to written probe receipts. If a probe is denied, times out, exceeds budgets, requests undeclared capabilities, or produces stale evidence, the gate fails closed.

### FCIS boundary

- `cairn-core` owns pure validation of probe declarations and probe receipts.
- `cairn-cli` owns command execution, filesystem reads/writes, stdout/stderr hashing, timeout handling, and JSON printing.
- Nickel policy owns capability classes, allowed probe command profiles, budgets, redaction classes, and receipt schema versions.
- Steel, if ever used for probe assertions, remains optional, disabled by default, and Rust-authorized.

## Receipt shape

A probe receipt should include stable fields such as:

```json
{
  "schema": "cairn.probe-receipt.v1",
  "probe_id": "archive-blocked-no-mutation",
  "requirements": ["cairn.archive.blocked_no_mutation"],
  "coverage_class": "negative",
  "command_identity": "cairn archive --execute",
  "status_class": "success",
  "assertions": [{"id": "blocked-true", "passed": true}],
  "lifecycle_bundle_hash": "...",
  "policy_hash": "...",
  "receipt_hash": "..."
}
```

The exact public fields should be Nickel-owned before stabilization.

## Verification

Focused checks for this change package:

```bash
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- gate proposal gate-probe-execution-boundary --root .
nix run path:$PWD#cairn -- gate design gate-probe-execution-boundary --root .
nix run path:$PWD#cairn -- gate tasks gate-probe-execution-boundary --root .
```

Implementation should include positive and negative tests for default gate non-execution, explicit probe execution receipts, missing receipt failure, tampered receipt failure, stale bundle/policy hash failure, and denied inline execution under default policy.
