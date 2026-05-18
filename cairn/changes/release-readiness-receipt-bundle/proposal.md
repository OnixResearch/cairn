## Why

Cairn now has deterministic validation receipts, agent-run receipt validation, strict MCP schemas, and an MCP agent smoke receipt. Operators still need to run several commands and manually correlate hashes before claiming a checkout is release-ready. That makes release evidence easy to omit, hard to compare, and difficult for agents to cite without overclaiming.

A maintained release-readiness receipt bundle should compose the existing local checks into one deterministic operator artifact. The bundle must not replace full CI or mutate lifecycle content; it should make the already-required proof surfaces easy to run and cite.

## What Changes

- Add a public `release-readiness` receipt bundle surface that runs or records the maintained local readiness checks for a Cairn checkout.
- Include stable evidence for Cairn validation, Tracey coverage, MCP agent smoke receipts, and repository verification commands.
- Emit a deterministic JSON receipt with per-check verdicts, stable hash fields, command metadata, and a top-level `receipt_hash`.
- Document the operator flow for creating and citing the bundle before releases or agent completion claims.

## Impact

- **Files**: CLI receipt surface, docs, focused tests, and accepted Cairn supersession spec after sync/archive.
- **Testing**: focused bundle tests, existing MCP smoke test, Cairn validation, Tracey coverage, workspace checks, and flake checks.
- **Compatibility**: additive command/receipt surface; no existing receipt fields are removed.
- **Risk**: low implementation blast radius if the bundle shells together existing read-only/local checks and reports failures without hiding underlying receipts.
