# Proposal: Add TUI explorer

## Why

Cairn projects lifecycle state into static views (kanban, mindmap, graph) and a minimal opt-in terminal canvas. Operators still leave the terminal to read proposals, specs, tasks, and dry-run plans, and the existing terminal mode covers only three view projections with navigation-only controls.

A full-screen terminal explorer keeps the whole review loop — browse changes and specs, read rendered Markdown, inspect task and requirement tables, preview sync/archive plan diffs — inside one read-only session, without creating a second source of truth.

The sibling `../rats/subwayrat` workspace already provides maintained Onix ratatui components, and Cairn already pins `rat-nodegraph` from it over public HTTPS. This change extends that precedent to a bounded component set.

## What Changes

- Add an opt-in `cairn tui` command that opens a full-screen, read-only lifecycle explorer.
- Add a pure, deterministic explorer view-model in `cairn-core`: screen-independent state, a key-event reducer, and hash-bound projection from the existing lifecycle snapshot.
- Add tree navigation across repository, changes, specs, requirements, scenarios, and tasks, plus incremental fuzzy jump.
- Add rendered Markdown for proposals, designs, and specs, and table projections for requirements, scenarios, and tasks.
- Add in-memory sync/archive dry-run plan previews rendered as unified diffs; `--execute` remains CLI-only.
- Extend the pinned public-HTTPS Subwayrat dependency set with an exact, minimal component list, lockfile entries, and Nix vendor hashes.
- Add positive and negative tests, documentation, and bounded non-claims.

## Impact

- **Core**: Add the pure explorer view-model and key-event reducer under `crates/cairn-core`, reusing the lifecycle view semantic model.
- **CLI**: Add the `cairn tui` shell in `crates/cairn-cli`: TTY admission, terminal setup/teardown, event loop, and widget adaptation.
- **Dependencies**: Add pinned Subwayrat components (`rat-markdown`, `rat-table`, `rat-diff`, `rat-chrome`, `rat-widgets`) alongside the existing `rat-nodegraph` pin.
- **Testing**: Add core reducer tests, non-TTY and unsupported-terminal rejection tests, read-only filesystem invariant tests, and deterministic projection tests.
- **Docs**: Document controls, the read-only boundary, the component pin policy, and traceability non-claims.

## Non-goals

- Do not add a desktop or web GUI. Subwayrat is a terminal (ratatui) component set; no GUI toolkit is part of this change.
- Do not edit, create, or delete lifecycle content from the TUI. Authoring stays in files; mutation commands stay CLI-only.
- Do not expose `sync --execute` or `archive --execute` through the TUI.
- Do not require `../rats/`, SSH credentials, or network access to build Cairn.
- Do not adopt Subwayrat components whose transitive dependencies are SSH-only (current `ratcore` consumers such as `rat-tree` and `rat-fuzzy`) until upstream provides a public pin.
- Do not treat a TUI session as gate, readiness, or release evidence.
- Do not claim terminal rendering parity across all emulators, mouse-required workflows, or accessibility certification.
