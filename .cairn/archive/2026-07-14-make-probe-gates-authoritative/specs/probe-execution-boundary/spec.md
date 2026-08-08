# Probe Execution Boundary Specification Delta

## Purpose

Makes saved probe evidence authoritative and outcome-consistent across visible gates and lifecycle mutation guards.

## ADDED Requirements

### Requirement: Authoritative gate composition

r[cairn.probe_execution_boundary.authoritative_gate_composition] Cairn MUST derive gate issues, validity, and verdict exactly once from the complete lifecycle snapshot and all normalized evidence observations required by the selected gate.

#### Scenario: Added shell finding produces one failing outcome

r[cairn.probe_execution_boundary.authoritative_gate_composition.shell_finding]
- GIVEN lifecycle artifacts pass the core checks but a denied inline request or saved-probe observation contributes an issue
- WHEN Cairn finalizes the gate receipt
- THEN the receipt MUST report `valid=false`, a failing verdict, and the issue
- AND it MUST NOT report a passing verdict for the same final issue set

#### Scenario: Complete passing inputs remain coherent

r[cairn.probe_execution_boundary.authoritative_gate_composition.passing]
- GIVEN lifecycle artifacts and every required saved-probe observation pass
- WHEN Cairn finalizes the gate receipt
- THEN the receipt MUST report `valid=true`, a passing verdict, and no blocking issues

### Requirement: Saved probe evidence blocks lifecycle mutation

r[cairn.probe_execution_boundary.authoritative_mutation_block] Cairn MUST apply the same saved-probe evidence checks to visible task gates and every sync, archive, or MCP mutation preflight governed by that gate.

#### Scenario: Missing saved receipt blocks sync

r[cairn.probe_execution_boundary.authoritative_mutation_block.missing]
- GIVEN a change declares a required saved probe receipt that is missing
- WHEN the operator requests sync execution
- THEN Cairn MUST report the probe blocker and MUST leave every accepted spec unchanged

#### Scenario: Tampered saved receipt blocks archive

r[cairn.probe_execution_boundary.authoritative_mutation_block.tampered]
- GIVEN a completed change declares a saved probe receipt whose content, policy identity, command identity, or receipt hash is invalid
- WHEN the operator requests archive execution
- THEN Cairn MUST report the same evidence failure class as the task gate and MUST leave the active and archive trees unchanged

#### Scenario: Valid saved receipt permits the existing plan

r[cairn.probe_execution_boundary.authoritative_mutation_block.valid]
- GIVEN all lifecycle artifacts and required saved probe receipts pass the authoritative task gate
- WHEN the operator requests an otherwise unblocked lifecycle mutation
- THEN probe enforcement MUST NOT add a blocker
- AND the existing explicit-execution and mutation-manifest rules MUST still apply

### Requirement: Gate receipts bind saved probe evidence

r[cairn.probe_execution_boundary.authoritative_evidence_binding] Cairn MUST bind normalized probe declaration and saved-receipt identities into the gate input and receipt hashes used for the decision.

#### Scenario: Probe evidence changes the gate identity

r[cairn.probe_execution_boundary.authoritative_evidence_binding.changed]
- GIVEN two otherwise identical gate inputs with different probe declarations or saved receipt bytes
- WHEN Cairn computes their gate input hashes
- THEN the hashes MUST differ even when both evidence sets produce the same verdict

#### Scenario: Binding remains bounded and private

r[cairn.probe_execution_boundary.authoritative_evidence_binding.private]
- GIVEN a saved probe receipt contains stdout or stderr hashes and shell-owned path metadata
- WHEN Cairn emits the gate receipt
- THEN the receipt MUST include only policy-approved identities, summaries, and issue codes
- AND it MUST NOT include raw command output, credentials, environment values, or absolute temporary paths

### Requirement: Authoritative probe gating preserves the execution boundary

r[cairn.probe_execution_boundary.authoritative_boundary] Cairn MUST keep probe file access and hashing in the CLI shell, keep gate composition and outcome derivation in pure deterministic core logic, and MUST NOT execute probes as part of default gate evaluation.

#### Scenario: Default gate remains evidence-only

r[cairn.probe_execution_boundary.authoritative_boundary.no_execution]
- GIVEN a gate references saved probe evidence
- WHEN Cairn evaluates the gate
- THEN Cairn MUST read and validate existing declarations and receipts without launching the declared probe command

