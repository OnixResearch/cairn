# Compile workflow guidance and completion predicates from typed obligations

## Why

Generated agent workflows describe goals, evidence and command classes as prose.
Readiness, native receipts, freshness, completion authority and verification
claim revisions each own a real evidence predicate, but nothing binds a
workflow's guidance to an evaluable completion expression over those exact
predicates. As a result, an agent can present checked task boxes, activity
counts or a success statement as if it discharged a native requirement, and an
adapter can silently restate evidence or command authority.

Issue #12 requires one admitted, versioned obligation definition to produce both
agent guidance and a machine-readable completion predicate, with presentation
changing only layout and prose. It also requires explicit alternatives,
conditional applicability, a bounded temporal check and a native
policy-exception path that never relabels the original condition as proved.

## What Changes

- Add `cairn-policy/workflow-obligations.ncl`, a standalone checked policy that
  references the five native evidence predicates by requirement ID and exact
  accepted revision plus one compensating-control predicate and one
  applicability predicate. It exports to
  `cairn-policy/generated/workflow-obligations.json`.
- Add `cairn_core::workflow_obligations` with `deny_unknown_fields` DTOs,
  policy admission against the admitted generated workflow catalog, finite
  `all_of`/`any_of`/conditional applicability, separate action preconditions,
  expected observations, progress facts and completion conditions, and a
  deterministic evaluation report.
- Derive guidance and the predicate from the same admitted definition.
  Presentations change heading level, bullet character and prose layout only;
  evidence requirements, permitted command classes and authority have no
  presentation inputs.
- Convert the `verify` workflow first through
  `render_with_workflow_obligations`. Workflows without an admitted definition
  render byte-identically to the legacy renderer.
- Add a bounded temporal extension for ordering between two named events in a
  declared finite trace interval with declared observer, clock, horizon,
  completeness and span. Missing, partial, untrusted or out-of-horizon
  observations remain unknown or unsatisfied.
- Reference one native policy-exception identity, request revision, policy
  identity, scope rule and compensating control. Current native applicability
  and control evidence are required; revocation, expiry, revision change,
  policy change, scope removal or control removal invalidate the path. The
  original condition and basis stay reported as unsatisfied.
- Export the bound predicate, obligation IDs, alternatives and evidence
  requirements through `cairn obligation list|explain|export` for #14 and Kiln
  composition. No defaults of the existing selected-work surfaces change.
- Add no workflow executor, clock service, scheduler, monitoring process or
  replacement exception authority.
