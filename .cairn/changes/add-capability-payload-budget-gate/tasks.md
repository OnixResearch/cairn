# Tasks

## Phase 1 — Policy Schema & Default-Off Budget Check
- [ ] Add optional, default-off `capability_payload_budget` configuration (`enabled`, `max_cross_repo_targets`, `enforcement`) in `crates/cairn-cli/src/policy.rs`.
- [ ] Implement deterministic cross-repo blast-radius and capability-payload check in `crates/cairn-cli/src/{cross_repo_dependencies.rs, readiness.rs}`, ensuring zero-overhead pass-through for single-repo changes.

## Phase 2 — CLI Integration & Regression Fixtures
- [ ] Surface structured `capability_payload_budget` diagnostics (with explicit non-claims) in `cairn validate` and `cairn gate proposal`.
- [ ] Add positive and negative fixture tests covering (a) default-off no-op, (b) single-repo zero-friction pass, (c) multi-repo plumbing change missing capability payload -> rejected/warned, (d) multi-repo change exceeding `max_cross_repo_targets` without scope justification -> rejected/warned, and (e) compliant multi-repo change with named payload -> pass.
