# Cairn Agent-Run Receipts Delta

## Purpose

Specify deterministic Cairn receipts for autonomous agent runs so plans, side effects, verification evidence, nondeterministic inputs, and completion claims can be gated and replay-tested.

## Requirements

### Requirement: Agent-run receipt schema
r[cairn.supersede.agent_run_receipts.schema] The system MUST define a canonical JSON agent-run receipt envelope for deterministic evidence about an agent task.

#### Scenario: Canonical receipt includes required evidence fields
r[cairn.supersede.agent_run_receipts.schema.required_fields]
- GIVEN an agent wrapper records a completed task attempt
- WHEN it emits a Cairn agent-run receipt
- THEN the canonical receipt includes `schema_version`, `task_id`, `agent_identity`, `input_hash`, `plan_hash`, `tool_manifest`, `mutation_manifest`, `verification_manifest`, `nondeterminism_manifest`, `verdict`, and `receipt_hash`
- AND `receipt_hash` is derived from canonical receipt content with the hash field excluded

#### Scenario: Receipt avoids raw transcript dependence
r[cairn.supersede.agent_run_receipts.schema.bounded_content]
- GIVEN an agent run includes prompts, tool outputs, logs, or generated artifacts
- WHEN the receipt records that evidence
- THEN the receipt stores bounded summaries, manifests, and content hashes instead of requiring an unbounded raw transcript
- AND secret-bearing values are represented by redacted metadata or hashes rather than plaintext

### Requirement: Agent-run plan linkage
r[cairn.supersede.agent_run_receipts.plan_link] The system MUST link side-effecting agent receipts to a canonical pre-mutation plan hash.

#### Scenario: Side-effecting run cites plan hash
r[cairn.supersede.agent_run_receipts.plan_link.side_effects]
- GIVEN an agent run performs filesystem mutation, lifecycle mutation, network write, secret access, deploy, or other side-effecting action
- WHEN the agent-run receipt is validated
- THEN the receipt includes a non-empty `plan_hash` derived from the canonical pre-mutation plan
- AND the mutation manifest is checked against the approved plan scope

#### Scenario: Read-only run may use explicit no-mutation plan
r[cairn.supersede.agent_run_receipts.plan_link.read_only]
- GIVEN an agent run performs only read-only inspection and produces no external side effects
- WHEN the agent-run receipt is validated
- THEN the receipt may cite a canonical no-mutation plan
- AND the mutation manifest reports no mutated files, lifecycle state, network writes, deploys, or secret-value exposures

### Requirement: Agent-run nondeterminism manifest
r[cairn.supersede.agent_run_receipts.nondeterminism] The system MUST record and classify nondeterministic inputs that can affect agent-run replay.

#### Scenario: Known entropy sources are captured
r[cairn.supersede.agent_run_receipts.nondeterminism.captured]
- GIVEN an agent run reads wall-clock time, network content, model/provider identity, environment variables, random values, remote API responses, or externally supplied artifacts
- WHEN the agent-run receipt is emitted
- THEN each source is represented in the nondeterminism manifest with a classification, bounded metadata, and content hash where content is available
- AND the manifest distinguishes allowed captured entropy from forbidden or uncaptured entropy

#### Scenario: Hidden entropy fails validation
r[cairn.supersede.agent_run_receipts.nondeterminism.hidden]
- GIVEN validation detects an input source that is required by policy but absent from the nondeterminism manifest
- WHEN the receipt is gated
- THEN the gate fails with a deterministic issue identifying the missing entropy class
- AND the receipt is not accepted as replayable evidence

### Requirement: Agent-run completion gate
r[cairn.supersede.agent_run_receipts.completion_gate] The system MUST provide a completion gate that validates agent-run receipts before a task is claimed complete.

#### Scenario: Verified completion passes gate
r[cairn.supersede.agent_run_receipts.completion_gate.verified]
- GIVEN an agent-run receipt reports a successful verdict
- WHEN the completion gate evaluates the receipt
- THEN the gate verifies required checks, artifact hashes, dirty-state disclosure, plan linkage, mutation scope, and receipt hash consistency
- AND it returns a deterministic pass receipt that can be cited by the agent final response

#### Scenario: Missing verification fails gate
r[cairn.supersede.agent_run_receipts.completion_gate.missing_verification]
- GIVEN an agent-run receipt reports a successful verdict without required verification evidence
- WHEN the completion gate evaluates the receipt
- THEN the gate fails with a deterministic issue for missing verification evidence
- AND the agent wrapper MUST NOT claim the task is complete based on that receipt

### Requirement: Agent-run replay fixtures
r[cairn.supersede.agent_run_receipts.replay] The system MUST maintain replay fixtures for canonical receipt stability and failure behavior.

#### Scenario: Positive replay fixture is byte-stable
r[cairn.supersede.agent_run_receipts.replay.positive]
- GIVEN a maintained replay fixture with captured inputs, tool manifest, plan, mutations, verification evidence, and nondeterminism manifest
- WHEN the canonical agent-run receipt is generated twice from the same fixture
- THEN both generated receipts are byte-identical
- AND all BLAKE3 hash fields are stable 64-character lowercase hexadecimal strings

#### Scenario: Negative replay fixtures fail deterministically
r[cairn.supersede.agent_run_receipts.replay.negative]
- GIVEN maintained replay fixtures with missing plan linkage, missing verification evidence, uncaptured nondeterminism, or mutation outside approved scope
- WHEN each fixture is gated
- THEN each failure returns a deterministic issue code and message
- AND no fixture is accepted as successful completion evidence
