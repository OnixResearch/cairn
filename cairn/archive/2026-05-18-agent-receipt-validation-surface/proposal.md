# Add agent receipt validation surface

## Why

Cairn now has a pure agent-run receipt completion gate and a native MCP surface, but wrappers still need to call Rust APIs directly to validate a receipt. That leaves the most important determinism loop incomplete: an agent should be able to emit a receipt artifact, ask Cairn to validate it through CLI or MCP, and cite the resulting deterministic gate receipt before claiming completion. r[cairn.supersede.agent_receipt_validation_surface]

## What changes

- Add a public CLI validation surface for agent-run receipt JSON files that returns a deterministic Cairn receipt with validity, issues, and receipt hash evidence. r[cairn.supersede.agent_receipt_validation_surface.cli]
- Expose the same validation through read-only MCP tooling so an agent wrapper can gate itself without mutating lifecycle content. r[cairn.supersede.agent_receipt_validation_surface.mcp]
- Add positive and negative dogfood fixtures that prove byte-stable success and deterministic failure modes for missing evidence or bad entropy. r[cairn.supersede.agent_receipt_validation_surface.fixtures]
- Document a minimal wrapper flow: plan, run, produce receipt, validate through Cairn, and cite the gate receipt. r[cairn.supersede.agent_receipt_validation_surface.docs]

## Impact

- **Code**: likely touches `crates/cairn-cli/src/main.rs`, `crates/cairn-cli/tests/cli.rs`, fixture/golden files, and docs. Core changes should stay minimal because the pure gate already exists.
- **Behavior**: adds public read-only validation commands/tools; no default mutation capability.
- **Testing**: repeated validation of the same fixture must be byte-identical; negative fixtures must fail with stable issue codes; existing full workspace/Nix gates remain required.
