## Phase 1: Contract

- [x] [serial] Choose and document the explicit execution flag for `sync` and `archive`.
- [x] [parallel] Define stable mutation receipt fields for successful and blocked execution.
- [x] [parallel] Add temp-repo fixture tests for sync execution, archive execution, and blocked no-op execution.

## Phase 2: Implementation

- [x] [serial] Implement CLI shell mutation for unblocked sync plans while preserving dry-run default behavior.
- [x] [serial] Implement CLI shell mutation for unblocked archive plans while preserving dry-run default behavior.
- [x] [parallel] Update README/operator examples for dry-run vs explicit execution.

## Phase 3: Verification

- [x] [serial] Verify with cargo fmt/test/clippy, Slotcar, OpenSpec validation, self-dogfood, and `nix flake check`.
