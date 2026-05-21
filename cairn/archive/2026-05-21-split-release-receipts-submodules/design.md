# Design

`crates/cairn-cli/tests/release_receipts.rs` remains the integration-test target facade and declares focused sibling modules under `crates/cairn-cli/tests/release_receipts/`.

The submodules are organized by cohesive receipt responsibility:

- `support.rs` contains local command, fixture, JSON, schema, and hash helpers used by this target.
- `portable.rs` covers portable receipt hashing across roots.
- `bundle.rs` covers agent evidence bundle verification.
- `manifest.rs` covers signed evidence manifests and indexed artifacts.
- `release_readiness.rs` covers release-readiness preflight and bundle receipts.
- `schema.rs` covers public receipt schema and README field documentation checks.
- `agent_replay.rs` covers agent receipt validation and replay evidence failure receipts.

The split intentionally avoids shared crate-root test support and keeps helper visibility at `pub(super)` so future review sees a narrow local bridge.
