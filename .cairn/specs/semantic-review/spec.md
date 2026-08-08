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

## Semantic Review Readiness Gate

r[cairn.semantic_review.readiness_gate.release_readiness] The system MUST let release readiness consume semantic review ledger evidence when it is supplied or conventionally present.

## Scenarios

### Supplied valid semantic review ledger passes readiness check
r[cairn.semantic_review.readiness_gate.release_readiness.supplied_valid]
- GIVEN a semantic review ledger receipt with `valid=true` and a passing verdict
- WHEN an operator runs `cairn release-readiness --review-ledger <path>`
- THEN the release-readiness receipt includes a `semantic_review_ledger` check marked passed with the ledger receipt hash

### Supplied invalid semantic review ledger fails readiness
r[cairn.semantic_review.readiness_gate.release_readiness.supplied_invalid]
- GIVEN a semantic review ledger receipt with `valid=false` or a failing verdict
- WHEN an operator runs `cairn release-readiness --review-ledger <path>`
- THEN the release-readiness receipt is invalid and includes the failing semantic-review check

### Missing semantic review ledger is explicit
r[cairn.semantic_review.readiness_gate.release_readiness.skipped_absent]
- GIVEN no semantic review ledger argument and no conventional semantic-review ledger artifact
- WHEN an operator runs `cairn release-readiness`
- THEN the release-readiness receipt includes a skipped semantic-review check explaining that Cairn did not run reviewers

r[cairn.semantic_review.readiness_gate.boundary] The system MUST preserve the semantic-review boundary when readiness consumes review evidence.

## Scenarios

### Readiness consumes receipts rather than running reviewers
r[cairn.semantic_review.readiness_gate.boundary.no_reviewer_execution]
- GIVEN a semantic review ledger artifact
- WHEN release readiness evaluates semantic review evidence
- THEN Cairn reads deterministic receipt fields and does not invoke reviewers, model providers, network calls, or hidden semantic judgment

### Requirement: Split semantic review core uses focused modules

r[semantic_review.split_semantic_review_core.focused_modules] The system MUST split the semantic-review functional core into focused modules that preserve the existing `cairn_core::verified::semantic_review` facade.

#### Scenario: Focused boundaries are visible

