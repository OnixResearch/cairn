## Why

Cairn emits deterministic JSON for many successful commands and validation failures. Dispatcher, argument, lookup, and policy-loading failures can still produce plain stderr text.

Agents that invoke the CLI need one explicit, versioned transport contract for controlled success and failure outcomes. The contract must preserve existing human and receipt surfaces.

## What Changes

- Add an opt-in top-level `--machine` mode for supported CLI commands.
- Emit exactly one versioned JSON envelope on stdout for every dispatcher-controlled success or failure.
- Add stable diagnostic objects and policy-owned exit classes.
- Keep stderr empty for controlled machine-mode outcomes.
- Reuse existing command result receipts as nested result values without changing their hashes.
- Keep default CLI output and MCP JSON-RPC framing unchanged.
- Add positive and negative fixtures for parsing, lookup, policy, blocking, privacy, compatibility, and uncontrolled process boundaries.

## Impact

- **Policy**: Add the machine contract version, exit classes, diagnostic fields, bounds, and required non-claims.
- **Core**: Add pure outcome classification, diagnostic normalization, ordering, and envelope construction.
- **CLI**: Add top-level mode parsing and one controlled serialization path.
- **Documentation**: Add an agent CLI contract and migration guide.
- **Testing**: Add positive and negative parser, dispatcher, command, compatibility, determinism, and privacy coverage.

## Out of Scope

- Replacing MCP JSON-RPC envelopes.
- Catching operating-system termination, process kill, memory exhaustion, or faults before Cairn starts.
- Changing existing command receipt schemas or receipt hashes.
- Making machine output proof of command correctness or host reliability.
