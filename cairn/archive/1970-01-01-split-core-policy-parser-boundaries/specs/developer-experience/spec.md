## ADDED Requirements

### Requirement: Core policy parser boundary split
r[developer_experience.core_policy_parser_boundary_split.map] The system SHOULD keep the core policy parser review surface split into named modules for public DTOs, errors, generic field extraction, Steel profile decoding, parser assembly, and tests.

#### Scenario: Policy parser responsibilities are visible
r[developer_experience.core_policy_parser_boundary_split.modules]
- GIVEN a maintainer inspects `crates/cairn-core/src/verified/policy`
- WHEN they review the module layout
- THEN public policy DTOs, errors, field helpers, Steel profile decoding, parser assembly, and tests SHALL live in named files behind the same public facade.

#### Scenario: Policy parser behavior is preserved
r[developer_experience.core_policy_parser_boundary_split.behavior]
r[developer_experience.core_policy_parser_boundary_split.tests]
r[developer_experience.core_policy_parser_boundary_split.validation]
- GIVEN the parser split is implemented
- WHEN focused policy tests and repository validation run
- THEN generated policy parsing, unsupported-version rejection, Steel profile validation, public exports, issue-code material, and Tracey coverage SHALL pass without intentional policy or fixture drift.
