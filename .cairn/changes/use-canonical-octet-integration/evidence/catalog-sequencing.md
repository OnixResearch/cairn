# Catalog sequencing decision

The remaining strict findings in `cairn-core` are dominated by the
`path-segment-repetition` family (~2,000 findings). The rule rejects names like
`AgentGuidanceBounds` inside module `agent_guidance` because the name repeats an
ancestor path word. The fix renames public types and functions across the crate
and every consumer call site.

Decision: the public rename campaign began after all utility changes integrated, then stopped at a catalog compatibility conflict.

Reasons:
1. All five utility changes are now integrated. The quiet-tree prerequisite is satisfied.
2. The first `agent_guidance` batch renamed 16 implementation types, six public functions, and five private helpers. Its target-file `path_segment_repetition` count fell from 54 to 0.
3. Preserving 22 documented public root names with `pub use ... as ...` compatibility reexports added 44 `renamed_imports` errors. The attempt remained uncommitted because the strict check was not green.
4. Per-item compatibility allowances are permitted only when their reason is specific. A facade-only allowance needs fresh compiler and strict-hook evidence before it can be accepted.
5. The local Rust-source store integrity failure now blocks fresh Nix dev-shell and strict-hook validation while `cargo-octet` rebuilds. The exact mismatch is retained in the persistent resume evidence. Do not commit the unvalidated facade allowance batch.
6. The remaining catalog families (assertion density, parameter limits, const-compatibility, file size) remain per-item judgment work.

When validation becomes available:
- Build the resolved public-name inventory first (the export facade split in
  `cb7e9f0` is the starting point).
- Re-run the `agent_guidance` compatibility facade batch with a distinct,
  reasoned `renamed_imports` allow on each public compatibility reexport, then
  prove the old root API and the strict target-file count.
- Rename per module with compiler-driven call-site updates.
- Add positive/negative controls in `tests/interface.rs` for compatibility names.

The other catalog families (assertion density, parameter limits,
const-compatibility, file-size) are likewise per-item judgment work on a quiet
tree. Import qualification, the mechanically safe family, is complete.
