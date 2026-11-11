# Implementation Ready Specification

## Purpose

Defines the `implementation-ready` capability.

## Requirements

### Requirement: Versioned implementation-ready projection

r[cairn.implementation_ready.projection] Cairn MUST expose a read-only, versioned `cairn.implementation-ready.v1` projection of implementation-eligible change and task units. It MUST reuse existing gate, task-ordering, dependency, cross-repository evidence, and workflow-profile evaluation; MUST keep archive readiness distinct; and MUST NOT claim, execute, or mutate work. Units MUST be stable explicit task IDs or one whole-change unit for legacy task lists, never list positions. The projection MUST include blockers, completed and blocked tasks, requirement references, and a deterministic snapshot digest.

#### Scenario: Unfinished work while archive is unready

r[cairn.implementation_ready.projection.scenario.unfinished]
- GIVEN a change with satisfied proposal/design/tasks gates and unfinished tasks
- WHEN the projection runs
- THEN the change MUST be implementation-ready with one selectable unit
- AND archive readiness MUST remain false

#### Scenario: Ready tasks exclude completed work

r[cairn.implementation_ready.projection.scenario.tasks]
- GIVEN explicit task IDs with one completed task and one ready successor
- WHEN the projection runs
- THEN only the ready successor MUST be selected
- AND the completed task MUST be reported but not dispatched

#### Scenario: Completion is not implementation

r[cairn.implementation_ready.projection.scenario.completion_only]
- GIVEN a change whose tasks are all complete
- WHEN the projection runs
- THEN it MUST report completion-only state with no selection
- AND archive readiness MUST remain separate

#### Scenario: Enumeration limit is explicit

r[cairn.implementation_ready.projection.scenario.limit]
- GIVEN more active changes than the requested limit
- WHEN the projection runs
- THEN it MUST return `incomplete_limit` with `complete: false`
- AND it MUST NOT report an empty or complete queue
