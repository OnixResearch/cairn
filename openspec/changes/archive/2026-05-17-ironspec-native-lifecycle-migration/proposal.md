# Proposal: Cairn Native Lifecycle Migration

## Why

`cairn` should stop treating upstream OpenSpec compatibility as the product boundary. The native lifecycle should live under an `cairn/` layout, with OpenSpec retained as a legacy import format and migration source. This lets future behavior evolve around Cairn's FCIS, deterministic receipts, policy, and validation model without chasing upstream CLI parity.

## What Changes

- Add a native `cairn/` repository layout for specs, active changes, and archives.
- Prefer native layout detection for status, read, validate, sync, and archive operations while preserving legacy OpenSpec fixture support during migration.
- Add `cairn migrate openspec --root PATH` as a dry-run planning command.
- Add `--execute` for the migration command to copy legacy `openspec/` content into native `cairn/` layout.
- Keep migration receipts deterministic and explicit about source, destination, actions, conflicts, and mutation.

## Out of Scope

- Removing legacy `openspec/` support entirely.
- Renaming the repository's own OpenSpec change-management directory in this slice.
- Changing policy format or adding interactive prompts.
- Deleting legacy files during migration.

## Verification

- Native layout fixtures validate and support read/status commands.
- Migration dry-run emits stable JSON.
- Migration `--execute` copies legacy content into native layout in a temp fixture.
- Full Rust, Slotcar, Cairn validation, and Nix flake gates pass.
