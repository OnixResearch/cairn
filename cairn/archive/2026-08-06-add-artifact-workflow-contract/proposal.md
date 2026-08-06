## Why

Cairn policy already declares artifact dependencies for the `spec-driven` lifecycle. The public status surface reports file presence and task counts, but agents must infer artifact readiness and the next safe action.

A deterministic artifact workflow contract will expose policy facts without weakening proposal, design, tasks, sync, or archive gates. It will also give future workflow profiles one stable CLI and MCP projection.

## What Changes

- Add a pure artifact-state model with `done`, `ready`, `blocked`, and `skipped` states.
- Add read-only `cairn workflow status CHANGE` output with dependencies, blockers, unlocks, and ordered next actions.
- Add read-only `cairn workflow instructions ARTIFACT --change CHANGE` output with the policy-owned output path, template, constraints, upstream identities, and non-claims.
- Add equivalent strict read-only MCP tools.
- Keep existing `status`, gate, sync, archive, and receipt compatibility surfaces unchanged.
- Add positive and negative fixtures for invalid graphs, malformed artifacts, unknown selectors, deterministic ordering, and non-mutation.

## Impact

- **Policy**: Extend the generated policy projection with validated workflow contract fields and stable bounds.
- **Core**: Add pure graph admission, artifact-state derivation, next-action selection, and instruction-envelope construction.
- **CLI and MCP**: Add two read-only commands and two strict read-only tools.
- **Documentation**: Add an operator and agent guide for artifact readiness and instruction envelopes.
- **Testing**: Add positive and negative core, CLI, MCP, determinism, compatibility, and non-claim coverage.

## Out of Scope

- Creating, editing, or deleting lifecycle artifacts.
- Replacing proposal, design, tasks, sync, or archive gates.
- Judging semantic quality, implementation correctness, or release eligibility.
- Loading user-global workflow schemas or evaluating Nickel at runtime.
- Injecting project guidance or generating tool-specific agent skills.
