# Add the convergence inspect MCP tool

## Why

Decision D4 of the declarative codebase loop made `cairn convergence inspect`
CLI-only and deferred an MCP surface to a later change. Agents and MCP clients
that already read readiness, adoption and selected-work receipts cannot ask the
convergence question without leaving the MCP transport, so a consumer must
shell out to the binary for the same advisory receipt.

Issue #12 asks for a typed native view; this change exposes the existing one
through the protocol rather than adding new semantics.

## What Changes

- Add the read-only MCP tool `cairn_convergence_inspect` with required `case`
  and optional `root` and `policy`, with the same meanings as the CLI flags.
- Share one inspect function between `cairn convergence inspect` and the tool,
  so the receipt, rejection and `receipt_blake3` cannot drift.
- Register the tool in the catalog, the `mcp-discovery` profiles and the
  generated exports, and regenerate policy the repository's way.
- Add positive tests that the MCP receipt equals the CLI receipt and repeats
  identically, and negative tests for a missing argument, an unknown argument,
  an unadmitted schema and a symlinked input.
- Document the tool in the convergence and MCP references and record the change
  in the CHANGELOG.

## Impact

- **Files**: `crates/cairn-cli/src/convergence.rs`,
  `crates/cairn-cli/src/mcp/`, `cairn-policy/mcp-discovery.ncl` and its
  generated export, `crates/cairn-cli/tests/convergence.rs`, `docs/` and
  `CHANGELOG.md`.
- **Assurance**: advisory only. The tool grants no gate, readiness, sync,
  archive or release authority, and it changes no existing receipt bytes.
- **Compatibility**: additive. The CLI command, its flags, exit codes and
  printed receipts keep their bytes; existing catalog profiles keep their
  tools, and the new tool joins the read-only inspect profiles.
