## Why

Cairn currently validates only structures it happens to find. Empty accepted specs and empty delta specs therefore produce no EARS diagnostics. Gate artifact checks use file-existence booleans, so empty proposal, design, and tasks files plus an empty delta spec pass proposal, design, and tasks gates. A zero-task change is also treated as complete by archive planning because it has no todo, in-progress, or unmarked tasks.

This makes a wholly vacuous lifecycle package appear valid and gate-ready. Malformed dependency tokens such as an empty `[depends:]` payload can similarly bypass task-to-requirement linkage because the current check only searches for a configured prefix.

## What Changes

- Add policy-owned minimum substance rules for required lifecycle artifacts and reject empty, whitespace-only, generated-placeholder-only, or structurally vacuous content. r[cairn.lifecycle_validation.artifact_substance]
- Require accepted and delta spec inputs to contain at least one well-formed requirement block with a canonical bare requirement ID and a normative statement in the requirement body. r[cairn.lifecycle_validation.spec_substance]
- Require task artifacts to contain at least one substantive checkbox task and make zero-task or placeholder-only changes non-archive-ready. r[cairn.lifecycle_validation.task_substance]
- Treat unchecked-count reduction, checked boxes, and task-file mutation as progress observations only; they cannot override required gate, probe, dependency, or receipt blockers. r[cairn.lifecycle_validation.task_progress_non_authority]
- Parse dependency task markers as structured values and reject empty, unclosed, malformed, or unsupported payloads instead of treating a prefix match as an exemption. r[cairn.lifecycle_validation.dependency_marker_shape]
- Apply the same non-vacuity diagnostics to validate, stage gates, sync/archive preflight, and MCP lifecycle surfaces. r[cairn.lifecycle_validation.mutation_block]
- Add positive and negative fixtures for substantive and vacuous lifecycle packages. r[cairn.lifecycle_validation.tests]

## Impact

- **Files**: Nickel policy/contracts, pure lifecycle/spec/task validation in `cairn-core`, artifact snapshot and receipt adapters in `cairn-cli`, fixtures, templates, and operator docs.
- **Testing**: empty/whitespace/placeholder artifacts, zero requirements, weak requirement bodies, zero or placeholder tasks, malformed dependency markers, task-progress-with-blockers, stage-specific gate failures, and mutation no-op regressions.
