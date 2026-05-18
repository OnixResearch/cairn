## ADDED Requirements

### Requirement: Tracey Coverage Rail

r[cairn.supersede.tracey_coverage] The system MUST expose a first-class Tracey coverage rail that links accepted Cairn requirements to implementation or verification annotations without making Tracey a runtime dependency.

#### Scenario: Tracey configuration is discoverable

r[cairn.supersede.tracey_coverage.config]
- GIVEN a repository checkout with accepted Cairn specs
- WHEN an operator runs Tracey from the repository root
- THEN Tracey MUST discover `.config/tracey/config.styx` and scan accepted specs plus Rust source and test files

#### Scenario: Coverage check is deterministic

r[cairn.supersede.tracey_coverage.check]
- GIVEN accepted specs with `r[...]` requirement IDs and Rust comments with `r[impl ...]` or `r[verify ...]` references
- WHEN the maintained Nix verification runs
- THEN the repository MUST fail the traceability check for missing references without invoking runtime Cairn code

#### Scenario: Runtime boundary is preserved

r[cairn.supersede.tracey_coverage.boundary]
- GIVEN Tracey coverage support
- WHEN `cairn-core` is compiled and checked
- THEN the core MUST remain independent of Tracey configuration, Tracey processes, and filesystem coverage scanning
