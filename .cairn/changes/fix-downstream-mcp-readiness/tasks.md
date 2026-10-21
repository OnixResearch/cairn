## Foundation

- [x] [serial] [task:foundation] Record the direct MCP correction scope and pass proposal, design and tasks gates before implementation. r[cairn.release_consumer.selection]

## Implementation and verification

- [x] [serial] [task:selection] [after:foundation] Reuse typed consumer selection and exact policy propagation, preserving fail-closed behavior and self-host defaults. r[cairn.release_consumer.selection]
- [x] [serial] [task:producer] [after:selection] Integrate the existing real dependency/native Nix producer mechanisms without weakening maintained gates. r[cairn.release_consumer.selection]
- [ ] [serial] [task:verification] [after:producer] Exercise real consumer acceptance and negative controls, run required repository gates, and retain exact-source evidence before reviewed sync/archive/publication. r[cairn.release_consumer.selection]

Integrated mechanism evidence: all 46 focused core checks, 12 local producer cases, three real native Nix cases and both consumer-selection integration cases passed. A real unformatted consumer produces a nonpassing `cargo_fmt` row; both original completion and read-only MCP report `valid=false` despite process exit zero. Null, unknown-field and unsafe present selections reject without fallback. The genuine consumer corpus passes freshness; a changed copy reports one stale case without mutation. These observations do not complete the full-cohort or full ChaosControl command obligations; the verification task remains unchecked.
