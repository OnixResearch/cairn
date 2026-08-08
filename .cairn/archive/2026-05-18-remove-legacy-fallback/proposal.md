## Why

Cairn now has native `cairn/` lifecycle support and an explicit `migrate openspec` bridge. Continuing to silently read legacy `openspec/` as a runtime fallback hides unmigrated repositories and keeps tests/specs tied to the old layout.

## What Changes

- Remove implicit legacy `openspec/` runtime fallback from normal read/status/doctor/validate/gate/sync/archive command selection.
- Keep `migrate openspec` as the explicit bridge for importing legacy content into native `cairn/`.
- Update fixtures, goldens, docs, and self-dogfood checks to use native lifecycle content.

## Impact

- **Files**: CLI layout detection, lifecycle docs/specs, fixture layout/goldens, tests, flake self-dogfood, Hermes skill source.
- **Behavior**: Repositories with only `openspec/` are no longer treated as valid Cairn repos by normal runtime commands; operators must initialize or migrate.
- **Testing**: Native fixture tests plus migration tests prove no fallback and explicit import behavior.