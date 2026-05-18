## Context

The sync planner is pure and the CLI performs filesystem effects. The current shell execution path copies each delta `spec.md` to the accepted specs destination. That is safe for creating a new spec, but wrong for updating an existing accepted spec because delta specs are partial documents.

## Goals / Non-Goals

**Goals:**
- Keep merge rules deterministic and pure in `cairn-core`.
- Keep filesystem reads/writes in `cairn-cli`.
- Preserve accepted spec wrappers and unrelated accepted requirements.
- Support new accepted specs when no destination exists.

**Non-Goals:**
- Full semantic OpenSpec conflict resolution.
- NLP-based requirement matching.
- Automatic deletion of removed requirements beyond preserving/removing sections by exact heading.

## Decisions

### 1. Pure markdown merge helper

**Choice:** Add a pure helper that accepts optional accepted markdown plus delta markdown and returns merged markdown.
**Rationale:** The CLI can read files and pass strings into core, preserving FCIS boundaries.
**Alternative:** Keep copy semantics and manually repair specs after sync; rejected because this is the recurring bug.

### 2. Section-based delta handling

**Choice:** Treat `## ADDED Requirements`, `## MODIFIED Requirements`, and `## REMOVED Requirements` as delta section markers. Added/modified requirement blocks are appended to the accepted `## Requirements` body after removing exact-heading duplicates.
**Rationale:** This prevents wrapper contraction and gives deterministic behavior without a complex parser.

### 3. New spec fallback

**Choice:** If no accepted spec exists, create a minimal accepted spec wrapper and insert delta requirement blocks.
**Rationale:** Sync should still create new specs from deltas without copying the delta marker wrapper as the accepted spec wrapper.

## Risks / Trade-offs

- Exact-heading replacement is intentionally conservative. Future work can add stronger requirement identity matching if needed.
- Removed requirements can be represented by exact headings only; ambiguous removals should remain explicit review work.
