# Design

## Context

The previous migration slice intentionally made native `cairn/` layout the creation target while keeping legacy `openspec/` repositories readable and migratable. The accepted supersession spec now contains both the newer native-layout requirement and older initialization/change-create requirements that still name legacy creation paths.

## Goals / Non-Goals

**Goals:**
- Make accepted requirements internally consistent with native layout creation.
- Avoid changing runtime code in a spec-alignment slice unless verification exposes a mismatch.
- Keep legacy support described as fallback/migration compatibility.

**Non-Goals:**
- Add gate commands.
- Add Tracey coverage.
- Remove legacy `openspec/` fallback.

## Decisions

### 1. Modify existing requirements instead of adding duplicates

**Choice:** Use a delta `MODIFIED Requirements` entry for initialization and change creation.
**Rationale:** The behavior already exists; the accepted spec wording is stale.
**Alternative:** Add another requirement describing native creation. Rejected because `r[cairn.supersede.native_layout]` already does that and duplicate requirements increase drift.

**Implementation:** Update the accepted spec prose/scenarios to say `cairn init` and `cairn change create` create native `cairn/` artifacts and identify the selected layout in JSON receipts.

## Risks / Trade-offs

**Sync implementation is append-oriented** → After sync, inspect the accepted spec and repair structure manually if needed before archiving, then validate with upstream and native validators.
