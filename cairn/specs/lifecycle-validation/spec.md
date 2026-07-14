# Lifecycle Validation Specification

## Purpose

Defines the `lifecycle-validation` capability.

## Requirements

### Requirement: Required lifecycle artifacts have policy-owned substance

r[cairn.lifecycle_validation.artifact_substance] Cairn MUST evaluate required proposal, design, tasks, accepted-spec, and delta-spec artifacts against policy-owned minimum substance rules rather than file existence alone.

#### Scenario: Substantive artifact passes its stage

r[cairn.lifecycle_validation.artifact_substance.positive]
- GIVEN a required artifact contains policy-recognized reviewed content and no unresolved placeholder-only body
- WHEN Cairn validates the artifact at its applicable stage
- THEN the artifact MUST satisfy the substance check

#### Scenario: Empty or placeholder-only artifact fails

r[cairn.lifecycle_validation.artifact_substance.empty]
- GIVEN a required artifact is empty, whitespace-only, or contains only generated placeholder instructions
- WHEN Cairn validates the repository or evaluates the applicable gate
- THEN Cairn MUST report a deterministic artifact-substance issue naming the artifact and missing content class

### Requirement: Lifecycle specs cannot pass vacuously

r[cairn.lifecycle_validation.spec_substance] Cairn MUST require every validated accepted or delta spec to contain at least one well-formed requirement block with one canonical bare requirement ID and normative strength in that requirement's own body.

#### Scenario: Empty spec fails validation

r[cairn.lifecycle_validation.spec_substance.empty]
- GIVEN an accepted or delta `spec.md` contains no requirement block
- WHEN Cairn validates lifecycle specs
- THEN validation MUST fail with a deterministic zero-requirement issue

#### Scenario: Scenario text cannot lend requirement strength

r[cairn.lifecycle_validation.spec_substance.borrowed_strength]
- GIVEN a requirement body has no MUST, SHALL, SHOULD, or MAY statement but a later scenario or fenced example contains such a token
- WHEN Cairn validates that requirement
- THEN the requirement MUST fail the normative-strength check

#### Scenario: Requirement identity is explicit

r[cairn.lifecycle_validation.spec_substance.identity]
- GIVEN a requirement block has no canonical bare requirement ID or has multiple candidate IDs
- WHEN Cairn validates the spec
- THEN validation MUST report the malformed identity and MUST NOT count the block as substantive

### Requirement: Task artifacts contain real work

r[cairn.lifecycle_validation.task_substance] Cairn MUST require a gate-ready task artifact to contain at least the policy-owned minimum number of substantive valid checkbox tasks and MUST NOT treat zero or placeholder-only tasks as archive completion.

#### Scenario: Empty tasks file fails tasks gate

r[cairn.lifecycle_validation.task_substance.empty]
- GIVEN `tasks.md` exists but contains no checkbox task
- WHEN Cairn evaluates the tasks gate
- THEN the gate MUST fail with a deterministic zero-task issue

#### Scenario: Placeholder checkbox does not count

r[cairn.lifecycle_validation.task_substance.placeholder]
- GIVEN a tasks artifact contains an empty checkbox label, generated placeholder text, a checkbox-only row, or a row without valid requirement/dependency linkage
- WHEN Cairn inventories substantive tasks
- THEN the row MUST NOT satisfy the minimum task count
- AND validation MUST report the missing task content or linkage class.

#### Scenario: Zero substantive tasks blocks archive

r[cairn.lifecycle_validation.task_substance.archive]
- GIVEN a change has zero substantive counted tasks
- WHEN the operator requests archive execution
- THEN Cairn MUST report the change as not archive-ready and MUST leave the active change in place

### Requirement: Dependency task markers are structurally valid

r[cairn.lifecycle_validation.dependency_marker_shape] Cairn MUST parse configured task dependency markers as complete structured values and MUST reject malformed markers instead of treating a prefix match as a linkage exemption.

#### Scenario: Valid dependency marker satisfies configured shape

