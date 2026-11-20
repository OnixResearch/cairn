# Assurance Targets Specification

## Purpose

Defines the `assurance-targets` capability.

## Requirements

### Requirement: Nickel-owned targets contract

r[cairn.assurance_targets.contract] Cairn MUST define the assurance targets file, its grouping into per-specification contracts, and every admitted value and bound in Nickel, and Rust admission MUST read only the checked policy export.

#### Scenario: A well-formed targets file is admitted
- GIVEN a targets file that applies `contracts/assurance-targets.ncl` and stays within the exported bounds
- WHEN Cairn admits it against `cairn-policy/generated/assurance-targets.json`
- THEN every target MUST be admitted with its declared assurance, claim, informal remainder and evidence job
- AND admission MUST NOT start the Nickel evaluator.

#### Scenario: Unknown values, unknown fields and exceeded bounds are rejected
- GIVEN a targets file with an unknown assurance value, `unassessed`, an unknown field, an unknown schema identifier, or more contracts, targets or text bytes than the policy allows
- WHEN Cairn admits it
- THEN Cairn MUST reject it with a distinct stable issue code for each defect
- AND it MUST NOT fall back to a default assurance, schema or bound.

### Requirement: Exact registry row binding

r[cairn.assurance_targets.row_binding] Each target MUST bind exactly one content-bound registry row by `requirement_id`, `specification_path`, `specification_blake3`, `source_span`, `requirement_body_blake3` and `row_blake3`, and each contract MUST bind the specification path and BLAKE3 of its targets' rows.

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

### Requirement: One evidence job and no authored hashes

r[cairn.assurance_targets.evidence_job] Each projected target MUST name exactly one evidence job by `job_id` and the path of its owner-authored definition, no target MUST carry a source, tool, artifact or policy hash, and source identities MUST come only from a separately generated source record.

#### Scenario: Projected targets name their job
- GIVEN a `checked_proof` or `bounded_check` target that names one evidence job
- WHEN the target is admitted
- THEN its job identity MUST be retained for convergence binding
- AND its source bindings MUST come from the generated source record rather than from the target.

#### Scenario: Missing jobs and authored hashes are rejected
- GIVEN a projected target with no evidence job, or a target that carries an authored hash field
- WHEN the target is admitted
- THEN Cairn MUST reject it with the missing-evidence-job or authored-hash code
- AND it MUST NOT infer a job or accept the hash.

### Requirement: Fixed clause projection

r[cairn.assurance_targets.projection] Cairn MUST project `checked_proof` targets to `formal_proof` clauses and `bounded_check` targets to `empirical_check` clauses with one fixed field mapping, MUST NOT project `reviewed` targets, and the Rust mapping MUST equal the Nickel projection.

#### Scenario: The projection reproduces Octet's example clause
- GIVEN targets and a generated source record that reproduce clause 0 of Octet's `standards/requirements/example-contract.json` at `v0.1.1-rc.1`, plus one `reviewed` target
- WHEN the targets are projected in Nickel and the mapping is recomputed in Rust
- THEN both MUST produce a clause equal to Octet's clause after JSON parsing
- AND the `reviewed` target MUST produce no clause.

#### Scenario: A weakened contract does not match the recomputed mapping
- GIVEN a contract file whose clause disposition, evidence identifiers or bindings differ from the mapping of its targets
- WHEN the Rust mapping is recomputed
- THEN Cairn MUST report the mismatch
- AND it MUST NOT accept that contract file as the projection of those targets.
