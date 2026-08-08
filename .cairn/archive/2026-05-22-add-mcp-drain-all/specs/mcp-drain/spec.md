## ADDED Requirements

### Requirement: MCP repository drain planning

r[cairn.mcp_drain_all.plan] Cairn MUST expose a read-only MCP tool that builds a deterministic drain plan for all active changes in a repository.

#### Scenario: Drain plan is read-only and deterministic

r[cairn.mcp_drain_all.plan.read_only]

- GIVEN an MCP client calls the repository drain planning tool
- WHEN Cairn evaluates the active lifecycle state
- THEN the tool MUST return a structured receipt with `mcp.read_only=true` and `mcp.destructive=false`
- AND it MUST NOT execute sync, archive, Git, network, process, or verification side effects.

#### Scenario: Active changes include gate and task readiness

r[cairn.mcp_drain_all.plan.readiness]

- GIVEN a repository has one or more active changes
- WHEN the MCP drain planning tool returns a receipt
- THEN each active change entry MUST include task counts plus proposal, design, and tasks gate validity/verdict summaries.

#### Scenario: Blockers and next actions are explicit

r[cairn.mcp_drain_all.plan.next_actions]

- GIVEN at least one active change is not drain-ready
- WHEN the MCP drain planning receipt is reviewed
- THEN the receipt MUST include deterministic blocker classes and safe next MCP or CLI actions for each blocked change.

#### Scenario: Mutation remains explicit

r[cairn.mcp_drain_all.plan.mutation_boundary]

- GIVEN a change is ready to sync and archive
- WHEN the MCP drain planning tool recommends the drain sequence
- THEN it MUST recommend existing mutation-capable sync/archive tools rather than performing those mutations itself.
