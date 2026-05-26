# Determinism Specification Delta

## ADDED Requirements

### Requirement: Preserves receipt workflow has operator documentation
r[cairn.determinism.preserves_operator_docs] Cairn MUST document how operators use internal Preserves receipt hashing without replacing public JSON receipt surfaces.

#### Scenario: Operator docs describe the Preserves workflow boundary
r[cairn.determinism.preserves_operator_docs.workflow]
- GIVEN Cairn exposes internal Preserves hashes, corpus fixtures, update rails, and CI drift guards
- WHEN an operator reads the Preserves receipt workflow documentation
- THEN the documentation distinguishes JSON `receipt_hash` compatibility from internal Preserves hash inspection
- AND it names the read-only agent receipt hash command, corpus update command, explicit execute rail, and CI checks

#### Scenario: Preserves workflow docs are discoverable
r[cairn.determinism.preserves_operator_docs.discoverability]
- GIVEN operators already consult README and agent-run receipt documentation
- WHEN the Preserves workflow docs are added
- THEN those existing docs link to the Preserves operator workflow

#### Scenario: Preserves operator docs are validated
r[cairn.determinism.preserves_operator_docs.validation]
- GIVEN the Preserves operator documentation
- WHEN Cairn validates lifecycle specs, Tracey coverage, and focused Preserves checks
- THEN the documentation contract has durable implementation and verification references
