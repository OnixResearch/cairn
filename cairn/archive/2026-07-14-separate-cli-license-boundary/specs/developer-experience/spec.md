# Cairn License Boundary Specification

## Purpose

Keep Cairn's reusable lifecycle core and copied project content permissive while applying AGPL reciprocity to the operator shell.

## Requirements

### Requirement: Package licenses follow the core-shell boundary

r[cairn.license_boundary.package_split] `cairn-core` MUST remain available under `MIT OR Apache-2.0`, and `cairn-cli` MUST declare `AGPL-3.0-or-later`.

#### Scenario: A consumer selects one package

- GIVEN a consumer inspects package metadata
- WHEN it selects `cairn-core` or `cairn-cli`
- THEN the selected package MUST expose the license assigned to its architectural role.

### Requirement: Complete license artifacts are checked in

r[cairn.license_boundary.artifacts] Cairn MUST ship complete texts for every repository-owned package license and MUST provide a boundary map that identifies permissive copied-content surfaces.

#### Scenario: A source distribution is reviewed

- GIVEN a recipient receives the Cairn source tree
- WHEN license files are inspected
- THEN AGPL, MIT, and Apache terms MUST be present and the package/content mapping MUST be explicit.

### Requirement: Consumer content remains permissive

r[cairn.license_boundary.consumer_boundary] Cairn project templates, examples, schemas, and starter lifecycle content MUST remain available under `MIT OR Apache-2.0`, and documentation MUST NOT claim that processing user-authored content changes its license.

#### Scenario: Cairn initializes another repository

- GIVEN Cairn copies starter content into a consumer repository
- WHEN that repository chooses its project license
- THEN the copied Cairn starter content MUST remain permissively licensed and the Cairn CLI license MUST NOT be represented as automatically licensing unrelated user-authored content.

### Requirement: License declarations are validated

r[cairn.license_boundary.validation] Cairn MUST deterministically reject repository-owned manifest or documentation drift that reverses the core-shell split while ignoring third-party dependency license expressions.

#### Scenario: Core metadata becomes AGPL accidentally

- GIVEN `cairn-core` inherits or declares AGPL
- WHEN the license-boundary check runs
- THEN the check MUST fail with a package-specific mismatch.

#### Scenario: CLI metadata remains permissive accidentally

- GIVEN `cairn-cli` inherits the permissive workspace license
- WHEN the license-boundary check runs
- THEN the check MUST fail with a package-specific mismatch.

### Requirement: Focused verification proves the boundary

r[cairn.license_boundary.final_validation] The change MUST verify package metadata, complete license artifacts, current boundary documentation, and both expected mismatch classes.

#### Scenario: The final repository state is checked

- GIVEN the intended core-shell mapping and license files are present
- WHEN focused verification and Cairn gates run
- THEN the boundary MUST pass without requiring product runtime tests unrelated to packaging.
