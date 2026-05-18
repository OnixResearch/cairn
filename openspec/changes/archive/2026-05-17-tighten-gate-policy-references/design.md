## Context

The CLI already discovers change paths and renders gate JSON. The core already evaluates stage eligibility from a pure `ChangeStatus` snapshot.

## Decisions

### 1. Add a pure gate artifact snapshot

**Choice:** Introduce a core snapshot containing proposal/design/tasks markdown and delta spec markdown.
**Rationale:** The CLI can read files once, while core remains deterministic and testable.
**Alternative:** Parse files directly in core; rejected because it violates FCIS boundaries.

### 2. Validate requirement reference hygiene

**Choice:** Require tasks-stage checkbox tasks to reference known delta requirement IDs with `r[...]`, and reject unknown `r[...]` references in proposal/design/tasks.
**Rationale:** Tasks should be tied to the requirements they implement or verify, and typoed references should fail the gate early.

### 3. Validate bounded exemptions

**Choice:** Allow exemption markers only when they contain reason, owner, and until fields.
**Rationale:** Exemptions can be useful, but unbounded exemptions are policy drift.

## Risks / Trade-offs

**Fixture churn** → Update only behavioral fixtures needed by the new gate contract.

**Over-strict parsing** → Keep marker syntax simple and deterministic rather than full markdown/NLP parsing.
