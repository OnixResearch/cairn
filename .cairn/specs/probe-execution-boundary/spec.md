# Probe Execution Boundary Specification

## Purpose

Defines how Cairn separates default gate evaluation from probe command execution while allowing gates to verify saved probe evidence.

## Requirements

### Requirement: Default gates do not execute commands
r[cairn.probe_execution_boundary.default_gate_no_execution] The system MUST keep default `gate proposal`, `gate design`, and `gate tasks` evaluation free of probe command execution, shell command execution, network calls, and arbitrary external tool execution.

#### Scenario: Gate validates existing data only
r[cairn.probe_execution_boundary.default_gate_no_execution.existing_data_only]
- GIVEN a change declares probes and saved probe receipt paths
- WHEN the operator runs `cairn gate tasks <change> --root <repo>` without an explicit probe execution flag
- THEN Cairn MUST evaluate lifecycle artifacts, policy data, declared probe metadata, and saved receipt metadata only
- AND Cairn MUST NOT launch probe commands or arbitrary external commands.

#### Scenario: Missing evidence fails without execution
r[cairn.probe_execution_boundary.default_gate_no_execution.missing_evidence]
- GIVEN a gate requires probe evidence and the declared probe receipt is missing
- WHEN the operator runs the default gate command
- THEN Cairn MUST report missing probe evidence with deterministic diagnostics
- AND Cairn MUST NOT attempt to create the missing evidence by executing the probe.

### Requirement: Explicit probe run command
r[cairn.probe_execution_boundary.explicit_probe_run] The system MUST provide an explicit probe execution surface that runs selected policy-authorized probes and emits deterministic probe receipts.

#### Scenario: Probe runner executes authorized local command
r[cairn.probe_execution_boundary.explicit_probe_run.authorized_local_command]
- GIVEN a structured probe declares a local CLI command, fixtures, expected status class, and assertions
- WHEN the operator runs `cairn probe run` for that probe under a policy that authorizes the command profile
- THEN Cairn MUST execute the command through the probe runner
- AND Cairn MUST emit a deterministic probe receipt containing probe identity, requirement IDs, command identity, fixture hashes, output hashes, assertion results, lifecycle bundle hash, policy hash, and receipt hash.

#### Scenario: Probe runner denies unauthorized command
r[cairn.probe_execution_boundary.explicit_probe_run.denied_command]
- GIVEN a structured probe requests undeclared filesystem, process, network, secret, mutation, or external tool capability
- WHEN the operator runs `cairn probe run`
- THEN Cairn MUST deny the probe before executing the requested behavior
- AND Cairn MUST emit a deterministic denial receipt or diagnostic that names the denied capability class without leaking secrets.

### Requirement: Gates verify saved probe receipts
r[cairn.probe_execution_boundary.gates_verify_receipts] The system MUST allow gates to verify saved probe receipts against declared lifecycle probe evidence without executing probe commands.

#### Scenario: Saved receipt satisfies gate evidence
r[cairn.probe_execution_boundary.gates_verify_receipts.valid_saved_receipt]
- GIVEN a declared probe has a saved receipt with a valid receipt hash, matching probe ID, matching requirement IDs, matching command identity, passing assertions, matching lifecycle bundle hash, matching policy hash, and sufficient coverage class
- WHEN the operator runs the relevant gate
- THEN Cairn MUST count the saved receipt as valid executable probe evidence for the declared requirement.

#### Scenario: Receipt command mismatch fails
r[cairn.probe_execution_boundary.gates_verify_receipts.command_mismatch]
- GIVEN a saved probe receipt was produced by a command identity different from the declared probe command identity
- WHEN the operator runs a gate that verifies the saved receipt
- THEN Cairn MUST reject the receipt with a deterministic command-mismatch issue
- AND Cairn MUST NOT rerun the probe during default gate evaluation.

#### Scenario: Coverage class remains visible
r[cairn.probe_execution_boundary.gates_verify_receipts.coverage_class]
- GIVEN saved probe receipts cover positive, negative, malformed-input, boundary, or regression cases
- WHEN Cairn summarizes gate evidence for a requirement
- THEN Cairn MUST preserve the coverage class for each receipt rather than treating all executable evidence as equivalent.

### Requirement: Optional inline probe mode is explicit and bounded
r[cairn.probe_execution_boundary.inline_probe_mode] The system MAY support inline probe execution during gate evaluation only when the operator requests it explicitly and policy authorizes bounded execution.

#### Scenario: Inline mode disabled by default
r[cairn.probe_execution_boundary.inline_probe_mode.disabled_default]
- GIVEN repository policy has not enabled inline probe execution for gates
- WHEN the operator runs `cairn gate tasks <change> --run-probes`
- THEN Cairn MUST deny inline probe execution with deterministic diagnostics
- AND Cairn MUST NOT execute probe commands.

#### Scenario: Inline mode writes probe receipts
r[cairn.probe_execution_boundary.inline_probe_mode.receipt_links]
- GIVEN policy enables inline probe execution with declared budgets, capabilities, redaction rules, and output paths
- WHEN the operator runs a gate with explicit inline probe execution
- THEN Cairn MUST execute probes through the same probe runner used by `cairn probe run`
- AND the gate receipt MUST include stable links or hashes for every generated probe receipt.

### Requirement: Probe evidence fails closed
r[cairn.probe_execution_boundary.fail_closed] The system MUST fail closed when probe evidence is missing, malformed, stale, tampered, denied, insufficient, or inconsistent with current lifecycle or policy data.

#### Scenario: Tampered receipt is rejected
r[cairn.probe_execution_boundary.fail_closed.tampered_receipt]
- GIVEN a saved probe receipt has been modified after its receipt hash was computed
- WHEN Cairn verifies the receipt for a gate
- THEN Cairn MUST reject the receipt with a deterministic tampered-receipt issue.

#### Scenario: Stale lifecycle or policy hash is rejected
r[cairn.probe_execution_boundary.fail_closed.stale_hash]
- GIVEN a saved probe receipt cites a lifecycle bundle hash or policy hash that differs from the current evaluated data
- WHEN Cairn verifies the receipt for a gate
- THEN Cairn MUST reject the receipt with deterministic stale-evidence diagnostics.

#### Scenario: Insufficient coverage is rejected
r[cairn.probe_execution_boundary.fail_closed.insufficient_coverage]
- GIVEN a requirement requires negative or malformed-input probe evidence and only positive probe receipts are present
- WHEN Cairn evaluates gate evidence sufficiency
- THEN Cairn MUST reject the evidence as insufficient and report the missing coverage class.

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
