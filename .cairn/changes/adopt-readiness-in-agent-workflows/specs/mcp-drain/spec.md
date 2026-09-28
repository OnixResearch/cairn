# Readiness-backed drain delta

## ADDED Requirements

### Requirement: Shared readiness observations
r[cairn.agent_workflow_readiness.shared] The system MUST derive the new drain view from the existing readiness model over explicit per-change evidence without duplicating gate or completion rules.

#### Scenario: Direct readiness and drain agree
- GIVEN the same admitted change, policy, source, and evidence selections
- WHEN direct readiness and the new drain view evaluate that change
- THEN every stage state and causal blocker agrees after transport metadata is excluded

#### Scenario: Missing evidence remains unavailable
- GIVEN complete artifacts and checked tasks without an explicit completion contract and results
- WHEN the new drain view evaluates the change
- THEN it retains unavailable completion and does not recommend execution as if completion passed

### Requirement: Existing order and authority remain distinct
r[cairn.agent_workflow_readiness.order] The system MUST preserve dependency order and declared ready-task order while keeping advisory next actions separate from execution and mutation permission.

#### Scenario: The next task respects predecessors
- GIVEN a valid change graph and declared task graph with one ready task
- WHEN the view selects a next action
- THEN it identifies that task and its exact instruction route without executing it

#### Scenario: Invalid order cannot select executable work
- GIVEN unknown dependencies, a cycle, or progress before a task predecessor completes
- WHEN the view derives next actions
- THEN it retains the ordering blocker and returns only supported diagnostic or repair guidance

### Requirement: Compatibility-scoped drain projection
r[cairn.agent_workflow_readiness.compatibility] The system MUST expose readiness-backed drain as an explicit versioned view while preserving the existing default drain receipt fields and hashes.

#### Scenario: The new view receives complete explicit selectors
- GIVEN per-change contract, result, subject, and cross-repository evidence selectors
- WHEN CLI or MCP requests the new view
- THEN the shell forwards only those admitted selectors to the shared observation adapter

#### Scenario: A malformed selector cannot become inferred evidence
- GIVEN duplicate change selectors, unsafe paths, unknown changes, or partial completion inputs
- WHEN the new view admits the request
- THEN it reports bounded errors without discovery, execution, or implicit completion inputs

### Requirement: Consumer compatibility evidence
r[cairn.agent_workflow_readiness.conformance] The system MUST compare legacy drain, direct validators, shared readiness, and generated workflow consumers across successful and failing maintained fixtures.

#### Scenario: The shared loop reduces redundant diagnostics
- GIVEN matched full-loop fixtures and equal completion criteria
- WHEN the benchmark compares legacy and readiness-backed workflows
- THEN it records complete token and call costs with unchanged oracle outcomes

#### Scenario: An unrelated observation failure preserves independent facts
- GIVEN a passing gate and an unavailable completion or cross-repository observation
- WHEN the shared view evaluates the fixture
- THEN it retains the passing gate and the separate failure without inferring overall readiness
