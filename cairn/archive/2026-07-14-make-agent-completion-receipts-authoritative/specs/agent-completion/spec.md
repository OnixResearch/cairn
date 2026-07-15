# Authoritative Agent Completion Receipt Specification Delta

## Purpose

Makes Cairn agent completion depend on repository-owned required checks and fresh passing execution receipts rather than non-empty producer assertions.

## ADDED Requirements

### Requirement: Agent completion uses an independent verification contract

r[cairn.supersede.agent_run_receipts.verification_contract] Cairn MUST evaluate authoritative agent completion against a normalized repository-owned verification contract that binds repository, registry, selection, policy, final subject, required check, and prerequisite identities.

#### Scenario: Matching contract is accepted

r[cairn.supersede.agent_run_receipts.verification_contract.match]
- GIVEN a repository supplies a current verification registry and deterministic selected required-check set for a final subject
- WHEN Cairn normalizes the contract for completion validation
- THEN the contract MUST preserve repository ID, registry ID and BLAKE3 hash, selection hash, policy hash, final subject hash, required check IDs, exact command identities, and prerequisite edges
- AND producer execution results MUST NOT be permitted to redefine the required set.

#### Scenario: Self-authored required set is rejected

r[cairn.supersede.agent_run_receipts.verification_contract.self_authored]
- GIVEN an agent receipt declares that its own result rows are the complete required set but no matching repository-owned contract is supplied
- WHEN authoritative completion validation runs
- THEN validation MUST fail with a missing-contract diagnostic
- AND the receipt MUST NOT establish completion even if every self-declared row says `passed`.

#### Scenario: Registry or selection drift fails closed

r[cairn.supersede.agent_run_receipts.verification_contract.drift]
- GIVEN execution results cite a registry hash, selection hash, policy hash, or required-check set different from the current supplied contract
- WHEN Cairn evaluates completion
- THEN validation MUST fail with a deterministic drift class naming the mismatched identity.

### Requirement: Required check coverage is exact and pass-only

r[cairn.supersede.agent_run_receipts.pass_only] Cairn MUST require exactly one eligible `passed` result for every selected required check and MUST treat every other result status as non-passing completion evidence.

#### Scenario: Every required check passed

r[cairn.supersede.agent_run_receipts.pass_only.complete]
- GIVEN each selected required check has exactly one current result with status `passed`, matching command and subject identities, passing prerequisites, and valid evidence hashes
- WHEN Cairn evaluates authoritative completion
- THEN the completion report MUST place every required check in the passed set
- AND it MAY return a passing verdict when no other completion blocker exists.

#### Scenario: Non-pass statuses remain non-pass

r[cairn.supersede.agent_run_receipts.pass_only.non_pass]
- GIVEN any required check has status `failed`, `blocked`, `skipped`, `deferred`, `timed_out`, `cancelled`, `dry_run`, `cleanup_failed`, `infrastructure_error`, `not_selected`, or `unknown`
- WHEN Cairn evaluates authoritative completion
- THEN that check MUST remain in a non-passing set with its exact status
- AND the aggregate completion verdict MUST fail.

#### Scenario: Duplicate or unknown rows fail closed

r[cairn.supersede.agent_run_receipts.pass_only.row_identity]
- GIVEN result rows duplicate a required check ID, omit a required check, cite an unknown check, use an unsupported status, or disagree about command identity
- WHEN Cairn evaluates exact coverage
- THEN validation MUST emit deterministic row-identity diagnostics
- AND it MUST NOT choose one duplicate or ignore the unknown row.

#### Scenario: Prerequisite failure blocks dependent pass

r[cairn.supersede.agent_run_receipts.pass_only.prerequisites]
- GIVEN a result says a dependent check passed but one of its required prerequisites is missing or non-passing
- WHEN Cairn evaluates the check graph
- THEN the dependent result MUST be ineligible for completion
- AND validation MUST identify the unsatisfied prerequisite edge.

### Requirement: Completion evidence is fresh for the final subject

r[cairn.supersede.agent_run_receipts.freshness] Cairn MUST bind every completion-eligible result and the aggregate completion receipt to the final post-mutation subject identity used for the decision.

