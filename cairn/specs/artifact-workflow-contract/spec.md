# Artifact Workflow Contract Specification

## Purpose

Defines the `artifact-workflow-contract` capability.

## Requirements

### Requirement: Workflow policy is typed and deterministic

r[cairn.artifact_workflow_contract.policy] Cairn MUST define workflow schemas through typed Nickel policy and a checked runtime projection. Each schema MUST bind ordered artifact IDs, safe output paths, dependencies, stages, instruction templates, apply prerequisites, bounds, and non-claims.

#### Scenario: Valid workflow graph exports deterministically

r[cairn.artifact_workflow_contract.policy.valid]
- GIVEN a workflow schema has unique artifact IDs, safe paths, known dependencies, and an acyclic graph
- WHEN Cairn exports and admits the runtime projection
- THEN equivalent policy values MUST produce the same canonical workflow bytes and identity
- AND ordinary workflow commands MUST use the checked projection without evaluating Nickel.

#### Scenario: Invalid workflow graph fails admission

r[cairn.artifact_workflow_contract.policy.invalid]
- GIVEN a workflow schema has duplicate IDs, unknown dependencies, a cycle, an unsafe path, or an invalid instruction template
- WHEN policy export or runtime admission runs
- THEN Cairn MUST reject the schema with deterministic diagnostics
- AND it MUST NOT derive workflow states or instructions from that schema.

### Requirement: Artifact states follow declared dependencies

r[cairn.artifact_workflow_contract.status] Cairn MUST derive `done`, `ready`, `blocked`, and `skipped` states from admitted policy and bounded observed artifact facts. State derivation MUST include stable blockers, requirements, missing dependencies, unlocks, and policy-ordered next actions.

#### Scenario: Dependencies derive deterministic states

r[cairn.artifact_workflow_contract.status.ready]
- GIVEN proposal is structurally admissible, design and delta specs depend on proposal, and tasks depend on their declared predecessors
- WHEN Cairn derives workflow status
- THEN proposal MUST be `done` and each satisfied absent successor MUST be `ready`
- AND next actions MUST use stable policy order when several artifacts are ready.

#### Scenario: Invalid present artifact blocks dependents

r[cairn.artifact_workflow_contract.status.invalid]
- GIVEN an artifact exists but fails its bounded structural admission
- WHEN Cairn derives workflow status
- THEN that artifact MUST be `blocked` with stable issue codes
- AND no dependent artifact may become `ready` because of its presence.

#### Scenario: Policy can skip an artifact

r[cairn.artifact_workflow_contract.status.skipped]
- GIVEN the selected admitted workflow profile explicitly omits an artifact
- WHEN Cairn derives workflow status
- THEN that artifact MUST be `skipped`
- AND dependents MAY treat the skipped artifact as complete only when policy permits that dependency transition.

### Requirement: Artifact instructions are bounded and read-only

r[cairn.artifact_workflow_contract.instructions] Cairn MUST return one deterministic instruction envelope for a selected workflow artifact. The envelope MUST bind schema, artifact, output path, policy template, constraints, dependencies, upstream paths and BLAKE3 identities, input identity, policy identity, and required non-claims.

#### Scenario: Ready artifact returns exact instructions

r[cairn.artifact_workflow_contract.instructions.ready]
- GIVEN a known artifact is `ready` in an admitted workflow
- WHEN the operator requests its instructions
- THEN Cairn MUST return the exact policy-owned output path, template, constraints, and upstream identities
- AND it MUST NOT create, edit, or delete any lifecycle artifact.

#### Scenario: Unknown artifact returns a stable error

r[cairn.artifact_workflow_contract.instructions.unknown]
- GIVEN an artifact selector is absent from the selected workflow schema
- WHEN the operator requests instructions
- THEN Cairn MUST reject the selector with a stable diagnostic and known-artifact list
- AND it MUST NOT fall back to an artifact from another schema or ambient configuration.

### Requirement: CLI workflow commands preserve existing lifecycle contracts

r[cairn.artifact_workflow_contract.cli] Cairn MUST expose read-only `workflow status` and `workflow instructions` CLI commands. Existing status, gate, sync, archive, and receipt compatibility surfaces MUST remain unchanged.

#### Scenario: CLI workflow inspection is deterministic

r[cairn.artifact_workflow_contract.cli.inspect]
- GIVEN unchanged policy, lifecycle bytes, root selection, and command arguments
- WHEN either workflow command runs repeatedly
- THEN it MUST emit byte-stable semantic output and receipt identities
- AND it MUST perform no repository, local-state, process, or network mutation.

#### Scenario: Invalid CLI selection fails without compatibility drift

