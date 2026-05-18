## MODIFIED Requirements

### Requirement: Cairn repository initialization
r[cairn.supersede.init] The system MUST initialize the native `cairn/` repository layout without requiring the upstream OpenSpec CLI while retaining legacy `openspec/` fallback for existing repositories.

#### Scenario: Initialize empty repository
r[cairn.supersede.init.empty]
- GIVEN a writable project root without an `cairn/` directory
- WHEN the operator runs `cairn init --root <path>`
- THEN the system creates `cairn/specs`, `cairn/changes`, and `cairn/archive`
- AND the JSON receipt reports `mutated=true`, the selected `cairn` layout, and the created paths

#### Scenario: Initialize existing repository idempotently
r[cairn.supersede.init.idempotent]
- GIVEN a project root with the compatible native directories already present
- WHEN the operator runs `cairn init --root <path>`
- THEN no existing artifacts are overwritten
- AND the JSON receipt reports existing paths, the selected `cairn` layout, and `mutated=false`

### Requirement: Cairn change creation
r[cairn.supersede.change_create] The system MUST create a named active change skeleton in the selected native `cairn/` layout by default.

#### Scenario: Create change skeleton
r[cairn.supersede.change_create.skeleton]
- GIVEN an initialized native repository
- WHEN the operator runs `cairn change create <name> --root <path>`
- THEN the system creates `cairn/changes/<name>/proposal.md`, `design.md`, `tasks.md`, and `specs/`
- AND the JSON receipt reports `mutated=true`, the selected `cairn` layout, and the created artifacts

#### Scenario: Reject invalid change names
r[cairn.supersede.change_create.invalid_name]
- GIVEN a change name containing path separators, traversal, or unsupported characters
- WHEN the operator runs `cairn change create <name>`
- THEN the system refuses to create files
- AND reports the validation error
