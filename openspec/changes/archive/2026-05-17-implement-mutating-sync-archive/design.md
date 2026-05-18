## Context

The current `sync` and `archive` commands are safe dry-run planners. The pure core returns deterministic typed plans; the CLI discovers filesystem state and renders JSON. Execution must preserve this FCIS split.

## Goals / Non-Goals

**Goals:**

- Add explicit opt-in mutation for sync/archive.
- Keep dry-run behavior as the default.
- Refuse mutation when the plan is blocked.
- Produce stable JSON receipts suitable for golden tests and dogfood checks.

**Non-Goals:**

- Git operations, commits, pushes, or remote publishing.
- Advanced merge conflict handling.
- Background execution or daemon mode.

## Decisions

### 1. Execution is an explicit CLI flag

**Choice:** Add a flag such as `--execute` or `--no-dry-run` rather than changing default command behavior.

**Rationale:** Current dogfood and safety guarantees rely on no mutation by default.

**Alternative:** Add separate `sync-execute`/`archive-execute` commands; rejected because it fragments the public surface.

### 2. Core owns plans and receipts, shell owns writes/moves

**Choice:** Core remains pure and may define typed receipt data; CLI performs file copy/merge/move effects and maps results into receipts.

**Rationale:** Preserves Slotcar-enforced FCIS and keeps mutation auditable.

### 3. Tests mutate temp copies of fixture repos

**Choice:** Integration tests copy fixtures to temporary directories before running execution mode.

**Rationale:** Tests prove real filesystem effects without mutating checked-in fixtures.

## Risks / Trade-offs

**Data loss from archive move** → require an unblocked plan and explicit execution flag; use deterministic archive destination and refuse overwrite unless specified by a future change.

**Spec merge ambiguity** → start with deterministic file-level copy/write behavior for delta specs that map cleanly to accepted spec paths; leave conflict-aware merge semantics for a future OpenSpec if needed.

**FCIS regression** → keep Slotcar and pure-core forbidden-effect searches in verification.
