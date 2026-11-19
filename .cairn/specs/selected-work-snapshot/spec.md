# Selected Work Snapshot Specification

## Purpose

Defines the `selected-work-snapshot` capability.

## Requirements

### Requirement: Versioned checked selected cohort

r[cairn.selected_work.contract] Cairn MUST expose a versioned typed full selected-work snapshot with distinct source, intent, context, verification-subject, dependencies, obligations, selection and observation identities, and MUST independently admit portable records against an expected cohort. The dependencies identity MUST cover exactly the selection, selected task IDs, prerequisite edges and observed prerequisite states, independent of prose and input ordering.

#### Scenario: Native intent changes without task-count change
- GIVEN the same selected task and unchanged task counts
- WHEN relevant proposal, design, metadata, specification, policy or profile, declared context or evidence bytes change
- THEN the appropriate binding MUST change and an old expected cohort MUST be refused.

#### Scenario: Crossed or malformed portable facts
- GIVEN a stale, unsupported-schema, partial or crossed-subject portable record
- WHEN an external consumer admits it against its expected cohort
- THEN admission MUST refuse without inferring completion or execution authority.

### Requirement: Explicit selected observation

r[cairn.selected_work.observation] Cairn MUST select one explicit repository or lifecycle store, a change and an optional authored task ID, and MUST compose native readiness and declared context over only that change's prerequisite closure under one aggregate read budget.

#### Scenario: Independent malformed change
- GIVEN selected A depending on B and an unrelated malformed C
- WHEN Cairn observes A
- THEN C MUST NOT become an A prerequisite, while a missing or out-of-scope B MUST remain visible.

#### Scenario: Selector ambiguity and exhaustion
- GIVEN unknown, duplicate or ambiguous authored IDs, unsafe source, partial enumeration or an exceeded aggregate budget
- WHEN the selected observation is requested
- THEN Cairn MUST refuse or report explicit incompleteness, and no list position MUST become persistent task identity.

### Requirement: Local and portable transport

r[cairn.selected_work.transport] Cairn MUST expose the same semantic selected snapshot through CLI and MCP, bind detail reads to the exact expected cohort, and keep raw admitted context and host absolute paths out of the portable record.

#### Scenario: Identity-bound detail
- GIVEN a detail request for one selected task and an expected observation cohort
- WHEN any relevant source fact changes before the detail read
- THEN the request MUST be refused instead of mixing cohorts.

### Requirement: Honest remaining obligations and conformance

r[cairn.selected_work.conformance] Cairn MUST retain typed and scoped requirement-marker and acceptance-marker references, implementation units, native phase readiness, verification-contract locators and remaining completion, sync and archive obligations. Unsupported dependency meanings MUST NOT appear as satisfied empty sets.

#### Scenario: Unfinished and completion-only round trips
- GIVEN an unfinished eligible task and a distinct completion-only change
- WHEN real CLI and MCP outputs are admitted by the independent consumer control
- THEN their remaining obligations MUST remain distinct and stage decisions MUST match native validators.

### Requirement: Typed workflow obligations in the selected snapshot

r[cairn.selected_work.obligations] When a caller selects an admitted workflow obligation definition, Cairn MUST carry its bound predicate, structured guidance, typed per-obligation facts and evaluation in the selected snapshot, bound to that snapshot's verification subject, context and workflow policy. Without a selection the section MUST be explicitly `not_selected`. Caller-supplied observations MUST be bound into the source manifest. Admission MUST refuse crossed subjects, forged or inconsistent evaluations, missing obligation facts and any outcome that turns missing observations into an empty satisfied set.

#### Scenario: Guidance, predicate and remaining obligations agree
- GIVEN a selected `verify` definition for one snapshot
- WHEN CLI, MCP and `cairn obligation export` describe it for the same subject and context
- THEN the predicate, obligation IDs and per-obligation identities MUST match.

#### Scenario: Incomplete alternatives and evidence changes
- GIVEN a selected alternative with missing evidence and a complete unselected alternative
- WHEN the snapshot is evaluated, and again after the missing evidence is supplied
- THEN the first evaluation MUST remain unsuccessful with remaining obligations, the second MAY prove, obligation identities MUST be unchanged, and the obligations cohort MUST change.

#### Scenario: Policy revision
- GIVEN an obligation catalog admitted for another workflow policy
- WHEN it is used for a snapshot
- THEN Cairn MUST refuse it, and a snapshot under the revised policy MUST carry new obligation identities.

### Requirement: No new lifecycle authority

r[cairn.selected_work.boundary] Cairn MUST treat transport success, context prose, obligation guidance and advisory actions as observations only. Kiln owns coordination identities, Lattice executes admitted work, and Cairn's original validators and obligation evaluator retain their authority.

#### Scenario: Successful read of blocked work
- GIVEN a successfully transported selected snapshot with a blocked or unavailable native stage
- WHEN a consumer renders next work
- THEN it MUST NOT infer completion, execution, sync, archive or release approval from transport success.
