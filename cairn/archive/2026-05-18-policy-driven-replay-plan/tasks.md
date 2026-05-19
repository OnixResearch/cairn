## Phase 1: Policy contract

- [x] [serial] Add policy schema/data for replay plan cases with case id, command kind, args, fixture root, expected mutability, hash fields, display-only paths, and normalization rules. Covers `r[cairn.supersede.determinism_replay.policy_plan]`.
- [x] [parallel] Regenerate policy JSON and document policy-driven replay fields. Covers `r[cairn.supersede.determinism_replay.policy_schema]`.

## Phase 2: Expanded executor

- [x] [serial] Load replay cases from generated policy instead of hard-coded Rust-only metadata. Covers `r[cairn.supersede.determinism_replay.policy_plan]`.
- [x] [serial] Expand default replay coverage to gates, lifecycle dry-runs, migration dry-run, agent-receipt validation, release-readiness, and MCP read-only calls. Covers `r[cairn.supersede.determinism_replay.expanded_surfaces]`.
- [x] [parallel] Preserve deterministic skipped-case and mutation-denied diagnostics for unsupported or mutating policy cases. Covers `r[cairn.supersede.determinism_replay.policy_diagnostics]`.

## Phase 3: Verification

- [x] [serial] Add tests for policy case coverage, replay receipt stability, and policy/CLI drift prevention. Covers `r[cairn.supersede.determinism_replay.policy_schema]` and `r[cairn.supersede.determinism_replay.expanded_surfaces]`.
- [x] [serial] Run final verification: policy export check, formatting, `cargo test -p cairn-cli`, Cairn validate/gates, Tracey coverage, and release-readiness. Covers `r[cairn.supersede.determinism_replay.policy_diagnostics]`.
