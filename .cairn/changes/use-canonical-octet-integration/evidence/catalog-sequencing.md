# Catalog sequencing decision

The remaining strict findings in `cairn-core` are dominated by the
`path-segment-repetition` family (~2,000 findings). The rule rejects names like
`AgentGuidanceBounds` inside module `agent_guidance` because the name repeats an
ancestor path word. The fix renames public types and functions across the crate
and every consumer call site.

Decision: defer that rename campaign until the active utility changes integrate.

Reasons:
1. The readiness and evidence branches are adding new public items in the current
   naming style (`ReadinessSnapshot`, `CheckEvidencePlan`, translator types). A
   rename pass now guarantees large conflicts on `verified/mod.rs`, the API facade,
   and every consumer module.
2. The lint permits repetition for documented compatibility APIs. The boundary
   between compatibility names and renamable names needs a maintained inventory
   of the exported API surface, not a mechanical sweep.
3. The Octet change's own task list orders this work under `core-catalog` after
   `core-owners`; nothing in the accepted spec requires the rename to precede the
   utility changes.

When the campaign runs, on a quiet tree:
- Build the resolved public-name inventory first (the export facade split in
  `cb7e9f0` is the starting point).
- Rename per module with compiler-driven call-site updates.
- Add positive/negative controls in `tests/interface.rs` for compatibility names.

The other catalog families (assertion density, parameter limits,
const-compatibility, file-size) are likewise per-item judgment work on a quiet
tree. Import qualification, the mechanically safe family, is complete.
