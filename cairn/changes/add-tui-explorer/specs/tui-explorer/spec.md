# TUI Explorer Specification Delta

## ADDED Requirements

### Requirement: TUI explorer is an opt-in CLI command

r[cairn.tui_explorer.command] Cairn MUST expose the explorer through an explicit `cairn tui` command that opens a full-screen terminal session only when stdin and stdout are terminals, and MUST fail explicitly otherwise.

#### Scenario: Interactive terminal opens the explorer

r[cairn.tui_explorer.command.tty]
- GIVEN a valid Cairn root and terminal stdin and stdout
- WHEN the operator runs `cairn tui --root .`
- THEN Cairn MUST initialize the explorer, restore the terminal on exit, and leave lifecycle files unchanged.

#### Scenario: Non-terminal session fails explicitly

r[cairn.tui_explorer.command.non_tty]
- GIVEN stdin or stdout is not a terminal
- WHEN the operator runs `cairn tui --root .`
- THEN Cairn MUST exit with a structured error and MUST NOT emit terminal control sequences.

#### Scenario: Unsupported terminal fails explicitly

r[cairn.tui_explorer.command.unsupported]
- GIVEN a terminal without the required ANSI capabilities
- WHEN the explorer checks terminal capabilities at startup
- THEN Cairn MUST fail with a structured error before entering full-screen mode.

### Requirement: Explorer state is a pure deterministic projection

r[cairn.tui_explorer.view_model] Cairn MUST project the lifecycle snapshot into screen-independent explorer state in the pure core, with an explicit key-event reducer, sorted ordering, and hash binding to the input snapshot.

#### Scenario: Identical inputs produce identical state

r[cairn.tui_explorer.view_model.determinism]
- GIVEN a fixed lifecycle snapshot and a fixed sequence of key events
- WHEN the explorer state is projected and reduced twice
- THEN both runs MUST produce identical state and identical `input_hash` values.

#### Scenario: Events on empty state are total

r[cairn.tui_explorer.view_model.empty]
- GIVEN an empty lifecycle snapshot
- WHEN any supported key event is reduced
- THEN the reducer MUST return a valid state without panicking, hanging, or fabricating nodes.

#### Scenario: Display state is excluded from hash material

r[cairn.tui_explorer.view_model.hash_boundary]
- GIVEN two sessions over one snapshot that differ only in selection, focus, scroll, or viewport
- WHEN Cairn computes explorer hash material
- THEN both sessions MUST produce the same hashes.

### Requirement: Explorer navigates the lifecycle hierarchy

r[cairn.tui_explorer.navigation] The explorer MUST present ordered navigation across repository, active changes, archived changes, specs, requirements, scenarios, and tasks, with an incremental text filter and deterministic selection movement.

#### Scenario: Navigation order is stable

r[cairn.tui_explorer.navigation.order]
- GIVEN a snapshot with multiple changes, specs, and requirements
- WHEN the explorer builds the navigation tree
- THEN sibling nodes MUST appear in deterministic sorted order in every session.

#### Scenario: Filter narrows without mutation

r[cairn.tui_explorer.navigation.filter]
- GIVEN a populated navigation tree and an incremental filter string
- WHEN the operator types filter characters
- THEN the visible node set MUST narrow deterministically and the underlying snapshot MUST remain unchanged.

#### Scenario: Out-of-range movement clamps

r[cairn.tui_explorer.navigation.clamp]
- GIVEN a selection at the first or last node
- WHEN the operator moves before the first or past the last node
- THEN the selection MUST remain at the boundary node.

### Requirement: Explorer renders lifecycle content read-only

r[cairn.tui_explorer.content_rendering] The explorer MUST render proposal, design, spec, and task Markdown through the pinned Subwayrat Markdown component, project requirements and tasks into tables, and render in-memory sync/archive dry-run plans as unified diffs.

#### Scenario: Rendered content matches source bytes

r[cairn.tui_explorer.content_rendering.fidelity]
- GIVEN a selected artifact with valid Markdown content
- WHEN the explorer renders the artifact
- THEN the rendered view MUST derive only from the exact source bytes and MUST NOT alter them.

