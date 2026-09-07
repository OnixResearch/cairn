# Measure agent lifecycle token cost

## Why

The audit at `cb0be66e0e992c0f4070956bb9d451d8a8fe7573` found avoidable output and repeated diagnostic work.
No end-to-end token benchmark established the size of either cost.
Response bytes and command counts cannot establish tokens per correctly completed change.

## What Changes

- Define typed Nickel contracts for scenarios, run observations, measurement provenance, and paired comparisons.
- Add pure admission and aggregation over explicitly supplied agent-run observations.
- Add bounded offline CLI import and report commands with versioned JSON output.
- Separate uncached input, cached input, output, and reported reasoning tokens without double counting.
- Keep correct completion, correct blocked behavior, false completion, failure, and incomplete runs separate.
- Retain baseline scenarios for the other agent-efficiency changes before their implementation.

## Impact

Current consumers: Cairn CLI/MCP agents and maintainers of the planned agent-efficiency changes.
Immediate outcome: a comparison identifies whether lower token use preserves the declared task outcome.
Durable contribution: a repeatable, repository-owned regression benchmark with explicit measurement limits.
Maintenance owner: Cairn verification maintainers, with CLI maintainers for bounded input and output adapters.

Expected surfaces are `contracts/`, `cairn-policy/`, `crates/cairn-core/src/verified/`, CLI adapters, fixtures, and benchmark documentation.
Reuse the existing test-harness cases, completion validators, receipt identities, and determinism comparison patterns.
Do not infer behavioral correctness from Cairn lifecycle gates alone.

## Repeatability evidence

Use fresh disposable fixture roots and repository-owned outcome oracles.
Compare the same declared model, tokenizer or usage source, agent configuration, tool catalog, fixture, and completion criteria.
Retain raw observations under explicit local authority and publish bounded redacted summaries.
Include successful and correctly blocked runs, plus fabricated success, unavailable usage, mixed cohorts, and truncated traces.

## Out of Scope

- Model execution, provider authentication, transcript collection from undeclared locations, or user-global metrics
- Estimated token counts derived from character or byte ratios
- Automatic optimization, a scheduler, or a new agent framework
- General productivity or code-correctness claims from one benchmark fixture

This is the prerequisite measurement package in `docs/agent-efficiency-roadmap.md`. Implementation remains open.
