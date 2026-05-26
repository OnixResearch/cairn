# Design: Preserves corpus CI guard

## Scope

This change adds CI-visible guardrails around the existing `cairn preserves-corpus update --path fixtures/preserves-canonicalization/corpus.json` command.

## Check shape

Add two Nix checks:

1. `preserves-corpus-fresh`
   - copies the flake source to a writable scratch tree,
   - runs `cairn preserves-corpus update --path fixtures/preserves-canonicalization/corpus.json`,
   - expects exit 0 for a clean corpus.

2. `preserves-corpus-stale-detects-drift`
   - copies the flake source to a writable scratch tree,
   - mutates the first known checked-in `expected_hash` to an all-zero hash,
   - runs the same dry-run command,
   - expects nonzero exit and a receipt containing stale evidence.

## Non-goals

- No automatic `--execute` in CI.
- No public receipt schema replacement.
- No Preserves DTO redesign.
- No network or credential access.

## Determinism

The guard delegates hashing to the existing CLI update rail, which delegates hash computation to `cairn_core::canonical_preserves_hash`. CI therefore checks the same canonical seam and corpus path used by developers/operators.
