## MODIFIED Requirements

### Requirement: Split release receipt integration-test target uses focused submodules

r[cli_maintainability.release_receipts_submodules.focused_modules] The system MUST split the `release_receipts` integration-test target into focused submodules that preserve a small facade and isolate cohesive receipt/release responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.release_receipts_submodules.focused_modules.visible_boundaries]
- GIVEN the submodule split is implemented
- WHEN a reviewer inspects `crates/cairn-cli/tests/release_receipts.rs` and its sibling module directory
- THEN portable receipt hash, agent bundle, signed manifest, release-readiness, public schema, and agent replay checks are separated into named modules
- AND the facade no longer contains the full large receipt/release review surface

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.release_receipts_submodules.focused_modules.narrow_bridges]
- GIVEN helpers are shared by the receipt/release submodules
- WHEN the implementation exposes local support functions
- THEN helper visibility is limited to the test target's module tree
- AND broad crate-root test shims or mutable shared state are avoided

### Requirement: Split release receipt integration-test target remains behavior preserving

r[cli_maintainability.release_receipts_submodules.behavior_preserved] The system MUST preserve public behavior while splitting the release receipt integration-test target into focused submodules.

#### Scenario: Focused checks pass

r[cli_maintainability.release_receipts_submodules.behavior_preserved.focused_checks]
- GIVEN the submodule split is implemented
- WHEN the operator runs the reorganized `release_receipts` integration-test target
- THEN the focused check passes without intentional receipt, issue-code, fixture, policy-hash, or Tracey-reference drift

#### Scenario: Repository checks pass

r[cli_maintainability.release_receipts_submodules.behavior_preserved.repository_checks]
- GIVEN the submodule split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
