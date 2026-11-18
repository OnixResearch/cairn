# Scoped claim derivations specification delta

## ADDED Requirements

### Requirement: Versioned exact claim identity

r[cairn.scoped_claims.identity] Cairn MUST support an optional versioned scoped claim that binds an exact native content-bound requirement row, interpretation identity, contract identity, subject, environment, bounds, declared claim kind and proposition, with unknown schemas rejected explicitly.

#### Scenario: Scope components are part of the identity
- GIVEN a claim with a revision, subject, target, environment and bounds
- WHEN any one of those components changes
- THEN the claim identity MUST change
- AND a claim whose declared kind differs from its proposition MUST be rejected rather than relabeled.

#### Scenario: Missing or unknown identity components fail
- GIVEN an unborn claim schema, absent subject or bounds, or a malformed requirement/interpretation/contract identity
- WHEN the claim is admitted
- THEN Cairn MUST return an explicit unsupported or invalid-input outcome
- AND it MUST NOT substitute a default revision, scope or kind.

### Requirement: Closed bounded acyclic derivation rules

r[cairn.scoped_claims.rules] Cairn MUST admit derivations only from a small closed versioned rule vocabulary whose derivations name exact premises, conclusion, applicability, assurance scope and residual assumptions, and MUST replay them deterministically under declared bounds.

#### Scenario: Admitted rules establish only their declared conclusion
- GIVEN one `NativeCompletionEligible` premise and one `ReviewedCorrespondence` premise in the same exact scope
- WHEN `reviewed_completion_v1` is applied
- THEN it MUST establish `ReviewedCompletionDecision` with the policy-declared assurance scope and both premises recorded
- AND the rule MUST NOT establish observations, behavioral satisfaction or any broader conclusion.

#### Scenario: Unsupported rules, cycles and missing premises stay explicit
- GIVEN an unadmitted rule identity or version, a different assurance scope, a premise set outside the rule's shape, or an undeclared premise
- WHEN the derivation graph is admitted
- THEN the unsupported cases MUST be rejected explicitly and any graph cycle MUST be reported as an explicit cycle outcome rather than truncated
- AND the undeclared premise MUST remain an explicit unresolved premise rather than being invented or ignored.

### Requirement: Admitted native producer conclusions only

r[cairn.scoped_claims.native_producers] Cairn MUST re-admit native producer inputs through their existing validators and MUST permit derivations to reference only the conclusions those validators actually established.

#### Scenario: Exact selected check observations are admitted
- GIVEN an authoritative agent-completion contract and saved result rows
- WHEN the native evaluator accepts exactly one passing selected check
- THEN the exact `CheckPassed` observation in the same scope MUST be supported
- AND missing, duplicate, blocked, unsupported-status or timed-out rows MUST establish neither support nor refutation.

#### Scenario: Structural receipts remain recorded-only
- GIVEN an ast-grep receipt that passes the native receipt validator
- WHEN the structural producer is admitted
- THEN only `ReceiptStructurallyValid` MAY be supported with a recorded-only assurance scope
- AND that evidence MUST NOT be reinterpreted as behavioral proof, completion eligibility or requirement satisfaction.

### Requirement: Visible support, refutation and dispute states

r[cairn.scoped_claims.states] Cairn MUST report unresolved, supported, refuted and disputed claim states from applicable admitted support and counterevidence, retaining both polarities together and applying counterevidence only to its exact proposition and scope.

#### Scenario: Failed evidence refutes only its exact observation
- GIVEN an unambiguous failed selected check row
- WHEN its claim is explained
- THEN only that exact `CheckPassed` observation MUST be refuted
- AND unrelated claims and a composite completion judgment MUST NOT be refuted by it.

#### Scenario: Applicable support and refutation remain visible together
- GIVEN two independently admitted native evaluations with compatible scope, one supporting and one refuting the same proposition
- WHEN the claim is explained
- THEN both links MUST remain visible as a disputed state
- AND neither MUST silently cancel the other or justify an unrelated claim.

### Requirement: Exact scope separation

r[cairn.scoped_claims.scope] Cairn MUST NOT combine claims, derivations or producer conclusions across different requirement revisions, subjects, targets, environments or bounds without an explicitly admitted relation.

#### Scenario: Cross-scope combinations fail
- GIVEN a claim from another revision, a derivation whose applicability differs from its conclusion, a premise from another environment, or a producer whose native repository identity differs from the declared target
- WHEN the request is evaluated
- THEN Cairn MUST return an explicit scope-mismatch outcome
- AND it MUST NOT attach that evidence to the claim or widen its applicability.

#### Scenario: Same-scope evidence is attached exactly
- GIVEN authority evidence whose proposition and complete context equal a declared claim
- WHEN the evidence is admitted
- THEN it MUST be attached to that claim only
- AND identical propositions in other contexts MUST remain unaffected.

### Requirement: Explicit bounds and residual assumptions

r[cairn.scoped_claims.bounds] Cairn MUST take claim, derivation, premise, producer and depth bounds from the checked policy export within fixed implementation ceilings and MUST report bound exhaustion or policy rejection explicitly.

#### Scenario: Policy bounds and vocabulary are enforced
- GIVEN an admitted policy with bounds and an enabled rule vocabulary
- WHEN a request exceeds a bound or uses a disabled rule, kind or proposition
- THEN Cairn MUST return an explicit bound-exhaustion or unsupported outcome
- AND it MUST NOT fall back to hidden defaults or a broader vocabulary.

#### Scenario: Residual trust and oracle dependencies stay shared
- GIVEN native observations that depend on a shared test oracle or verifier
- WHEN explanations are emitted
- THEN each exact premise/rule link MUST retain its residual assumptions
- AND the report MUST deduplicate shared dependencies without claiming actor, oracle or statistical independence.

### Requirement: Deterministic replay

r[cairn.scoped_claims.determinism] Cairn MUST reproduce the same conclusions, states, unresolved premises and residual assumptions when the same admitted inputs are replayed.

#### Scenario: Identical admitted inputs replay identically
- GIVEN admitted claims, producers, review and derivations
- WHEN evaluation runs twice
- THEN the conclusion links, states, unresolved premises, residual assumptions, input identity and report identity MUST be identical.

### Requirement: Read-only advisory inspection

r[cairn.scoped_claims.inspection] Cairn MUST provide a read-only advisory CLI explanation of an actual native completion case with bounded reads, explicit admission errors, and no lifecycle mutation or registry ownership.

#### Scenario: Positive completion case is explained
- GIVEN a case with one native completion producer and a reviewed correspondence
- WHEN `claim-derivation explain` runs
- THEN it MUST print the versioned advisory report with exact links and residual assumptions
- AND it MUST NOT write lifecycle state, create a registry or change existing command defaults.

#### Scenario: Rejection is visible and non-accepting
- GIVEN a malformed case, an unadmitted scope or an unsupported rule
- WHEN explanation runs
- THEN it MUST print an explicit rejected outcome and exit non-zero
- AND an unresolved claim in an otherwise valid evaluation MUST remain a successful advisory explanation rather than a pass.

### Requirement: Native authority is preserved

r[cairn.scoped_claims.authority] Claim support, accepted exceptions and action authorization MUST remain distinct facts; native applicability, readiness, verification roles, stage validators and the `Pass | Block` gate MUST retain their current meanings and authority.

#### Scenario: Advisory support does not authorize action
- GIVEN a supported claim or an admitted reviewed completion decision
- WHEN the CLI prints the explanation
- THEN it MUST state that support is advisory and not an exception, authorization, readiness or behavioral satisfaction
- AND it MUST NOT upgrade any existing completion verdict or execute a lifecycle action.
