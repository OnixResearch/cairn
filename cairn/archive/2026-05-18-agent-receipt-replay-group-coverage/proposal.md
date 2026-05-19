## Why

Agent receipt validation now requires replay evidence, and replay groups are policy-owned. The remaining deterministic gap is that `agent-receipt validate` still encodes the required replay case set in Rust. That makes agent evidence requirements less reviewable than replay case/group policy and can drift when the policy-defined `agent` group changes.

## What Changes

- **Policy-owned agent coverage**: derive required agent receipt replay coverage from `determinism_replay_policy.groups["agent"]` instead of a Rust case list.
- **Stable receipt evidence**: expose the required replay group and policy-derived cases in the validation receipt so CI/agents can see what policy required.
- **Deterministic failures**: emit stable validation issues for missing policy group metadata, stale policy hash, missing group selection evidence, and insufficient expanded case coverage.
- **Documentation and tests**: update agent receipt docs, replay docs, policy schema/goldens/fixtures, and Tracey coverage.

## Capabilities

### Modified Capabilities
- `ironspec-supersession`: agent-run receipt validation uses reviewed policy replay group metadata for deterministic replay evidence requirements.

## Impact

- **Files**: `crates/cairn-cli/src/main.rs`, `crates/cairn-core/src/verified/agent_receipt.rs`, `crates/cairn-cli/tests/cli.rs`, `cairn-policy/default.ncl`, `cairn-policy/generated/cairn-policy.json`, agent receipt fixtures, docs, accepted specs after sync.
- **APIs**: `agent-receipt validate` receipt schema gains stable policy/group coverage fields; input `replay_evidence` may include `replay_selected_groups`.
- **Testing**: focused agent receipt tests, replay group tests, policy export check, Tracey, full CLI/core tests, release-readiness, and determinism replay.
