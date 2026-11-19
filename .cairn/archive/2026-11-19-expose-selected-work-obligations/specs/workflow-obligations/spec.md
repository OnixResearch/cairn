# Workflow obligation delta

## ADDED Requirements

### Requirement: Typed per-obligation consumer facts

r[cairn.workflow_obligations.consumer_facts] Cairn MUST derive typed per-obligation facts from the same admitted definition, predicate and guidance used by evaluation. Each fact MUST carry its node kind, parent, alternative, children, selection state, applicability conditions, exact native evidence references, expected observations, command classes, temporal check, native exception reference and recomputable obligation, guidance and remaining-condition identities. A consumer verifier MUST refuse facts that omit nodes or native requirements, promote an unselected alternative, alter an evidence revision or carry stale identities.

#### Scenario: Selection changes presentation identities but not meaning
- GIVEN the `verify` definition with and without a selected alternative
- WHEN facts are derived
- THEN obligation identities MUST stay equal, and guidance and remaining-condition identities MUST change.

#### Scenario: Tampered facts
- GIVEN facts derived for a predicate
- WHEN an evidence revision, selection state, exception fact, node or identity is changed or removed
- THEN verification MUST fail.
