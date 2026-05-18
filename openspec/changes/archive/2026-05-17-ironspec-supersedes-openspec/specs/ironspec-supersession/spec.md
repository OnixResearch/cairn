## ADDED Requirements

### Requirement: Cairn repository initialization
r[cairn.supersede.init] The system MUST initialize an OpenSpec-compatible repository layout without requiring the upstream OpenSpec CLI.

#### Scenario: Initialize empty repository
r[cairn.supersede.init.empty]
- GIVEN a writable project root without an `openspec/` directory
- WHEN the operator runs `cairn init --root <path>`
- THEN the system creates `openspec/specs`, `openspec/changes`, and `openspec/changes/archive`
- AND the JSON receipt reports `mutated=true` and the created paths

#### Scenario: Initialize existing repository idempotently
r[cairn.supersede.init.idempotent]
- GIVEN a project root with the compatible directories already present
- WHEN the operator runs `cairn init --root <path>`
- THEN no existing artifacts are overwritten
- AND the JSON receipt reports existing paths and `mutated=false`

### Requirement: Cairn change creation
r[cairn.supersede.change_create] The system MUST create a named active change skeleton using the OpenSpec-compatible layout.

#### Scenario: Create change skeleton
r[cairn.supersede.change_create.skeleton]
- GIVEN an initialized repository
- WHEN the operator runs `cairn change create <name> --root <path>`
- THEN the system creates `openspec/changes/<name>/proposal.md`, `design.md`, `tasks.md`, and `specs/`
- AND the JSON receipt reports `mutated=true` and the created artifacts

#### Scenario: Reject invalid change names
r[cairn.supersede.change_create.invalid_name]
- GIVEN a change name containing path separators, traversal, or unsupported characters
- WHEN the operator runs `cairn change create <name>`
- THEN the system refuses to create files
- AND reports the validation error

### Requirement: Supersession boundary
r[cairn.supersede.boundary] The system MUST keep lifecycle templates and validation decisions in the pure core while filesystem mutation remains in the CLI shell.

#### Scenario: Pure lifecycle core
r[cairn.supersede.boundary.pure_core]
- GIVEN initialization and change creation support
- WHEN Slotcar verifies the pure core boundary
- THEN the core remains free of filesystem, environment, process, and presentation effects
