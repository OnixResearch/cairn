# Design: Native Lifecycle with Legacy OpenSpec Migration

## Context

Current commands hard-code `openspec/specs`, `openspec/changes`, and `openspec/changes/archive`. That was useful for parity, but continued interop chasing keeps upstream OpenSpec as the implicit authority. The next product boundary is native Cairn layout plus a migration bridge.

## Goals / Non-Goals

**Goals:**
- Introduce native `cairn/specs`, `cairn/changes`, and `cairn/archive` layout support.
- Prefer native layout when present.
- Provide deterministic dry-run and execute migration from `openspec/` to `cairn/`.
- Keep filesystem effects and JSON in CLI shell.
- Keep path/layout decisions that can be pure in `cairn-core`.

**Non-Goals:**
- Removing legacy layout support.
- Mutating/deleting legacy OpenSpec content.
- Supporting arbitrary custom upstream OpenSpec extensions.

## Decisions

### 1. Native layout wins when present

**Choice:** Repository layout detection selects `cairn/` when it exists; otherwise commands fall back to `openspec/`.

**Rationale:** This makes migration opt-in and non-breaking while making the native layout the preferred runtime path.

### 2. Archive directory moves to `cairn/archive`

**Choice:** Native active changes live under `cairn/changes/<name>` and completed changes move to `cairn/archive/<date>-<name>`.

**Rationale:** A top-level archive directory is simpler and avoids preserving upstream's nested `changes/archive` convention.

### 3. Migration copies, never deletes

**Choice:** `migrate openspec` plans or copies from legacy `openspec/` to native `cairn/`; it does not remove legacy content.

**Rationale:** Migration should be reversible and safe to review. Deletion can be a later explicit cleanup feature.

### 4. Pure plan, imperative shell

**Choice:** Core defines layout constants and migration plan/action construction over caller-supplied path snapshots. CLI discovers filesystem state, copies files, and renders JSON.

**Rationale:** Preserves Slotcar FCIS boundary while keeping migration receipts deterministic.

## Risks / Trade-offs

- Dual-layout ambiguity: mitigated by explicit `layout` in receipts and native-first selection.
- Existing tests expect `openspec/`: keep legacy fallback and update/add tests for native behavior.
- Migration conflicts: report existing native destinations as blockers and do not overwrite.
