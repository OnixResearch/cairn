# Change Dependency Graph Specification

## Purpose

Improves local change dependency authoring and discovery for the accepted `change-dependency-graph` capability.

## Requirements

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
