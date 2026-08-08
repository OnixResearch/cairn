## Why

Sync, archive, gate, and change-context commands are core Cairn lifecycle operations and still occupy the main CLI shell. Moving them into a lifecycle module will make mutation boundaries and dry-run contracts easier to audit.

## What Changes

- Move sync/archive/gate/change-context lifecycle command handling into `crates/cairn-cli/src/lifecycle.rs` or another focused lifecycle module.
- Preserve dry-run defaults, explicit mutation flags, mutation manifests, archive-date validation, and gate receipt shapes.
- Keep execution tests focused because this command family mutates temp fixtures.

## Scope

In scope: Clarify lifecycle mutation boundaries., Reduce `main.rs` without broad helper churn., Verify both dry-run and execute fixture paths..

Out of scope: Changing lifecycle semantics, archive layout, task-gate policy, or accepted-spec merge behavior., Archiving unrelated active changes..

## Verification

Run Cairn validation and proposal/design/tasks gates for this change package. During implementation, run focused tests for the moved or added command family, then the maintained repository gates before commit.
