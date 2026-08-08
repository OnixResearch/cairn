## Phase 1: Deterministic core

- [x] [serial] Define the lifecycle-view input, node, edge, issue, board, and receipt models with stable sorting and BLAKE3 hash material. Covers `r[cairn.lifecycle_views.semantic_model]` and `r[cairn.lifecycle_views.determinism]`.
- [x] [serial] Parse Markdown requirement/scenario hierarchy and itemized task state/coverage into the pure model with explicit compatibility identity boundaries. Covers `r[cairn.lifecycle_views.semantic_model.markdown]`.
- [x] [serial] Build kanban classification, mindmap containment, and typed knowledge-graph projections with dangling-link diagnostics. Covers `r[cairn.lifecycle_views.kanban]`, `r[cairn.lifecycle_views.mindmap]`, and `r[cairn.lifecycle_views.knowledge_graph]`.

## Phase 2: CLI and terminal shell

- [x] [serial] Add deterministic JSON and human-readable render formats under `cairn view kanban|mindmap|graph`. Covers `r[cairn.lifecycle_views.cli]`.
- [x] [serial] Add the opt-in read-only terminal viewer using pinned Subwayrat graph components where appropriate, excluding display state from receipts. Covers `r[cairn.lifecycle_views.terminal]` and `r[cairn.lifecycle_views.terminal.read_only]`.
- [x] [parallel] Document view commands, controls, semantic boundaries, and the pinned Subwayrat reference. Covers `r[cairn.lifecycle_views.docs]`.

## Phase 3: Verification and lifecycle completion

- [x] [serial] Add positive tests for stable board classification, hierarchy, typed links, render output, terminal model adaptation, and repeated receipt equality. Covers `r[cairn.lifecycle_views.verification.positive]`.
- [x] [serial] Add negative tests for malformed Markdown, dangling requirement links, unsupported formats, conflicting interactive options, non-TTY invocation, and non-mutation. Covers `r[cairn.lifecycle_views.verification.negative]`.
- [x] [serial] Run focused core/CLI checks, policy freshness, formatting, Clippy, Cairn validation/gates, traceability coverage, and the broad repository rail or record exact environmental blockers. Covers `r[cairn.lifecycle_views.verification]`.
- [x] [serial] Sync the accepted lifecycle-view specification and archive this change only after implementation and verification evidence are complete. Covers `r[cairn.lifecycle_views.verification.archive]`.

## Verification evidence

- `276` `cairn-core` tests passed, including positive and negative lifecycle-view tests.
- Focused lifecycle-view CLI and terminal-adapter tests passed.
- `cargo fmt --all -- --check`, `cargo check -p cairn-cli`, scoped Clippy, and `nix build path:$PWD#cairn --no-link` passed.
- `cairn policy export --check`, `cairn validate`, proposal/design/tasks gates, lifecycle-view replay cases, and determinism coverage audit passed.
- The broad determinism test binary remains bounded by the pre-existing disk-quota failure caused when replay copies the repository-local `target/` tree; focused lifecycle-view replay cases passed repeated and cross-root comparison without mutation.
- Pre-archive traceability reports the active change's requirement markers as dangling because accepted specs are the default requirement source; archive synchronization resolves that expected lifecycle boundary.
- An independent detached-worktree replay with cap-std changes excluded passed formatting, lifecycle-view core and CLI tests, core Clippy, policy freshness, repository validation, and `1243/1243` traceability coverage.
- Workspace-wide CLI Clippy remains blocked by three pre-existing `native/authoring.rs` findings outside this change: two `cloned_ref_to_slice_refs` findings and one `too_many_arguments` finding.
