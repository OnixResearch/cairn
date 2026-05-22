## Why

Cairn has already superseded OpenSpec for normal development, but this repository still carries a top-level `openspec/` lifecycle tree and OpenSpec-named local gate artifacts. That keeps `cairn doctor --root .` in a mixed-layout warning state and makes the product identity look partially unmigrated.

## What Changes

- Remove the checked-in top-level legacy `openspec/` lifecycle tree from this repository after its accepted/archive content has native Cairn counterparts.
- Rename repo-local flake/check/docs examples that describe Cairn itself with OpenSpec-primary names.
- Preserve explicit legacy import compatibility (`migrate openspec`) for external repositories and fixtures.

## Impact

- **Files**: `openspec/` removals, repo docs, flake check naming, and native Cairn accepted spec coverage.
- **Testing**: doctor/validate must report the repository as native Cairn without mixed-layout warnings; focused Rust tests and diff checks cover the cleanup.
