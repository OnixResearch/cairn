# Add requirement consistency analysis

## Why

Cairn can validate requirement identity, admit one optional finite meaning and
explain native claim derivations, but nothing yet asks whether the formalized
requirements describe a possible, activated and compatible obligation.
Issue #8 and Roadmap #4 require the specification itself to be evaluated before
implementation evidence: contradictory assumptions, unreachable activation,
conflicting guarantees and vacuous implications are currently indistinguishable
from a requirement that merely has no passing implementation yet.

## What Changes

- Add separate bounded queries over admitted typed contracts: shared-assumption
  satisfiability, per-requirement activation reachability, joint guarantee
  compatibility, and whether withholding one guarantee admits a previously
  excluded behavior.
- Bind every query to exact contract revisions, interpretation and mapping
  identities, contract-local quantified variables, the declared finite domains
  and observable tables, the selected requirement set and the shared
  environmental assumptions.
- Emit exactly five explicit statuses — `consistent`, `inconsistent`, `vacuous`,
  `undetermined`, `unsupported` — with a retained search basis naming whether the
  outcome came from a validated witness, a complete finite exploration, an
  exhausted bound or an unsupported construct.
- Retain a found model as complete total observable tables for every selected
  contract, and add a pure replay validator that re-admits the case and the
  tables, cross-checks the typed-contract evaluator and requires the named query.
- Report contradictions, unreachable activation, conflicting subsets with
  separate minimality, redundant-within-model guarantees, single-instance
  quantifiers and domains that observations cannot distinguish, each naming exact
  requirement revisions and reproducing formula fragments.
- Attach findings to caller-named native obligations with disposition
  `unresolved`; no obligation, gate, readiness or completion verdict changes.
- Add a read-only `cairn requirement-analysis inspect` command with bounded reads,
  native registry verification, optional retained-witness replay, and Nickel-owned
  bounds and query vocabulary.

## Impact

Current consumers: maintainers deciding whether a formalized requirement set is
worth implementing, and the refinement/composition workstreams that need
feasibility evidence.
Immediate outcome: one pilot corpus of five native requirements is reported as
consistent with a replayed model, and the contradictory, unreachable, redundant
and single-instance cases are explicit instead of being silently treated like an
ordinary passing requirement.
Durable contribution: a bounded query/witness/status contract that revision
classification, compatibility and workflow compilation can reuse.
Maintenance owner: Cairn verified-core maintainers own status and witness
semantics; CLI shell maintainers own bounded reads and presentation.

Existing substance and coverage reports, typed-contract evaluation, native
applicability validators and the Pass | Block gate keep their current meanings.
This change adds no solver authority, no automatic policy weakening and no second
requirement registry.

## Dependencies

`add-typed-requirement-contracts` and `add-scoped-claim-derivations` are already
archived on this base branch; this change reuses their public normalization,
identity and bounded-evaluation APIs without modifying them.

## Out of Scope

- External solver execution, proof search or unbounded theorem proving
- Claiming unbounded validity, reactive realizability, liveness or source-code
  correctness from a finite model
- Prose correspondence review, requirement authoring or automatic repair
- Translating a counterexample into an implementation failure or an admitted
  proof artifact
- Resolving, discharging or weakening any native obligation or gate decision
