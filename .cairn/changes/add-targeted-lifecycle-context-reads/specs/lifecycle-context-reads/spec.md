# Targeted lifecycle context delta

## ADDED Requirements

### Requirement: Exact typed selectors
r[cairn.targeted_context.selectors] The system MUST admit mutually exclusive requirement, task, or section selectors within an explicit artifact and lifecycle scope without inventing persistent identities.

#### Scenario: A declared identity selects one source block
- GIVEN an admitted specification requirement ID or task ID within the selected artifact
- WHEN the selector model resolves the request
- THEN it selects exactly that definition and not references, code examples, or another artifact

#### Scenario: Missing or ambiguous identity cannot choose a first match
- GIVEN an unknown ID, duplicate definition, undeclared task identity, or ambiguous heading path
- WHEN the model resolves the selector
- THEN it returns a typed missing, unsupported, or ambiguous result without synthetic IDs or silent fallback

### Requirement: Exact bounded source spans
r[cairn.targeted_context.spans] The system MUST bind selected content to the source BLAKE3 identity, UTF-8 byte span, artifact scope, selector, and explicit completeness status.

#### Scenario: The selected span matches admitted source bytes
- GIVEN a well-formed artifact with nested headings and fenced examples
- WHEN the shell returns the selected block
- THEN its span reproduces the exact admitted source bytes and respects structural section boundaries

#### Scenario: A bound cannot split or misrepresent selected content
- GIVEN a selected block exceeds its named bound or a supplied source identity is stale
- WHEN the read executes
- THEN it reports a bounded error rather than silently truncating content or returning a different source

### Requirement: Context and authority stay explicit
r[cairn.targeted_context.authority] The system MUST preserve source authority classes, document diagnostics, and bounded prerequisite references without automatic transitive content expansion.

#### Scenario: Relevant prerequisites remain discoverable
- GIVEN a selected task with declared requirement and predecessor links
- WHEN Cairn returns the task
- THEN it returns bounded typed references and source authority labels without claiming those links satisfy the task

#### Scenario: A valid fragment cannot hide invalid source
- GIVEN a resolvable block in an artifact with malformed or duplicate declarations elsewhere
- WHEN the selected read returns content
- THEN it reports selection validity separately from artifact validity and retains the applicable diagnostics

### Requirement: Read-only CLI and MCP parity
r[cairn.targeted_context.transport] The system MUST expose equivalent narrow reads through CLI and MCP while preserving existing whole-artifact reads and no-follow source admission.

#### Scenario: Both transports return the same selection
- GIVEN equal explicit roots, selectors, source identities, and policy
- WHEN CLI and MCP read one block
- THEN selected bytes, spans, diagnostics, authority labels, and semantic identities agree

#### Scenario: Unsafe paths and unsupported options fail without effects
- GIVEN traversal, a linked component, invalid UTF-8, or an unknown selector field
- WHEN the shell admits the request
- THEN it rejects the read without mutation, network access, or ambient prompt discovery

### Requirement: Narrow-read acceptance evidence
r[cairn.targeted_context.conformance] The system MUST retain positive and negative selector fixtures and compare complete narrow-read tasks with whole-artifact baselines under the shared benchmark.

#### Scenario: A narrow read supplies the required source
- GIVEN the same task and repository-owned oracle for both read strategies
- WHEN the benchmark runs complete diagnostic or authoring attempts
- THEN it records exact exposed tokens, follow-up reads, and oracle outcomes

#### Scenario: Missing context prevents a false savings claim
- GIVEN a smaller selection causes the agent to miss a required constraint or fail the oracle
- WHEN the benchmark compares outcomes
- THEN it retains that failure and withholds an improvement claim
