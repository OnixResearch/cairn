# Add Preserves corpus CI guard

## Why

Preserves is now an internal canonicalization seam with checked-in golden corpus hashes and an explicit update rail. The remaining drift risk is that a future Preserves dependency bump changes packed canonical bytes while CI only notices if a developer runs the corpus update command manually.

## What Changes

- Add a first-class Nix/CI check that runs the Preserves corpus update rail in dry-run mode against the checked-in corpus.
- Add a negative CI check that proves stale corpus hashes fail closed instead of silently passing.
- Keep Preserves internal/additive; no public or stored receipt schema changes.

## Impact

- **Files**: `flake.nix`, native Cairn determinism spec/archive files.
- **Testing**: focused Nix checks, Cairn validate/gates, Tracey coverage, and existing Preserves corpus tests.
