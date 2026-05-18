## Phase 1: Public validation surface

- [x] [serial] Add a read-only CLI command that validates one bounded agent-run receipt JSON file and emits a deterministic Cairn validation receipt. r[cairn.supersede.agent_receipt_validation_surface.cli]
  - Evidence 2026-05-18T17:23:03Z: `nix develop -c bash -lc 'cargo fmt && cargo test -p cairn-cli agent_receipt -- --nocapture && cargo test -p cairn-cli mcp_agent_receipt -- --nocapture && cargo clippy -p cairn-cli --all-targets -- -D warnings'` passed.
- [x] [serial] Include stable public fields for command, input path/content identity, gate verdict, issues, agent receipt hash, and top-level receipt hash. r[cairn.supersede.agent_receipt_validation_surface.cli.schema]
  - Evidence 2026-05-18T17:23:03Z: CLI fixture replay asserts stable keys and reproducible hash fields for positive and negative receipts.

## Phase 2: MCP dogfood tool

- [x] [serial] Add a read-only MCP tool for agent receipt validation using the same validation core as the CLI path. r[cairn.supersede.agent_receipt_validation_surface.mcp]
  - Evidence 2026-05-18T17:23:03Z: `mcp_agent_receipt_validation_is_read_only_and_bounded` passed for catalog, path, and inline validation calls.
- [x] [parallel] Prove the MCP validation tool is listed in the default read-only catalog and rejects path traversal or oversized inline input. r[cairn.supersede.agent_receipt_validation_surface.mcp.read_only]
  - Evidence 2026-05-18T17:23:03Z: focused MCP test passed and asserts read-only/non-destructive annotations plus traversal rejection; CLI/MCP implementation enforces 64 KiB receipt input bounds.

## Phase 3: Fixtures, docs, and gates

- [x] [parallel] Add positive and negative source-controlled dogfood fixtures for byte-stable pass/fail validation. r[cairn.supersede.agent_receipt_validation_surface.fixtures]
  - Evidence 2026-05-18T17:23:03Z: `fixtures/agent-receipts/positive.json` and `fixtures/agent-receipts/missing-verification.json` replay through the public CLI surface with deterministic verdicts.
- [x] [parallel] Document the wrapper loop: plan, run, emit receipt, validate through Cairn, cite the validation receipt. r[cairn.supersede.agent_receipt_validation_surface.docs]
  - Evidence 2026-05-18T17:23:03Z: updated `docs/agent-run-receipts.md`, `docs/cairn-mcp.md`, and `README.md` with validation surface guidance.
- [x] [serial] Run focused CLI/MCP tests, formatting, workspace tests, Clippy, Slotcar, Tracey coverage, `cairn validate --root .`, and `nix flake check`. r[cairn.supersede.agent_receipt_validation_surface.verification]
  - Evidence 2026-05-18T17:23:03Z: focused CLI/MCP tests, formatting, and `cargo clippy -p cairn-cli --all-targets -- -D warnings` passed. Full post-archive verification will be rerun after accepted spec promotion.
