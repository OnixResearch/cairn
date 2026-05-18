# Proposal: Portable Determinism Usage Rails

## Why

Cairn already emits deterministic receipts for core lifecycle decisions, but agent and CI use will replay those receipts across different checkout roots, temp directories, machines, and shells. Absolute paths, hidden environment influence, execution-side mutation ambiguity, and untested MCP/read surfaces can make otherwise deterministic decisions look different when Cairn is used outside the maintainer checkout.

## What Changes

- Add portable receipt semantics so evidence hashes exclude ambient absolute checkout paths unless a command explicitly contracts them as input.
- Add an explicit determinism replay oracle that runs selected commands repeatedly and compares normalized receipts.
- Strengthen nondeterminism disclosure for environment, clock/date, external command, and path inputs that affect replay.
- Add before/after mutation manifests for side-effecting lifecycle executions.
- Add MCP and read-side reproducibility fixtures so agent-facing Cairn use is covered, not just direct CLI happy paths.

## Capabilities

### Modified Capabilities
- `cairn.supersede.blake3_receipts`: Clarifies portable path canonicalization and hash material boundaries.
- `cairn.supersede.deterministic_gates`: Adds a maintained replay oracle for unchanged command inputs.
- `cairn.supersede.agent_run_receipts.nondeterminism`: Extends captured entropy expectations to environment and path inputs that affect Cairn replay.
- `sync_archive.execute.sync` / `sync_archive.execute.archive`: Require mutation receipts to identify changed file manifests.

### New Capabilities
- `cairn.supersede.determinism_replay`: Maintained command-level determinism checking for agent/CI use.
- `cairn.supersede.portable_receipts`: Portable receipt normalization across checkout roots.

## Impact

- **Files**: CLI receipt presentation, command test fixtures, possibly policy schema/docs, and lifecycle execution receipts.
- **APIs**: May add a `determinism-check` CLI command or an equivalent maintained test helper; existing command JSON should remain backward-compatible unless a new schema version is introduced.
- **Dependencies**: No new runtime dependency expected.
- **Testing**: Add positive tests proving repeated commands match across copied fixture roots, and negative tests proving hidden entropy/path/date influence is either captured or rejected deterministically.
