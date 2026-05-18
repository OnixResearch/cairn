# Proposal: Create Cairn Hermes Skill

## Why

`cairn` now has enough project-specific workflow, FCIS constraints, verification rails, and supersession semantics that future Hermes sessions need a durable, repo-maintained skill source. Keeping the skill content only in local agent memory makes it harder to audit, update, and reuse across installs.

## What Changes

- Add a repository-maintained Hermes skill source for `cairn` workflow.
- Document how the skill maps `cairn` development to OpenSpec-first changes, FCIS boundaries, Slotcar checks, and Nix verification gates.
- Keep the skill source copyable into Hermes while avoiding secrets or machine-local paths beyond the repo path convention.

## Out of Scope

- Building automatic Hermes skill installation into the CLI.
- Changing Hermes Agent configuration or provider settings.
- Replacing the existing `cairn` command behavior.

## Verification

- `cairn validate --root .` succeeds with no `spec_issues`.
- Repository gates pass.
- The change is archived and committed with the skill artifact present.
