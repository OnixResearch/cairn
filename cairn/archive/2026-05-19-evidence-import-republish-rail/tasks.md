## Phase 1: Spec Package

- [x] [serial] Define evidence republish command requirements. r[cairn.supersede.evidence_import_republish_rail.command]
- [x] [serial] Define import receipt and declared-artifact verification requirements. r[cairn.supersede.evidence_import_republish_rail.artifacts]
- [x] [serial] Define deterministic failure and receipt requirements. r[cairn.supersede.evidence_import_republish_rail.failures]

## Phase 2: Implementation

- [x] [serial] Implement `cairn evidence republish --root . --imported <dir> --out <dir>`. r[cairn.supersede.evidence_import_republish_rail.command]
- [x] [serial] Add policy schema, docs, tests, fixture/golden updates. r[cairn.supersede.evidence_import_republish_rail.artifacts]

## Phase 3: Verification and Archive

- [x] [serial] Run final gates including export, verify, import, and republish smoke. r[cairn.supersede.evidence_import_republish_rail.gates]
- [x] [serial] Sync, archive, commit, and push the completed change. r[cairn.supersede.evidence_import_republish_rail.archive]
