## MODIFIED Requirements

### Requirement: Split semantic-review core into focused modules
r[semantic_review.split_semantic_review_core.focused_modules] The system MUST split the semantic-review functional core into focused modules that preserve the existing `cairn_core::verified::semantic_review` facade, including a dedicated test submodule when regressions would otherwise dominate the facade.

#### Scenario: Focused module boundaries are visible
r[semantic_review.split_semantic_review_core.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a maintainer inspects `crates/cairn-core/src/verified/semantic_review/`
- THEN review receipts, revision handoffs, ledger analytics, completeness receipts, policy profiles, metrics regressions, shared validators, and facade regression tests SHALL be separated into named modules
- AND `mod.rs` SHALL remain a small public facade rather than the dominant review surface.

#### Scenario: Bridges remain narrow
r[semantic_review.split_semantic_review_core.focused_modules.narrow_bridges]
- GIVEN existing CLI and core consumers use semantic-review functions
- WHEN the split exposes functions across module boundaries
- THEN those bridges SHALL remain limited to existing public facade exports and helper visibility needed by sibling modules.

### Requirement: Preserve semantic-review split behavior
r[semantic_review.split_semantic_review_core.behavior_preserved] The system MUST preserve existing semantic-review public behavior, receipt schemas, hash material, issue codes, deterministic evidence, and Tracey coverage while draining this split.

#### Scenario: Focused checks pass
r[semantic_review.split_semantic_review_core.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs focused semantic-review core checks
- THEN the checks SHALL pass without intentional receipt, issue-code, schema, fixture, or hash drift.

#### Scenario: Repository checks pass
r[semantic_review.split_semantic_review_core.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN repository validation and Tracey coverage run
- THEN the checks SHALL pass and every semantic-review requirement SHALL remain referenced.
