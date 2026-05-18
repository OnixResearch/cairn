## Overview

This change tightens Cairn's MCP contract at the transport shell boundary. Tool schemas remain JSON data emitted by `cairn mcp --describe` and `tools/list`; validation happens before a tool dispatch reads lifecycle artifacts, constructs plans, or invokes mutation-capable shell paths.

## Schema shape

Each tool should advertise an object schema with:

- explicit `properties` for accepted arguments;
- `required` for mandatory lifecycle selectors such as `change`, `stage`, `spec`, or `path` where applicable;
- `additionalProperties: false` unless a tool intentionally accepts a free-form receipt object;
- bounded string descriptions for roots, lifecycle identifiers, selectors, and migration sources;
- mutation-tool schemas that are identical to their CLI semantics and remain omitted unless `--allow-mutations` is present.

The schema does not need a full JSON Schema engine in the first implementation. A small deterministic validator can enforce the subset Cairn emits: object inputs, required keys, string-only lifecycle selectors, known enum values, no unknown keys, and inline receipt size/type boundaries.

## FCIS boundary

Schema construction and argument validation are part of the MCP/CLI shell because they depend on transport-facing JSON and user input. Pure lifecycle planning, validation, receipts, and policy behavior remain in `cairn-core`; this change must not add MCP or filesystem concerns to the core.

## Errors and receipts

Invalid arguments should fail before lifecycle reads or writes. `--call` may continue to surface these as command errors, and JSON-RPC `tools/call` should return a JSON-RPC error object with deterministic text. Mutation-denied responses remain structured capability errors and must still report `mutated=false`.

## Rollout

Implement schemas and validation in one local MCP slice, then update docs. Sync/archive the accepted MCP requirements only after focused MCP tests and Cairn gates pass.
