## MODIFIED Requirements

### Requirement: Explicit old-directory migration
r[cairn.lifecycle_directory.legacy_migration] The system MUST provide a deterministic preview-first migration from old `cairn/` lifecycle content to canonical `.cairn/` content.

#### Scenario: Preview old-directory migration
r[cairn.lifecycle_directory.legacy_migration.preview]
- GIVEN a repository with old `cairn/` lifecycle content and no conflicting `.cairn/` destination files
- WHEN the operator runs `cairn migrate cairn --root <path>`
- THEN the CLI reports exact copy actions from `cairn/` to `.cairn/`
- AND the receipt includes a stable BLAKE3 `plan_hash`
- AND no filesystem content changes

#### Scenario: Bind execution to the reviewed plan
r[cairn.lifecycle_directory.legacy_migration.plan_binding]
- GIVEN a reviewed old-directory migration preview and its `plan_hash`
- WHEN the operator runs `cairn migrate cairn --root <path> --execute --plan-hash <hash>`
- THEN the system recomputes the plan from current source, destination, and policy facts
- AND mutation is eligible only when the recomputed hash equals the supplied hash

#### Scenario: Reject execution without a plan hash
r[cairn.lifecycle_directory.legacy_migration.plan_binding.execute]
- GIVEN old `cairn/` content that is eligible for migration
- WHEN the operator runs migration execution without `--plan-hash`
- THEN the command reports a missing-plan-hash error
- AND no filesystem content changes

#### Scenario: Reject stale source facts
r[cairn.lifecycle_directory.legacy_migration.plan_binding.stale]
- GIVEN a reviewed migration plan and source content that changed after preview
- WHEN the operator executes migration with the reviewed `plan_hash`
- THEN the command reports a plan-hash mismatch
- AND no destination file is created

#### Scenario: Publish without destination replacement
r[cairn.lifecycle_directory.legacy_migration.no_replace]
- GIVEN a matching migration plan and destinations that were absent during preview
- WHEN a destination appears before publication
- THEN the command reports a destination conflict
- AND it does not replace the destination content

#### Scenario: Execute old-directory migration
r[cairn.lifecycle_directory.legacy_migration.execute]
- GIVEN a matching reviewed old-directory migration plan without destination conflicts
- WHEN the operator executes migration with its `plan_hash`
- THEN the system copies each planned file into `.cairn/`
- AND it leaves the source content unchanged

#### Scenario: Reject a conflicting destination
r[cairn.lifecycle_directory.legacy_migration.conflict]
- GIVEN old `cairn/` content and an existing `.cairn/` destination file
- WHEN the operator previews or executes migration
- THEN the migration reports a blocked result
- AND it does not overwrite the destination file

## ADDED Requirements

### Requirement: Legacy-only lifecycle diagnostic
r[cairn.lifecycle_directory.legacy_diagnostic] The system MUST fail closed when a normal lifecycle command selects a root that has old `cairn/` lifecycle content but no native `.cairn/` lifecycle content.

#### Scenario: Reject a hidden legacy lifecycle
r[cairn.lifecycle_directory.legacy_diagnostic.normal_command]
- GIVEN a repository with `cairn/specs`, `cairn/changes`, or `cairn/archive` and no native lifecycle subdirectory
- WHEN the operator runs a normal lifecycle command
- THEN the command reports that explicit migration is required
- AND it does not report an empty native lifecycle result

#### Scenario: Preserve migration command access
r[cairn.lifecycle_directory.legacy_diagnostic.migrate]
- GIVEN a repository with only old `cairn/` lifecycle content
- WHEN the operator runs `cairn migrate cairn --root <path>`
- THEN the migration command remains available
- AND it reads old content only as explicit migration input