#### Scenario: Final subject matches every result

r[cairn.supersede.agent_run_receipts.freshness.match]
- GIVEN repository selection, required check results, artifacts, and execution receipts all cite the same final subject hash
- WHEN completion validation runs
- THEN subject freshness MUST pass and the subject hash MUST enter canonical completion receipt material.

#### Scenario: Mutation after validation invalidates completion

r[cairn.supersede.agent_run_receipts.freshness.post_check_mutation]
- GIVEN required checks passed for one subject hash and repository content later changes to a different final subject hash
- WHEN the prior evidence is presented for completion
- THEN Cairn MUST classify the evidence as stale
- AND it MUST require fresh results for the changed subject before completion can pass.

#### Scenario: Evidence and artifact hashes are required

r[cairn.supersede.agent_run_receipts.freshness.hashes]
- GIVEN a required result omits or malforms its command hash, subject hash, evidence hash, execution receipt hash, or required artifact hash
- WHEN Cairn validates result freshness
- THEN validation MUST fail with a field-specific hash diagnostic.

### Requirement: Verification feedback is actionable and bounded

r[cairn.supersede.agent_run_receipts.actionable_diagnostics] Cairn MUST normalize completion blockers into bounded diagnostics containing stable identity, status, evidence linkage, remediation, reproduction, artifact, and non-claim fields suitable for an agent follow-up turn.

#### Scenario: Failed check yields actionable feedback

r[cairn.supersede.agent_run_receipts.actionable_diagnostics.failed_check]
- GIVEN a required check fails with bounded captured evidence
- WHEN Cairn emits its completion diagnostic
- THEN the diagnostic MUST include a stable diagnostic ID and code, check ID, result status, applicable subject/registry/selection/evidence/receipt hashes, severity, remediation label and bounded summary, exact reproduction argv, safe working-root label, and safe artifact references
- AND it SHOULD include a bounded repository-relative source path and line/column span when the producer supplied one.

#### Scenario: Diagnostic safety is enforced

r[cairn.supersede.agent_run_receipts.actionable_diagnostics.safe]
- GIVEN a diagnostic candidate contains raw prompts, model output, credentials, environment values, absolute private paths, shell-expanded secrets, raw PTY bytes, or unbounded stdout/stderr
- WHEN Cairn validates the diagnostic
- THEN it MUST reject or redact the unsafe fields according to policy
- AND it MUST retain hashes and bounded safe references rather than the raw payload.

#### Scenario: Non-claims survive feedback projection

r[cairn.supersede.agent_run_receipts.actionable_diagnostics.non_claims]
- GIVEN a diagnostic or passing check is projected into an agent follow-up or completion receipt
- WHEN the projection is validated
- THEN it MUST preserve that receipt validation does not prove check-oracle soundness, honest command execution, model correctness, whole-product correctness, or release eligibility outside the selected contract.

### Requirement: Cairn validation receipt is completion authority

r[cairn.supersede.agent_run_receipts.completion_authority] A task or agent run MUST NOT be represented as authoritatively complete unless a current passing Cairn completion-validation receipt covers the final subject and every repository-required check.

#### Scenario: Model completion text is advisory only

r[cairn.supersede.agent_run_receipts.completion_authority.model_text]
- GIVEN a model response contains `COMPLETE`, success prose, or another completion promise
- WHEN no current passing Cairn completion-validation receipt is supplied
- THEN the text MAY request completion evaluation
- BUT it MUST NOT satisfy the completion gate.

#### Scenario: Process success and task counters are insufficient

r[cairn.supersede.agent_run_receipts.completion_authority.observations]
- GIVEN an agent process exits successfully, an unchecked task count decreases, task checkboxes are marked, or an archive directory appears
- WHEN required verification evidence is missing or non-passing
- THEN those observations MAY report progress
- BUT Cairn MUST keep completion blocked.

#### Scenario: Passing receipt is citeable