#### Scenario: Core remains independently testable

r[cairn.probe_execution_boundary.authoritative_boundary.pure_core]
- GIVEN normalized lifecycle and probe observations in memory
- WHEN the gate core is tested
- THEN the decision MUST require no filesystem, environment, process, clock, network, stdout, or stderr access

### Requirement: Required probe evidence is pass-only

r[cairn.probe_execution_boundary.authoritative_status] Cairn MUST count only a current explicit `passed` saved-probe result as satisfying required probe evidence and MUST preserve every other supported outcome as non-passing.

#### Scenario: Passing result is eligible

r[cairn.probe_execution_boundary.authoritative_status.passed]
- GIVEN a saved probe receipt is current, structurally valid, hash-consistent, command-matched, policy-matched, requirement-matched, and reports `passed`
- WHEN Cairn composes the authoritative gate
- THEN the probe result MAY satisfy its required evidence row
- AND every other lifecycle and mutation rule MUST still apply.

#### Scenario: Non-pass result blocks

r[cairn.probe_execution_boundary.authoritative_status.non_pass]
- GIVEN a required saved probe reports failed, blocked, skipped, deferred, timed-out, cancelled, dry-run, cleanup-failed, infrastructure-failed, or unknown status
- WHEN Cairn composes the gate
- THEN the result MUST remain non-passing with its exact normalized status
- AND the gate and every governed mutation preflight MUST fail.

#### Scenario: Unknown status fails closed

r[cairn.probe_execution_boundary.authoritative_status.unknown]
- GIVEN a saved probe receipt uses an unsupported or missing result status
- WHEN Cairn validates the receipt
- THEN it MUST emit a deterministic status diagnostic
- AND it MUST NOT infer success from process exit, a wrapper verdict, or the absence of raw error output.

### Requirement: Probe failures produce actionable bounded diagnostics

r[cairn.probe_execution_boundary.authoritative_diagnostics] Cairn MUST normalize probe blockers into safe diagnostics that preserve stable identity, status, evidence linkage, remediation, reproduction, artifact, and non-claim fields.

#### Scenario: Probe failure is reproducible

r[cairn.probe_execution_boundary.authoritative_diagnostics.actionable]
- GIVEN a required probe is missing, stale, malformed, non-passing, or mismatched
- WHEN Cairn emits the authoritative gate diagnostic
- THEN it MUST include stable diagnostic and probe IDs, result status, applicable declaration/policy/subject/evidence/receipt hashes, severity, remediation label and bounded summary, exact reproduction argv, safe working-root label, and safe artifact references
- AND it SHOULD include a bounded repository-relative source path and line/column span when the producer supplied one.

#### Scenario: Diagnostic remains safe

r[cairn.probe_execution_boundary.authoritative_diagnostics.safe]
- GIVEN a probe receipt or diagnostic candidate contains raw stdout/stderr, prompts, credentials, environment values, private absolute paths, raw PTY bytes, or unbounded logs
- WHEN Cairn normalizes the diagnostic
- THEN it MUST reject or redact unsafe fields according to policy
- AND it MUST retain only bounded summaries, hashes, safe spans, argv, artifact refs, and explicit non-claims.

### Requirement: Authoritative probe gate regressions

r[cairn.probe_execution_boundary.authoritative_tests] Cairn MUST verify authoritative probe gating with positive and negative core, CLI, mutation, MCP, schema, and determinism fixtures.

#### Scenario: Positive fixtures pass

r[cairn.probe_execution_boundary.authoritative_tests.positive]
- GIVEN current valid saved probe receipts and identical repositories under different checkout roots
- WHEN focused gate and mutation preflight tests run
- THEN the fixtures MUST produce coherent passing outcomes and stable portable evidence hashes

#### Scenario: Negative fixtures fail closed

r[cairn.probe_execution_boundary.authoritative_tests.negative]
- GIVEN missing, unreadable, malformed, oversized, tampered, stale-policy, command-mismatched, requirement-mismatched, denied-inline, every non-pass status, unsupported-status, or unsafe-diagnostic evidence fixtures
- WHEN focused gate and mutation preflight tests run
- THEN every fixture MUST produce its expected deterministic issue and MUST NOT permit mutation
