## Context

Evidence indexes make single bundles discoverable. Operators need to publish a set of indexes as one deterministic release/channel catalog without downloading each artifact first.

## Goals

- Build a registry from explicit index receipt files.
- Verify a saved registry by recomputing each index through `evidence index verify` semantics.
- Preserve portable paths and stable hashes while avoiding host paths, env values, credentials, or secrets.

## Non-Goals

- Network fetch, signing, transparency logs, or remote anchoring.
- Regenerating evidence bundles or indexes.
- A mutable server-side package repository.

## Decisions

### Explicit index inputs

`registry build` accepts repeated `--index FILE` arguments. This keeps ordering deterministic after sorting entries by canonical hash and avoids directory traversal surprises.

### Portable index references

Registry entries store `index = "<index:N>"`, optional `index_relative` only when safe and sibling/descendant-local, and the existing index receipt hash. Raw absolute paths are not emitted.

### Composed verification

Registry verification rehashes the registry, recomputes each referenced index summary, compares indexed fields and hashes, and emits stable issue codes for missing indexes, malformed hashes, duplicate index hashes, stale entries, and unsafe relative references.

## Risks

- Indexes outside the registry directory may not be portable. The command records portable relative references only when safe; otherwise verification expects colocated registry/index layout.
- Policy/schema changes stale fixture hashes and lifecycle goldens; regenerate after intentional schema changes.
