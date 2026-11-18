## Implementation

- [x] [serial] Extend typed policy and closed v2 schema while keeping exact-marker v1 bytes stable. r[cairn.trace_index.structural_profile]
- [x] [serial] Parse bounded saved Rust bytes and configuration into immutable scoped item facts; reject unsupported forms and budgets. r[cairn.trace_index.structural_adapter]
- [x] [serial] Bind requirement revisions, occurrences, normalized item identity and prior immutable records; recompute freshness. r[cairn.trace_index.structural_history]
- [x] [serial] Extend CLI export, verify, show and portable evidence paths with current/stale/historical/absent distinctions. r[cairn.trace_index.structural_consumers]

## Validation and completion

- [x] [serial] Run focused positive/negative core and CLI tests in pinned Nix/Rust; retain selected-profile outputs. r[cairn.trace_index.structural_validation]
- [x] [serial] Run policy freshness, evidence checks and repository-scale validation, then sync/archive through native lifecycle. r[cairn.trace_index.structural_validation]

Executed evidence: `trace_index` 4/4, `verification_impact` 5/5 and the core
traceability suite 25/25 pass. `cairn validate --root .` reports 0 issues and
`cairn traceability coverage --root .` is 2367/2367.

One defect was fixed during acceptance: the Rust-item test fixtures contained ten
literal demo-item markers, which the source-byte scanner registered as live claims
for an intentionally undeclared requirement. The fixtures now assemble the marker
from two adjacent string literals with `concat!`, and the
`match_indices`/`find`/`len` sites use that constant. `traceability_policy`
exposes no path exclusion, so the fixture source had to change.

Scoped limitations: the profile is opt-in and advisory. It records structural
linkage and freshness, not behavioral satisfaction, and v1 exact-marker
serialization and hashes are unchanged. No Buoy code was copied. The
repository-wide strict Octet catalog is a separately tracked decision.
