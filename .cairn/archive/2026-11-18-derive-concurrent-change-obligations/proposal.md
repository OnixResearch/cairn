# Derive concurrent-change compatibility obligations

## Why

Cairn can validate one change revision at a time against its exact typed
requirements and can explain textual collaboration conflicts and verification
impact. It cannot yet say whether two individually valid changes that touch the
same interface or invariant remain compatible when composed.

Issue #11 names the concrete failure: asynchronous retries and authorization
checked only at initial dispatch can both be valid in isolation and textually
disjoint while leaving unresolved whether every retried effect is authorized.
File disjointness, stable caller syntax and individually passing checks cannot
establish behavioural independence.

## What Changes

- Add a versioned change footprint that binds exact source and intent
  revisions, the typed requirement revisions it changes or consumes, consumed
  assumptions, changed guarantees, preserved invariants, invalidated
  obligations, affected interfaces and configurations, and explicit unknown or
  unsupported scope.
- Derive interactions when a changed guarantee affects another change's
  assumption, when an invalidation affects another change's evidence basis, when
  invariant commitments overlap, or when interface configuration or operation
  declarations conflict.
- Emit one compatibility obligation per interacting participant set naming the
  combined candidate, participant revisions, declared composition orders,
  context, affected claims and required joint evidence, with coexistence,
  invariant preservation and commutativity as separate propositions.
- Distinguish `conflict`, `needs_joint_evidence`,
  `supported_within_declared_scope` and `unknown`, and refuse to promote a
  satisfiability witness into preservation over all admitted compositions.
- Explain the deterministic path from changed guarantee to consumer assumption
  to resulting obligation, including finite checks, witness tables and evidence
  admission.
- Invalidate prior obligation applicability when participants, relevant
  dependencies, configuration or composition change.
- Add read-only `cairn footprints inspect --case FILE [--policy FILE] [--root DIR]`.
- Put every admitted bound in a standalone Nickel policy source.

## Impact

Current consumers: maintainers reviewing two or more open changes that share an
interface, invariant or declared assumption, and downstream workflow tools
(Kiln) that may qualify or serialize work from the resulting obligations.

Immediate outcome: a finite declared pair yields a deterministic interaction
path and a specific combined-candidate obligation.

Durable contribution: a bounded compatibility surface that reuses the native
typed contract and refinement identities rather than creating a second
requirement registry.

Cairn owns footprint admission, interaction derivation, obligation outcomes,
joint-evidence admission and prior-applicability invalidation. Existing textual
collaboration conflict resolution, verification-impact reports, native
applicability validators, readiness states, gates and lifecycle decisions keep
their meanings and remain independently usable.

## Out of Scope

- Whole-program semantic conflict detection, prose inference or unbounded
  theorem proving
- Scheduler, merge, ordering or execution authority
- Automatic evidence transfer, receipt promotion, obligation discharge or
  release authorization
- Replacing textual collaboration resolution or verification impact
- Treating a finite satisfiability witness as universal preservation

Dependencies: typed requirement contracts [#5], scoped claim derivations [#6],
refinement and typed dependencies [#7], semantic revision classification [#9],
support maintenance [#10], the reasoning kernel [#13] and bounded consistency
queries [#8]. The issue's retained corpus requirement is covered by authored
positive and negative core and CLI cases; parent verification remains open.

## Acceptance

- Disjoint changes that interact through a declared assumption produce a joint
  compatibility obligation.
- The retry/authorization fixture identifies the affected invariant and the
  missing integration condition.
- A compatible fixture names the exact assumptions, query meaning, evidence and
  composition identity supporting its result.
- Individually valid receipts cannot discharge the combined-candidate
  obligation.
- Order-sensitive composition is represented; invariant preservation does not
  imply commutativity.
- Unknown behaviour, incomplete footprints, unsupported interpretations and
  bounded exploration remain explicit.
- A retained corpus contains compatible, conflicting, order-sensitive and
  incomplete-declaration cases.
- The report explains the path from changed guarantee to consumer assumption to
  obligation.
- Existing textual conflict resolution and verification-impact reports remain
  independently usable.
