# Catalog sequencing decision

The remaining strict findings in `cairn-core` are dominated by the
`path-segment-repetition` family (~2,000 findings). The rule rejects names like
`AgentGuidanceBounds` inside module `agent_guidance` because the name repeats an
ancestor path word. The fix renames public types and functions across the crate
and every consumer call site.

Decision: the public rename campaign began after all utility changes integrated. The first compatibility-facade batch is now verified and committed separately from the remaining catalog work.

Reasons:
1. All five utility changes are now integrated. The quiet-tree prerequisite is satisfied.
2. The first `agent_guidance` batch renamed 16 implementation types, six public functions, and five private helpers. Its target-file `path_segment_repetition` count fell from 54 to 0.
3. Preserving 22 documented public root names with `pub use ... as ...` compatibility reexports first added 44 `renamed_imports` errors.
4. The accepted repair gives every facade alias one conditional `renamed_imports` allowance with a specific compatibility reason. `cairn-core` declares the established Octet cfg so that the compiler checks the conditional attribute.
5. Remote pinned-source validation proved the aliases, root API controls, formatting, core tests, CLI tests, Clippy, and the strict catalog run. The focused total fell from 8,142 to 8,088 findings without new compatibility-test findings.
6. The local Rust-source store integrity failure still blocks fresh local Nix validation. The remote path is a validation workaround, not a replacement for repairing that store output.
7. The remaining catalog families (assertion density, parameter limits, const-compatibility, file size) remain per-item judgment work.

Next catalog batches:
- Build the resolved public-name inventory first (the export facade split in
  `cb7e9f0` is the starting point).
- Repair the separate `verified/policy/agent_guidance.rs` repetition only after
  selecting a non-conflicting local function name.
- Rename one implementation module at a time with compiler-driven call-site
  updates and a facade-only compatibility decision where public names must stay.
- Add positive and negative compatibility controls with each public facade
  repair.

The other catalog families (assertion density, parameter limits,
const-compatibility, file-size) are likewise per-item judgment work on a quiet
tree. Import qualification, the mechanically safe family, is complete.
