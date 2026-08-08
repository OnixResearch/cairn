# Tasks: Repo-owned traceability rail

## Phase 1: Policy and core

- [x] [parallel] Add Nickel-owned traceability profile schema, generated runtime policy fields, and default Cairn plus Trellis-style fixture profiles. r[cairn.supersede.traceability_ownership.configured_profiles]
  - Evidence 2026-07-08: added `traceability_policy` contracts/default profiles in Nickel, exported generated runtime JSON, and added parser/invalid-profile tests covering empty IDs, unsupported verbs, unsafe paths, and unsupported source kinds.
- [x] [parallel] Extract marker parsing and coverage classification into pure core functions over in-memory profiles and file snapshots. r[cairn.supersede.traceability_ownership.authoritative_checker]
  - Evidence 2026-07-08: added pure `cairn-core::verified::traceability` classifier over `TraceabilityProfile` and in-memory `TraceabilityFile` snapshots with positive, missing-ref, and dangling-ref unit tests.

## Phase 2: CLI, receipts, and compatibility

- [x] [depends:1,2] Add `cairn traceability coverage --root <path>` with deterministic JSON/receipt fields and keep `cairn tracey coverage` as a compatibility alias. r[cairn.supersede.traceability_ownership.command]
  - Evidence 2026-07-08: added the maintained `traceability coverage` command with policy/profile/input/receipt hashes, preserved `tracey coverage` as an alias, and added CLI tests proving alias-equivalent verdicts.
- [x] [depends:3] Update Nix checks, release-readiness, README/workflow docs, and operator guidance so maintained gates use the repo-owned traceability command and external Tracey is optional. r[cairn.supersede.traceability_ownership.optional_tracey]
  - Evidence 2026-07-08: updated `flake.nix`, release-readiness, README, Hermes workflow docs, MCP docs, and runbook examples to use `cairn traceability coverage` while documenting external Tracey as optional diagnostics only.

## Phase 3: Validation and archive

- [x] [depends:1,2,3] Add positive and negative fixtures for complete coverage, missing refs, dangling refs, malformed profiles, alias compatibility, and Trellis-style source roots. r[cairn.supersede.traceability_ownership.validation]
  - Evidence 2026-07-08: added focused CLI tests for complete Cairn coverage, missing refs, dangling refs, malformed policy profiles, `tracey` alias compatibility, and `trellis-rust` docs/src/tests roots.
- [x] [depends:4,5] Run policy export, formatting, workspace tests, Clippy, Cairn validation/gates, traceability coverage, Slotcar, release-readiness, and flake check before sync/archive. r[cairn.supersede.traceability_ownership.validation]
  - Evidence 2026-07-08: passed policy export check, `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cairn validate`, proposal/design/tasks gates, `cairn traceability coverage --root . --json`, `cairn release-readiness`, Slotcar, and `nix flake check path:$PWD -L`.
