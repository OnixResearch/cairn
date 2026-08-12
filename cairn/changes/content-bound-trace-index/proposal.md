## Why

Cairn reports requirement coverage, but operators must filter the lifecycle graph to find the source links for one requirement.
The current marker result stores paths and line numbers without a portable artifact that binds links to exact requirement and source content.
A content-bound trace index will make declared links deterministic, queryable, and stale when the observed content changes.

## What Changes

- Add a canonical trace-index model in `cairn-core` that binds each declared link to a content-bound requirement row.
- Bind each link to its evidence role, safe source path, exact file BLAKE3, marker bytes, and byte span.
- Add `traceability export`, `traceability show`, and `traceability verify` CLI commands.
- Add typed Nickel profile fields for the declared assurance level and exact-marker anchor kind.
- Preserve the boundary that a marker proves declared linkage only.

## Impact

- **Files**: Traceability core, policy, CLI, integration tests, lifecycle specification, and operator documentation.
- **Testing**: Run positive determinism and query tests. Run negative tests for stale sources, moved markers, copied markers, malformed indexes, unsafe paths, and policy drift.
