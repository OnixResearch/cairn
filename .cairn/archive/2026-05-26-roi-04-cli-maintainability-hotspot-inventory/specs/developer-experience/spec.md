# Developer Experience Specification Delta

## ADDED Requirements

### Requirement: CLI maintainability hotspot inventory
r[developer_experience.cli_maintainability_hotspot_inventory.scan] The system SHOULD maintain an operator-readable inventory of large Cairn source/test hotspots after release checkpoints.

#### Scenario: Inventory records future split seams
r[developer_experience.cli_maintainability_hotspot_inventory.docs]
- GIVEN a post-release maintainability review
- WHEN the operator scans source and test sizes
- THEN the inventory SHALL list the largest hotspots, candidate future splits, and focused verification commands.

#### Scenario: Inventory remains non-mutating advisory evidence
r[developer_experience.cli_maintainability_hotspot_inventory.boundary]
r[developer_experience.cli_maintainability_hotspot_inventory.validation]
- GIVEN the inventory is committed
- WHEN validation runs
- THEN it SHALL avoid behavior-change claims, generated artifacts, broad rewrites, and unverified refactor assertions.
