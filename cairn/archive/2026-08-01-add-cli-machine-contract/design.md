# Design: Versioned CLI machine contract

## Context

Many Cairn commands print one JSON result. Several top-level errors still return plain stderr text and a nonzero status.

MCP already has JSON-RPC framing and strict tool schemas. Direct CLI agents need a separate contract that covers parsing and dispatcher errors before a normal command receipt exists.

## Goals

- Give CLI agents one explicit JSON transport for controlled outcomes.
- Preserve stable nested command receipts and their hashes.
- Use stable diagnostic codes and exit classes.
- Keep default human and compatibility behavior unchanged.

## Non-Goals

- Replace MCP protocol framing.
- Convert every process failure into a Cairn receipt.
- Hide nonzero process status from callers.
- Claim that structured output proves command or host correctness.

## Decisions

### Decision: Make machine mode explicit and additive

**Choice:** Add a top-level invocation form:

```text
cairn --machine <command> ...
```

Default invocations keep their current output and stderr behavior. Machine mode will reject incompatible presentation options with a stable diagnostic.

**Rationale:** An opt-in boundary avoids silent compatibility changes for scripts that already consume raw command receipts.

### Decision: Emit one versioned envelope for controlled outcomes

**Choice:** A machine-mode invocation will emit exactly one JSON object on stdout:

```text
{
  "schema_version": "cairn.cli-machine.v1",
  "ok": true,
  "command": "validate",
  "exit_class": "success",
  "result": {},
  "diagnostics": [],
  "non_claims": ["a valid machine envelope proves controlled transport only"]
}
```

Failure envelopes will set `ok` to false, set `result` to null, include sorted diagnostics, and retain a nonzero process status.

The shell will run the same command through a bounded child-process adapter. It will parse one default JSON result before it builds the envelope.

The shell will write the completed object once. Controlled machine-mode outcomes will not write stderr.

**Rationale:** One bounded adapter preserves result receipts and prevents mixed JSON and display text.

### Decision: Keep nested command receipts unchanged

**Choice:** On success, `result` will contain the command value that the default mode already emits. Cairn will not add fields to that nested value or recalculate its receipt hash for machine mode.

Commands with a valid blocked receipt remain successful transport outcomes when their existing contract returns a result. Their nested `valid`, `blocked`, or `verdict` field remains authoritative.

**Rationale:** Transport success and lifecycle success are different facts.

### Decision: Use policy-owned exit classes and stable diagnostics

**Choice:** Generated policy will define supported exit classes and diagnostic fields. The initial classes are `success`, `invalid_arguments`, `not_found`, `policy_error`, `blocked`, and `internal_error`.

Each diagnostic will include stable `code`, `severity`, `message`, and `target` fields. A bounded `fix` field is optional. Display text will not determine sorting or classification.

The pure core will classify typed shell errors and sort diagnostics by policy order, code, and target.

**Rationale:** Agents need stable decisions rather than message matching.

### Decision: Bound the controlled-failure claim

**Choice:** Machine mode covers errors returned through Cairn argument parsing, dispatch, policy loading, root selection, file admission, command planning, and serialization preparation.

It does not cover operating-system termination, process kill, memory exhaustion, panic before the boundary, broken stdout, or failure before Cairn starts. Documentation and envelopes will state this boundary.

**Rationale:** A process cannot guarantee output after host-level or pre-boundary failure.

### Decision: Preserve privacy and path normalization

**Choice:** Diagnostics will use policy-bounded targets and normalized root-relative paths when available. Machine envelopes will not add environment values, usernames, credentials, secret file content, or unbounded command output.

Nested receipts retain their existing privacy contracts.

**Rationale:** Structured errors must not become a new data-leak surface.

### Decision: Preserve the functional-core boundary

**Choice:** `cairn-core` will own typed outcome classification, diagnostic normalization, ordering, and envelope values.

`cairn-cli` will own argument collection, command dispatch, error capture, policy loading, serialization, stdout, stderr, and process status.

**Rationale:** Classification remains deterministic and testable without process effects.

## Rejected Alternatives

### Change default output globally

This would break scripts that expect raw command receipts. Machine mode remains explicit.

### Print JSON errors to stderr

This would still require agents to merge streams. Controlled envelopes use stdout only.

### Return zero for every structured failure

This would hide process failure from normal supervisors. Exit status remains meaningful.

### Wrap MCP results in the CLI envelope

MCP already has a protocol envelope. It may reuse diagnostic values without changing JSON-RPC framing.

## Test Design

Positive tests cover successful read commands, valid blocked command receipts, nested receipt identity preservation, deterministic ordering, and repeated byte-stable output.

Negative tests cover unknown commands, missing and unknown arguments, missing changes, malformed policy, unsafe paths, incompatible output flags, serializer preparation failures, secret-like fixtures, and nonzero status.

Compatibility tests compare default command bytes before and after the change. Boundary tests confirm that documentation does not promise output after host-level failure.

## Risks and Trade-offs

- A second output mode increases parser and documentation surface. One shared adapter limits drift.
- The word `ok` can be confused with lifecycle validity. Documentation separates transport success from nested verdicts.
- Existing errors may lack typed codes. The change must remove message-based classification from the machine path.

## Claim Boundary

A valid machine envelope proves only that Cairn reached the controlled output boundary and serialized one typed outcome.

It does not prove nested evidence truth, command correctness, mutation safety, process reliability, host integrity, or release eligibility.

## Source Adaptation

The explicit machine contract adapts the OpenSpec agent contract. Cairn preserves existing receipts, nonzero failure status, MCP framing, Nickel policy, and bounded evidence claims.
