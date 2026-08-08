## Context

Cairn has a deterministic local change DAG, aggregate Markdown task states, structured lifecycle bundles, and traceability occurrence maps. These facts are exposed through separate CLI and MCP receipts. The view feature must compose them without moving filesystem access into `cairn-core`, treating visual layout as evidence, or letting an interactive widget edit lifecycle state.

## Decisions

### 1. One semantic model, multiple projections

**Choice:** Build a sorted `LifecycleViewModel` in the pure core from normalized in-memory repository, task, spec, dependency, and traceability inputs. Represent repository, change, spec, requirement, scenario, task, and source-artifact nodes plus typed containment, coverage, dependency, blocking, implementation, verification, and relationship edges.

**Rationale:** A common model prevents each renderer from inventing different lifecycle semantics and makes positive/negative tests independent of terminal I/O.

### 2. Explicit stable identity and BLAKE3 receipt binding

**Choice:** Prefer authored lifecycle IDs. Compatibility Markdown nodes use deterministic repository-relative identities derived from owning IDs and source ordinals. Sort nodes and edges before rendering and bind semantic data with BLAKE3. Exclude terminal coordinates, selection, viewport, pan, zoom, color, and wrapping from semantic hash material.

**Rationale:** Visual layout may vary with terminal dimensions or renderer behavior and therefore cannot support Cairn evidence claims. Semantic identity and ordering can remain portable.

### 3. Kanban columns are derived, not authored state

**Choice:** Classify active changes into `planned`, `ready`, `in_progress`, `blocked`, or `ready_to_archive` from policy-valid metadata status, dependency readiness/issues, required artifact presence, and itemized task states. Preserve the contributing facts and classification reasons on each card.

**Rationale:** A board should explain current Cairn state rather than add a second workflow-state file. Archived changes remain outside the active board.

### 4. Mindmap is a deterministic containment projection

**Choice:** Build the mindmap from repository/spec/change containment, requirement/scenario hierarchy, and change/task hierarchy. Keep non-hierarchical links in the knowledge graph instead of selecting an arbitrary spanning tree.

**Rationale:** Multi-parent traceability and dependency edges would make a mindmap ambiguous. Restricting it to containment gives every node one reviewed parent.

### 5. Knowledge graph preserves typed links and diagnostics

**Choice:** Include dependency, task coverage, and traceability edges in the graph. Preserve dangling references as deterministic issues rather than dropping them. Source artifacts use bounded repository-relative paths and never include file contents in view receipts.

**Rationale:** Missing links are operationally important, while raw source content and private absolute paths are outside the view contract.

### 6. Static exports are authoritative; Subwayrat is a read-only display shell

**Choice:** Make JSON the default output. Support stable Markdown for kanban, Mermaid for mindmap, and Mermaid/DOT for the graph. Add explicit `--interactive` terminal mode; use `rat-nodegraph` from a pinned public Subwayrat revision for mindmap/graph canvas rendering and Ratatui primitives for the board. Ignore all widget edit actions and expose only navigation controls.

**Rationale:** Static receipts remain automation-friendly and testable. Subwayrat adds useful pan/zoom/node rendering without placing UI concerns in the core or relying on a sibling checkout at runtime.

## Risks / Trade-offs

- **Markdown compatibility IDs can move when tasks are reordered.** Structured lifecycle task IDs remain preferred; compatibility IDs are scoped to the exact hashed snapshot and make no durable identity claim.
- **Large repositories can produce crowded graphs.** The terminal viewer provides pan/zoom, while filters and workspace aggregation remain future changes.
- **Pinned Git dependencies require Nix lock handling.** Pin an HTTPS revision and add required fixed-output hashes so standalone builds do not depend on `../rats/` or SSH credentials.
- **Traceability links do not prove correctness.** View receipts preserve Cairn's existing non-claim boundary: they show declared linkage only.
