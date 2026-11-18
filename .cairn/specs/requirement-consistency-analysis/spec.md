# Requirement Consistency Analysis Specification

## Purpose

Defines the `requirement-consistency-analysis` capability.

## Requirements

### Requirement: Nickel-owned bounded analysis policy

r[cairn.requirement_consistency.policy] Cairn MUST take the enabled analysis query
vocabulary and every analysis bound from one reviewed Nickel projection, and MUST
reject an unsupported schema, an empty or duplicated vocabulary, or bounds outside
the implementation ceilings explicitly.

#### Scenario: Admitted bounds bound every search
- GIVEN a checked analysis policy with contract, guarantee, cell, state,
  evaluation and nesting bounds
- WHEN a case is analyzed
- THEN every search MUST charge its evaluations and states against those bounds
- AND an exhausted bound MUST produce `undetermined` rather than a verdict.

#### Scenario: Disabled or unknown vocabulary fails explicitly
- GIVEN a policy that enables no query, duplicates a query or names an unsupported
  schema
- WHEN the policy is admitted
- THEN Cairn MUST return an explicit unsupported or bound-exhaustion outcome
- AND a query kind the policy does not enable MUST NOT be evaluated or reported as
  a positive conclusion.

### Requirement: Exact revision-bound query scope

r[cairn.requirement_consistency.scope] Cairn MUST bind every consistency query to
exact native content-bound requirement rows, normalized contract, interpretation
and observable-mapping identities, declared finite domains and total observable
cells, contract-local quantified variables, the selected requirement set and the
shared environmental assumptions.

#### Scenario: Contract-local variables stay separate
- GIVEN two selected contracts that quantify equal variable names over different
  declared domains
- WHEN the analysis composes their assumptions, activations and guarantees
- THEN each variable MUST remain bound inside its own contract and declared domain
- AND no alpha-equivalence, rename or shared variable MUST be inferred.

#### Scenario: Cross-contract identity requires an explicit declaration
- GIVEN two contracts with equal observable names and contradictory guarantee
  literals
- WHEN the case declares a reviewed correspondence for those observables with
  agreeing declared domains
- THEN the disputed guarantees MUST be reported as conflicting with a retained
  minimal subset
- AND without that declaration the analysis MUST report independence instead of
  inventing a shared observation, and a correspondence whose declared domains
  disagree MUST be rejected.

#### Scenario: Unadmitted input is rejected, not analyzed
- GIVEN an unsupported profile, a stale native revision, an absent or duplicated
  selection, a malformed subject identity or an invalid retained correspondence
  review
- WHEN the case is admitted
- THEN Cairn MUST return the corresponding explicit error
- AND it MUST NOT substitute a default profile, revision or reviewer.

### Requirement: Bounded finite queries with explicit statuses

r[cairn.requirement_consistency.queries] Cairn MUST answer four separate bounded
queries over admitted contracts — shared-assumption satisfiability, per-requirement
activation reachability, joint guarantee compatibility and whether withholding one
guarantee admits a previously excluded behavior — and MUST report exactly
`consistent`, `inconsistent`, `vacuous`, `undetermined` or `unsupported` for each.

#### Scenario: Witnesses and complete searches are distinguished
- GIVEN a satisfiable query and an unsatisfiable query inside the declared finite
  domains
- WHEN the analysis runs
- THEN the satisfiable query MUST report `consistent` with total observable tables
  and a witness basis
- AND the unsatisfiable query MUST report `inconsistent` only with a complete
  search basis that retains the explored states, evaluations and domain
  cardinalities.

#### Scenario: Withholding a guarantee is the removal and weakening test
- GIVEN several selected guarantees under one shared assumption set
- WHEN one guarantee is withheld and negated
- THEN a found model MUST be reported as an admitted distinguishing behavior for
  that guarantee
- AND no found model MUST be reported as redundancy inside that model rather than
  as an instruction to delete the requirement.

