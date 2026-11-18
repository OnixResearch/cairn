## Implementation

- [ ] [serial] Extend typed policy and closed v2 schema while keeping exact-marker v1 bytes stable. r[cairn.trace_index.structural_profile]
- [ ] [serial] Parse bounded saved Rust bytes and configuration into immutable scoped item facts; reject unsupported forms and budgets. r[cairn.trace_index.structural_adapter]
- [ ] [serial] Bind requirement revisions, occurrences, normalized item identity and prior immutable records; recompute freshness. r[cairn.trace_index.structural_history]
- [ ] [serial] Extend CLI export, verify, show and portable evidence paths with current/stale/historical/absent distinctions. r[cairn.trace_index.structural_consumers]

## Validation and completion

- [ ] [serial] Run focused positive/negative core and CLI tests in pinned Nix/Rust; retain selected-profile outputs. r[cairn.trace_index.structural_validation]
- [ ] [serial] Run policy freshness, evidence checks and repository-scale validation, then sync/archive through native lifecycle. r[cairn.trace_index.structural_validation]
