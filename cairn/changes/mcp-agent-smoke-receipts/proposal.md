## Why

Cairn now exposes both an MCP tool surface and an agent-run receipt validation surface, but operators do not yet have a single maintained smoke workflow that proves those two surfaces compose into release-grade evidence. That gap makes MCP regressions harder to detect: a tool catalog can look valid, receipt validation can pass in isolation, and malformed tool calls can still drift without one small end-to-end operator receipt scenario tying the contract together.

## What Changes

- Specify a maintained MCP/agent smoke receipt flow that exercises `cairn mcp --describe`, `cairn_agent_receipt_validate`, and one deterministic invalid MCP call.
- Require the smoke flow to emit or assert stable receipt fields that operators can use as release evidence without scraping prose logs.
- Keep the flow read-only by default: it must not require `--allow-mutations`, must not mutate lifecycle content, and must fail if the invalid-call branch reaches lifecycle dispatch.

## Impact

- **Files**: Future implementation likely touches `crates/cairn-cli/tests/cli.rs`, `fixtures/agent-receipts/`, `docs/cairn-mcp.md`, and possibly a small smoke helper or golden fixture.
- **Testing**: Focused MCP smoke test, negative invalid-call assertion, `cairn validate --root .`, Tracey coverage, and maintained flake checks before archive.
