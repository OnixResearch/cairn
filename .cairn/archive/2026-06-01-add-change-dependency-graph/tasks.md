# Tasks

- [x] I1 [serial] Inspect the Trellis graph and scheduling primitives, then define the minimal Cairn-owned pure core surface for dependency graph validation. r[cairn.change_dependency_graph.trellis_primitives]
- [x] I2 [serial] Extend change metadata parsing and validation with optional `depends_on` declarations and deterministic issue codes. r[cairn.change_dependency_graph.metadata]
- [x] I3 [serial] Implement pure dependency graph validation with stable nodes, edges, reverse edges, cycle detection, and topological drain order. r[cairn.change_dependency_graph.graph_validation]
- [x] I4 [serial] Expose dependency graph data through a read-only CLI/MCP receipt and extend `cairn_drain_plan` with dependency blockers and dependency-first next actions. r[cairn.change_dependency_graph.drain_order]
- [x] I5 [serial] Add sync/archive dry-run and execute blockers for unsatisfied dependencies while preserving existing gate blockers. r[cairn.change_dependency_graph.mutation_block]
- [x] V1 [serial] Add positive tests for empty, archived, and active ordered dependency graphs. r[cairn.change_dependency_graph.positive_negative_tests]
- [x] V2 [serial] Add negative tests for unknown, self, duplicate, and cyclic dependencies. r[cairn.change_dependency_graph.positive_negative_tests]
- [x] V3 [serial] Run focused graph/metadata/drain/mutation tests plus Cairn validation, gates, and Tracey coverage. r[cairn.change_dependency_graph.validation]
