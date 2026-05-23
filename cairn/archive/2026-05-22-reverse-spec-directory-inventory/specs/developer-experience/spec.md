# Developer Experience Specification

## Purpose

Improve reverse-spec/import richness by letting operators point Cairn at bounded directories rather than hand-enumerating every file.

## Requirements

### Requirement: Directory include expansion
r[cairn.developer_experience.reverse_spec_directory_inventory.expand] The system MUST expand explicit directory includes into deterministic safe relative file snapshots before reverse-spec planning.

#### Scenario: Expand source directory
r[cairn.developer_experience.reverse_spec_directory_inventory.expand.source_dir]
- GIVEN an operator includes a repository-relative directory containing source, docs, tests, hidden files, and build artifacts
- WHEN Cairn prepares reverse-spec discovery snapshots
- THEN only safe text-like files under reviewed directory classes are selected in sorted order
- AND hidden, build, vendor, and unsafe paths are skipped without reading them as candidate evidence

### Requirement: Directory inventory receipt evidence
r[cairn.developer_experience.reverse_spec_directory_inventory.receipt] The system MUST expose selected and skipped directory inventory paths in the normal discovery receipt.

#### Scenario: Review directory receipt
r[cairn.developer_experience.reverse_spec_directory_inventory.receipt.review]
- GIVEN directory includes produce selected and skipped paths
- WHEN Cairn emits the discovery receipt
- THEN the receipt includes stable selected_paths and skipped_paths arrays
- AND optional draft output remains review-needed candidate material, not accepted specification content

### Requirement: Directory inventory documentation
r[cairn.developer_experience.reverse_spec_directory_inventory.docs] The system MUST document directory include usage and non-claims for reverse-spec discovery.

#### Scenario: Read operator guidance
r[cairn.developer_experience.reverse_spec_directory_inventory.docs.read]
- GIVEN an operator reads Cairn's quickstart command list
- WHEN reverse-spec discovery is described
- THEN directory include examples are present
- AND the documentation states that generated candidates still require review

### Requirement: Directory inventory verification
r[cairn.developer_experience.reverse_spec_directory_inventory.verification] The system MUST verify directory inventory expansion with positive and negative regression coverage.

#### Scenario: Run directory inventory tests
r[cairn.developer_experience.reverse_spec_directory_inventory.verification.run]
- GIVEN fixture directories with eligible and ineligible files
- WHEN focused developer-experience tests run
- THEN eligible files produce deterministic candidates
- AND ineligible paths appear as skipped or absent without mutating accepted specs
