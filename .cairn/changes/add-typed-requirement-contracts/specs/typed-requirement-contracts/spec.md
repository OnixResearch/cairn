# Typed requirement contracts specification delta

## ADDED Requirements

### Requirement: Exact optional finite requirement meaning

r[cairn.typed_requirements.finite_contract] Cairn MUST support one optional versioned finite semantics profile directly bound to exact native content-bound requirement rows, with deterministic normalization and BLAKE3 identities covering requirement revision, interpretation, observable mapping and contract.

#### Scenario: Five pilot meanings bind native accepted revisions
- GIVEN the five Roadmap #4 requirements and their current native registry rows
- WHEN their Nickel-authored contracts are normalized
- THEN each contract MUST preserve its exact row and observable mapping, supported portion and informal remainder
- AND identical admitted inputs MUST produce identical normalized contracts and identities without replacing Markdown authority or creating another registry.

#### Scenario: Invalid or unsupported formulas fail explicitly
- GIVEN ill-typed expressions, free variables, unknown profiles, unsupported arithmetic/temporal constructs or exhausted resource bounds
- WHEN the pure core admits or evaluates the contract
- THEN it MUST return the corresponding explicit outcome without interpreting unsupported behavior, silently weakening a formula or reporting proof.

### Requirement: Revision-bound independent correspondence declarations

r[cairn.typed_requirements.correspondence] Cairn MUST distinguish well-formed contracts from independently reviewed correspondence and MUST admit review declarations only when they bind the exact current native row, interpretation, contract, observable mapping and proposer under explicit reviewer policy.

#### Scenario: Current independently retained review is admitted as a declaration
- GIVEN an actual retained approved correspondence review by an authorized reviewer different from the revision-bound proposer
- WHEN every review identity matches the normalized contract
- THEN admission MUST return a deterministic review identity while stating the external reviewer/artifact authenticity trust boundary.

#### Scenario: Revision or self-review invalidates correspondence
- GIVEN changed requirement, contract, interpretation or mapping identities, a self-review, absent authorization or a proposed-only record
- WHEN correspondence admission runs
- THEN the old or unauthorized review MUST be rejected
- AND EARS keywords, structured validation and example success MUST NOT fabricate approval.

### Requirement: Pure finite example evaluation is not proof

r[cairn.typed_requirements.examples] Cairn MUST evaluate typed finite equality, membership, Boolean connectives, declared-domain quantifiers and single-step relations using supplied total observations while keeping example evaluation separate from admitted proof evidence and unbounded or source correctness.

#### Scenario: Concrete duplicate and stale states are distinguishable
- GIVEN selected finite checks and result slots or final-subject identities
- WHEN one example has exactly one eligible passed row per required check and another has a duplicate, missing, unknown, non-passing or stale row
- THEN the finite contracts MUST distinguish their guarantees even though a predicate-name classification is identical.

#### Scenario: Example tables and vacuity remain explicit
- GIVEN supplied finite observable tables and bounded interpretation
- WHEN evaluation runs
- THEN non-total, duplicate, out-of-domain or unbound observations MUST be rejected
- AND assumptions, activation, guarantee, implication, resource exhaustion and example-only scope MUST remain separately visible.

### Requirement: Read-only opt-in semantic inspection

r[cairn.typed_requirements.inspection] Cairn MUST provide read-only CLI inspection of normalized meaning, correspondence and optional finite example evaluation with explicit separate well-formedness, reviewed-declaration, example and admitted-proof statuses while preserving existing requirement wire formats and command defaults.

#### Scenario: Missing contract stays visibly informal
- GIVEN a current native requirement without a typed contract
- WHEN inspection runs without example or review requests
- THEN output MUST identify it as informal rather than a failed or proved requirement
- AND existing commands and legacy requirements MUST retain their prior behavior.

#### Scenario: Inspection does not grant completion authority
- GIVEN a well-formed contract, admitted review declaration or true finite example
- WHEN the CLI prints the inspection
- THEN it MUST state that no proof evidence was supplied or admitted through this interface
- AND it MUST NOT execute checks, mutate lifecycle state or upgrade any legacy completion verdict.
