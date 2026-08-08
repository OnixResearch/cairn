## Phase 1: Authoring core and CLI

- [x] [serial] Add pure metadata dependency update helpers for add/remove/list normalization. r[cairn.change_dependency_graph.authoring_cli]
- [x] [serial] Implement `cairn change depend list|add|remove` with preview-by-default mutation receipts and explicit `--execute`. r[cairn.change_dependency_graph.authoring_cli.preview]
- [x] [serial] Reject invalid, unknown, duplicate, self, and cycle-forming dependency edits before mutation. r[cairn.change_dependency_graph.authoring_cli.invalid]

## Phase 2: Discovery and rendering

- [x] [serial] Add `cairn change graph --format json|mermaid|dot` with stable sorted output. r[cairn.change_dependency_graph.render_formats]
- [x] [serial] Add `cairn change next` to report the first dependency-ready change and deterministic blockers for blocked changes. r[cairn.change_dependency_graph.next_ready]
- [x] [parallel] Document `depends_on`, authoring commands, graph formats, active/archived semantics, and safe drain workflow. r[cairn.change_dependency_graph.docs.discoverability]

## Phase 3: Verification

- [x] [serial] Add positive tests for add/list/remove, executed metadata updates, archived dependencies, render formats, and next-ready output. r[cairn.change_dependency_graph.authoring_cli.tests.positive]
- [x] [serial] Add negative tests for malformed metadata, unknown dependencies, self-dependencies, duplicate dependency input, cycle-forming edits, and blocked mutations. r[cairn.change_dependency_graph.authoring_cli.tests.negative]
- [x] [serial] Run focused CLI/core tests plus Cairn validation and gates for this change before sync/archive. r[cairn.change_dependency_graph.authoring_cli.validation]
