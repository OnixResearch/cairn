# Agent Receipt Validation Surface Delta

## MODIFIED Requirements

### Requirement: Agent receipt validation surface
r[cairn.supersede.agent_receipt_validation_surface] The system MUST expose a deterministic public validation surface for agent-run receipts so wrappers can gate completion claims without linking against Cairn internals.

#### Scenario: CLI validates receipt file
r[cairn.supersede.agent_receipt_validation_surface.cli]
- GIVEN an operator has a JSON agent-run receipt file under an allowed path
- WHEN they run the Cairn agent receipt validation command for that file
- THEN the system MUST evaluate the receipt with the pure agent-run completion gate
- AND it MUST return structured JSON rather than prose-only output
- AND it MUST NOT mutate lifecycle content.

#### Scenario: CLI validation receipt has stable schema
r[cairn.supersede.agent_receipt_validation_surface.cli.schema]
- GIVEN the CLI validation command emits a validation receipt
- WHEN the same receipt input is validated repeatedly in the same repository state
- THEN the output MUST expose stable fields for command, path or input identity, validity, verdict, issue list, agent receipt hash, input hash, and top-level receipt hash
- AND the repeated outputs MUST be byte-stable except for explicitly captured nondeterministic inputs, if any.

#### Scenario: MCP validates receipt through read-only tool
r[cairn.supersede.agent_receipt_validation_surface.mcp]
- GIVEN an agent wrapper is connected to the Cairn MCP server
- WHEN it calls the agent receipt validation tool with a bounded receipt path or inline receipt object
- THEN the server MUST evaluate the receipt through the same validation behavior as the CLI surface
- AND it MUST return a structured validation receipt suitable for the wrapper to cite before claiming completion.

#### Scenario: MCP validation remains read-only by default
r[cairn.supersede.agent_receipt_validation_surface.mcp.read_only]
- GIVEN the Cairn MCP server starts without mutation capability
- WHEN the client lists tools or calls the agent receipt validation tool
- THEN the validation tool MUST appear as read-only and non-destructive
- AND path input MUST reject traversal or root escape
- AND inline input MUST be bounded before validation.

#### Scenario: Dogfood fixtures replay pass and fail deterministically
r[cairn.supersede.agent_receipt_validation_surface.fixtures]
- GIVEN source-controlled positive and negative agent-run receipt fixtures
- WHEN validation tests replay those fixtures through the public surface
- THEN positive fixtures MUST produce byte-stable passing validation receipts
- AND negative fixtures MUST fail deterministically with expected issue codes.

#### Scenario: Wrapper flow is documented
r[cairn.supersede.agent_receipt_validation_surface.docs]
- GIVEN an agent wrapper author wants deterministic completion evidence
- WHEN they read the Cairn documentation
- THEN it MUST describe the flow for creating a plan, producing an agent-run receipt, validating it through CLI or MCP, and citing the validation receipt hash.

#### Scenario: Validation surface is verified by repository gates
r[cairn.supersede.agent_receipt_validation_surface.verification]
- GIVEN the validation surface implementation is complete
- WHEN repository verification runs
- THEN focused CLI/MCP tests, formatting, workspace tests, Clippy, Slotcar, Tracey coverage, Cairn validation, and Nix flake checks MUST pass.
