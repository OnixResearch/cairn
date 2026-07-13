# Lifecycle Views Specification

## Purpose

Defines the `lifecycle-views` capability.

## Requirements

### Requirement: Shared semantic lifecycle-view model
r[cairn.lifecycle_views.semantic_model] Cairn MUST build kanban, mindmap, and knowledge-graph projections from one pure deterministic semantic model over explicitly supplied in-memory lifecycle facts.

#### Scenario: Views share lifecycle semantics
r[cairn.lifecycle_views.semantic_model.shared]
- GIVEN normalized repository, change, spec, task, dependency, and traceability inputs
- WHEN Cairn builds lifecycle views
- THEN every projection MUST use the same stable node identities, typed relationships, issue vocabulary, and source facts
- AND the pure core MUST NOT read files, inspect the environment, execute processes, or render terminal output.

#### Scenario: Markdown compatibility input is explicit
r[cairn.lifecycle_views.semantic_model.markdown]
- GIVEN a repository still authors lifecycle content in Markdown
- WHEN Cairn parses requirements, scenarios, or tasks for a view
- THEN the model MUST preserve bounded source identities and deterministic ordinals
- AND it MUST NOT claim compatibility identities remain durable after the source is reordered.

### Requirement: Deterministic view receipts
r[cairn.lifecycle_views.determinism] Cairn MUST emit deterministic lifecycle-view receipts bound to sorted semantic data with BLAKE3 identities.

#### Scenario: Equivalent inputs produce equivalent receipts
r[cairn.lifecycle_views.determinism.repeat]
- GIVEN equivalent lifecycle inputs under different checkout roots
- WHEN Cairn renders the same static view repeatedly
- THEN semantic nodes, edges, issues, input hashes, and receipt hashes MUST be equivalent after portable path normalization.

#### Scenario: Display state is not evidence
r[cairn.lifecycle_views.determinism.display_state]
- GIVEN an interactive terminal changes selection, viewport, pan, zoom, color, wrapping, or node coordinates
- WHEN Cairn renders the same semantic lifecycle snapshot
- THEN those display-only values MUST NOT change semantic view hash material or lifecycle claims.

### Requirement: Kanban projection
r[cairn.lifecycle_views.kanban] Cairn MUST provide a read-only kanban projection of active changes with deterministic classification reasons.

#### Scenario: Active changes are classified
r[cairn.lifecycle_views.kanban.classification]
- GIVEN active changes with metadata, required artifacts, task states, dependencies, and dependency issues
- WHEN Cairn builds the kanban view
- THEN each change MUST appear exactly once in `planned`, `ready`, `in_progress`, `blocked`, or `ready_to_archive`
- AND each card MUST preserve task counts, dependency facts, metadata status, and deterministic classification reasons.

#### Scenario: Blockers take precedence
r[cairn.lifecycle_views.kanban.blocked]
- GIVEN a change declares blocked metadata, unsatisfied dependencies, or dependency diagnostics
- WHEN Cairn classifies the change
- THEN the card MUST appear in `blocked` and MUST expose the safe blocker reasons rather than silently presenting the change as ready.

### Requirement: Mindmap projection
r[cairn.lifecycle_views.mindmap] Cairn MUST provide a deterministic mindmap projection over lifecycle containment hierarchy.

#### Scenario: Mindmap has unambiguous parents
r[cairn.lifecycle_views.mindmap.hierarchy]
- GIVEN repository, spec, requirement, scenario, change, and task nodes
- WHEN Cairn builds the mindmap
- THEN every non-root mindmap node MUST have exactly one containment parent
- AND dependency, coverage, and traceability relationships MUST remain outside the containment tree.

### Requirement: Lifecycle knowledge graph
r[cairn.lifecycle_views.knowledge_graph] Cairn MUST provide a typed knowledge graph over lifecycle containment, change dependencies, task coverage, and declared traceability relationships.

