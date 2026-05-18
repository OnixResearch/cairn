## MODIFIED Requirements

### Requirement: Native Gate Policy Commands

r[cairn.supersede.gate_policy] The system MUST provide native `cairn gate proposal <change>`, `cairn gate design <change>`, and `cairn gate tasks <change>` commands that evaluate AgentKit-compatible stage policy from Cairn-owned Nickel configuration without depending on AgentKit at runtime.

#### Scenario: Gate receipt for a ready change

- GIVEN a repository with an active change whose required artifacts satisfy the requested gate stage
- WHEN an operator runs `cairn gate tasks <change>`
- THEN the command MUST emit a deterministic JSON receipt with the change name, stage, verdict, issues, policy, and layout.

#### Scenario: Gate failure for a missing artifact

- GIVEN a repository with an active change that is missing an artifact required by a requested gate stage
- WHEN an operator runs `cairn gate design <change>`
- THEN the command MUST emit a deterministic JSON receipt whose verdict is `FAIL` and whose issues identify the missing artifact.

#### Scenario: AgentKit policy alignment without vendoring

- GIVEN AgentKit gate policy defines stage vocabulary, marker policy, finding defaults, archive-ignore behavior, and bounded exemptions
- WHEN Cairn exports its default policy
- THEN the exported policy MUST include equivalent native fields owned under `cairn-policy/` rather than importing AgentKit policy at runtime.
