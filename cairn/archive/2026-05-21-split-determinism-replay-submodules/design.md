# Design

Create `crates/cairn-cli/src/determinism/replay/` with narrow modules for:

- `mod.rs`: facade and top-level replay orchestration.
- `options.rs`: CLI option parsing and output format selection.
- `policy.rs`: replay case/group/entropy/environment policy decoding and selection.
- `execution.rs`: per-case execution, fixture-root handling, mutation checks, temp-root copy helpers.
- `receipt.rs`: receipt assembly, mismatch/failure shaping, environment attestation, and entropy receipt fields.
- `render.rs`: JSON/JSONL/summary rendering.
- `types.rs`: shared replay DTOs scoped to the module tree.

Visibility remains `pub(super)`/`pub(crate)` only where existing consumers require it. Existing focused determinism integration tests remain the behavioral guard.
