## Phase 1: MCP drain planning

- [x] [serial] Define the MCP drain-plan contract. r[cairn.mcp_drain_all.plan] r[cairn.mcp_drain_all.plan.read_only] r[cairn.mcp_drain_all.plan.readiness] r[cairn.mcp_drain_all.plan.next_actions] r[cairn.mcp_drain_all.plan.mutation_boundary]
- [x] [serial] Register `cairn_drain_plan` in the MCP catalog with a strict read-only schema. r[cairn.mcp_drain_all.plan] r[cairn.mcp_drain_all.plan.read_only]
- [x] [serial] Implement deterministic per-change readiness, blockers, and next-action receipt content. r[cairn.mcp_drain_all.plan.readiness] r[cairn.mcp_drain_all.plan.next_actions] r[cairn.mcp_drain_all.plan.mutation_boundary]
- [x] [serial] Add positive and negative MCP integration coverage for read-only drain planning and mutation denial boundaries. r[cairn.mcp_drain_all.plan.read_only] r[cairn.mcp_drain_all.plan.readiness] r[cairn.mcp_drain_all.plan.next_actions] r[cairn.mcp_drain_all.plan.mutation_boundary]
- [x] [serial] Validate and gate the native Cairn package. r[cairn.mcp_drain_all.plan]
