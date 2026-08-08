## Why

Cairn already understands local change dependency graphs through `metadata.json` `depends_on`, graph validation, drain ordering, and mutation blockers. That capability is still too easy to miss and too easy to author incorrectly because operators must hand-edit JSON and remember which commands expose dependency readiness.

Cairn should make local dependency use obvious, safe, and deterministic: operators should be able to add, remove, list, render, and discover the next dependency-ready change without hand-editing metadata or reverse-engineering graph receipts.

## What Changes

- Add first-class local dependency authoring commands for `depends_on` metadata.
- Preserve existing graph semantics: active dependencies block, archived dependencies satisfy, malformed/self/unknown/cyclic edges fail closed.
- Add stable human-oriented graph render formats while keeping JSON as the machine-readable default.
- Add a small next-ready command that surfaces the first dependency-ready change and explains blockers for the rest.
- Document dependency authoring, graph rendering, and safe drain workflow in the README/tutorial docs.

## Impact

- **Files**: CLI change command handling, pure metadata-update core helpers, graph rendering helpers, tests, README/tutorial docs, and the accepted `change-dependency-graph` spec when synced.
- **Testing**: Positive and negative tests for add/remove/list, dry-run/execute behavior, graph render formats, next-ready output, invalid dependencies, self-dependencies, unknown dependencies, and cycle prevention.
