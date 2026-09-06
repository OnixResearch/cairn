# Change verification guidance

## ADDED Requirements

### Requirement: Exact impact inputs
r[cairn.verification_impact.inputs] The system MUST admit explicit before and after source identities, changed paths, trace indexes, requirement registries, and a repository-owned check map.

#### Scenario: A complete source comparison is admitted
- GIVEN exact source identities and current matching trace and check-map inputs
- WHEN the operator requests verification guidance
- THEN the result binds the complete selected input set and its observation scope

#### Scenario: Invalid source comparisons remain blocked
- GIVEN a stale index, mismatched root, unsafe path, invalid range, or unrecognized source adapter
- WHEN the model admits the comparison
- THEN it reports a bounded input blocker without inferring impact from filenames alone

### Requirement: Explainable conservative selection
r[cairn.verification_impact.selection] The system MUST derive affected requirements and declared checks with stable causal paths while preserving the complete required gate set.

#### Scenario: A linked edit selects its declared check
- GIVEN a changed source member linked to a requirement and a required check
- WHEN the pure model derives impact
- THEN it selects that check and records the source-to-requirement-to-check explanation

#### Scenario: Missing mapping cannot justify omission
- GIVEN a changed source member without complete admitted dependency or check mapping
- WHEN the model derives impact
- THEN it reports uncertainty and retains the broader configured verification set

### Requirement: Explicit coverage uncertainty
r[cairn.verification_impact.uncertainty] The system MUST preserve unknown coverage for unsupported constructs, incomplete graphs, and scope mismatches rather than claiming independent behavior.

#### Scenario: Structural producer evidence strengthens only its declared scope
- GIVEN current Octet structural evidence with matching Valence source and span identities
- WHEN the model consumes the evidence
- THEN it preserves the producer assurance level and extends only the declared impact relationships

#### Scenario: Opaque or conflicting coverage requires conservative work
- GIVEN macro opacity, generated-source gaps, a cycle, feature drift, or conflicting producer edges
- WHEN the model evaluates coverage
- THEN it exposes the unknown or invalid scope and does not suppress affected required checks

### Requirement: Evidence refresh remains subject-bound
r[cairn.verification_impact.applicability] The system MUST evaluate refresh needs against existing evidence applicability rules without weakening exact final-subject or policy requirements.

#### Scenario: Existing evidence matches all required identities
- GIVEN evidence current for the selected subject, policy, dependencies, target, and feature scope
- WHEN the report evaluates refresh needs
- THEN it reports applicability according to the existing validator

#### Scenario: An unlinked edit cannot preserve whole-subject evidence
- GIVEN a changed final subject and a previous completion receipt
- WHEN the changed file has no declared requirement link
- THEN the old whole-subject receipt remains stale despite that missing link

### Requirement: Read-only action integration
r[cairn.verification_impact.interfaces] The system MUST expose equivalent CLI and MCP guidance with typed actions compatible with unified readiness and check-evidence plans, without executing checks.

#### Scenario: An admitted selection produces a check plan request
- GIVEN a valid guidance result and supported producer profile
- WHEN the operator inspects the next action
- THEN it names the exact producer-plan request and its prerequisites without authorizing execution

#### Scenario: Advisory guidance cannot remove full-gate obligations
- GIVEN a focused selection that fits the local loop
- WHEN a consumer requests release or archive readiness
- THEN the complete repository-owned gate contract remains authoritative

### Requirement: Positive and negative selection evidence
r[cairn.verification_impact.conformance] The system MUST retain deterministic edit, deletion, rename, marker, graph, scope, and stale-evidence fixtures with omission mutation controls.

#### Scenario: Equivalent comparisons preserve selection identity
- GIVEN equal admitted comparisons with different input ordering
- WHEN the model derives guidance
- THEN the selected checks, explanations, uncertainty, and semantic identity agree

#### Scenario: An unsafe omission mutant fails acceptance
- GIVEN a mutant that treats missing dependency coverage as independence
- WHEN the negative fixture evaluates the mutant
- THEN the required-check preservation assertion fails
