## Context

`cairn` already validates OpenSpec-style repos, emits stable JSON receipts, and safely syncs/archives changes. It cannot yet create the repository or change skeletons that operators need to start using it instead of `openspec`.

## Goals / Non-Goals

**Goals:**
- Add compatible `openspec/` layout initialization.
- Add compatible active-change skeleton creation.
- Emit stable JSON receipts for both commands.
- Preserve the FCIS boundary by keeping templates/decisions pure and filesystem writes in the CLI.

**Non-Goals:**
- Implement every upstream OpenSpec command in this slice.
- Introduce a new non-compatible directory layout.
- Add interactive prompts.

## Decisions

### 1. Preserve the `openspec/` layout first

**Choice:** `cairn init` creates `openspec/specs`, `openspec/changes`, and `openspec/changes/archive`.

**Rationale:** Supersession starts with drop-in compatibility so existing repositories and habits continue to work.

### 2. Use explicit skeleton artifacts

**Choice:** `cairn change create <name>` creates `proposal.md`, `design.md`, `tasks.md`, and `specs/` for the named change.

**Rationale:** A complete skeleton makes status/validate receipts immediately meaningful and nudges operators into the full lifecycle.

### 3. Keep receipts stable

**Choice:** Init/create commands emit JSON with command, root/change, created/existing artifacts, and mutation status.

**Rationale:** Automation should be able to distinguish first creation from idempotent re-runs.

## Risks / Trade-offs

**Name validation too strict** → Use a conservative slug rule now and relax later only with tests.

**Template churn** → Keep starter templates minimal and stable.
