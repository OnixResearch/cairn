## Why

`crates/cairn-cli/src/steel_orchestration.rs` is one of the largest remaining CLI hotspots after the post-release maintainability inventory. It currently mixes command dispatch, policy/profile selection, script-envelope evaluation, promotion evidence checks, receipt rendering, and unit tests in a single review surface.

## What Changes

- Split Steel orchestration into a directory module with narrow boundaries for dispatch, profile defaults, script evaluation, and receipt preparation.
- Preserve the existing `steel-orchestration receipt` and `steel-orchestration default-gate` JSON behavior.
- Add Tracey-bound regression coverage proving the split is behavior-preserving and keeps host effects disabled.

## Impact

- **Files**: `crates/cairn-cli/src/steel_orchestration/`, lifecycle-orchestration specs, focused CLI tests.
- **Testing**: focused Steel CLI tests, `cairn validate`, `cairn tracey coverage`, and final repo gate.
