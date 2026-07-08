# Tasks: Repo-owned traceability rail

## Phase 1: Policy and core

- [ ] [parallel] Add Nickel-owned traceability profile schema, generated runtime policy fields, and default Cairn plus Trellis-style fixture profiles. r[cairn.supersede.traceability_ownership.configured_profiles]
- [ ] [parallel] Extract marker parsing and coverage classification into pure core functions over in-memory profiles and file snapshots. r[cairn.supersede.traceability_ownership.authoritative_checker]

## Phase 2: CLI, receipts, and compatibility

- [ ] [depends:1,2] Add `cairn traceability coverage --root <path>` with deterministic JSON/receipt fields and keep `cairn tracey coverage` as a compatibility alias. r[cairn.supersede.traceability_ownership.command]
- [ ] [depends:3] Update Nix checks, release-readiness, README/workflow docs, and operator guidance so maintained gates use the repo-owned traceability command and external Tracey is optional. r[cairn.supersede.traceability_ownership.optional_tracey]

## Phase 3: Validation and archive

- [ ] [depends:1,2,3] Add positive and negative fixtures for complete coverage, missing refs, dangling refs, malformed profiles, alias compatibility, and Trellis-style source roots. r[cairn.supersede.traceability_ownership.validation]
- [ ] [depends:4,5] Run policy export, formatting, workspace tests, Clippy, Cairn validation/gates, traceability coverage, Slotcar, release-readiness, and flake check before sync/archive. r[cairn.supersede.traceability_ownership.validation]
