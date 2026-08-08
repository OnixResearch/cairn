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
