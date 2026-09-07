# Keep agent guidance current

## Why

The source audit found installed guidance that still describes `cairn/`, while current Cairn requires `.cairn/`.
Cairn already generates project-local workflows and detects stale or modified generated files.
The next improvement is a small, source-bound entry guide that leads agents to exact current instructions.

## What Changes

- Add a compact entry-guide representation to the existing generated workflow catalog.
- Derive layout names, supported profiles, commands, and stage instruction routes from checked versioned contracts.
- Bind the guide to policy, generator, adapter, and command-catalog identities.
- Extend existing freshness checks and preview-first installation to the entry guide.
- Add contract fixtures for documentation examples and a report-only path for stale copied guidance.

## Impact

Current consumers: project-local Agent Skills, Claude, and Codex adapters maintained by Cairn.
Immediate outcome: agents load a small current guide instead of a large copied command manual.
Durable contribution: one maintained generation and freshness path for entry and stage guidance.
Maintenance owner: Cairn generated-workflow and CLI maintainers.

Reuse `agent workflow list`, `show`, `install`, and `check` with existing manifests and no-overwrite behavior.
Expected changes include generated workflow policy, adapter templates, source workflow documentation, and example fixtures.
The agent host still chooses which instructions it loads.

## Repeatability evidence

Render all supported adapters from the same admitted catalog and compare semantic guidance.
Cover old layouts, unknown profiles, stale command identities, unsupported adapters, modified files, and interrupted installation.
Measure the actual entry and selected stage content exposed by a declared host adapter.
Do not claim token savings from a smaller file that the host never loads.

## Out of Scope

- Automatic user-home installation, ambient instruction discovery, or rewriting unrelated AGENTS.md files
- New agent authority, executable hooks, or evidence claims from generated text
- Silent replacement of modified or unmanaged instructions

Dependency: `adopt-readiness-in-agent-workflows`. Implementation remains open.
