## Why

Competitors reduce friction by turning an operator request into a complete proposal/design/tasks/spec scaffold. Cairn currently has strong gates once a package exists, but agents still hand-author the initial package shape, which creates avoidable task-reference and requirement-ID mistakes.

## What Changes

- Add a prompt/request-to-change scaffolding command that creates a reviewed draft package from structured operator input.
- Keep generation deterministic by requiring explicit input text or input files and recording input hashes.
- Generate requirement IDs, proposal/design/task skeletons, and delta specs that pass Cairn structural validation before implementation begins.

## Impact

- **Files**: CLI scaffold command, pure core draft planner, policy schema for scaffold defaults, fixtures, tests, docs.
- **Testing**: Stable scaffold golden tests, invalid-input tests, task-reference gate tests, and no-network/no-LLM proof tests.
