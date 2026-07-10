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

### Requirement: Dependency authoring CLI
r[cairn.change_dependency_graph.authoring_cli] Cairn MUST expose local change dependency authoring commands that list, add, and remove `depends_on` metadata without requiring operators to hand-edit `metadata.json`.

#### Scenario: Dependency list is deterministic
r[cairn.change_dependency_graph.authoring_cli.list]
- GIVEN a change declares local dependencies in metadata
- WHEN the operator runs the dependency list command
- THEN Cairn MUST emit the dependencies in deterministic change-id order and MUST NOT mutate lifecycle files.

#### Scenario: Dependency add previews by default
r[cairn.change_dependency_graph.authoring_cli.preview]
- GIVEN an operator requests adding a valid dependency between known changes
- WHEN the command runs without `--execute`
- THEN Cairn MUST emit a dry-run receipt showing the proposed metadata change and MUST leave files unchanged.

#### Scenario: Dependency add executes explicitly
r[cairn.change_dependency_graph.authoring_cli.execute]
- GIVEN an operator requests adding a valid dependency between known changes
- WHEN the command runs with `--execute`
- THEN Cairn MUST write deterministic metadata preserving unrelated supported fields and sorted unique `depends_on` entries.

#### Scenario: Dependency remove clears empty field
r[cairn.change_dependency_graph.authoring_cli.remove]
- GIVEN a change has one declared dependency
- WHEN the operator removes that dependency with `--execute`
- THEN Cairn MUST remove the `depends_on` field rather than writing an empty dependency array.

#### Scenario: Invalid dependency edit fails closed
r[cairn.change_dependency_graph.authoring_cli.invalid]
- GIVEN a requested dependency edit references an invalid change id, an unknown change, the same change, a duplicate dependency input, or a cycle-forming edge
- WHEN the operator previews or executes the edit
- THEN Cairn MUST reject the edit with deterministic diagnostics and MUST NOT mutate lifecycle files.

### Requirement: Human graph render formats
r[cairn.change_dependency_graph.render_formats] Cairn MUST keep the dependency graph JSON receipt as the automation default and MAY expose stable human-oriented graph render formats for review and documentation.

#### Scenario: JSON graph remains default
r[cairn.change_dependency_graph.render_formats.json_default]
- GIVEN an operator runs the existing change graph command without a format override
- WHEN Cairn renders the graph
- THEN the output MUST remain the deterministic JSON graph receipt.

#### Scenario: Mermaid and DOT outputs are stable
r[cairn.change_dependency_graph.render_formats.human]
- GIVEN an operator requests Mermaid or DOT graph output
- WHEN Cairn renders the graph
- THEN the output MUST sort nodes and edges deterministically and label active blockers separately from archived satisfied dependencies.

#### Scenario: Unknown format fails closed
r[cairn.change_dependency_graph.render_formats.invalid]
- GIVEN an operator requests an unsupported graph render format
- WHEN Cairn parses the command
- THEN Cairn MUST reject the request with a deterministic error and MUST NOT fall back to another format silently.

### Requirement: Next-ready change discovery
r[cairn.change_dependency_graph.next_ready] Cairn MUST expose a read-only command that identifies the next dependency-ready active change from the dependency-first drain order.

#### Scenario: Ready change is reported first
r[cairn.change_dependency_graph.next_ready.ready]
- GIVEN at least one active change has no unsatisfied dependency blockers or dependency issues
- WHEN the operator asks for the next change
- THEN Cairn MUST report the first dependency-ready change in deterministic drain order.

#### Scenario: Blocked repository explains blockers
r[cairn.change_dependency_graph.next_ready.blocked]
- GIVEN no active change is dependency-ready
- WHEN the operator asks for the next change
- THEN Cairn MUST report that no change is ready and MUST include deterministic blockers for blocked changes.

### Requirement: Dependency discoverability docs
r[cairn.change_dependency_graph.docs.discoverability] Cairn documentation MUST explain how to author local dependencies, inspect graph order, and drain active changes safely.

#### Scenario: Documentation covers local dependency semantics
r[cairn.change_dependency_graph.docs.discoverability.local]
- GIVEN an operator reads the local lifecycle documentation
- WHEN dependency graph behavior is described
- THEN the documentation MUST show `depends_on` metadata, authoring commands, graph rendering commands, active dependency blockers, archived dependency satisfaction, and cycle rejection.

#### Scenario: Documentation distinguishes evidence dependencies
r[cairn.change_dependency_graph.docs.discoverability.boundary]
- GIVEN cross-repo evidence dependencies are also documented
- WHEN local dependency graph behavior is described
- THEN the documentation MUST distinguish local ordering dependencies from cross-repo receipt/evidence dependencies.

### Requirement: Dependency authoring verification
r[cairn.change_dependency_graph.authoring_cli.validation] Cairn MUST verify dependency authoring behavior with positive and negative tests across pure core and CLI surfaces.

#### Scenario: Positive authoring tests pass
r[cairn.change_dependency_graph.authoring_cli.tests.positive]
- GIVEN dependency add, list, remove, render, archived-satisfaction, and next-ready happy paths
- WHEN focused tests run
- THEN the tests MUST prove deterministic receipts, sorted metadata, stable render output, and expected readiness behavior.

#### Scenario: Negative authoring tests fail correctly
r[cairn.change_dependency_graph.authoring_cli.tests.negative]
- GIVEN malformed metadata, unknown dependencies, self-dependencies, duplicate dependency input, cycle-forming edits, unsupported render formats, and blocked mutation attempts
- WHEN focused tests run
- THEN the tests MUST prove Cairn reports expected diagnostics, does not panic, and preserves existing lifecycle files.
