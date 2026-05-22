# Developer Experience Specification

## Purpose

Defines deterministic scaffolding features that help operators and agents create reviewable Cairn change packages from explicit intent while preserving auditability and FCIS boundaries.

## Requirements

### Requirement: Prompt-to-change scaffold planner
r[cairn.developer_experience.prompt_to_change_scaffold.planner] The system MUST plan a complete draft change package from explicit operator-provided request material without network or LLM execution.

#### Scenario: Plan package artifacts from request text
r[cairn.developer_experience.prompt_to_change_scaffold.planner.request_text]
- GIVEN explicit request text, a change name, and an optional spec domain
- WHEN Cairn plans a prompt-to-change scaffold
- THEN the planner returns proposal, design, tasks, and delta spec artifact bodies
- AND the plan records a stable input hash over the provided request material

#### Scenario: Reject insufficient scaffold input
r[cairn.developer_experience.prompt_to_change_scaffold.planner.insufficient_input]
- GIVEN empty request material or an invalid change name
- WHEN Cairn plans a prompt-to-change scaffold
- THEN the planner returns deterministic issues
- AND no package artifacts are approved for execution

### Requirement: Prompt-to-change scaffold CLI
r[cairn.developer_experience.prompt_to_change_scaffold.cli] The system MUST expose prompt-to-change scaffolding through preview and explicit execute CLI flows.

#### Scenario: Preview scaffold writes
r[cairn.developer_experience.prompt_to_change_scaffold.cli.preview]
- GIVEN a valid scaffold request
- WHEN the operator runs the scaffold command without execute mode
- THEN Cairn emits a deterministic receipt listing the root-relative files that would be written
- AND no filesystem mutation occurs

#### Scenario: Execute scaffold writes
r[cairn.developer_experience.prompt_to_change_scaffold.cli.execute]
- GIVEN a valid scaffold request and an initialized native Cairn repository
- WHEN the operator runs the scaffold command in execute mode
- THEN Cairn writes the planned change package files under `cairn/changes/<change>/`
- AND the mutation receipt records stable before and after file hashes

### Requirement: Gate-ready scaffold output
r[cairn.developer_experience.prompt_to_change_scaffold.gate_ready] The system MUST generate scaffolded task references and delta scenarios that are structurally gate-ready.

#### Scenario: Scaffolded task references match delta IDs
r[cairn.developer_experience.prompt_to_change_scaffold.gate_ready.refs]
- GIVEN a scaffolded change package
- WHEN Cairn validates the package and evaluates the proposal, design, and tasks gates
- THEN task checkboxes reference requirement IDs declared by the same delta spec or explicit dependency markers
- AND the gates do not fail because of unknown task references or missing EARS scenario clauses

### Requirement: Prompt-to-change operator documentation
r[cairn.developer_experience.prompt_to_change_scaffold.docs] The system MUST document prompt-to-change scaffolding as deterministic structure generation rather than built-in AI implementation.

#### Scenario: Docs state non-claims
r[cairn.developer_experience.prompt_to_change_scaffold.docs.non_claims]
- GIVEN an operator reads the prompt-to-change scaffold documentation
- WHEN they inspect the command description
- THEN the documentation states that Cairn consumes explicit local input and does not call remote LLMs, credentials, network services, or hidden agent state

### Requirement: Prompt-to-change scaffold verification
r[cairn.developer_experience.prompt_to_change_scaffold.verification] The system MUST test scaffold generation with positive and negative deterministic fixtures.

#### Scenario: Verify scaffold fixtures
r[cairn.developer_experience.prompt_to_change_scaffold.verification.fixtures]
- GIVEN reviewed scaffold input fixtures
- WHEN the workspace verification suite runs
- THEN valid fixtures produce stable artifact plans and receipts
- AND invalid inputs produce stable failure issue codes without writing files
