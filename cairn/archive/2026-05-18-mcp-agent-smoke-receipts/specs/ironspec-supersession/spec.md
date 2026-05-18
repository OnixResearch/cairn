## ADDED Requirements

### Requirement: MCP agent smoke receipts
r[cairn.supersede.mcp_agent_smoke_receipts] The system MUST provide maintained smoke evidence that proves the default Cairn MCP surface can describe tools, validate an agent-run receipt, and reject malformed tool arguments without mutation.

#### Scenario: Smoke flow exercises catalog and agent receipt validation
r[cairn.supersede.mcp_agent_smoke_receipts.flow]
- GIVEN a repository with the default Cairn MCP tool surface and a maintained positive agent-run receipt fixture
- WHEN the operator runs the maintained MCP agent smoke flow without mutation capability
- THEN the flow MUST inspect the MCP tool catalog
- AND it MUST invoke `cairn_agent_receipt_validate` through the MCP call path against the fixture
- AND it MUST complete without requiring an external MCP client or network service

#### Scenario: Smoke evidence contains stable fields
r[cairn.supersede.mcp_agent_smoke_receipts.evidence]
- GIVEN the maintained MCP agent smoke flow completes
- WHEN its evidence is reviewed by an operator or release gate
- THEN the evidence MUST assert the presence of the `cairn_agent_receipt_validate` tool schema
- AND it MUST assert a structured MCP response containing `structuredContent`
- AND it MUST assert the agent receipt validation verdict and stable hash fields where the receipt contract provides them
- AND it MUST avoid depending on human prose summaries as the only proof

#### Scenario: Smoke flow remains read-only and rejects malformed calls
r[cairn.supersede.mcp_agent_smoke_receipts.read_only]
- GIVEN the maintained MCP agent smoke flow runs without `--allow-mutations`
- WHEN the flow submits a malformed or schema-invalid MCP tool call
- THEN the response MUST be a deterministic invalid-argument error
- AND no lifecycle read, lifecycle plan, or lifecycle mutation dispatch MUST occur for that invalid call
- AND any default mutation-denial branch observed by the smoke evidence MUST report `mutated=false`

#### Scenario: Smoke workflow is documented for operators
r[cairn.supersede.mcp_agent_smoke_receipts.docs]
- GIVEN an operator wants to verify the Cairn MCP and agent receipt surfaces before release
- WHEN they consult the maintained Cairn MCP or receipt documentation
- THEN the documentation MUST name the local smoke command or fixture
- AND it MUST describe which stable receipt fields are intended as release evidence

#### Scenario: Smoke workflow is verified before archive
r[cairn.supersede.mcp_agent_smoke_receipts.verification]
- GIVEN the MCP agent smoke receipt change is ready to archive
- WHEN maintainers run final verification
- THEN focused smoke tests, Cairn validation, stage gates, Tracey coverage, and maintained repository checks MUST pass before the change is synced and archived
