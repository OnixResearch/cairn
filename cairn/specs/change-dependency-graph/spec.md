# Change Dependency Graph Specification

## Purpose

Defines the `change-dependency-graph` capability.

## Requirements

### Requirement: Change dependency metadata
r[cairn.change_dependency_graph.metadata] Cairn MUST support optional change-package dependency declarations in change metadata without making dependencies mandatory for ordinary changes.

#### Scenario: Known dependencies are accepted
r[cairn.change_dependency_graph.metadata.known]
- GIVEN an active change metadata file declares `depends_on` entries for known active or archived change identifiers
- WHEN Cairn validates or reports the repository lifecycle state
- THEN the dependency declarations MUST be preserved in deterministic receipts.

#### Scenario: Malformed dependency metadata fails closed
r[cairn.change_dependency_graph.metadata.invalid]
- GIVEN a change metadata file declares a non-array `depends_on`, an empty dependency id, an invalid change id, an unknown change id, a duplicate dependency, or the change itself
- WHEN Cairn validates or reports the repository lifecycle state
- THEN Cairn MUST emit deterministic dependency issue codes and MUST NOT silently ignore the bad edge.

### Requirement: Pure dependency graph validation
r[cairn.change_dependency_graph.graph_validation] Cairn MUST build and validate the change dependency graph in pure deterministic core logic over in-memory lifecycle data.

#### Scenario: Acyclic dependency graph has stable order
r[cairn.change_dependency_graph.graph_validation.acyclic]
- GIVEN active changes with acyclic dependency declarations
- WHEN Cairn evaluates the dependency graph
- THEN the graph receipt MUST include stable nodes, dependency edges, reverse `blocks` edges, and dependency-first topological order.

#### Scenario: Cycles are rejected
r[cairn.change_dependency_graph.graph_validation.cycle]
- GIVEN active changes whose dependency declarations form a cycle
- WHEN Cairn evaluates the dependency graph
- THEN Cairn MUST report the cycle as invalid and MUST NOT emit the cyclic changes as drain-ready.

### Requirement: Trellis graph primitives guide implementation
r[cairn.change_dependency_graph.trellis_primitives] When Cairn introduces dependency graph algorithms, it SHOULD use the corresponding Trellis graph and scheduling primitives as the model source for topological order, cycle detection, and dependency readiness.

#### Scenario: Trellis primitive boundary remains portable
r[cairn.change_dependency_graph.trellis_primitives.portable]
- GIVEN Cairn implementation uses ideas or code derived from `../trellis/src/topo_sort.rs`, `../trellis/src/cycle_detect.rs`, or `../trellis/src/job_dag.rs`
- WHEN Cairn builds, validates, and releases from a standalone checkout
- THEN normal Cairn runtime builds MUST remain self-contained and MUST NOT require a sibling `../trellis` checkout.

### Requirement: Dependency-aware drain planning
r[cairn.change_dependency_graph.drain_order] Cairn MUST include change dependency graph readiness in read-only drain planning receipts.

#### Scenario: Drain plan orders dependencies first
r[cairn.change_dependency_graph.drain_order.topological]
- GIVEN active changes where one change depends on another active change
- WHEN the operator requests a repository drain plan
- THEN the receipt MUST place the dependency before the dependent in `drain_order` and MUST show the dependent as blocked by the dependency.

#### Scenario: Archived dependencies satisfy blockers
r[cairn.change_dependency_graph.drain_order.archived]
- GIVEN a change depends only on archived changes
- WHEN the operator requests a repository drain plan
- THEN those dependencies MUST be reported as satisfied and MUST NOT block the dependent change's drain readiness.

### Requirement: Dependency blockers protect mutation
r[cairn.change_dependency_graph.mutation_block] Cairn MUST prevent explicit lifecycle mutation for a change while declared dependencies remain unsatisfied.

#### Scenario: Sync and archive execution block on unsatisfied dependencies
r[cairn.change_dependency_graph.mutation_block.unsatisfied]
- GIVEN a change depends on another change that is not archived
- WHEN the operator runs sync or archive with `--execute` for the dependent change
- THEN Cairn MUST refuse mutation, report the unsatisfied dependency blocker, and preserve the existing files unchanged.

#### Scenario: Dry-run plans show dependency blockers
r[cairn.change_dependency_graph.mutation_block.dry_run]
- GIVEN a change has unsatisfied dependencies
- WHEN the operator runs sync or archive without `--execute`
- THEN Cairn MUST report the dependency blockers in the dry-run plan without mutating files.

### Requirement: Positive and negative graph coverage
r[cairn.change_dependency_graph.positive_negative_tests] Cairn MUST verify dependency graph behavior with positive and negative tests across core and CLI surfaces.

#### Scenario: Positive graph cases pass
r[cairn.change_dependency_graph.positive_negative_tests.positive]
- GIVEN changes with no dependencies, archived dependencies, or active acyclic dependencies
- WHEN focused graph and drain tests run
- THEN the tests MUST prove deterministic receipts and expected readiness/order behavior.

#### Scenario: Negative graph cases fail correctly
r[cairn.change_dependency_graph.positive_negative_tests.negative]
- GIVEN changes with unknown dependencies, self dependencies, duplicate dependencies, invalid metadata, or cycles
- WHEN focused graph and drain tests run
- THEN the tests MUST prove Cairn reports the expected issue codes and does not panic or mutate repository state.

### Requirement: Dependency graph verification rail
r[cairn.change_dependency_graph.validation] Cairn MUST include dependency graph checks in the verification rail for this feature.

#### Scenario: Repository checks cover graph behavior
r[cairn.change_dependency_graph.validation.repository_checks]
- GIVEN the dependency graph feature is implemented
- WHEN focused tests, Cairn validation, gates, and Tracey coverage run
- THEN the verification evidence MUST cover metadata parsing, graph validation, drain planning, mutation blockers, and the Trellis primitive boundary.
