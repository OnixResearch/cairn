# Tasks: Add TUI explorer

## Phase 1: Dependency selection and pinning

- [ ] [serial] I1 Inventory Subwayrat components and transitive dependencies; select the minimal public-HTTPS-buildable set (`rat-markdown`, `rat-table`, `rat-diff`, `rat-chrome`, `rat-widgets`) and record the exclusion of SSH-transitive components (`rat-tree`, `rat-fuzzy`, `rat-inline`, `rat-datepicker`, `rat-leaderkey`). r[cairn.tui_explorer.dependencies]
- [ ] [serial] I2 Pin one exact public-HTTPS Subwayrat revision in the workspace manifest, refresh `Cargo.lock`, and add Nix vendor hashes. r[cairn.tui_explorer.dependencies]
- [ ] [parallel] V1 Verify a clean Nix build resolves every explorer component without SSH credentials or sibling checkouts, and verify vendor-hash drift fails the build. r[cairn.tui_explorer.dependencies]

## Phase 2: Pure explorer view model

- [ ] [serial] I3 Add the pure `view::explorer` module in `crates/cairn-core`: screen-independent state, deterministic ordering, hash binding to the lifecycle snapshot, and the key-event reducer. r[cairn.tui_explorer.view_model]
- [ ] [serial] I4 Add navigation projection across repository, changes, specs, requirements, scenarios, and tasks, plus the incremental filter with clamped selection movement. r[cairn.tui_explorer.navigation]
- [ ] [parallel] V2 Add positive core tests for determinism, ordering, filtering, and hash boundaries, and negative tests for empty snapshots, out-of-range movement, and malformed snapshots. r[cairn.tui_explorer.verification]

## Phase 3: Terminal shell and rendering

- [ ] [serial] I5 Add the `cairn tui` subcommand in `crates/cairn-cli` with TTY admission, capability checks, terminal setup and restoration, and the event loop. r[cairn.tui_explorer.command]
- [ ] [serial] I6 Wire Markdown rendering, requirement/task tables, and in-memory sync/archive dry-run plan diffs through the pinned components, with declared display bounds. r[cairn.tui_explorer.content_rendering]
- [ ] [parallel] V3 Add shell tests for non-TTY rejection, unsupported-terminal rejection, terminal restoration, and oversized-content truncation. r[cairn.tui_explorer.verification]

## Phase 4: Read-only boundary and closeout

- [ ] [serial] I7 Enforce the read-only boundary: discard widget edit actions, exclude display state from hash material, and keep every `--execute` path unreachable from the explorer. r[cairn.tui_explorer.read_only]
- [ ] [parallel] V4 Add the scripted-session filesystem invariant test comparing BLAKE3 hashes of the lifecycle tree before and after explorer sessions, plus negative tests proving edit actions are inert. r[cairn.tui_explorer.read_only]
- [ ] [serial] I8 Document controls, the component pin policy, the read-only boundary, and bounded non-claims in the explorer and lifecycle-views documentation. r[cairn.tui_explorer.verification]
- [ ] [serial] V5 Run policy freshness, formatting, workspace tests, Clippy, core purity, traceability, Cairn validation, all change gates, release readiness, self-dogfood, and the supported-system flake check. r[cairn.tui_explorer.verification]

## Verification Coverage

- `Scenario: Interactive terminal opens the explorer` -> I5, V3
- `Scenario: Non-terminal session fails explicitly` -> I5, V3
- `Scenario: Unsupported terminal fails explicitly` -> I5, V3
- `Scenario: Identical inputs produce identical state` -> I3, V2
- `Scenario: Events on empty state are total` -> I3, V2
- `Scenario: Display state is excluded from hash material` -> I3, I7, V2
- `Scenario: Navigation order is stable` -> I4, V2
- `Scenario: Filter narrows without mutation` -> I4, V2
- `Scenario: Out-of-range movement clamps` -> I4, V2
- `Scenario: Rendered content matches source bytes` -> I6, V3
- `Scenario: Dry-run plan preview executes nothing` -> I6, V4
- `Scenario: Oversized content is bounded` -> I6, V3
- `Scenario: Session leaves files byte-identical` -> I7, V4
- `Scenario: Edit actions are inert` -> I7, V4
- `Scenario: Execution keys do not exist` -> I7, V4
- `Scenario: Selected components build without SSH` -> I2, V1
- `Scenario: SSH-transitive components are excluded` -> I1, V1
- `Scenario: Vendor hash drift fails closed` -> I2, V1
- `Scenario: Core reducer tests cover both paths` -> V2
- `Scenario: Shell tests cover admission failures` -> V3
- `Scenario: Docs state bounded claims` -> I8, V5
