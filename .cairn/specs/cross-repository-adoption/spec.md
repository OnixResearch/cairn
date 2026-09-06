# Cross Repository Adoption Specification

## Purpose

Defines the `cross-repository-adoption` capability.

## Requirements

### Requirement: Explicit producer snapshots
r[cairn.adoption_report.inputs] The system MUST admit only explicitly supplied, bounded, versioned dependency, health, lifecycle, and consumer-evidence snapshots with exact source and policy identities.

#### Scenario: A supported receipt cohort forms a report input
- GIVEN current matching stack-deps audit and reconciliation receipts plus explicit lifecycle and consumer evidence
- WHEN the operator requests a report for the selected scope
- THEN the report binds each source identity and the exact supplied observation set

#### Scenario: Registry presence cannot repair a missing input
- GIVEN missing upstream evidence or an unsupported dependency receipt
- WHEN a registry also contains a matching repository label
- THEN the missing or unsupported input remains a blocker without registry or network fallback

### Requirement: Separate adoption stages
r[cairn.adoption_report.stages] The system MUST distinguish upstream lifecycle completion, consumer pin observation, consumer compatibility, and current integration evidence for each selected relationship.

#### Scenario: Exact consumer evidence supports integration state
- GIVEN admitted upstream completion, a matching consumer pin, and current required consumer checks
- WHEN the report evaluates the relationship
- THEN it identifies the supported integration state and its bounded evidence references

#### Scenario: Upstream completion alone leaves an adoption gap
- GIVEN an archived upstream change and an unchanged consumer pin
- WHEN the report evaluates the relationship
- THEN it reports the pin gap without claiming downstream readiness

### Requirement: Unknown and unsafe states remain visible
r[cairn.adoption_report.unknowns] The system MUST preserve producer-owned behind, ahead, diverged, missing, unavailable, blocked, stale, and partial-scope observations without optimistic normalization.

#### Scenario: A known blocked relationship has a bounded explanation
- GIVEN a consumer check failure with exact safe evidence references
- WHEN the report derives the relationship state
- THEN it reports the failed integration stage and the responsible consumer action

#### Scenario: Cycles and conflicting revisions cannot create update authority
- GIVEN an audit-only cycle or contradictory observations for one relationship
- WHEN the report derives adoption state
- THEN it preserves the cycle or conflict without inventing a bulk update order or successful state

### Requirement: Deterministic bounded queries
r[cairn.adoption_report.queries] The system MUST provide equivalent CLI and MCP report queries with explicit scope, stable order, canonical identities, provenance, and typed next actions.

#### Scenario: Equivalent receipt sets produce equal semantic reports
- GIVEN equal admitted receipts in different input orders
- WHEN the report queries the same relationships
- THEN the semantic rows, actions, and portable report identity agree

#### Scenario: Partial observation cannot imply complete workspace health
- GIVEN a report limited to selected relationships
- WHEN the query omits another known consumer or exhausts a configured bound
- THEN the receipt preserves partial coverage and does not claim complete workspace adoption

### Requirement: External owners retain authority
r[cairn.adoption_report.boundary] The system MUST retain workspace catalog and producer semantics outside Cairn and perform no sibling scan, transport, pin update, check execution, or release mutation.

#### Scenario: Saved-input reporting needs no live sibling workspace
- GIVEN the same portable input bundle in a fresh directory
- WHEN the operator requests the adoption report
- THEN the semantic result agrees without access to producer checkouts or services

#### Scenario: A remediation hint grants no mutation permission
- GIVEN a relationship that needs a stack-deps plan or consumer check
- WHEN the report emits its next action
- THEN it identifies the external owner and prerequisites without running the action or changing lifecycle readiness

### Requirement: Maintained consumer-chain conformance
r[cairn.adoption_report.conformance] The system MUST retain positive and negative Cairn-to-Octet and Octet-to-Lattice fixtures with direct producer-contract comparison.

#### Scenario: A complete saved chain reports the expected bounded stages
- GIVEN the maintained current consumer-chain fixture
- WHEN both transports derive the adoption report
- THEN they agree with the producer-owned relationship states and consumer evidence checks

#### Scenario: Matching pins without checks remain incomplete
- GIVEN matching consumer pins but missing, stale, failed, or wrong-subject integration evidence
- WHEN the same fixture evaluates adoption
- THEN it reports incomplete integration without promoting pin equality to compatibility or release readiness
