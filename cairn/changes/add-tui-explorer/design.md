# Design: Add TUI explorer

## Context

Cairn already splits lifecycle views into a pure semantic model in `crates/cairn-core` (`view` module) and shells in `crates/cairn-cli` (JSON/Markdown/Mermaid/DOT renderers plus a minimal interactive terminal mode using the pinned `rat-nodegraph` canvas). The interactive mode is navigation-only and covers three projections.

Operators reviewing a change must still open proposals, designs, specs, tasks, and dry-run plan output in separate tools. A single read-only explorer session removes that context switch while preserving the rule that files and Git remain the source of truth.

The sibling `../rats/subwayrat` workspace provides the components. Its workspace pins `ratcore` through an SSH-only Git URL, and `rat-tree`, `rat-fuzzy`, `rat-inline`, `rat-datepicker`, and `rat-leaderkey` depend on `ratcore`. The existing Cairn pin policy requires public HTTPS so standalone builds need neither `../rats/` nor SSH credentials.

## Decisions

### Decision: Reuse the existing view model and add a pure explorer module

**Choice:** Add `view::explorer` in `crates/cairn-core`. The module takes the existing lifecycle snapshot and produces screen-independent explorer state: ordered navigation nodes, per-node content references (bounded repository-relative paths and line numbers, not contents), table projections, and a deterministic key-event reducer.

The reducer is pure: same state plus same event gives same next state. Selection, scroll, and focus are explicit state fields. The module performs no filesystem, environment, clock, or terminal access.

**Rationale:** The lifecycle-views change already proved this core/shell split. A pure reducer is testable with plain assertions, including negative cases such as events on empty states and out-of-range navigation.

### Decision: Extend the cairn-cli terminal shell with `cairn tui`

**Choice:** Add a `cairn tui` subcommand in `crates/cairn-cli` beside the existing `view` terminal code. The shell owns TTY admission, terminal initialization and restoration, the event loop, and adaptation from reducer state to Subwayrat widgets.

Non-TTY stdin or stdout fails explicitly with a structured error. No new crate is introduced.

**Rationale:** TTY handling, terminal teardown, and the Subwayrat dependency already live in `cairn-cli`. A separate crate would duplicate that plumbing without changing the trust boundary.

### Decision: Bound the Subwayrat component set to public-HTTPS-buildable crates

**Choice:** Extend the workspace pin with `rat-markdown` (rendered artifact reading), `rat-table` (requirement and task tables), `rat-diff` (dry-run plan diffs), `rat-chrome` (frames and overlays), and `rat-widgets` (dialogs and notifications). All components come from one exact public-HTTPS Subwayrat revision, recorded in `Cargo.lock` and Nix vendor hashes like the current `rat-nodegraph` pin.

`rat-tree`, `rat-fuzzy`, and other `ratcore` consumers are excluded in this change because `ratcore` is pinned through an SSH-only URL upstream. Navigation uses `rat-widgets` tree view and an incremental filter implemented in the pure reducer. Adopting SSH-transitive components requires a separate change after upstream provides a public pin.

**Rationale:** The documented Cairn policy is that standalone builds must not require sibling checkouts or SSH credentials. Breaking that policy for convenience would make Nix and contributor builds fail closed.

### Decision: Preview plans in memory; never execute from the TUI

**Choice:** The explorer can compute the existing sync/archive dry-run plans in memory and render them through `rat-diff`. The TUI exposes no key binding, widget action, or code path that calls an `--execute` mutation. Widget edit actions are ignored, matching the existing terminal shell.

A post-session filesystem invariant test records BLAKE3 hashes of the lifecycle tree before and after scripted TUI sessions and requires equality.

**Rationale:** Preview-first mutation is an established Cairn contract. Keeping execution out of the TUI removes an entire class of accidental-mutation risk from an interactive surface.

### Decision: Display state stays outside hash material

**Choice:** Selection, focus, scroll offsets, viewport, and theme are display-only and excluded from receipt and projection hashes, consistent with the accepted `lifecycle-views` terminal boundary. The explorer binds its `input_hash` to the lifecycle snapshot only.

**Rationale:** Hash stability must not depend on where an operator moved a cursor.

## Risks / Trade-offs

- Subwayrat APIs are pre-1.0 and may drift; the exact-revision pin plus a thin adapter module in the CLI shell confines breakage, at the cost of manual rev bumps.
- Terminal capability varies across emulators; the explorer requires a ANSI-color terminal and fails explicitly otherwise, which excludes some minimal terminals.
- Excluding `rat-tree` and `rat-fuzzy` duplicates some navigation behavior in the pure reducer; this is accepted to preserve the public-HTTPS build policy.
- Full-screen ratatui sessions are hard to test end-to-end in CI; verification targets the pure reducer, admission failures, and filesystem invariants instead of pixel output.
