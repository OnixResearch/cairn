# Support Maintenance Specification

## Purpose

Defines the `support-maintenance` capability.

## Requirements

### Requirement: Alternative support branches remain distinct

r[cairn.support_maintenance.alternatives] Cairn MUST represent alternative support for one claim as separate derivation branches, each naming its own premise conjunction, polarity, rule identity, assurance scope, applicability and residual assumptions, and MUST NOT flatten alternatives into jointly required evidence.

#### Scenario: A surviving alternative keeps the claim supported
- GIVEN a claim with one `(A AND B)` branch and one `C` branch in the same exact scope
- WHEN premise B becomes inapplicable
- THEN the first branch MUST be withdrawn and the `C` branch MUST survive
- AND the claim MUST remain supported only by the surviving branch

#### Scenario: Refuting branches remain distinct alternatives
- GIVEN a supporting branch and a refuting branch for the same exact claim
- WHEN both branches have cleanly supported premises
- THEN the report MUST retain the support link and the counterevidence link separately
- AND it MUST NOT convert the refuting branch into support or drop the support branch

### Requirement: Exact branch scope and explicit relations

r[cairn.support_maintenance.scope] Cairn MUST admit a branch only when its conclusion applicability and every premise context are exactly equal, and MUST require an explicit admitted relation identity before any premise from another requirement or interpretation revision can contribute.

#### Scenario: Cross-scope premises are rejected
- GIVEN a branch whose premise names another subject, environment, bound scope, requirement revision or interpretation
- WHEN the branch declares no admitted relation
- THEN Cairn MUST return an explicit scope-mismatch outcome
- AND it MUST NOT attach that premise or widen the branch applicability

#### Scenario: A declared relation still needs current applicability
- GIVEN a cross-revision branch with a non-empty relation identity
- WHEN either revision's policy, context or requirement applicability is not current
- THEN the branch MUST be withdrawn with the triggering change as its reason
- AND the relation MUST NOT be treated as an automatic evidence transfer

### Requirement: Shared assumptions withdraw every dependent branch

r[cairn.support_maintenance.assumptions] Cairn MUST track each branch's and evidence record's residual assumptions and MUST withdraw every branch or record that depends directly or transitively on a withdrawn assumption, retaining the shared label and historical reason.

#### Scenario: One assumption affects all dependent derivations
- GIVEN two alternative branches that both declare one shared assumption
- WHEN that assumption is withdrawn
- THEN every dependent branch MUST be withdrawn
- AND each explanation MUST name the same withdrawn assumption and triggering change

#### Scenario: Unrelated alternatives remain available
- GIVEN one branch that depends on a withdrawn assumption and another branch that does not
- WHEN evaluation recomputes the claim
- THEN only the dependent branch MUST be withdrawn
- AND the independent branch MUST remain available for the claim

### Requirement: Invalidation is not refutation

r[cairn.support_maintenance.invalidation] Cairn MUST remove current applicability of stale or withdrawn evidence without rewriting its historical polarity, and MUST distinguish missing applicable support from applicable counterevidence.

#### Scenario: A stale receipt becomes inapplicable
- GIVEN a supported observation backed by one saved receipt
- WHEN the receipt is marked stale
- THEN the claim MUST become unresolved with no counterevidence
- AND the historical observation MUST remain recorded with its withdrawal reason

#### Scenario: Admitted counterevidence remains visible
- GIVEN a support branch that survives after one alternative is withdrawn
- WHEN an applicable counterevidence record is admitted for the same exact claim
- THEN the claim MUST remain disputed
- AND the report MUST retain both the surviving support link and the counterevidence link

### Requirement: Dispute remains visible beside surviving alternatives

r[cairn.support_maintenance.dispute] Cairn MUST retain applicable support and refuting branches together as a disputed state and MUST NOT let one surviving support alternative conceal another branch's declared counterevidence or make missing evidence look refuted.

#### Scenario: Surviving support does not cancel counterevidence
- GIVEN a claim with a surviving support branch and an applicable refuting branch
- WHEN the claim is explained
- THEN its state MUST be disputed with both polarities present
- AND the surviving branch MUST NOT be reported as a clean support verdict