r[cairn.supersede.agent_run_receipts.completion_authority.citation]
- GIVEN authoritative completion validation passes
- WHEN Cairn emits the validation result
- THEN it MUST include final subject, registry, selection, policy, required/passed check-set, input, and receipt hashes
- AND an agent wrapper MAY cite that validation receipt hash in its final response without embedding raw evidence.

### Requirement: Authoritative completion preserves core and shell boundaries

r[cairn.supersede.agent_run_receipts.execution_boundary] Cairn MUST keep authoritative completion decisions pure and deterministic while keeping registry loading, repository inspection, process execution, output capture, artifact reads, and receipt persistence in explicit shell-owned behavior.

#### Scenario: Pure core needs no host world

r[cairn.supersede.agent_run_receipts.execution_boundary.pure_core]
- GIVEN normalized contract, result, subject, and policy values in memory
- WHEN the authoritative completion core is tested
- THEN it MUST return deterministic required, passed, non-passing, diagnostic, verdict, and hash material without filesystem, process, environment, clock, network, stdout, or stderr access.

#### Scenario: Validation does not execute checks

r[cairn.supersede.agent_run_receipts.execution_boundary.no_execution]
- GIVEN a required result is missing or stale
- WHEN Cairn validates completion through core, CLI, or MCP surfaces
- THEN Cairn MUST report the blocker from supplied evidence
- AND it MUST NOT launch, retry, or escalate the repository check automatically.

### Requirement: Authoritative schema migration fails closed

r[cairn.supersede.agent_run_receipts.schema_migration] Cairn MUST version authoritative agent completion separately from legacy unscoped receipt behavior and MUST keep CLI, MCP, policy, fixtures, and hash consumers synchronized.

#### Scenario: Legacy receipt remains inspectable but ineligible

r[cairn.supersede.agent_run_receipts.schema_migration.legacy]
- GIVEN a valid historical v1 agent-run receipt has a successful wrapper verdict and a non-empty verification array
- WHEN it is inspected by the current validator without an authoritative contract
- THEN Cairn MAY report its legacy envelope and hash consistency
- BUT it MUST classify the receipt as `legacy_unscoped` and MUST NOT return authoritative completion `PASS`.

#### Scenario: Public surfaces agree

r[cairn.supersede.agent_run_receipts.schema_migration.parity]
- GIVEN the same authoritative contract, result receipts, final subject, and policy are supplied to core, CLI, and MCP validation
- WHEN each surface evaluates completion
- THEN they MUST agree on status sets, validity, verdict, diagnostic codes, hashes, and non-claims.

#### Scenario: Policy and golden drift is atomic

r[cairn.supersede.agent_run_receipts.schema_migration.goldens]
- GIVEN the authoritative receipt schema or policy hash material changes
- WHEN maintainers refresh generated policy and fixtures
- THEN JSON fixtures, replay cases, plan fixtures, public schema checks, and Preserves/receipt hash goldens MUST be updated and validated together.

### Requirement: Authoritative completion has positive and negative regression evidence

r[cairn.supersede.agent_run_receipts.authoritative_verification] Cairn MUST verify the authoritative agent completion contract with positive and negative core, CLI, MCP, schema, determinism, compatibility, and safety fixtures.

#### Scenario: Positive matrix passes

r[cairn.supersede.agent_run_receipts.authoritative_verification.positive]
- GIVEN current matching contracts and final-subject results in which all required checks and prerequisites passed
- WHEN the focused fixture matrix runs across different checkout roots
- THEN completion MUST pass with deterministic sets, diagnostics, and portable BLAKE3 identities.

#### Scenario: Negative matrix fails closed

r[cairn.supersede.agent_run_receipts.authoritative_verification.negative]
- GIVEN fixtures for absent contracts, self-authored selections, missing/duplicate/unknown checks, every non-pass status, stale subjects, registry/selection/policy drift, command mismatch, malformed hashes, failed prerequisites, unsafe diagnostics, legacy v1 success, contradictory aggregate outcomes, and tampered receipts
- WHEN focused validation runs
- THEN each fixture MUST fail with its expected deterministic diagnostic
- AND none may be accepted as authoritative completion evidence.
