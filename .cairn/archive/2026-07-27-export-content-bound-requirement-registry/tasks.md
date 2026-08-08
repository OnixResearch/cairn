# Tasks: Export a content-bound requirement registry

## Core contract

- [x] [serial] Define versioned registry, row, revision, issue, and receipt DTOs in the pure core r[cairn.requirement_registry.content_bound_export]
- [x] [serial] Implement deterministic row extraction, duplicate rejection, domain-separated BLAKE3 identities, and canonical registry evaluation r[cairn.requirement_registry.content_bound_export]
- [x] [parallel] Add pure positive and negative tests for sorting, duplicate IDs, malformed revisions, unsafe paths, and hash drift r[cairn.requirement_registry.content_bound_export]

## CLI and publication

- [x] [serial] Add capability-rooted `requirement-registry export` and `verify` shells with explicit repository and revision inputs r[cairn.requirement_registry.current_root_verification]
- [x] [parallel] Add stable safe diagnostics and prove failed export or verification leaves no successful output r[cairn.requirement_registry.current_root_verification]
- [x] [serial] Add the optional `requirement_registry` role to Cairn evidence manifests and nested verification r[cairn.requirement_registry.downstream_boundary]

## Fixtures and documentation

- [x] [parallel] Add positive and negative CLI fixtures for current, stale, duplicate, malformed, and cross-repository registries r[cairn.requirement_registry.current_root_verification]
- [x] [parallel] Document declared-revision limits, downstream use, compatibility, and non-claims r[cairn.requirement_registry.downstream_boundary]

## Validation

- [x] [serial] Run focused core and CLI tests, policy freshness, formatting, strict Clippy, Cairn validation, and traceability coverage r[cairn.requirement_registry.content_bound_export]
- [x] [serial] Run proposal, design, and tasks gates before sync and archive r[cairn.requirement_registry.downstream_boundary]
