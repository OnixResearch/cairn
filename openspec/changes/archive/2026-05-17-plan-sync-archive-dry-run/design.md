## Context

The FCIS refactor established `cairn-core/src/verified/` as the pure boundary and `cairn-cli` as the shell/presentation layer. Sync/archive should follow that split: core computes a deterministic plan from a supplied change snapshot; CLI discovers filesystem state and renders JSON.

## Decisions

### 1. Plans are pure typed core outputs

**Choice:** Add `SyncPlan` and `ArchivePlan`-style core models under `verified/`.
**Rationale:** Planning is deterministic policy/status transformation and should be testable without filesystem side effects.

### 2. CLI remains dry-run only

**Choice:** Keep `mutated: false` and avoid file mutation in this change.
**Rationale:** This change creates acceptance evidence and stable contracts before implementing destructive behavior.

### 3. JSON shape is stable and explicit

**Choice:** Render plan fields such as `change`, `dry_run`, `mutated`, `actions`, `blocked`, and `reasons` from the CLI.
**Rationale:** Operators and future automation need machine-readable receipts rather than prose placeholders.
