# Revision classification specification delta

## ADDED Requirements

### Requirement: Exact revision-bound semantic relation

r[cairn.revision_classification.requirement_relation] Cairn MUST compare two exact native requirement revisions under an explicit common domain, value and observable correspondence, MUST check both implication directions over the declared finite observation universe, and MUST classify the new obligation as `editorial`, `clarifying`, `strengthening`, `weakening`, `incompatible`, or `undetermined` with replayable evidence or an explicit unsupported or inconclusive basis.

#### Scenario: Equivalent structure is editorial and changed structure is clarifying
- GIVEN two revisions with mutually implying obligations and identical mapped assumptions, activation, guarantee, domains and observables
- WHEN the comparison runs
- THEN the result MUST be `editorial`
- AND an equivalent pair with a changed mapped domain or observable structure MUST be `clarifying`, not `editorial`

#### Scenario: Strengthening and weakening follow implication direction
- GIVEN the new obligation implies the old and not conversely
- WHEN the comparison completes
- THEN the result MUST be `strengthening`
- AND the reversed relation MUST be `weakening`

#### Scenario: Incomparable and unsupported differences stay non-editorial
- GIVEN each implication direction has a distinguishing finite observation world
- WHEN the comparison completes
- THEN the result MUST be `incompatible` with both witnesses
- AND an unmapped, extra, mistyped or non-injective correspondence MUST be `incompatible`, never `editorial`

#### Scenario: Exhausted bounds remain undetermined
- GIVEN the declared world count or evaluation steps exceed the admitted policy bound
- WHEN the comparison cannot finish
- THEN the result MUST be `undetermined` with the bound basis
- AND it MUST NOT report an implication it did not establish

### Requirement: Assumptions and guarantees remain separate

r[cairn.revision_classification.assumption_guarantee] Cairn MUST report assumptions, activation and guarantee relations separately from the overall obligation relation and MUST NOT hide a changed environmental assumption behind one strengthening or weakening label.

#### Scenario: Stronger assumptions reduce an unchanged guarantee
- GIVEN the new guarantee is structurally equal to the old guarantee and the new assumptions imply the old assumptions
- WHEN the comparison completes
- THEN the assumption relation MUST be reported as strengthened
- AND the overall obligation relation MUST show the corresponding reduction without claiming the guarantee changed

### Requirement: Declared acceptance-policy fragment

r[cairn.revision_classification.policy_fragment] Cairn MUST compare acceptance only over a declared fragment of named required checks, admissible evidence roles and allowed assumption sets within one accepted-evidence universe, MUST distinguish tightening from widening, and MUST report feasibility and emptiness separately from containment.

#### Scenario: Tightening and widening are distinguished
- GIVEN the new accepted set is a strict subset of the old set
- WHEN the fragments are compared
- THEN the result MUST be `strengthening` with a `tightened` relation
- AND a failed containment direction MUST retain a replayable accepted-evidence candidate outside its conclusion
- AND the reverse containment MUST be `weakening` with a `widened` relation

#### Scenario: Incomparable fragments retain both witnesses
- GIVEN neither accepted set contains the other
- WHEN the fragments are compared
- THEN the result MUST be `incompatible`
- AND each failed direction MUST name a candidate accepted by its premise and rejected by its conclusion

#### Scenario: Empty and infeasible acceptance remains visible
- GIVEN one fragment has an empty admissible-role or assumption-set dimension
- WHEN containment is reported
- THEN its accepted count and empty status MUST remain visible
- AND containment that is vacuously true MUST be marked as vacuous

#### Scenario: Universe or fragment mismatch is unsupported
- GIVEN the fragments declare different accepted-evidence universes or an unsupported fragment identifier
- WHEN the comparison runs
- THEN the result MUST be `incompatible`
- AND no containment or equivalence claim MUST be made

### Requirement: Per-claim evidence reuse decision

r[cairn.revision_classification.evidence_reuse] Cairn MUST bind each retained evidence claim to its old revision and current context, MUST report the old receipt as not current, and MUST return an explicit `reusable` or `not_reusable` decision for every obligation.

