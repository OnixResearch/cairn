## Context

Sync, archive, gate, and change-context commands are core Cairn lifecycle operations and still occupy the main CLI shell. Moving them into a lifecycle module will make mutation boundaries and dry-run contracts easier to audit.

## Goals / Non-Goals

**Goals:**
- Clarify lifecycle mutation boundaries.
- Reduce `main.rs` without broad helper churn.
- Verify both dry-run and execute fixture paths.

**Non-Goals:**
- Changing lifecycle semantics, archive layout, task-gate policy, or accepted-spec merge behavior.
- Archiving unrelated active changes.

## Decisions

### 1. Keep the slice narrow

**Choice:** Implement exactly the scoped command or test boundary described by this change.

**Rationale:** Cairn has been reducing large CLI hotspots through small verified slices. Keeping this package narrow preserves reviewability and makes focused verification meaningful.

**Alternative:** Combine multiple command families or helper rewrites in one change. Rejected because it increases behavior drift risk and makes receipt/test regressions harder to localize.

### 2. Preserve deterministic public contracts

**Choice:** Treat command names, argument semantics, deterministic JSON shapes, receipt hashes for unchanged inputs, and issue codes as compatibility contracts unless this change explicitly adds a new public receipt.

**Rationale:** Cairn evidence is consumed by fixtures, release-readiness checks, and downstream operators. Refactors should not silently change evidence contracts.

**Alternative:** Opportunistically clean output schemas while moving code. Rejected; schema changes require their own explicit evidence and fixture updates.

### 3. Verify with focused and full gates

**Choice:** Run focused tests for the affected family first, then full workspace and Cairn gates before commit.

**Rationale:** Focused tests catch movement mistakes quickly; full gates prove no lifecycle, policy, or Tracey regressions were introduced.

## Risks / Trade-offs

**Helper coupling** → Export only narrow `pub(crate)` bridge functions and defer broad utility-module extraction to a separate change.

**Fixture drift** → Regenerate fixtures only when an intentional contract change requires it, and explain the reason in task evidence.

**Mutation risk** → For lifecycle or materialization work, use temp fixtures and verify dry-run/default behavior before execute paths.
