# Agent efficiency changes

## Goal and status

Reduce tokens per correctly completed Cairn change without weaker gates, hidden blockers, or new mutation authority.
These packages contain plans only. Their implementation tasks remain open.

The baseline source revision is `cb0be66e0e992c0f4070956bb9d451d8a8fe7573`.
The initial source audit found duplicate MCP receipt serialization, a separate drain model, and stale installed guidance.
Cairn already provides readiness, machine JSON, scoped guidance, task identities, and generated workflow installation.
This work extends those components instead of introducing another lifecycle engine.

## Packages and order

| Change | Owned outcome | Required predecessor |
| --- | --- | --- |
| `measure-agent-lifecycle-token-cost` | Reproducible accounting and correctness oracles for agent runs | None |
| `reduce-duplicate-mcp-receipts` | One complete payload per selected MCP representation | Measurement |
| `add-compact-agent-readiness` | A bounded machine projection with exact actions and detail retrieval | Measurement |
| `adopt-readiness-in-agent-workflows` | Drain and workflow consumers use shared readiness decisions | Compact readiness |
| `add-targeted-lifecycle-context-reads` | Exact requirement, task, and section retrieval | Measurement |
| `add-incremental-agent-observations` | Source-bound pages, unchanged responses, and observation differences | Targeted reads and compact readiness |
| `keep-agent-guidance-current` | Small project-local entry guidance with freshness checks | Readiness adoption |
| `add-task-scoped-mcp-discovery` | Strict task-specific catalogs and selective schema lookup | Measurement |

The native `metadata.json` dependency graph owns execution order. The labels in this table are explanatory.
The first implementation wave establishes measurement, then removes duplicate payloads and adds compact readiness.
The remaining changes can proceed after their declared predecessors complete.

## Shared acceptance rules

- Retain baseline receipts before core changes and current-source receipts after them.
- Keep existing full command receipts, hashes, and text-only compatibility paths unless an explicit new representation is selected.
- Keep transport success separate from lifecycle state, report validity, and mutation authority.
- Preserve negative outcomes for missing evidence, stale sources, unsafe paths, invalid inputs, and denied mutation.
- Keep complete diagnostics available through identity-bound retrieval. Report omitted counts and continuation state explicitly.
- Do not treat cached observations, task checkboxes, summaries, or model statements as completion authority.
- Keep pure decisions and identity calculations in the core. Keep file access, output, cache storage, and process effects in the shell.
- Reuse Cairn parsers, readiness, command schemas, generated policy, and BLAKE3 contracts. Review published components before adding another dependency.
- Run the existing strict repository checks. A pre-existing failure is a blocker or an explicit scoped limitation, not permission to weaken a gate.

## Measurement contract

The benchmark change owns the common scenario manifest, accounting schema, paired-run admission, and deterministic summaries.
Each consumer change adds its own positive and negative fixtures and records a paired comparison through that contract.

The fixture set covers successful authoring, invalid artifacts, missing execution evidence, stale receipts, interrupted work, and denied mutation.
Repository-owned fixture oracles decide correct completion or correct blocked behavior. Cairn structural gates alone do not decide implementation correctness.
Reports keep successful, correctly blocked, falsely completed, failed, and incomplete runs separate.

Token accounting distinguishes uncached input, cached input, output, and provider-reported reasoning tokens when available.
It also records tool calls, repeated reads, retries, and elapsed-time observations.
Byte counts remain byte counts. Missing tokenizer or provider usage data produces an unavailable token result.

A comparison binds model, tokenizer or usage source, agent configuration, tool catalog, source, policy, fixture, and completion criteria.
Controlled deterministic replays measure representation cost. They do not establish end-to-end agent success.
Live agent comparisons require explicit host authorization and a declared run budget outside Cairn's pure benchmark core.
No package may claim a general token reduction from one fixture or a smaller response alone.

## Planning and completion evidence

Planning evidence belongs in each package's `evidence/planning/` directory.
The [planning evidence index](agent-efficiency-planning-evidence.md) records shared validation, exact commands, and scope limits.
Proposal, design, and tasks receipts establish planning conformance only.
Implementation evidence must come from fresh runs on the implemented source.

After implementation, retain focused tests, policy freshness, formatting, workspace tests, Clippy, lifecycle validation, traceability, and Nix checks.
Review semantic deltas before sync and archive. Neither action is part of this planning request.

## Owners and exclusions

Cairn lifecycle-core maintainers own lifecycle decisions and deterministic projections.
CLI/MCP maintainers own transport compatibility, bounded observation, and generated guidance adapters.
Cairn verification maintainers own scenario oracles and benchmark accounting.
Agent hosts retain model execution, credentials, provider quotas, and transcript capture.

This work does not add a scheduler, autonomous execution, a model provider, a new serialization language, or user-global installation.
It does not authorize automatic sync, archive, publication, or deletion.
