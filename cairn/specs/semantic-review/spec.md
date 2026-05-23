# Semantic Review Specification

## Purpose

Defines the `semantic-review` capability.

## Requirements

### Requirement: The system
r[cairn.semantic_review.semantic_review_receipts.schema] The system MUST define a stable semantic review receipt schema for externally produced review outcomes.

#### Scenario: Schema Fields
r[cairn.semantic_review.semantic_review_receipts.schema.schema.fields]
- GIVEN a semantic review receipt with review kind, decision, reviewed inputs, findings, reviewer metadata, and receipt hash
- WHEN Cairn validates the receipt
- THEN it accepts only policy-known fields and emits deterministic issues for missing or malformed fields

#### Scenario: Schema Hash Binding
r[cairn.semantic_review.semantic_review_receipts.schema.schema.hash_binding]
- GIVEN reviewed artifact paths and content hashes in a receipt
- WHEN Cairn validates the receipt against a repository root
- THEN it verifies the referenced root-relative artifacts match their declared hashes

### Requirement: The system
r[cairn.semantic_review.semantic_review_receipts.cli] The system MUST expose semantic review receipt validation through a deterministic CLI command.

#### Scenario: Cli Valid
r[cairn.semantic_review.semantic_review_receipts.cli.cli.valid]
- GIVEN a valid semantic review receipt
- WHEN the operator runs `cairn review validate <receipt>`
- THEN Cairn emits a valid deterministic validation receipt

#### Scenario: Cli Invalid
r[cairn.semantic_review.semantic_review_receipts.cli.cli.invalid]
- GIVEN a malformed semantic review receipt
- WHEN the operator validates it
- THEN Cairn emits stable issue codes without network, LLM, or reviewer execution

### Requirement: The system
r[cairn.semantic_review.semantic_review_receipts.docs] The system MUST document semantic review receipts as externally produced evidence that Cairn validates structurally.

#### Scenario: Docs Non Claims
r[cairn.semantic_review.semantic_review_receipts.docs.docs.non_claims]
- GIVEN an operator reads the semantic review documentation
- WHEN they inspect the command contract
- THEN it states Cairn does not perform the semantic review, call remote models, or prove reviewed work is correct

### Requirement: The system
r[cairn.semantic_review.review_rejection_taxonomy.policy] The system MUST define policy-owned review decision and rejection taxonomy vocabularies.

#### Scenario: Policy Categories
r[cairn.semantic_review.review_rejection_taxonomy.policy.policy.categories]
- GIVEN generated Cairn policy
- WHEN review taxonomy data is loaded
- THEN allowed categories include omission, systematic, incoherent, and infra

#### Scenario: Policy Decisions
r[cairn.semantic_review.review_rejection_taxonomy.policy.policy.decisions]
- GIVEN generated Cairn policy
- WHEN review decisions are loaded
- THEN allowed decisions include approved, needs_revision, escalate, and unknown

### Requirement: The system
r[cairn.semantic_review.review_rejection_taxonomy.finding_validation] The system MUST require categorized findings for non-approved semantic review receipts.

#### Scenario: Finding Validation Required
r[cairn.semantic_review.review_rejection_taxonomy.finding_validation.finding_validation.required]
- GIVEN a semantic review receipt with decision `needs_revision`
- WHEN Cairn validates the receipt
- THEN each finding has a policy-known category and stable finding id

#### Scenario: Finding Validation Approved
r[cairn.semantic_review.review_rejection_taxonomy.finding_validation.finding_validation.approved]
- GIVEN an approved semantic review receipt
- WHEN Cairn validates the receipt
- THEN the receipt may have an empty findings list and still remains valid

### Requirement: The system
r[cairn.semantic_review.completeness_check_rail.mapping] The system MUST produce a deterministic completeness map across requirements, tasks, implementation refs, and verification refs.

#### Scenario: Mapping Active Change
r[cairn.semantic_review.completeness_check_rail.mapping.mapping.active_change]
- GIVEN an active change package
- WHEN Cairn runs the completeness check
- THEN the receipt lists requirement ids and their linked tasks, implementation refs, and verification refs

#### Scenario: Mapping Accepted Spec
r[cairn.semantic_review.completeness_check_rail.mapping.mapping.accepted_spec]
- GIVEN accepted specs with durable refs
- WHEN Cairn runs the completeness check
- THEN the receipt includes accepted requirement coverage without mutating the repository

### Requirement: The system
r[cairn.semantic_review.completeness_check_rail.omissions] The system MUST emit policy-owned omission findings for missing completeness links.

#### Scenario: Omissions Missing Task
r[cairn.semantic_review.completeness_check_rail.omissions.omissions.missing_task]
- GIVEN a requirement with no task or dependency mapping
- WHEN completeness is checked
- THEN Cairn emits an omission finding for missing task coverage

#### Scenario: Omissions Missing Verification
r[cairn.semantic_review.completeness_check_rail.omissions.omissions.missing_verification]
- GIVEN a requirement with implementation refs but no verification refs
- WHEN completeness is checked
- THEN Cairn emits an omission finding for missing verification coverage

### Requirement: The system
r[cairn.semantic_review.review_ledger_analytics.aggregate] The system MUST aggregate semantic review receipts into deterministic trust-ledger analytics.

#### Scenario: Aggregate Counts
r[cairn.semantic_review.review_ledger_analytics.aggregate.aggregate.counts]
- GIVEN a set of valid semantic review receipts
- WHEN Cairn builds review ledger analytics
- THEN the receipt reports counts by review kind, decision, category, and source

