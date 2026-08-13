## Context

Cairn recognizes `[parallel]`, `[serial]`, and `[depends:...]`. The dependency marker can provide lifecycle linkage, but it does not identify a local task edge.

Task ordering needs local task identity, graph admission, state-aware readiness, and deterministic receipt data. The core must remain free of file and process effects.

## Decisions

### Decision: Add separate task identity and predecessor markers

**Choice:** Use `[task:<id>]` for identity and `[after:<task-id>]` for local predecessor edges.

**Rationale:** This syntax is explicit and does not change existing `[depends:...]` semantics. Ordering stays optional until either new marker appears.

### Decision: Require complete identities when ordering is active

**Choice:** If one checkbox task uses either ordering marker, every checkbox task must have exactly one valid task identity.

**Rationale:** A complete node set prevents hidden work and makes missing dependency checks deterministic.

### Decision: Admit the graph in the functional core

**Choice:** Parse in-memory Markdown and return typed nodes, edges, issues, topological order, blockers, and ready tasks.

**Rationale:** The shell reads files and renders JSON. The core owns graph meaning, progress invariants, and fail-closed admission.

### Decision: Reuse an internal deterministic DAG component

**Choice:** Extract Cairn's existing topological-order mechanism into a narrow internal component used by change and task graphs.

**Rationale:** Trellis remains the algorithm reference. Choregraph remains unnecessary because Cairn does not execute task effects.

### Decision: Enforce state order

**Choice:** A done or in-progress task is invalid while any predecessor is not done.

**Rationale:** A declared order must constrain recorded progress. Invalid graphs expose no ready tasks or actionable order.

## Alternatives Reviewed

- Reusing `[depends:...]` would make old linkage markers local graph edges. This can break existing change packages.
- Delegating graph admission to Choregraph would import execution-machine semantics that Cairn does not need.
- Inferring order from Markdown position would hide dependencies and prevent reliable missing-target checks.

## Risks / Trade-offs

- Ordering mode requires identities on all checkbox tasks. This is deliberate fail-closed behavior.
- Gate and drain receipts gain new fields. Existing fields and `[depends:...]` behavior remain unchanged.
- Topological ties use stable task identifiers, not visual line order.
