## Why

Cairn already records lifecycle hierarchy, task progress, dependency readiness, and requirement-to-evidence links, but operators must currently inspect separate JSON receipts and Markdown files to understand that state. Deterministic kanban, mindmap, and knowledge-graph projections would make the same reviewed lifecycle facts easier to navigate without introducing another source of truth.

## What Changes

- Add a pure semantic lifecycle-view model for active changes, specs, requirements, scenarios, tasks, dependency edges, and traceability evidence.
- Add read-only kanban, mindmap, and knowledge-graph projections with deterministic JSON receipts and stable text graph formats.
- Add an opt-in terminal viewer that uses the pinned Subwayrat node-graph widget where graph navigation benefits from the shared stack.
- Keep terminal layout, selection, pan, and zoom state outside semantic receipt hashes and deny lifecycle mutation from every view surface.

## Impact

- **Core**: new pure lifecycle-view parsing, classification, graph construction, and deterministic rendering helpers.
- **CLI**: new `cairn view kanban|mindmap|graph` read-only command family and optional terminal shell.
- **Dependencies**: pinned public Subwayrat revision plus terminal rendering dependencies in the CLI shell only.
- **Testing**: positive and negative core fixtures, CLI format/error tests, deterministic receipt replay, and non-mutation checks.