#### Scenario: Aggregate Hashes
r[cairn.semantic_review.review_ledger_analytics.aggregate.aggregate.hashes]
- GIVEN multiple input review receipts
- WHEN Cairn builds analytics
- THEN the output includes sorted input receipt hashes and a stable analytics receipt hash

### Requirement: The system
r[cairn.semantic_review.review_ledger_analytics.input_validation] The system MUST fail closed on invalid ledger inputs.

#### Scenario: Input Validation Invalid Receipt
r[cairn.semantic_review.review_ledger_analytics.input_validation.input_validation.invalid_receipt]
- GIVEN a directory containing an invalid review receipt
- WHEN ledger analytics run
- THEN Cairn reports a deterministic invalid-input issue and does not include that receipt in approved aggregates

#### Scenario: Input Validation Sample Warning
r[cairn.semantic_review.review_ledger_analytics.input_validation.input_validation.sample_warning]
- GIVEN a category or gate kind with a policy-small sample count
- WHEN analytics are emitted
- THEN the receipt includes a warning rather than hiding the small sample

### Requirement: The system
r[cairn.semantic_review.revision_handoff_receipts.schema] The system MUST define a revision handoff receipt schema that links review feedback to a revision attempt.

#### Scenario: Schema Chain
r[cairn.semantic_review.revision_handoff_receipts.schema.schema.chain]
- GIVEN a revision handoff receipt
- WHEN Cairn validates it
- THEN it verifies the prior review receipt hash, revision artifact hashes, finding dispositions, and receipt hash

#### Scenario: Schema Dispositions
r[cairn.semantic_review.revision_handoff_receipts.schema.schema.dispositions]
- GIVEN prior review findings
- WHEN a revision handoff is validated
- THEN each prior finding has a policy-known disposition

### Requirement: The system
r[cairn.semantic_review.revision_handoff_receipts.validation] The system MUST validate revision handoff chains deterministically.

#### Scenario: Validation Addressed Evidence
r[cairn.semantic_review.revision_handoff_receipts.validation.validation.addressed_evidence]
- GIVEN a finding marked addressed
- WHEN Cairn validates the handoff
- THEN the finding includes evidence refs or artifact hashes required by policy

#### Scenario: Validation New Findings
r[cairn.semantic_review.revision_handoff_receipts.validation.validation.new_findings]
- GIVEN a follow-up review that introduces new findings
- WHEN Cairn validates the handoff
- THEN the receipt preserves those findings as new rather than overwriting prior findings

### Requirement: The system
r[cairn.semantic_review.review_gate_policy_profiles.profiles] The system MUST define policy-owned semantic review gate profiles.

#### Scenario: Profiles Kinds
r[cairn.semantic_review.review_gate_policy_profiles.profiles.profiles.kinds]
- GIVEN generated Cairn policy
- WHEN semantic review profiles are loaded
- THEN plan, design, code, and diff profiles are present with stable ids

#### Scenario: Profiles Criteria
r[cairn.semantic_review.review_gate_policy_profiles.profiles.profiles.criteria]
- GIVEN a review profile
- WHEN Cairn validates a receipt for that profile
- THEN required criteria ids and receipt fields are enforced

### Requirement: The system
r[cairn.semantic_review.review_gate_policy_profiles.sources] The system MUST validate reviewer source metadata through policy.

#### Scenario: Sources Allowed
r[cairn.semantic_review.review_gate_policy_profiles.sources.sources.allowed]
- GIVEN a semantic review receipt
- WHEN reviewer metadata is checked
- THEN the reviewer source is one of the policy-owned allowed sources

#### Scenario: Sources No Secret
r[cairn.semantic_review.review_gate_policy_profiles.sources.sources.no_secret]
- GIVEN reviewer metadata
- WHEN Cairn serializes validation receipts
- THEN it does not require or expose credentials, prompts with secrets, or remote tokens

### Requirement: The system
r[cairn.semantic_review.semantic_review_boundary.boundary] The system MUST define and enforce the boundary between semantic review production and Cairn receipt validation.

#### Scenario: Boundary Non Execution
r[cairn.semantic_review.semantic_review_boundary.boundary.boundary.non_execution]
- GIVEN a semantic review validation command
- WHEN it runs
- THEN Cairn does not call remote LLMs, network services, or reviewer tools

#### Scenario: Boundary Claims
r[cairn.semantic_review.semantic_review_boundary.boundary.boundary.claims]
- GIVEN semantic review documentation and receipts
- WHEN an operator inspects them
- THEN they state that Cairn validates externally produced evidence rather than proving semantic correctness

### Requirement: The system
r[cairn.semantic_review.semantic_review_boundary.fcis] The system MUST preserve the functional-core/imperative-shell boundary for semantic review support.

#### Scenario: Fcis Pure Core
r[cairn.semantic_review.semantic_review_boundary.fcis.fcis.pure_core]
- GIVEN semantic review validation logic
- WHEN it is implemented
- THEN schema and policy checks live in pure core code without filesystem, process, network, or clock access

#### Scenario: Fcis Cli Shell
r[cairn.semantic_review.semantic_review_boundary.fcis.fcis.cli_shell]
- GIVEN artifact hash validation against a repository root
- WHEN it is implemented
- THEN filesystem reads and JSON presentation remain in the CLI shell