r[semantic_review.split_semantic_review_core.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a maintainer inspects `crates/cairn-core/src/verified/semantic_review/`
- THEN review receipts, revision handoffs, ledger analytics, completeness receipts, policy profiles, and shared validators are separated into named modules
- AND the facade remains small enough to expose constants, DTOs, re-exports, and tests without owning every implementation detail

#### Scenario: Bridge APIs stay narrow

r[semantic_review.split_semantic_review_core.focused_modules.narrow_bridges]
- GIVEN existing CLI and core consumers use semantic-review functions
- WHEN the split exposes functions across module boundaries
- THEN the public facade preserves the existing exported functions and constants
- AND internal helpers remain scoped to the semantic-review module tree unless an existing consumer requires them

### Requirement: Split semantic review core remains behavior preserving

r[semantic_review.split_semantic_review_core.behavior_preserved] The system MUST preserve existing semantic-review public behavior, receipt schemas, hash material, issue codes, deterministic evidence, and Tracey coverage while draining this split.

#### Scenario: Focused checks pass

r[semantic_review.split_semantic_review_core.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN focused semantic-review tests and related core checks run
- THEN they pass without intentional receipt, policy, fixture, hash, or issue-code drift

#### Scenario: Repository checks pass

r[semantic_review.split_semantic_review_core.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs native Cairn validation, formatting, linting, Tracey coverage, and diff checks
- THEN all checks pass and the repository remains ready for commit

### Requirement: Review metrics regression rail

r[cairn.semantic_review.review_metrics_regression_rail]
The system MUST provide a deterministic rail that keeps metrics-derived review regression categories synchronized across registry entries, fixtures, operator guidance, task/spec references, and sanitized evidence.

#### Scenario: Registry binds all category surfaces

r[cairn.semantic_review.review_metrics_regression_rail.registry_binds_surfaces]
- GIVEN a metrics-derived review regression category is declared
- WHEN Cairn validates the category registry
- THEN the registry MUST include a stable category code, metrics class/stage/source, diagnostic code, requirement or scenario id, operator-guidance anchor, positive fixture reference, negative fixture reference, and sanitized evidence reference
- AND missing or duplicate bindings MUST produce deterministic fail-closed issues

#### Scenario: Fixture pair proves category enforcement

r[cairn.semantic_review.review_metrics_regression_rail.fixture_pair_enforcement]
- GIVEN a declared metrics-derived review regression category
- WHEN its fixture coverage is checked
- THEN at least one negative fixture MUST fail with the category diagnostic
- AND at least one positive fixture MUST pass while demonstrating the expected task, spec, evidence, command, helper, or oracle shape

#### Scenario: Guidance and task references stay synchronized

r[cairn.semantic_review.review_metrics_regression_rail.guidance_task_sync]
- GIVEN a registry entry names a diagnostic code and requirement id
- WHEN Cairn checks guidance and task/spec references
- THEN operator guidance MUST document the diagnostic and remediation shape
- AND the related task/spec artifacts MUST reference the declared requirement or scenario id

#### Scenario: Sanitized evidence boundary is enforced

r[cairn.semantic_review.review_metrics_regression_rail.secret_free_evidence]
- GIVEN repeated review findings are summarized as metrics evidence
- WHEN Cairn validates the evidence reference for a registry entry
- THEN the evidence MUST include safe category metadata and provenance sufficient for review
- AND it MUST NOT include credentials, tokens, provider payload bodies, private transcript dumps, raw hidden prompts, account identifiers, or connection strings

#### Scenario: Downstream projects retain category ownership

r[cairn.semantic_review.review_metrics_regression_rail.downstream_category_ownership]
- GIVEN a downstream project has project-specific review diagnostics and fixtures
- WHEN it uses the Cairn review metrics regression rail
- THEN Cairn MUST validate the generic registry/coverage invariant without requiring those project-specific categories to be added to Cairn's own accepted specs

### Requirement: Metrics regression rail module split

r[cairn.semantic_review.metrics_regression_split] The system MUST keep the metrics regression rail behavior-preserving while splitting its implementation into focused modules.

#### Scenario: Module Boundaries

r[cairn.semantic_review.metrics_regression_split.boundaries]
- GIVEN the semantic review metrics regression rail is maintained
- WHEN its implementation is split
- THEN the split MUST separate public entrypoints/report hashing from registry, fixture, guidance-sync, evidence-boundary, and shared helper logic

#### Scenario: Public Api

r[cairn.semantic_review.metrics_regression_split.public_api]
- GIVEN existing callers use the semantic review metrics regression validators and schema constants
- WHEN the module split is applied
- THEN the same public functions and constants MUST remain exported from `verified::semantic_review`

#### Scenario: Focused Helpers

r[cairn.semantic_review.metrics_regression_split.focused_helpers]
- GIVEN the split implementation is inspected
- WHEN registry, fixture, guidance, and evidence boundary logic are changed later
- THEN each behavior class MUST live behind a focused private module seam

#### Scenario: Behavior Preserved

r[cairn.semantic_review.metrics_regression_split.behavior_preserved]
- GIVEN existing positive and negative metrics regression fixtures
- WHEN the split implementation validates them
- THEN issue codes, verdicts, and deterministic receipt behavior MUST remain stable

#### Scenario: Focused Verification

r[cairn.semantic_review.metrics_regression_split.focused_verification]
- GIVEN the split is complete
- WHEN focused semantic-review tests run
- THEN they MUST cover registry, fixture, guidance sync, evidence boundary, and split-boundary behavior

#### Scenario: Repository Verification

r[cairn.semantic_review.metrics_regression_split.repository_verification]
- GIVEN the split is archived
- WHEN repository validation runs
- THEN Cairn validation, Tracey coverage, and the repository gate MUST pass
