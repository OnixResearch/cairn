## Phase 1: Policy and schema

- [x] [serial] Add policy/replay evidence schema support for agent group coverage. r[cairn.supersede.agent_receipt_replay_group_coverage.schema]
- [x] [serial] Implement `agent-receipt validate` policy-derived `agent` group requirements and deterministic group/policy failure issues. r[cairn.supersede.agent_receipt_replay_group_coverage.validation]

## Phase 2: Evidence and docs

- [x] [serial] Update agent receipt fixtures, public schema docs, and agent-run receipt docs for `replay_selected_groups`, `required_replay_group`, and policy-derived required cases. r[cairn.supersede.agent_receipt_replay_group_coverage.docs]
- [x] [serial] Add CLI/core tests for group-backed coverage, stale/missing group evidence, and deterministic failures. r[cairn.supersede.agent_receipt_replay_group_coverage.failures]

## Phase 3: Verification and archive

- [x] [serial] Run policy export, format, focused/full tests, Tracey, release-readiness, and replay group checks. r[cairn.supersede.agent_receipt_replay_group_coverage.replay]
- [x] [serial] Sync specs, archive the change, commit, and push. r[cairn.supersede.agent_receipt_replay_group_coverage.replay]
