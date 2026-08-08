# Change: Canonical Cairn Hermes workflow skill

## Why

Cairn has superseded the old Ironspec/OpenSpec workflow language for normal repository work, but the live local Hermes skill is still named `ironspec-workflow` and the repository artifact is still stored as `docs/hermes/ironspec-workflow-skill.md` even though its content describes Cairn. Future agents should load a clearly named `cairn-workflow` skill for `/home/brittonr/git/cairn` without treating OpenSpec/Ironspec as the primary project identity.

## What Changes

- Rename the repository-maintained skill source to `docs/hermes/cairn-workflow-skill.md`.
- Keep the skill source copyable, audit-friendly, and aligned with Cairn-native lifecycle, MCP, receipt, release-readiness, Tracey, Slotcar, and Nix gates.
- Create/update the live Hermes `cairn-workflow` skill from the repo source.
- Redirect the older `ironspec-workflow` skill so Cairn work loads `cairn-workflow` instead.

## Out of Scope

- Removing legacy `ironspec` CLI aliases or explicit OpenSpec migration support.
- Changing Cairn runtime command behavior.
- Deleting historical archives that mention OpenSpec or Ironspec.
