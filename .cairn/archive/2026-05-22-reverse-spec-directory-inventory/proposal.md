## Why

Gap 8 is reverse-spec/import richness. `discover-specs` can inspect explicit files, but operators still have to hand-enumerate code, docs, and tests before Cairn can draft candidate requirements.

## What Changes

- Allow explicit directory includes to expand into a deterministic bounded inventory of reviewable text files.
- Keep discovery read-only by default and preserve the existing optional `--out` draft behavior.
- Emit receipt evidence that distinguishes selected files from skipped directories/files.

## Impact

- **Files**: Reverse-spec CLI shell, deterministic discovery helper tests, README usage.
- **Testing**: Positive directory inventory test plus negative/skipped-path coverage.
