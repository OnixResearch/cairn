# Change: Publish the trace index in Cairn evidence bundles

## Why

The repository-scale Nix check proves the trace-index command chain, but normal Cairn evidence bundles omit that proof. Operators cannot import, republish, index, or verify the content-bound trace artifact through Cairn's standard evidence workflow.

## What Changes

- Make `cairn evidence export` generate a content-bound requirement registry, trace index, and current-root verification receipt.
- Add typed evidence-manifest roles for the trace index and its verification receipt.
- Make `cairn evidence verify` validate artifact hashes, schemas, registry binding, and current-root freshness.
- Preserve the separate authority of aggregate traceability coverage.
- Add positive export/verify tests and negative tamper/staleness tests.

## Impact

- **Files**: evidence export, manifest, verification, integration tests, trace-index documentation, and lifecycle artifacts.
- **Testing**: focused evidence tests, workspace tests, policy freshness, lifecycle validation, traceability coverage, Clippy, and Nix checks.
- **Claim boundary**: the bundle proves exact declared linkage and current-input freshness only. It does not prove requirement satisfaction or code correctness.
