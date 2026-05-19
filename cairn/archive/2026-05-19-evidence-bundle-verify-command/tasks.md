## Phase 1: Spec Package

- [x] [serial] Define evidence bundle verification command requirements. r[cairn.supersede.evidence_bundle_verify_command.command]
- [x] [serial] Define artifact/hash verification requirements. r[cairn.supersede.evidence_bundle_verify_command.artifacts]
- [x] [serial] Define deterministic failure and docs requirements. r[cairn.supersede.evidence_bundle_verify_command.failures]

## Phase 2: Implementation

- [x] [serial] Implement `cairn evidence verify --root . --bundle <dir>`. r[cairn.supersede.evidence_bundle_verify_command.command]
- [x] [serial] Add policy receipt schema, tests, docs, and fixture/golden repairs. r[cairn.supersede.evidence_bundle_verify_command.artifacts]

## Phase 3: Verification and Archive

- [x] [serial] Run Cairn gates, tests, Tracey, clippy, export/verify smoke, and flake checks. r[cairn.supersede.evidence_bundle_verify_command.gates]
- [x] [serial] Sync, archive, commit, and push the completed change. r[cairn.supersede.evidence_bundle_verify_command.archive]