#### Scenario: Missing evidence is not refutation
- GIVEN a claim with neither applicable support nor applicable counterevidence
- WHEN the claim is explained
- THEN its state MUST be unresolved with empty support and counterevidence
- AND a withdrawn refuting branch MUST retain its own historical reason separately

### Requirement: Bounded ordered changes with historical reasons

r[cairn.support_maintenance.revisions] Cairn MUST apply a bounded, strictly ordered change log for evidence invalidation, assumption withdrawal, admitted evidence, claim revision, and policy or context changes, and MUST explain each withdrawn branch from its triggering change without transferring support automatically.

#### Scenario: Revision classification does not transfer support
- GIVEN a prior claim and a new requirement revision with an admitted classification
- WHEN the new claim has no explicitly admitted relation branch
- THEN the new claim MUST remain unresolved
- AND the old claim MUST remain a historical supported observation

#### Scenario: Explicit relation and current applicability are required
- GIVEN a branch that connects a premise from another requirement revision
- WHEN the branch declares no admitted relation
- THEN evaluation MUST reject it as a scope mismatch
- AND a branch with an explicit relation and current common applicability MUST be the only path that can support the new revision

#### Scenario: Policy and context changes are scoped
- GIVEN claims selected under an old policy hash or context identity
- WHEN that policy or context is changed
- THEN only claims, evidence and branches in the old scope MUST lose applicability
- AND claims in the new scope MUST remain unaffected

### Requirement: Deterministic full recomputation

r[cairn.support_maintenance.determinism] Cairn MUST compute support by deterministic full recomputation from immutable records and the ordered change log, with canonical ordering and stable input/report identities.

#### Scenario: Equivalent inputs replay identically
- GIVEN the same admitted claims, branches, evidence, obligations and change sequence in any array order
- WHEN evaluation runs twice
- THEN support, counterevidence, surviving and withdrawn branches, causes, affected claims and report identity MUST be identical

#### Scenario: Event order is explicit and visible
- GIVEN two changes with distinct sequences
- WHEN the supplied array order is reversed
- THEN the applied change order MUST follow the explicit sequences
- AND changing those sequences MUST change the reported order and report identity

### Requirement: Bounds and incomplete analysis remain visible

r[cairn.support_maintenance.bounds] Cairn MUST take claim, branch, evidence, premise, assumption, event, obligation and depth bounds from the checked policy export within fixed implementation ceilings and MUST reject out-of-bound or cyclic analysis explicitly rather than certifying a complete unaffected-support set.

#### Scenario: A bound failure is explicit
- GIVEN a case that exceeds an admitted branch or depth bound
- WHEN evaluation runs
- THEN Cairn MUST return an explicit bound-exhaustion outcome
- AND it MUST NOT return a partial result as complete

#### Scenario: Completed analysis exposes its limits
- GIVEN a case within every admitted bound
- WHEN the report is printed
- THEN it MUST include the limits, observed counts, maximum depth and full-recomputation mode

### Requirement: Read-only advisory inspection

r[cairn.support_maintenance.inspection] Cairn MUST provide a read-only advisory CLI explanation of a support-maintenance case with bounded reads and explicit admission errors, without lifecycle mutation, registry ownership or a new completion decision.

#### Scenario: Surviving and withdrawn alternatives are explained
- GIVEN a valid case with alternatives and an evidence invalidation
- WHEN `cairn support-maintenance explain` runs
- THEN it MUST print surviving branches, withdrawn branches with causes, claim states, affected obligations and report identity
- AND it MUST NOT write lifecycle state or create a registry

#### Scenario: Rejection is visible and non-accepting
- GIVEN malformed JSON, an unsupported schema or an unadmitted change kind
- WHEN explanation runs
- THEN it MUST print an explicit rejected outcome and exit non-zero

### Requirement: Native authority is preserved

r[cairn.support_maintenance.authority] Support maintenance MUST remain advisory and MUST NOT mutate native lifecycle decisions, authorize an external effect, transfer evidence between revisions, or change applicability, readiness, verification-role or `Pass | Block` gate meanings.

#### Scenario: Surviving support does not authorize action
- GIVEN a claim with a surviving support alternative
- WHEN the explanation is printed
- THEN it MUST state that support is not an accepted exception, authorization, readiness or behavioral satisfaction
- AND it MUST NOT alter any existing gate or lifecycle decision
