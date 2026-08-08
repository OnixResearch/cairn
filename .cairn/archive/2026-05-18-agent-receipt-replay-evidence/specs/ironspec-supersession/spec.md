## MODIFIED Requirements

### Requirement: Agent-run receipts

Cairn SHALL validate deterministic agent-run receipts that bind agent completion evidence to replay-checked Cairn surfaces.

#### Scenario: Replay evidence schema

- **GIVEN** an agent wrapper records completion evidence for a Cairn task
- **WHEN** an agent-run receipt is produced
- **THEN** it SHALL include a `replay_evidence` object containing `replay_receipt_hash`, `replay_policy_hash`, `replay_selected_cases`, and `replay_verdict`
- **AND** the receipt MAY include a display-only `replay_artifact_path`
- **AND** the agent receipt hash SHALL remain the canonical BLAKE3 hash of receipt content excluding only `receipt_hash`
r[cairn.supersede.agent_receipt_replay_evidence.schema]

#### Scenario: Replay evidence validation

- **GIVEN** an agent-run receipt cites deterministic replay evidence
- **WHEN** `agent-receipt validate` or the MCP validation tool validates an agent-run receipt
- **THEN** Cairn SHALL verify that replay evidence cites a passing replay verdict, a BLAKE3-shaped replay receipt hash, the current generated policy hash, and the required replay case coverage
- **AND** the validation receipt SHALL expose the current replay policy hash and required replay cases used for that decision
r[cairn.supersede.agent_receipt_replay_evidence.validation]

#### Scenario: Deterministic replay evidence failures

- **GIVEN** an agent-run receipt has absent, stale, failed, or incomplete replay evidence
- **WHEN** Cairn validates the receipt
- **THEN** agent receipt validation SHALL fail deterministically with stable issue codes and messages
r[cairn.supersede.agent_receipt_replay_evidence.failures]

#### Scenario: Replay oracle covers receipt validation

- **GIVEN** the policy-owned replay plan includes agent receipt validation
- **WHEN** `cairn determinism replay` runs the policy-owned replay plan
- **THEN** the replay oracle SHALL continue to cover `agent-receipt validate` using a replay-evidence-positive receipt fixture
r[cairn.supersede.agent_receipt_replay_evidence.replay]

#### Scenario: Operator documentation and schema

- **GIVEN** operators inspect Cairn public docs or generated policy schemas
- **WHEN** they review agent receipt validation requirements
- **THEN** the stable replay-evidence fields and validation failure semantics SHALL be documented and tested
r[cairn.supersede.agent_receipt_replay_evidence.docs]
