# CLI machine contract

Cairn provides an explicit JSON transport for direct CLI agents.

Use this form:

```text
cairn --machine <command> [args]
```

The default CLI does not use this envelope. MCP keeps its JSON-RPC framing.

## Envelope

A controlled invocation writes one JSON object to stdout.

```json
{
  "schema_version": "cairn.cli-machine.v1",
  "ok": true,
  "command": "status",
  "exit_class": "success",
  "result": {},
  "diagnostics": [],
  "non_claims": []
}
```

The envelope always has these fields:

- `schema_version` identifies the transport schema.
- `ok` reports transport completion only.
- `command` identifies the admitted command.
- `exit_class` gives the stable process class.
- `result` contains the unchanged command result. It is `null` after a controlled transport failure.
- `diagnostics` contains sorted, policy-owned diagnostic values.
- `non_claims` states the policy-owned evidence boundary.

A diagnostic has `code`, `severity`, `message`, and `target`. It can also have `fix`.

Use `code` and `exit_class` for decisions. Do not parse `message` text.

## Results and process status

A result can contain `valid: false`, `blocked: true`, or another failing lifecycle verdict.

In that case, the envelope can have `ok: true`. The nested verdict stays authoritative.

Cairn also preserves the command process status. JSON output does not convert a nonzero status to success.

The generated Nickel policy owns these transport exit classes:

- `success`
- `invalid_arguments`
- `not_found`
- `policy_error`
- `blocked`
- `internal_error`

The checked policy projection owns the numeric status for each class.

## Controlled failures

Machine mode covers these controlled stages:

- top-level argument admission
- command dispatch
- checked policy loading
- root selection
- file and path admission
- command planning
- bounded output capture
- serialization preparation

A controlled failure has `ok: false`, a `null` result, sorted diagnostics, and a nonzero status.

Controlled machine outcomes keep stderr empty. Cairn does not copy child stderr into the envelope.

An invalid machine policy is different. Cairn cannot use a contract that failed admission.

For that case, Cairn emits no envelope. It writes only `error: cairn.cli.policy_error` to stderr.

## Compatibility

The machine shell runs the same Cairn command through a bounded child-process boundary.

It parses one default JSON result and nests that value without modification. Cairn does not recalculate nested receipt hashes.

Human help, MCP, interactive output, and alternate formats are incompatible with machine mode.

For traceability coverage, include `--json` in a machine invocation.

These examples are incompatible:

```text
cairn --machine mcp
cairn --machine view graph --format mermaid
cairn --machine view graph --interactive
cairn --machine policy export
cairn --machine traceability coverage
```

The default forms keep their existing stdout, stderr, fields, and framing.

## Privacy

Machine diagnostics use fixed policy messages and bounded targets.

They do not copy raw errors, environment values, usernames, credentials, file contents, backtraces, or process output.

The `unknown` command label replaces an unrecognized command value. This rule prevents an unknown argument from becoming diagnostic data.

Nested receipts keep their existing privacy and redaction rules. The envelope does not expand omitted values.

## Agent handling

Use this sequence:

1. Run Cairn with an external timeout and signal supervisor.
2. Read stdout as one JSON object.
3. Check `schema_version` before other fields.
4. Use `ok`, `exit_class`, and diagnostic `code` values for transport decisions.
5. If `result` is present, inspect its lifecycle verdict separately.
6. Preserve and inspect the process status.
7. Treat missing output as a host or pre-boundary failure.

Do not infer success from JSON presence alone. Do not infer lifecycle validity from `ok`.

## Claim boundary

A valid envelope proves only that Cairn reached the controlled output boundary and serialized one typed outcome.

It does not prove nested evidence truth, command correctness, mutation safety, process reliability, or host integrity.

It does not prove lifecycle readiness or release eligibility.

No envelope is guaranteed after host termination, process kill, memory exhaustion, a pre-boundary panic, broken stdout, or failure before Cairn starts.

External supervisors remain responsible for timeouts, signals, and missing output.

<!--
r[verify cairn.cli_machine_contract.docs.complete]
r[verify cairn.cli_machine_contract.boundary.host]
r[verify cairn.cli_machine_contract.boundary.nonclaim]
-->
