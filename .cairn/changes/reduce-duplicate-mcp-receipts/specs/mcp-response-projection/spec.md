# MCP response projection delta

## ADDED Requirements

### Requirement: Explicit response profile
r[cairn.mcp_projection.profile] The system MUST select an admitted response profile explicitly and preserve the existing response shape when no profile is selected.

#### Scenario: A structured client selects the compact response profile
- GIVEN an operator selects the policy-known structured-summary profile
- WHEN Cairn starts MCP or handles a direct MCP call
- THEN the shell uses that profile without changing the admitted tool catalog or capability set

#### Scenario: Unknown client support does not remove compatibility
- GIVEN no explicit profile or an unknown profile value
- WHEN the shell admits startup arguments
- THEN it preserves the legacy default or rejects the unknown value before dispatch

### Requirement: One full payload in the structured profile
r[cairn.mcp_projection.single_payload] The system MUST emit one complete structured receipt and a bounded nonduplicating text summary in the structured-summary profile.

#### Scenario: The structured receipt stays exact
- GIVEN a valid tool result and the structured-summary profile
- WHEN Cairn renders the response
- THEN structuredContent equals the legacy structured payload and retains every receipt field and hash
- AND the text summary does not embed the full receipt again

#### Scenario: An oversized summary does not truncate the receipt
- GIVEN diagnostic prose exceeds the configured summary bound
- WHEN Cairn renders the response
- THEN it emits a bounded summary with explicit omission information and preserves the complete structured payload

### Requirement: Accurate status summaries
r[cairn.mcp_projection.status] The system MUST summarize transport completion, lifecycle state, report validity, and mutation denial without converting one into another.

#### Scenario: Blocked lifecycle state remains visible
- GIVEN a successful transport result with a blocked or unavailable lifecycle state
- WHEN Cairn creates the text summary
- THEN the summary states that lifecycle state and does not claim task completion

#### Scenario: An unknown receipt shape stays uninterpreted
- GIVEN a receipt without an admitted status projection
- WHEN Cairn creates the text summary
- THEN it reports an unclassified result with a structured-detail reference instead of inferring success

### Requirement: Text-only compatibility and effect denial
r[cairn.mcp_projection.compatibility] The system MUST retain complete text in the legacy profile and preserve strict argument validation, mutation denial, and existing execution authority.

#### Scenario: A text-only client retains complete evidence
- GIVEN the legacy response profile and a supported tool result
- WHEN the client reads content.text
- THEN it receives the existing complete receipt representation

#### Scenario: A response profile cannot authorize mutation
- GIVEN mutation capability is absent and a mutation tool is requested under either profile
- WHEN Cairn handles the request
- THEN it denies mutation before argument-dependent product effects and reports mutated as false

### Requirement: Consumer-visible cost evidence
r[cairn.mcp_projection.conformance] The system MUST compare both response profiles through declared host presentations with positive and negative parity fixtures and source-bound benchmark receipts.

#### Scenario: A structured host exposes less duplicate content
- GIVEN a maintained host adapter that exposes structured results to the model
- WHEN the paired benchmark runs the same successful and blocked tasks
- THEN it records exposed token cost and unchanged oracle outcomes for both profiles

#### Scenario: Hidden structured content prevents a misleading savings claim
- GIVEN a host that exposes only text or already removes duplicate payloads
- WHEN the benchmark compares profiles
- THEN it reports that host behavior and withholds unsupported end-to-end savings claims
