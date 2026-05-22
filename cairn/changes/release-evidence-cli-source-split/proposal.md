## Why

`crates/cairn-cli/src/release_evidence.rs` has grown into the largest current CLI source hotspot after the current-HEAD index, publish materialization, and retention/rotation slices landed. The file now mixes dispatcher code, index generation, publish materialization, retention planning, rotation execution, shared path/hash helpers, and unit tests in one review surface.

This increases the chance that future release-evidence changes accidentally couple independent rails or broaden helper visibility. A behavior-preserving module split is the highest-ROI next step because it reduces review and merge risk before adding more release-evidence capabilities.

## What Changes

- Split the release-evidence CLI implementation into focused source modules while preserving the existing public `cairn release-evidence ...` command behavior.
- Keep the dispatcher/facade small and move cohesive families into named modules, likely:
  - current-HEAD index generation,
  - publish materialization,
  - retention planning,
  - rotation execution,
  - shared local helpers used by those families.
- Preserve narrow bridge APIs only where another release-evidence module needs them.
- Preserve existing receipts, issue codes, policy schema, docs, tests, and CLI usage behavior unless a later implementation discovers a small bug that must be fixed with focused regression coverage.

## Impact

- **Files**: `crates/cairn-cli/src/release_evidence.rs` and sibling `release_evidence/` modules; possibly no policy/doc changes if behavior is strictly preserved.
- **Testing**: focused `cargo test -p cairn-cli release_evidence -- --nocapture`, full workspace tests, clippy, `cairn validate`, Tracey coverage, and `git diff --check`.
