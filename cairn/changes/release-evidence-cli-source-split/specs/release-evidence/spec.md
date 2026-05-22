# Release Evidence Specification Delta

## ADDED Requirements

### Requirement: Release evidence CLI source split uses focused modules

r[release_evidence.cli_source_split.focused_modules] The system MUST split the release-evidence CLI implementation into focused source modules that preserve a small dispatcher facade and isolate cohesive responsibilities.

#### Scenario: Focused module boundaries are visible

r[release_evidence.cli_source_split.focused_modules.visible_boundaries]
- GIVEN the release-evidence source split is implemented
- WHEN a reviewer inspects the release-evidence CLI source tree
- THEN current-HEAD index generation, publication materialization, retention planning, and rotation execution are separated into named modules or equivalent focused files
- AND the dispatcher facade no longer contains the full implementation for every release-evidence family

#### Scenario: Index generation has a focused home

r[release_evidence.cli_source_split.index_module]
- GIVEN the release-evidence source split is implemented
- WHEN a reviewer inspects current-HEAD evidence index code
- THEN index option parsing, orchestration, receipt assembly, and rendering live in a focused module or clearly bounded source region
- AND the public `release-evidence index` behavior remains unchanged

#### Scenario: Publication materialization has a focused home

r[release_evidence.cli_source_split.publish_module]
- GIVEN the release-evidence source split is implemented
- WHEN a reviewer inspects local publication materialization code
- THEN publish option parsing, artifact copying, manifest assembly, and receipt generation live in a focused module or clearly bounded source region
- AND the public `release-evidence publish` behavior remains unchanged

#### Scenario: Retention planning has a focused home

r[release_evidence.cli_source_split.retention_module]
- GIVEN the release-evidence source split is implemented
- WHEN a reviewer inspects retention planning code
- THEN candidate inventory, plan decisions, label filtering, and plan receipt generation live in a focused module or clearly bounded source region
- AND the public `release-evidence retention-plan` behavior remains unchanged

#### Scenario: Rotation execution has a focused home

r[release_evidence.cli_source_split.rotate_module]
- GIVEN the release-evidence source split is implemented
- WHEN a reviewer inspects rotation execution code
- THEN saved-plan verification, dry-run mutation reporting, execute-mode local deletion, and rotation receipt generation live in a focused module or clearly bounded source region
- AND the public `release-evidence rotate` behavior remains unchanged

### Requirement: Release evidence CLI source split keeps bridge APIs narrow

r[release_evidence.cli_source_split.narrow_bridges] The system MUST expose only narrow shared helpers across release-evidence modules and avoid broad crate-root shims or unrelated helper movement.

#### Scenario: Shared helpers are limited to cross-family needs

r[release_evidence.cli_source_split.narrow_bridges.shared_helpers]
- GIVEN multiple release-evidence modules need common path, hash, issue, or JSON helper logic
- WHEN the implementation exposes shared helpers
- THEN helper visibility is limited to the release-evidence module tree wherever practical
- AND unrelated CLI families do not gain new broad public bridge APIs

### Requirement: Release evidence CLI source split remains behavior preserving

r[release_evidence.cli_source_split.behavior_preserved] The system MUST preserve release-evidence command behavior, deterministic receipts, issue codes, fixtures, and policy hash material while splitting source modules.

#### Scenario: Focused behavior checks pass

r[release_evidence.cli_source_split.behavior_preserved.focused_checks]
- GIVEN the source split is implemented
- WHEN the operator runs focused release-evidence tests
- THEN index, publish, retention, rotation, and dirty-worktree checks pass without intentional receipt, issue-code, fixture, or policy drift

#### Scenario: Repository checks pass

r[release_evidence.cli_source_split.repository_checks]
- GIVEN the source split is implemented
- WHEN the operator runs formatting, workspace tests, clippy, Cairn validation, Tracey coverage, and diff checks
- THEN all checks pass and the repository remains ready for commit

#### Scenario: Package can be archived after verification

r[release_evidence.cli_source_split.archive_ready]
- GIVEN the source split has passed focused and repository verification
- WHEN the operator syncs and archives the change package
- THEN accepted specs include the release-evidence source-split requirements
- AND no active package remains for this completed source-organization slice
