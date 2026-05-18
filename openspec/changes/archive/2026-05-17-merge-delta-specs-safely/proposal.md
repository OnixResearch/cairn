## Why

`cairn sync --execute` currently copies delta spec files over accepted spec files. That preserves a useful dry-run/action model, but it can collapse accepted spec wrappers and history when a delta contains only `## ADDED/MODIFIED/REMOVED Requirements` sections.

## What Changes

- Replace raw delta spec copying with deterministic merge behavior for accepted spec markdown.
- Preserve accepted spec title, purpose, and requirements wrapper when applying delta sections.
- Append added/modified requirement sections without rewriting unrelated accepted content.
- Add regression coverage for the accepted-spec contraction failure seen during recent lifecycle slices.

## Impact

- **Files:** sync execution path in CLI plus pure merge logic in core.
- **Specs:** `sync-archive` execution behavior is strengthened.
- **Testing:** CLI fixture tests and core merge tests verify wrapper preservation and new-spec creation.
