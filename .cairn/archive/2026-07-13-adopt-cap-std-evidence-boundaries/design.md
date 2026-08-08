## Design

The CLI remains the only layer that opens ambient directories. It opens the requested repository root, generated artifact root, evidence bundle root, registry root, release root, replay working root, or scan root as a `cap_std::fs::Dir`, then passes typed wrappers to shell modules.

The pure `cairn-core` validation and lifecycle logic continues to operate on loaded markdown, policy records, receipt DTOs, in-memory file inventories, and relative locators. It must not receive `cap-std` types or perform filesystem I/O.

The first conversion targets are `crates/cairn-cli/src/evidence*`, `evidence_registry*`, `release_evidence*`, `support/scan.rs`, and `determinism/replay/execution.rs`. Existing issue codes and receipt schemas should be preserved where possible; newly detected escape attempts should receive stable path-boundary diagnostics that do not echo host-private absolute paths.

Tests should exercise valid repository-relative artifacts, missing roots, traversal attempts, absolute path input, symlink escapes, and replay/scan roots. Documentation should make clear that capability roots protect local filesystem authority only; they do not prove artifact truth, replay completeness, lifecycle correctness, or release readiness by themselves.
