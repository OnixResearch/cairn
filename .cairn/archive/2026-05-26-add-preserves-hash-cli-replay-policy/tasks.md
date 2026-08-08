# Tasks

## Implementation

- [x] [serial] I1 Add `agent_receipt_preserves_hash` to replay policy cases and the full replay group without changing required agent/release groups. r[cairn.determinism.preserves_hash_cli.replay_policy]
- [x] [serial] I2 Add the Preserves hash CLI to `determinism_surface_inventory`. r[cairn.determinism.preserves_hash_cli.surface_inventory]
- [x] [serial] I3 Add replay execution support for the Preserves hash case without shelling out. r[cairn.determinism.preserves_hash_cli.replay_execution]

## Validation

- [x] [serial] V1 Add focused replay and coverage-audit assertions for the new case/surface. r[cairn.determinism.preserves_hash_cli.replay_validation]
- [x] [serial] V2 Run policy export/check, focused determinism tests, Cairn validate/gates, Tracey, and workspace checks. r[cairn.determinism.preserves_hash_cli.replay_validation]
