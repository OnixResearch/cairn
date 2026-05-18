## Phase 1: Spec and model

- [x] [serial] r[cairn.supersede.gate_policy.references] Add accepted requirements for stricter native gate references and exemptions.

## Phase 2: Implementation

- [x] [serial] r[cairn.supersede.gate_policy.references] r[cairn.supersede.gate_policy.task_links] r[cairn.supersede.gate_policy.exemptions] Implement pure core gate artifact/reference/exemption checks and CLI artifact snapshots.
- [x] [parallel] r[cairn.supersede.gate_policy.references] r[cairn.supersede.gate_policy.task_links] r[cairn.supersede.gate_policy.exemptions] Add positive and negative tests for task references, unknown references, and bounded exemptions.
- [x] [parallel] r[cairn.supersede.gate_policy.references] Update README, workflow skill source, and Tracey markers for the stronger gate rail.

## Phase 3: Verification

- [x] [serial] [depends:phase-2] Run focused/full verification, sync, archive, and commit the clean worktree.
