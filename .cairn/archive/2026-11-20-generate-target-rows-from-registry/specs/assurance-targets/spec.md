# Assurance targets specification delta

## MODIFIED Requirements

### Requirement: Exact registry row binding

r[cairn.assurance_targets.row_binding] Each target MUST bind exactly one content-bound registry row by `requirement_id`, `specification_path`, `specification_blake3`, `source_span`, `requirement_body_blake3` and `row_blake3`, and each contract MUST bind the specification path and BLAKE3 of its targets' rows. A target's `requirement` and a contract's `specification` MAY be derived from a `cairn requirement-registry export` by requirement ID, and derivation MUST fail closed when the export does not declare `cairn.content-bound-requirement-registry.v1` or does not contain exactly one row for that ID.

#### Scenario: Matching rows are bound
- GIVEN a registry exported at one revision and targets copied from its rows
- WHEN the targets are admitted against that registry
- THEN each target MUST bind its row and each contract MUST bind its specification
- AND neither the row identity nor the specification text MUST change.

#### Scenario: A stale or duplicated row is rejected
- GIVEN a target whose row differs from the registry in any of the six fields, a duplicated requirement, or a contract whose specification differs from its rows
- WHEN the targets are admitted
- THEN Cairn MUST reject the input with the stale-row, duplicate-requirement or specification-mismatch code
- AND it MUST NOT rebind the target to the current row.

#### Scenario: A derived row equals its copied row
- GIVEN a registry export and a targets file whose targets and contract specification are derived from it by requirement ID, alongside the same targets file with the row and specification copied
- WHEN both are exported and admitted
- THEN the two exports MUST be byte-identical after canonical sorting
- AND Rust admission MUST re-verify every derived row's six fields against the registry.

#### Scenario: Derivation fails closed
- GIVEN a registry export with no row for the ID, two rows for the ID, or a schema identifier other than `cairn.content-bound-requirement-registry.v1`
- WHEN a target derives its requirement or the contract specification by that ID
- THEN Nickel MUST fail with an error naming the requirement ID or the admitted schema identifier
- AND it MUST NOT select another row or emit a partial target.
