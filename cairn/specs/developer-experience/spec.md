# Developer Experience Specification

## Purpose

Defines user-facing Cairn workflow improvements that make deterministic lifecycle work easier to start without weakening policy ownership, native layout semantics, receipt evidence, or FCIS boundaries.

## Requirements

### Requirement: Onboarding template catalog
r[cairn.developer_experience.onboarding_templates.catalog] The system MUST expose a reviewed onboarding template catalog for supported first-use repository profiles.

#### Scenario: List reviewed templates
r[cairn.developer_experience.onboarding_templates.catalog.list]
- GIVEN a Cairn policy bundle with reviewed onboarding template metadata
- WHEN the operator asks Cairn to list onboarding templates
- THEN the CLI emits a deterministic receipt containing sorted template identifiers, descriptions, supported files, and policy hash
- AND the receipt does not depend on filesystem traversal order, current time, network access, or agent-specific environment values

### Requirement: Onboarding template application
r[cairn.developer_experience.onboarding_templates.apply] The system MUST apply onboarding templates only through explicit safe preview or execute flows.

#### Scenario: Preview template materialization
r[cairn.developer_experience.onboarding_templates.apply.preview]
- GIVEN a repository root and a supported onboarding template identifier
- WHEN the operator previews template application
- THEN the CLI emits a deterministic plan receipt listing every file that would be created or skipped
- AND no filesystem mutation occurs

#### Scenario: Execute template materialization
r[cairn.developer_experience.onboarding_templates.apply.execute]
- GIVEN a repository root and a reviewed template plan with no blockers
- WHEN the operator executes template application
- THEN the CLI creates only safe root-relative template paths
- AND the mutation receipt records before and after BLAKE3 hashes for touched files

#### Scenario: Reject unsafe or conflicting template paths
r[cairn.developer_experience.onboarding_templates.apply.reject_unsafe]
- GIVEN a template expansion contains traversal, absolute paths, or an overwrite conflict without an explicit allowed policy
- WHEN the operator previews or executes the template
- THEN Cairn emits a deterministic failure receipt
- AND no unsafe path is written

### Requirement: Onboarding quickstart documentation
r[cairn.developer_experience.onboarding_templates.quickstart] The system MUST document a concise template-based first-use path for new Cairn repositories.

#### Scenario: Follow documented quickstart
r[cairn.developer_experience.onboarding_templates.quickstart.follow]
- GIVEN a user follows the documented onboarding template quickstart in a clean test repository
- WHEN they run the listed Cairn commands
- THEN the repository has a native `cairn/` lifecycle layout and at least one reviewable change package
- AND the documented validation command emits a passing deterministic receipt

### Requirement: Onboarding template verification
r[cairn.developer_experience.onboarding_templates.verification] The system MUST regression-test onboarding templates with deterministic positive and negative coverage.

#### Scenario: Verify template receipts
r[cairn.developer_experience.onboarding_templates.verification.receipts]
- GIVEN positive and negative onboarding template fixtures
- WHEN the workspace verification suite runs
- THEN supported templates produce stable passing receipts
- AND unsupported identifiers, unsafe paths, and overwrite conflicts produce stable failure issue codes

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

### Requirement: Reverse-spec discovery policy
r[cairn.developer_experience.reverse_spec_discovery.policy] The system MUST own reverse-spec discovery scopes, limits, ignore classes, and public receipt fields in reviewed policy.

#### Scenario: Load discovery policy
r[cairn.developer_experience.reverse_spec_discovery.policy.load]
- GIVEN a generated Cairn policy bundle with reverse-spec discovery metadata
- WHEN the discovery command starts
- THEN the command uses policy-owned file count limits, byte limits, ignored path classes, redaction classes, and receipt field declarations
- AND missing or malformed policy produces a deterministic failure receipt

### Requirement: Reverse-spec discovery planner
r[cairn.developer_experience.reverse_spec_discovery.planner] The system MUST derive candidate requirements from explicit caller-provided repository evidence snapshots deterministically.

#### Scenario: Plan candidates from source snapshots
r[cairn.developer_experience.reverse_spec_discovery.planner.candidates]
- GIVEN sorted source, test, and documentation snapshots within policy limits
- WHEN Cairn plans reverse-spec discovery
- THEN the planner emits sorted candidate capabilities with stable identifiers, evidence references, review-needed status, and input hashes
- AND the planner performs no filesystem, environment, process, time, stdout/stderr, network, or JSON presentation effects

#### Scenario: Enforce discovery limits
r[cairn.developer_experience.reverse_spec_discovery.planner.limits]
- GIVEN repository evidence that exceeds policy-owned file count or byte limits
- WHEN Cairn plans reverse-spec discovery
- THEN the planner emits deterministic limit issues
- AND no candidate is marked accepted or verified

### Requirement: Reverse-spec discovery CLI
r[cairn.developer_experience.reverse_spec_discovery.cli] The system MUST expose reverse-spec discovery as a read-only command with optional draft artifact output that never mutates accepted specs.

#### Scenario: Emit discovery receipt
r[cairn.developer_experience.reverse_spec_discovery.cli.receipt]
- GIVEN explicit include paths in a repository
- WHEN the operator runs reverse-spec discovery
- THEN Cairn emits a deterministic receipt with selected paths, skipped paths, candidate requirements, policy hash, input hash, and receipt hash
- AND the command does not modify repository files unless an explicit output path for draft artifacts is provided

#### Scenario: Write draft artifacts only under explicit output
r[cairn.developer_experience.reverse_spec_discovery.cli.out]
- GIVEN a valid discovery plan and an explicit safe output path
- WHEN the operator requests draft artifact output
- THEN Cairn writes only the draft files declared in the receipt
- AND it does not modify `cairn/specs/` or promote candidates to accepted requirements

### Requirement: Reverse-spec discovery documentation
r[cairn.developer_experience.reverse_spec_discovery.docs] The system MUST document reverse-spec discovery as candidate generation requiring human or agent review.

#### Scenario: Docs state review-needed semantics
r[cairn.developer_experience.reverse_spec_discovery.docs.review_needed]
- GIVEN an operator reads reverse-spec discovery documentation
- WHEN they inspect the workflow
- THEN the documentation states that candidates are not accepted specs, proof of implementation, or verification evidence until reviewed through normal Cairn lifecycle gates

### Requirement: Reverse-spec discovery verification
r[cairn.developer_experience.reverse_spec_discovery.verification] The system MUST regression-test reverse-spec discovery for deterministic output, limits, redaction, and read-only behavior.

#### Scenario: Verify discovery fixtures
r[cairn.developer_experience.reverse_spec_discovery.verification.fixtures]
- GIVEN reviewed discovery fixtures with source, test, docs, oversized input, and sensitive-looking content
- WHEN the workspace verification suite runs
- THEN valid inputs produce stable candidate receipts
- AND oversized or redacted inputs produce stable issue codes without accepting inferred specs
