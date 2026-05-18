## Phase 1: Public validation surface

- [ ] [serial] Add a read-only CLI command that validates one bounded agent-run receipt JSON file and emits a deterministic Cairn validation receipt. r[cairn.supersede.agent_receipt_validation_surface.cli]
- [ ] [serial] Include stable public fields for command, input path/content identity, gate verdict, issues, agent receipt hash, and top-level receipt hash. r[cairn.supersede.agent_receipt_validation_surface.cli.schema]

## Phase 2: MCP dogfood tool

- [ ] [serial] Add a read-only MCP tool for agent receipt validation using the same validation core as the CLI path. r[cairn.supersede.agent_receipt_validation_surface.mcp]
- [ ] [parallel] Prove the MCP validation tool is listed in the default read-only catalog and rejects path traversal or oversized inline input. r[cairn.supersede.agent_receipt_validation_surface.mcp.read_only]

## Phase 3: Fixtures, docs, and gates

- [ ] [parallel] Add positive and negative source-controlled dogfood fixtures for byte-stable pass/fail validation. r[cairn.supersede.agent_receipt_validation_surface.fixtures]
- [ ] [parallel] Document the wrapper loop: plan, run, emit receipt, validate through Cairn, cite the validation receipt. r[cairn.supersede.agent_receipt_validation_surface.docs]
- [ ] [serial] Run focused CLI/MCP tests, formatting, workspace tests, Clippy, Slotcar, Tracey coverage, `cairn validate --root .`, and `nix flake check`. r[cairn.supersede.agent_receipt_validation_surface.verification]
