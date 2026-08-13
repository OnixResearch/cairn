# Task Ordering Specification

## Purpose

Defines optional, deterministic task dependency ordering for Cairn change packages.

## Requirements

### Requirement: Explicit task ordering markers

r[cairn.task_ordering.markers] Cairn MUST support policy-owned task identity and predecessor markers without changing existing dependency-link marker semantics.

#### Scenario: A change does not use task ordering

r[cairn.task_ordering.markers.optional]
- GIVEN a tasks artifact contains no task identity or predecessor marker
- WHEN Cairn evaluates task ordering
- THEN Cairn MUST report ordering as inactive and MUST preserve existing task validation behavior

#### Scenario: A change activates task ordering

r[cairn.task_ordering.markers.complete]
- GIVEN any checkbox task contains a task identity or predecessor marker
- WHEN Cairn evaluates task ordering
- THEN every checkbox task MUST contain exactly one valid task identity
- AND each predecessor marker MUST contain one valid task identity target

#### Scenario: Existing dependency linkage remains compatible

r[cairn.task_ordering.markers.compatibility]
- GIVEN a checkbox task contains an existing `[depends:...]` marker
- WHEN Cairn evaluates task ordering
- THEN the marker MUST retain its lifecycle-link meaning
- AND the marker MUST NOT create a local task-order edge

### Requirement: Pure task dependency graph admission

r[cairn.task_ordering.graph] Cairn MUST admit task dependency graphs in pure deterministic core logic over in-memory task Markdown and policy data.

#### Scenario: Valid graph has stable order

r[cairn.task_ordering.graph.valid]
- GIVEN identified checkbox tasks contain valid acyclic predecessor declarations
- WHEN Cairn admits the task graph
- THEN the graph MUST contain stable nodes, edges, reverse edges, and dependency-first topological order
- AND topological ties MUST use stable task identity order

#### Scenario: Invalid graph fails closed

r[cairn.task_ordering.graph.invalid]
- GIVEN task ordering has missing identities, malformed markers, duplicate identities, unknown predecessors, duplicate edges, self-dependencies, or a cycle
- WHEN Cairn admits the task graph
- THEN Cairn MUST emit deterministic issue codes
- AND Cairn MUST expose no actionable task order or ready task

### Requirement: Task progress follows declared order

r[cairn.task_ordering.progress] Cairn MUST derive task blockers and readiness from admitted predecessors and observed checkbox states.

#### Scenario: A task becomes ready

r[cairn.task_ordering.progress.ready]
- GIVEN a valid task graph contains a pending task whose predecessors are all done
- WHEN Cairn evaluates task readiness
- THEN Cairn MUST report the task as ready

#### Scenario: Recorded progress violates order

r[cairn.task_ordering.progress.invalid]
- GIVEN a task is done or in progress while a predecessor is not done
- WHEN Cairn evaluates task readiness
- THEN Cairn MUST reject the task graph with a deterministic progress issue
- AND Cairn MUST report no task as ready

### Requirement: Task gate admission

r[cairn.task_ordering.gate] Cairn MUST include task-order graph admission in task-gate evaluation and receipts.

#### Scenario: Invalid order blocks the task gate

r[cairn.task_ordering.gate.invalid]
- GIVEN a change contains an invalid or progress-inconsistent task graph
- WHEN the operator evaluates the tasks gate
- THEN the gate MUST fail with task-order issue codes
- AND the receipt MUST include the task-order graph report

#### Scenario: Valid order passes graph admission

r[cairn.task_ordering.gate.valid]
- GIVEN a change contains a valid task graph and satisfies all other task-gate rules
- WHEN the operator evaluates the tasks gate
- THEN task ordering MUST add no gate issue
- AND the receipt MUST include stable task-order data

### Requirement: Dependency-aware task drain guidance

r[cairn.task_ordering.drain] Cairn MUST include admitted task order and ready-task guidance in read-only drain-plan receipts.

#### Scenario: Drain plan names ready tasks

r[cairn.task_ordering.drain.ready]
- GIVEN an unfinished change has a valid active task graph
- WHEN the operator requests a drain plan
- THEN the change receipt MUST include dependency-first task order and ready task identities
- AND implementation guidance MUST name only ready tasks

#### Scenario: Invalid order blocks implementation guidance

r[cairn.task_ordering.drain.invalid]
- GIVEN an unfinished change has an invalid task graph
- WHEN the operator requests a drain plan
- THEN the change MUST remain drain-blocked
- AND the plan MUST recommend task-order repair instead of dependent implementation work

### Requirement: Positive and negative task-order tests

r[cairn.task_ordering.tests] Cairn MUST test accepted task ordering and each rejected graph or progress class.

#### Scenario: Verification covers both outcomes

r[cairn.task_ordering.tests.coverage]
- GIVEN task-order graph, task-gate, and drain-plan behavior changes
- WHEN Cairn runs focused verification
- THEN tests MUST cover valid ordering and ready-task output
- AND tests MUST cover malformed, missing, duplicate, unknown, self, cyclic, and progress-invalid inputs
