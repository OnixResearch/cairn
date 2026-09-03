# Lifecycle Workflow Profiles Delta

## ADDED Requirements

### Requirement: Outcome-machine profile is explicit and opt-in

r[cairn.outcome_machine_profile.selection] Cairn MUST register an `outcome-machine` workflow profile without changing the compatibility default. Repository policy MAY select or require this profile, but ambient user, environment, agent, or remote state MUST NOT select it.

#### Scenario: Change selects outcome-machine profile

- GIVEN repository policy allows the `outcome-machine` profile
- WHEN an operator creates a change with `--profile outcome-machine`
- THEN Cairn MUST record the exact profile ID and version in change metadata
- AND Cairn MUST scaffold every profile artifact

#### Scenario: Existing change has no profile metadata

- GIVEN an existing change has no workflow profile metadata
- WHEN Cairn evaluates the change
- THEN Cairn MUST retain the `spec-driven` compatibility profile
- AND the new profile MUST NOT change existing behavior

### Requirement: Profile scaffolds a typed review artifact

r[cairn.outcome_machine_profile.scaffold] The `outcome-machine` profile MUST scaffold `outcome-machine.json` with schema `cairn.outcome-machine-review.v1`, bounded placeholder values, and all required common and route fields.

#### Scenario: New profile scaffold is complete

- GIVEN a valid new change name and admitted outcome-machine policy
- WHEN Cairn creates the change
- THEN the scaffold MUST include proposal, delta-spec, outcome-machine, design, and tasks artifacts
- AND `outcome-machine.json` MUST contain the supported schema and explicit placeholder values

#### Scenario: Scaffold uses another profile

- GIVEN an operator selects `spec-driven`, `assurance`, or `no-spec-delta`
- WHEN Cairn creates the change
- THEN Cairn MUST NOT add an outcome-machine artifact

### Requirement: Pure admission validates three routes

r[cairn.outcome_machine_profile.routes] Cairn MUST validate delivery, foundation, and bounded-exception review routes in the pure core. Unknown fields, unsupported routes, empty required values, placeholders, hidden session state, or non-repository completion authority MUST fail admission.

#### Scenario: Delivery route is valid

- GIVEN a delivery record names its immediate outcome, current consumer, durable capability, repeatability plan, owner, tracked inputs, checks, AI facts, and non-claims
- WHEN Cairn evaluates the review record
- THEN route admission MUST pass

#### Scenario: Foundation route has no current consumer

- GIVEN a foundation record omits its current consumer or target outcome
- WHEN Cairn evaluates the review record
- THEN route admission MUST fail with a stable design-stage issue

#### Scenario: Bounded exception promotes durable capability

- GIVEN a bounded-exception record omits its capability non-claim or uses hidden session state
- WHEN Cairn evaluates the review record
- THEN route admission MUST fail with a stable design-stage issue

### Requirement: Planning and completion evidence remain distinct

r[cairn.outcome_machine_profile.completion] Cairn MUST permit planned completion state during proposal, design, and tasks gates. Cairn MUST block validation, sync, archive, and drain readiness until the record contains observed, BLAKE3-bound completion evidence.

#### Scenario: Planned record enters design review

- GIVEN a structurally valid route record has planned completion and fresh-state states
- WHEN proposal, design, or tasks gates run
- THEN completion-stage findings MUST NOT fail those planning gates

#### Scenario: Planned record enters lifecycle completion

- GIVEN a structurally valid route record still has planned completion
- WHEN validation, sync, archive, or drain planning evaluates completion blockers
- THEN lifecycle completion MUST remain blocked

#### Scenario: Observed completion evidence is present

- GIVEN a valid route record has observed completion with bounded evidence references
- WHEN lifecycle completion runs
- THEN Cairn MUST read each safe change-local evidence path
- AND the outcome-machine profile MUST add no completion blocker when every BLAKE3 identity matches

#### Scenario: Referenced evidence changed

- GIVEN a completion or fresh-state evidence file is missing, unsafe, oversized, or different from its declared BLAKE3 identity
- WHEN lifecycle completion runs
- THEN completion MUST remain blocked

### Requirement: Fresh-state reproduction has an explicit disposition

r[cairn.outcome_machine_profile.fresh_state] Cairn MUST require fresh-state reproduction to be planned, observed with BLAKE3-bound evidence, or not applicable with a bounded rationale. Hidden state from the producing session MUST remain prohibited.

#### Scenario: Fresh-state reproduction is observed

- GIVEN the record declares observed fresh-state reproduction
- WHEN Cairn evaluates the record
- THEN the record MUST contain at least one bounded evidence reference

#### Scenario: Fresh-state reproduction is not applicable

- GIVEN the record declares that fresh-state reproduction is not applicable
- WHEN Cairn evaluates the record
- THEN the record MUST contain a substantive rationale

#### Scenario: Fresh-state reproduction remains planned

- GIVEN the record declares planned fresh-state reproduction
- WHEN lifecycle completion runs
- THEN completion MUST remain blocked

### Requirement: Outcome-machine claims remain bounded

r[cairn.outcome_machine_profile.boundary] Cairn MUST bind the review artifact and findings into workflow identities and receipts. Cairn MUST NOT claim product value, evidence truth, semantic correctness, reuse quality, maintenance quality, or release eligibility.

#### Scenario: Profile passes all structural checks

- GIVEN an outcome-machine record passes route and completion admission
- WHEN Cairn emits workflow, gate, sync, or archive output
- THEN output MUST retain the profile non-claims
- AND repository-owned checks and reviewers MUST retain semantic authority

#### Scenario: Activity count is presented as evidence

- GIVEN a record claims that commit, task, archive, or generated-code volume establishes completion
- WHEN Cairn evaluates its authority fields
- THEN Cairn MUST reject activity counts as completion authority

### Requirement: Profile verification preserves compatibility

r[cairn.outcome_machine_profile.verification] The change MUST include positive and negative policy, core, CLI, gate, sync, archive, and compatibility tests. Existing profiles, stable gate stages, and receipt meanings MUST remain compatible.

#### Scenario: Focused suites cover valid and invalid paths

- GIVEN valid records for all routes and invalid records for each required rejection class
- WHEN focused tests and policy checks run
- THEN all valid records MUST pass their applicable stage
- AND every invalid record MUST fail with its expected stable issue

#### Scenario: Existing profiles run unchanged

- GIVEN existing profile fixtures and unprofiled compatibility changes
- WHEN the full workflow and lifecycle suites run
- THEN their documented artifacts, gates, plans, and stable receipt meanings MUST remain unchanged

### Requirement: Operator documentation explains the profile

r[cairn.outcome_machine_profile.docs] Cairn MUST document profile selection, route fields, planning behavior, completion evidence, fresh-state disposition, repository authority, and non-claims.

#### Scenario: Operator prepares an outcome-machine change

- GIVEN an operator reads the workflow profile guide
- WHEN the operator selects the outcome-machine profile
- THEN the guide MUST identify every required artifact and transition
- AND it MUST distinguish structural conformance from semantic value