#### Scenario: Strengthening refuses stale evidence
- GIVEN retained evidence bound to the old strengthening revision and a current context
- WHEN the comparison runs
- THEN its decision MUST be `not_reusable`
- AND the old receipt MUST remain not current

#### Scenario: Weakening is never silently promoted
- GIVEN a weakened new obligation and retained evidence admitted against the old obligation
- WHEN the comparison runs
- THEN its decision MUST be `not_reusable`
- AND the report MUST expose the scope change instead of promoting the receipt

#### Scenario: Equivalent revision still stales the receipt
- GIVEN a semantically equivalent revision and current retained evidence
- WHEN the comparison runs
- THEN the payload MAY be marked `reusable` only for a fresh derivation
- AND the old receipt MUST remain not current
- AND automated transfer MUST remain disallowed

#### Scenario: Revision or context drift refuses reuse
- GIVEN a claim bound to another revision or a changed subject, policy, target or environment
- WHEN reuse is decided
- THEN the decision MUST be `not_reusable`

### Requirement: Disabled automated evidence transfer

r[cairn.revision_classification.transfer_disabled] Cairn MUST define a future transfer record naming old evidence, both claims, the comparison identity and current subject and environment applicability, and MUST keep automated transfer disabled until its admission rules and consumers are separately qualified.

#### Scenario: Transfer request is explicit but refused
- GIVEN an exact comparison report and a matching transfer request
- WHEN transfer admission runs
- THEN it MUST return an explicit unsupported outcome with the disabled reason
- AND it MUST NOT create a transfer record, refresh a receipt or mutate lifecycle state

### Requirement: Deterministic bounded read-only inspection

r[cairn.revision_classification.inspection] Cairn MUST provide read-only CLI classification with bounded reads, explicit admission errors, deterministic identity-bearing reports, and no obligation discharge, release authorization or lifecycle mutation.

#### Scenario: Requirement classification is advisory
- GIVEN a valid requirement-pair input
- WHEN `cairn revision classify --requirement` runs
- THEN it MUST print the versioned advisory report
- AND it MUST NOT change any gate, readiness, verification-role or completion decision

#### Scenario: Policy classification is advisory
- GIVEN a valid policy-fragment input
- WHEN `cairn revision classify --policy-fragment` runs
- THEN it MUST print the supported fragment, universe and containment result
- AND it MUST NOT alter policy, evidence or lifecycle state

#### Scenario: Invalid input is non-accepting
- GIVEN unknown options, both inputs, a missing input, malformed JSON or a tampered binding
- WHEN the command runs
- THEN it MUST exit nonzero with an explicit rejection
- AND it MUST NOT report a successful classification

### Requirement: Bounded deterministic replay

r[cairn.revision_classification.bounds] Cairn MUST take comparison and policy-fragment bounds from the checked Nickel export within fixed implementation ceilings and MUST produce identical reports, witnesses and identities for identical admitted inputs.

#### Scenario: Policy bounds are enforced
- GIVEN declared world, step, evidence, domain, observable or fragment limits
- WHEN an input exceeds a limit
- THEN the result MUST be an explicit bound outcome or `undetermined`
- AND it MUST NOT fall back to hidden defaults

#### Scenario: Identical inputs replay identically
- GIVEN identical admitted revisions, correspondence, context and policy
- WHEN the comparison runs twice
- THEN the classification, relation, witnesses, input identity and report identity MUST be identical

### Requirement: Consumer-visible conformance corpus

r[cairn.revision_classification.conformance] Cairn MUST retain positive and negative consumer-level cases for equivalent, stronger, weaker, incomparable, unsupported and inconclusive revisions, policy tightening and widening, infeasible acceptance, witness replay and stale-evidence refusal.

#### Scenario: Corpus distinguishes every classification
- GIVEN the retained core and CLI cases
- WHEN the corpus runs
- THEN each named classification and refusal MUST be observable
- AND a stale witness, correspondence or binding MUST be rejected rather than accepted
