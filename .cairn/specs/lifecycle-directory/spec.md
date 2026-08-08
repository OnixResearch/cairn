# Lifecycle Directory Specification

## Purpose

Defines the `lifecycle-directory` capability.

## Requirements

### Requirement: Hidden canonical lifecycle directory
r[cairn.lifecycle_directory.hidden_default] The system MUST use `.cairn/` as the canonical lifecycle directory for initialization and normal runtime commands.

#### Scenario: Initialize the hidden lifecycle directory
r[cairn.lifecycle_directory.hidden_default.init]
- GIVEN a writable project root without a `.cairn/` directory
- WHEN the operator runs `cairn init --root <path>`
- THEN the system creates `.cairn/specs`, `.cairn/changes`, and `.cairn/archive`
- AND the receipt identifies the stable `cairn` layout

#### Scenario: Use only the hidden canonical directory
r[cairn.lifecycle_directory.hidden_default.runtime]
- GIVEN a repository that contains `.cairn/` lifecycle content and old `cairn/` lifecycle content
- WHEN an operator runs a normal read, gate, sync, archive, collaboration, or lifecycle-store command
- THEN the command uses `.cairn/` lifecycle content
- AND it does not read old `cairn/` content as an implicit fallback

#### Scenario: Keep project pointers distinct from lifecycle content
r[cairn.lifecycle_directory.hidden_default.project_pointer]
- GIVEN `.cairn/` contains lifecycle-project pointer files but no lifecycle subdirectories
- WHEN an operator runs a normal lifecycle read command from the project
- THEN the system does not classify the pointer-only directory as a native lifecycle root
- AND the declared lifecycle-store pointer remains eligible for root selection

### Requirement: Explicit old-directory migration
r[cairn.lifecycle_directory.legacy_migration] The system MUST provide a deterministic preview-first migration from old `cairn/` lifecycle content to canonical `.cairn/` content.

#### Scenario: Preview old-directory migration
r[cairn.lifecycle_directory.legacy_migration.preview]
- GIVEN a repository with old `cairn/` lifecycle content and no conflicting `.cairn/` destination files
- WHEN the operator runs `cairn migrate cairn --root <path>`
- THEN the CLI reports exact copy actions from `cairn/` to `.cairn/`
- AND no filesystem content changes

#### Scenario: Execute old-directory migration
r[cairn.lifecycle_directory.legacy_migration.execute]
- GIVEN a reviewed old-directory migration plan without destination conflicts
- WHEN the operator runs `cairn migrate cairn --root <path> --execute`
- THEN the system copies each planned file into `.cairn/`
- AND it leaves the source content unchanged

#### Scenario: Reject a conflicting destination
r[cairn.lifecycle_directory.legacy_migration.conflict]
- GIVEN old `cairn/` content and an existing `.cairn/` destination file
- WHEN the operator runs `cairn migrate cairn --root <path> --execute`
- THEN the migration reports a blocked result
- AND it does not overwrite the destination file

### Requirement: Native layout receipt compatibility
r[cairn.lifecycle_directory.receipt_compatibility] The system MUST preserve `cairn` as the native layout receipt value after it changes the canonical directory path.

#### Scenario: Report stable layout identity with hidden paths
r[cairn.lifecycle_directory.receipt_compatibility.paths]
- GIVEN a command reads or writes canonical `.cairn/` lifecycle content
- WHEN the command emits a deterministic receipt
- THEN its native layout field remains `cairn`
- AND all reported lifecycle paths use `.cairn/`
