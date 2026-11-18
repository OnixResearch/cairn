## Why

Exact marker bytes establish a source occurrence but not the Rust declaration that owns it. Issue #2 requires a separately versioned, current-root-checkable item binding without changing the requirement registry or historical exact-marker v1 bytes.

## What Changes

- Add an opt-in `rust_item_v1` traceability profile with a policy-owned safe Rust configuration path.
- Resolve bounded saved UTF-8 Rust bytes to scoped declarations with a pinned parser and explicit target/features/cfg; reject ambiguity, malformed syntax, unsupported expansion and exhausted budgets.
- Bind requirement-row revision, exact marker/source occurrence and normalized parsed item separately in v2 indexes. Preserve immutable prior bindings through explicit `--previous-index` revision export.
- Expose current/stale/historical/absent state in export, verify, show and portable evidence paths while retaining v1 defaults.

## Impact

- **Files**: Nickel traceability contract/parser, trace index core and CLI, evidence export/verify, pinned Rust parser dependencies, trace contracts and positive/negative tests.
- **Testing**: focused core/CLI tests, policy export freshness, portable evidence tests and pinned Nix/Rust checks are required before completion.
