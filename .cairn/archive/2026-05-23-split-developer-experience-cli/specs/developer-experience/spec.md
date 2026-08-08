# Developer Experience Specification Delta

## ADDED Requirements

### Requirement: Developer-experience CLI focused modules
r[cairn.developer_experience.cli_modules.focused_modules] The system MUST keep developer-experience CLI command families in focused source modules with clear responsibilities.

#### Scenario: Review focused command modules
r[cairn.developer_experience.cli_modules.focused_modules.review]
- GIVEN the developer-experience CLI implementation is inspected
- WHEN a maintainer reviews template, scaffold, discovery, and shared helper logic
- THEN each command family is located in a focused module
- AND shared filesystem helper code is separated from command dispatch

### Requirement: Developer-experience CLI façade boundary
r[cairn.developer_experience.cli_modules.facade_boundary] The system MUST preserve the existing developer-experience CLI façade used by top-level dispatch.

#### Scenario: Dispatch through stable façade
r[cairn.developer_experience.cli_modules.facade_boundary.dispatch]
- GIVEN `main.rs` dispatches `template`, `scaffold`, and `discover-specs` commands
- WHEN the developer-experience implementation is split into submodules
- THEN the façade still exposes `run_template`, `run_scaffold`, and `run_discover_specs`
- AND top-level CLI dispatch does not need command-specific submodule knowledge

### Requirement: Developer-experience CLI behavior preservation
r[cairn.developer_experience.cli_modules.behavior_preservation] The system MUST preserve developer-experience CLI receipts and mutation semantics across the module split.

#### Scenario: Verify unchanged command receipts
r[cairn.developer_experience.cli_modules.behavior_preservation.receipts]
- GIVEN existing focused developer-experience CLI tests
- WHEN the module split is verified
- THEN template, scaffold, and discovery commands keep their receipt fields, policy IDs, safe path checks, and mutation manifest semantics
- AND no new user-facing command syntax is required

### Requirement: Developer-experience CLI archived design context
r[cairn.developer_experience.cli_modules.archived_design] The system MUST archive the module-boundary rationale for future maintainability review.

#### Scenario: Review archived split context
r[cairn.developer_experience.cli_modules.archived_design.review]
- GIVEN the split change is archived
- WHEN a maintainer reviews the archive
- THEN the proposal, design, tasks, and spec delta describe the façade and submodule boundaries
- AND the archive records that the split is behavior-preserving rather than a new capability