#### Scenario: Dry-run plan preview executes nothing

r[cairn.tui_explorer.content_rendering.plan_preview]
- GIVEN an active change with a valid sync or archive plan
- WHEN the operator requests the plan preview
- THEN the explorer MUST compute the plan in memory, render it as a unified diff, and MUST NOT write lifecycle files.

#### Scenario: Oversized content is bounded

r[cairn.tui_explorer.content_rendering.bounds]
- GIVEN an artifact that exceeds the configured display bound
- WHEN the explorer loads the artifact
- THEN it MUST truncate at a declared boundary and mark the truncation visibly.

### Requirement: Explorer never mutates lifecycle state

r[cairn.tui_explorer.read_only] The explorer MUST NOT create, modify, or delete lifecycle files, MUST ignore widget edit actions, and MUST NOT expose any sync or archive execution path.

#### Scenario: Session leaves files byte-identical

r[cairn.tui_explorer.read_only.invariant]
- GIVEN a scripted explorer session over a fixture repository
- WHEN BLAKE3 hashes of the lifecycle tree are compared before and after the session
- THEN every hash MUST be identical.

#### Scenario: Edit actions are inert

r[cairn.tui_explorer.read_only.edit_actions]
- GIVEN a focused widget that emits an edit, move, create, or delete action
- WHEN the shell receives the action
- THEN the shell MUST discard the action and MUST NOT change explorer state beyond display-only fields.

#### Scenario: Execution keys do not exist

r[cairn.tui_explorer.read_only.no_execute]
- GIVEN any explorer key sequence
- WHEN the operator attempts to reach sync or archive execution
- THEN no binding or code path MUST reach an `--execute` mutation.

### Requirement: Subwayrat components are pinned and publicly buildable

r[cairn.tui_explorer.dependencies] Cairn MUST consume Subwayrat components from one exact public-HTTPS revision recorded in the workspace manifest, `Cargo.lock`, and Nix vendor hashes, and MUST NOT require sibling checkouts, SSH credentials, or network access at build time.

#### Scenario: Selected components build without SSH

r[cairn.tui_explorer.dependencies.https]
- GIVEN a fresh checkout and no SSH credentials
- WHEN Cairn is built through the supported Nix path
- THEN every explorer component MUST resolve from the recorded public-HTTPS pin and vendor hashes.

#### Scenario: SSH-transitive components are excluded

r[cairn.tui_explorer.dependencies.exclusion]
- GIVEN a Subwayrat component whose transitive dependencies resolve only through SSH URLs
- WHEN the component is proposed for the explorer
- THEN it MUST be rejected until upstream provides a public-HTTPS pin.

#### Scenario: Vendor hash drift fails closed

r[cairn.tui_explorer.dependencies.drift]
- GIVEN a component revision that no longer matches the recorded vendor hash
- WHEN the Nix build evaluates the explorer dependencies
- THEN the build MUST fail instead of fetching a substitute silently.

### Requirement: Explorer verification covers positive and negative paths

r[cairn.tui_explorer.verification] Cairn MUST verify the explorer with positive and negative tests at the pure core boundary and the shell admission boundary.

#### Scenario: Core reducer tests cover both paths

r[cairn.tui_explorer.verification.core]
- GIVEN the explorer test suite
- WHEN core tests run
- THEN they MUST include positive navigation, filter, and rendering projections AND negative empty-state, clamp, oversized-content, and malformed-snapshot cases.

#### Scenario: Shell tests cover admission failures

r[cairn.tui_explorer.verification.shell]
- GIVEN the explorer test suite
- WHEN shell tests run
- THEN they MUST include non-TTY rejection, unsupported-terminal rejection, terminal restoration, and the filesystem read-only invariant.

#### Scenario: Docs state bounded claims

r[cairn.tui_explorer.verification.non_claims]
- GIVEN explorer documentation and receipts
- WHEN they describe verification
- THEN they MUST state that an explorer session is not gate, readiness, release, or requirement-satisfaction evidence.
