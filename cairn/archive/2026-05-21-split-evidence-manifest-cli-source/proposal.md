## Why

Evidence manifest generation, verification, artifact role validation, metadata hashing, and receipt rendering are currently colocated. Splitting keeps manifest semantics audit-friendly as registry bindings expand.

Current target: `crates/cairn-cli/src/evidence_manifest.rs` (~701 lines).

## What Changes

- Add a behavior-preserving refactor package for evidence manifest generation, verification, role validation, metadata hashing, and receipt helpers.
- Define module/test boundaries before implementation so each split can be landed as a narrow verified slice.
- Keep public CLI behavior, deterministic JSON receipts, issue codes, fixture output, policy hashes, and Tracey references stable unless a later explicit Cairn changes them.

## Impact

- **Files**: `crates/cairn-cli/src/evidence_manifest.rs` (~701 lines) and any newly introduced sibling modules/test targets.
- **Testing**: evidence manifest focused tests plus evidence export/registry manifest-binding checks, then repository validation, Tracey coverage, Clippy, and diff checks.
- **Compatibility**: No public command, schema, or receipt semantics are intended to change.