r[cairn.lifecycle_validation.dependency_marker_shape.valid]
- GIVEN a checkbox task contains a closed dependency marker with a non-empty policy-valid target
- WHEN Cairn validates task markers
- THEN the marker MAY satisfy the configured dependency-link alternative

#### Scenario: Empty or unclosed dependency marker fails

r[cairn.lifecycle_validation.dependency_marker_shape.invalid]
- GIVEN a checkbox task contains `[depends:]`, an unclosed dependency marker, an empty target, or an unsupported target spelling
- WHEN Cairn validates task markers
- THEN Cairn MUST report a deterministic malformed-marker issue
- AND the marker MUST NOT exempt the task from a known requirement reference

### Requirement: Task progress observations are not completion authority

r[cairn.lifecycle_validation.task_progress_non_authority] Cairn MUST treat checkbox counts and task-state deltas as scheduling/progress observations only and MUST NOT let them override current gate, probe, dependency, or receipt blockers.

#### Scenario: Reduced unchecked count reports progress only

r[cairn.lifecycle_validation.task_progress_non_authority.reduced]
- GIVEN one task artifact has fewer unchecked substantive tasks than an earlier observation
- WHEN Cairn or a drain consumer classifies progress
- THEN it MAY report task progress bound to the observed task input hashes
- BUT it MUST NOT infer gate readiness, archive readiness, verification success, or completion from the count delta alone.

#### Scenario: Checked tasks cannot override evidence blocker

r[cairn.lifecycle_validation.task_progress_non_authority.blocked]
- GIVEN every substantive task checkbox is marked complete but a required gate, probe, dependency, or receipt is missing, stale, malformed, skipped, blocked, deferred, timed-out, dry-run, infrastructure-failed, or otherwise non-passing
- WHEN tasks gate or mutation preflight runs
- THEN the authoritative outcome MUST remain blocked
- AND task counts MUST NOT replace the blocker diagnostic or its evidence identity.

### Requirement: Vacuous packages cannot mutate lifecycle state

r[cairn.lifecycle_validation.mutation_block] Cairn MUST apply artifact, spec, task, and dependency-marker substance findings consistently to validation, stage gates, sync/archive preflight, and mutation-capable MCP tools.

#### Scenario: Vacuous package is blocked everywhere

r[cairn.lifecycle_validation.mutation_block.consistent]
- GIVEN a change has any blocking substance issue
- WHEN validation, the applicable stage gate, sync/archive preflight, or an equivalent MCP tool evaluates the change
- THEN every surface MUST report the same stable issue class
- AND no explicit mutation request may change lifecycle files

#### Scenario: Passing substance makes no correctness overclaim

r[cairn.lifecycle_validation.mutation_block.boundary]
- GIVEN every required lifecycle artifact satisfies its minimum substance rules
- WHEN Cairn emits a passing substance result
- THEN the result MUST claim only structural non-vacuity and policy conformance
- AND it MUST NOT claim prose quality, requirement satisfaction, implementation correctness, verification soundness, or release eligibility

### Requirement: Lifecycle non-vacuity regressions

r[cairn.lifecycle_validation.tests] Cairn MUST verify lifecycle substance behavior with positive and negative core, CLI, MCP, policy, and mutation fixtures.

#### Scenario: Positive fixtures pass

r[cairn.lifecycle_validation.tests.positive]
- GIVEN substantive artifacts with supported requirement, scenario, task, and dependency-marker variants
- WHEN focused validation and gate tests run
- THEN every fixture MUST pass with deterministic inventories and no false vacuity issue

#### Scenario: Negative fixtures fail closed

r[cairn.lifecycle_validation.tests.negative]
- GIVEN missing, empty, whitespace-only, placeholder-only, zero-requirement, borrowed-strength, malformed-identity, zero-task, or malformed-dependency fixtures
- WHEN focused validation, gate, and mutation tests run
- THEN every fixture MUST report its expected deterministic issue and MUST preserve lifecycle files
