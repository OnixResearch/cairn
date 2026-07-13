# Cairn lifecycle views

Cairn projects one lifecycle snapshot into three read-only views:

- **kanban** classifies active changes from metadata, required artifacts, task states, and dependency readiness;
- **mindmap** renders the unambiguous containment hierarchy from repository to changes/specs, requirements, scenarios, and tasks;
- **graph** adds typed dependency, blocking, task-coverage, implementation, verification, dependency-marker, and related-marker links.

The views do not add a second source of truth. Edit normal Cairn lifecycle artifacts and rerun the view command.

## Static commands

JSON is the default deterministic automation format:

```bash
cairn view kanban --root .
cairn view mindmap --root .
cairn view graph --root .
```

Human-readable exports are explicit:

```bash
cairn view kanban --root . --format markdown
cairn view mindmap --root . --format mermaid
cairn view graph --root . --format mermaid
cairn view graph --root . --format dot
```

Unsupported combinations fail explicitly. For example, kanban does not silently reinterpret `--format dot`.

Every static output is derived from the same semantic model. JSON receipts include `input_hash` and `receipt_hash`; Markdown, Mermaid, and DOT exports include those hashes in comments. Nodes, edges, issues, columns, cards, and reasons are sorted before receipt materialization.

## Kanban classification

Each active change appears exactly once:

- `blocked`: explicit blocked metadata, active dependency blockers, dependency issues, or metadata diagnostics;
- `ready_to_archive`: required artifacts are present, dependencies are satisfied, and all marked tasks are complete;
- `in_progress`: task work has started or metadata status is `current`;
- `planned`: metadata is `planned`/`deferred` or tasks are not authored;
- `ready`: dependencies are satisfied and task work has not started.

Blockers take precedence. Cards preserve task counts, declared dependencies, active blockers, metadata status, and the reasons for their classification.

## Interactive terminal mode

Interactive mode is opt-in and requires terminal stdin and stdout:

```bash
cairn view kanban --root . --interactive
cairn view mindmap --root . --interactive
cairn view graph --root . --interactive
```

Controls:

- `q` or `Esc`: quit;
- arrow keys: pan graph and mindmap canvases;
- `+` / `-`: zoom;
- `0`: reset zoom;
- `Tab`: select the next graph node.

The mindmap and graph canvases use the pinned Subwayrat `rat-nodegraph` component. The board uses Ratatui layout primitives because Subwayrat does not currently provide a kanban widget. The dependency is pinned through public HTTPS so standalone Cairn builds do not require `../rats/` or SSH credentials.

The terminal shell intentionally ignores widget actions that could move semantic nodes, create edges, or delete edges. It never writes lifecycle files. Selection, viewport, pan, zoom, color, wrapping, and auto-layout coordinates are display-only and excluded from Cairn hash material.

## Semantic and privacy boundaries

Compatibility Markdown tasks receive snapshot-scoped deterministic ordinal IDs. Reordering source tasks can change those compatibility IDs; authored structured lifecycle IDs remain preferred for durable identity.

Source artifact nodes contain bounded repository-relative paths and source line numbers, not source contents. Cross-repository graph nodes require explicit future receipt inputs; views do not inspect sibling repositories or fetch network state.

A passing view receipt proves only that Cairn deterministically projected the presented lifecycle facts and declared links. It is not proof of requirement satisfaction, implementation correctness, verification soundness, lifecycle readiness, or release eligibility.