#### Scenario: Vacuity is relative to a satisfiable shared premise
- GIVEN guarantees that jointly have no model while the shared assumptions and
  activations do
- WHEN the guarantee queries run
- THEN joint compatibility MUST report `inconsistent`, not `vacuous`
- AND an unsatisfiable shared premise MUST report `vacuous` with a vacuous
  requirement-set finding.

### Requirement: Nonvacuity, triviality and indistinguishability diagnostics

r[cairn.requirement_consistency.nonvacuity] Cairn MUST distinguish unreachable
activation from inconsistent assumptions, detect vacuous implications, retain
conflicting subsets with separate minimality, identify guarantees redundant within
the selected model, and name single-instance quantifiers and domains that observable
projections cannot distinguish.

#### Scenario: Genuine vacuity survives identical guarantee text
- GIVEN two requirements with an identical guarantee formula where one contract's
  assumptions make activation impossible and the other's do not
- WHEN the analysis runs
- THEN the impossible contract MUST report a vacuous activation and the reachable
  contract a witness
- AND the result MUST NOT depend on predicate or fragment names, which are equal.

#### Scenario: Minimality and triviality are not overstated
- GIVEN a conflicting pair and a quantifier over a single-valued domain
- WHEN findings are reported
- THEN minimality MUST be claimed only after every smaller subset was separately
  shown satisfiable
- AND the trivial quantification finding MUST name the exact variable, domain and
  formula fragment without claiming the requirement is false.

### Requirement: Replayable witnesses

r[cairn.requirement_consistency.replay] Cairn MUST retain every found model as
complete total observable tables bound to exact contract identities and MUST
provide a pure validator that re-admits the case and tables, independently checks
them, and reports replay success or an explicit rejection.

#### Scenario: A retained model replays
- GIVEN a reported witness for one exact query
- WHEN replay runs against the same admitted case
- THEN the tables MUST re-admit through the typed-contract evaluator, the
  whole-contract booleans MUST agree with the analysis evaluator, and the named
  query MUST hold
- AND the returned replay MUST state the exact query and per-contract evaluations.

#### Scenario: Tampering is an explicit rejection
- GIVEN a changed table value, contract identity, model identity or query label
- WHEN replay runs
- THEN it MUST report `replays: false` with the rejection reason
- AND it MUST NOT panic, silently accept the witness, or turn the mismatch into a
  proof or lifecycle outcome.

### Requirement: Five statuses are never proof or authority

r[cairn.requirement_consistency.statuses] Cairn MUST NOT report `consistent` from
an exhausted bound, a disabled query, an unsupported construct or an incomplete
exploration, and MUST NOT treat any finite witness as unbounded validity, reactive
realizability, prose correspondence or implementation correctness.

#### Scenario: Bound exhaustion stays undetermined
- GIVEN a bound too small to decide the query
- WHEN the search stops
- THEN the outcome MUST be `undetermined` with an exhausted basis
- AND no finding, witness or report field MAY describe the requirement set as
  consistent.

#### Scenario: Findings attach without resolving
- GIVEN an attached native obligation and a generated finding
- WHEN the report is produced
- THEN the finding MUST keep disposition `unresolved` and the advisory boundary
- AND no obligation state, readiness state or gate verdict MUST change.

### Requirement: Read-only bounded inspection

r[cairn.requirement_consistency.inspection] Cairn MUST expose the analysis through
one read-only command that verifies the native registry, rejects stale native rows,
supports optional retained-witness replay, and states that no proof evidence or
lifecycle authority is produced.

#### Scenario: Inspection cannot become proof or mutation
- GIVEN a well-formed case, a current native registry and an admitted policy
- WHEN inspection runs with or without a retained witness
- THEN it MUST print the report, per-query statuses and the replay result, and
  state that no proof evidence was supplied and no lifecycle authority is granted
- AND it MUST NOT execute checks, resolve obligations, mutate lifecycle state or
  change any legacy command default.