r[cairn.artifact_workflow_contract.cli.invalid]
- GIVEN the change, schema, or artifact selector is unknown or malformed
- WHEN CLI parsing or workflow admission runs
- THEN Cairn MUST return a stable failure diagnostic
- AND existing lifecycle command output schemas MUST remain unchanged.

### Requirement: MCP exposes equivalent strict workflow tools

r[cairn.artifact_workflow_contract.mcp] Cairn MUST expose strict read-only MCP tools for workflow status and instructions. MCP results MUST derive from the same pure semantic values as CLI results.

#### Scenario: CLI and MCP return equivalent workflow facts

r[cairn.artifact_workflow_contract.mcp.parity]
- GIVEN the same admitted policy, root facts, change, and artifact selector
- WHEN CLI and MCP workflow inspection run
- THEN both surfaces MUST report equivalent schema, artifact states, blockers, identities, and non-claims
- AND MCP protocol framing MAY remain transport-specific.

#### Scenario: MCP rejects unknown fields

r[cairn.artifact_workflow_contract.mcp.invalid]
- GIVEN an MCP workflow request has an unknown field or missing required selector
- WHEN strict argument validation runs
- THEN the tool MUST reject the request before lifecycle observation
- AND no mutation-capable tool may be selected as a fallback.

### Requirement: Existing lifecycle receipt compatibility is preserved

r[cairn.artifact_workflow_contract.compatibility] The workflow contract MUST be additive. It MUST NOT change stable fields or meanings in existing status, gate, sync, archive, migration, replay, evidence, or release-readiness receipts.

#### Scenario: Existing lifecycle receipts remain compatible

r[cairn.artifact_workflow_contract.compatibility.existing]
- GIVEN an existing compatibility fixture and unchanged inputs
- WHEN Cairn adds workflow inspection commands
- THEN the existing command MUST preserve its documented fields and meanings
- AND workflow-specific fields MUST appear only on the new workflow surfaces.

### Requirement: Workflow decisions preserve the functional-core boundary

r[cairn.artifact_workflow_contract.functional_boundary] Cairn MUST keep workflow graph admission, state derivation, next-action selection, and instruction construction pure. The CLI MUST own policy loading, file observation, hashing, root selection, serialization, transport, and exits.

#### Scenario: Workflow decisions remain pure

r[cairn.artifact_workflow_contract.functional_boundary.pure]
- GIVEN admitted workflow policy and normalized observed artifact facts
- WHEN Cairn derives status or instructions
- THEN the result MUST be deterministic without filesystems, environments, processes, clocks, networks, mutation, or output effects.

#### Scenario: Shell observation does not enter core logic

r[cairn.artifact_workflow_contract.functional_boundary.shell]
- GIVEN the shell reads bounded artifact bytes and computes their identities
- WHEN it invokes the workflow core
- THEN it MUST pass explicit in-memory observations
- AND the core MUST NOT discover paths or read policy and artifact files itself.

### Requirement: Workflow claims remain bounded and verified

r[cairn.artifact_workflow_contract.boundary] Workflow output MUST state that artifact state and instructions prove only policy-graph and observed structural facts. It MUST NOT claim semantic quality, review approval, implementation correctness, gate passage, safe mutation, or release eligibility.

#### Scenario: Workflow output does not promote lifecycle claims

r[cairn.artifact_workflow_contract.boundary.nonclaim]
- GIVEN an artifact is `done` or `ready`
- WHEN Cairn emits workflow status or instructions
- THEN the result MUST retain required non-claims
- AND no existing gate, evidence input, or release check may become satisfied by workflow state alone.

#### Scenario: Positive and negative suites close the contract

r[cairn.artifact_workflow_contract.verification]
- GIVEN valid workflow fixtures and cyclic, duplicate, unknown, unsafe, malformed, and overclaim fixtures
- WHEN focused tests and repository lifecycle checks run
- THEN valid cases MUST produce deterministic passing results
- AND invalid cases MUST fail with stable diagnostics and no mutation.

### Requirement: Workflow contract documentation is complete

r[cairn.artifact_workflow_contract.docs] Cairn MUST document workflow commands, fields, state meanings, ordering, policy ownership, compatibility, and non-claims.

#### Scenario: Operator can distinguish readiness from approval

r[cairn.artifact_workflow_contract.docs.boundary]
- GIVEN an operator reads the workflow contract guide
- WHEN the guide describes `ready`, `done`, instructions, and next actions
- THEN it MUST state that these are structural workflow facts
- AND it MUST identify proposal, design, tasks, sync, archive, and release checks as separate authority surfaces.
