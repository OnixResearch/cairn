# Adopt readiness in agent workflows

## Why

Unified readiness already composes artifact, gate, completion, sync, and archive observations.
At `cb0be66e0e992c0f4070956bb9d451d8a8fe7573`, `mcp_drain_plan()` still constructs a separate gate and plan summary.
Generated workflow descriptors also direct agents through older diagnostic sequences.
This separation creates repeated calls and different interpretations of completion evidence.

## What Changes

- Add a versioned readiness-backed drain view while preserving the existing drain receipt contract.
- Supply explicit per-change completion and cross-repository evidence selectors to shared readiness observation.
- Derive next actions from existing dependency order, task order, and readiness causes.
- Update generated workflow descriptors to use readiness for diagnosis and exact artifact instructions for authoring.
- Keep execution commands, completion validators, and mutation preflight authoritative.

## Impact

Current consumers: Cairn drain operators and generated apply, verify, drain, and archive workflows.
Immediate outcome: agents use one readiness decision model instead of reconciling separate summaries.
Durable contribution: shared consumer behavior with a maintained compatibility matrix.
Maintenance owner: Cairn lifecycle-core maintainers, with CLI/MCP and generated-workflow maintainers.

Expected surfaces include `crates/cairn-cli/src/mcp/read_tools.rs`, readiness adapters, generated workflow policy, dependency and task projections, and tests.
Reuse existing dependency graphs and `ready_tasks`. Do not add a scheduler or execution engine.

## Repeatability evidence

Compare direct validators, legacy drain, and the new view across explicit and compatibility workflow profiles.
Cover missing completion rows, stale subjects, malformed evidence declarations, blocked dependencies, cycles, and unavailable observations.
Keep successful stage observations when unrelated observations fail.
Measure diagnostic calls and tokens for a complete selected-change loop.

## Out of Scope

- Redefining task completion as checked boxes or model testimony
- Inferring evidence paths, actor authority, or cross-repository facts
- Running suggested actions, automatically archiving, or changing legacy receipt fields
- Extending Cairn into a workflow scheduler

Dependency: `add-compact-agent-readiness`. Implementation remains open.