#### Scenario: Typed links remain distinguishable
r[cairn.lifecycle_views.knowledge_graph.links]
- GIVEN tasks cover requirements, changes depend on other changes, and source artifacts declare implementation, verification, dependency, or related markers
- WHEN Cairn builds the knowledge graph
- THEN the graph MUST preserve each relationship kind as a distinct typed edge
- AND source artifact nodes MUST use bounded repository-relative paths without embedding source contents.

#### Scenario: Dangling links remain visible
r[cairn.lifecycle_views.knowledge_graph.dangling]
- GIVEN a task or source artifact references an unknown requirement or change
- WHEN Cairn builds the graph
- THEN Cairn MUST emit a deterministic dangling-link issue and MUST NOT silently drop or reinterpret the invalid relationship.

### Requirement: Read-only CLI exports
r[cairn.lifecycle_views.cli] Cairn MUST expose static lifecycle views through a read-only `cairn view kanban|mindmap|graph` command family.

#### Scenario: Stable formats are explicit
r[cairn.lifecycle_views.cli.formats]
- GIVEN an operator selects a lifecycle view
- WHEN the operator requests JSON, Markdown, Mermaid, or DOT where supported
- THEN Cairn MUST render a stable documented format
- AND unsupported view-format combinations MUST fail explicitly without falling back to another format.

### Requirement: Read-only terminal viewer
r[cairn.lifecycle_views.terminal] Cairn MAY provide an opt-in terminal viewer using maintained Onix terminal components while preserving static receipts as the automation surface.

#### Scenario: Subwayrat renders graph views
r[cairn.lifecycle_views.terminal.subwayrat]
- GIVEN the operator selects interactive mindmap or knowledge-graph mode in a terminal
- WHEN Cairn opens the viewer
- THEN Cairn SHOULD use a pinned Subwayrat node-graph component for bounded node rendering, pan, and zoom behavior
- AND normal builds MUST NOT depend on a sibling checkout or SSH credentials.

#### Scenario: Terminal mode cannot mutate lifecycle state
r[cairn.lifecycle_views.terminal.read_only]
- GIVEN an interactive widget can produce node-move, edge-create, or edge-delete actions
- WHEN Cairn displays lifecycle data
- THEN the shell MUST ignore mutation actions and MUST NOT write lifecycle files, metadata, task state, or graph edges.

### Requirement: Lifecycle-view documentation
r[cairn.lifecycle_views.docs] Cairn documentation MUST explain the view commands, supported formats, terminal controls, source-of-truth boundary, deterministic hash boundary, and traceability non-claims.

#### Scenario: Operators can distinguish views from authority
r[cairn.lifecycle_views.docs.boundary]
- GIVEN an operator reads the lifecycle-view documentation
- WHEN the documentation describes visual state
- THEN it MUST state that views project current Cairn artifacts and declared links
- AND they do not independently prove requirement satisfaction, implementation correctness, verification soundness, lifecycle readiness, or release eligibility.

### Requirement: Positive and negative lifecycle-view verification
r[cairn.lifecycle_views.verification] Cairn MUST verify lifecycle views with positive and negative tests across pure core, static CLI, and terminal adaptation boundaries.

#### Scenario: Valid view fixtures pass
r[cairn.lifecycle_views.verification.positive]
- GIVEN valid changes, tasks, specs, dependencies, and traceability links
- WHEN focused lifecycle-view tests run
- THEN tests MUST prove stable classification, hierarchy, typed links, render output, terminal model adaptation, and repeated receipt equality.

#### Scenario: Invalid view fixtures fail correctly
r[cairn.lifecycle_views.verification.negative]
- GIVEN malformed Markdown, dangling links, unsupported formats, conflicting interactive arguments, or a non-terminal interactive invocation
- WHEN focused lifecycle-view tests run
- THEN Cairn MUST return deterministic diagnostics, MUST NOT panic, and MUST preserve repository files unchanged.

#### Scenario: Archive requires verification evidence
r[cairn.lifecycle_views.verification.archive]
- GIVEN lifecycle-view implementation tasks are complete
- WHEN the change is synced and archived
- THEN focused tests, repository formatting and lint checks, Cairn validation and gates, and traceability coverage MUST pass or record an exact bounded environmental blocker before archive.
