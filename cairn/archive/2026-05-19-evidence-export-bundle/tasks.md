# Tasks: Evidence Export Bundle

## Phase 1: Specification

- [x] [serial] Define evidence export bundle requirements. r[cairn.supersede.evidence_export_bundle.command]
- [x] [serial] Define artifact/hash manifest requirements. r[cairn.supersede.evidence_export_bundle.artifacts]
- [x] [serial] Define deterministic failure and documentation requirements. r[cairn.supersede.evidence_export_bundle.failures]

## Phase 2: Implementation

- [x] [serial] Implement `cairn evidence export --root . --out <dir>` orchestration. r[cairn.supersede.evidence_export_bundle.command]
- [x] [serial] Add stable export receipt schema and policy/generated policy updates. r[cairn.supersede.evidence_export_bundle.artifacts]
- [x] [serial] Add CLI tests for export generation, verification linkage, and deterministic reruns. r[cairn.supersede.evidence_export_bundle.command]
- [x] [serial] Update README and agent receipt docs with canonical export flow. r[cairn.supersede.evidence_export_bundle.docs]

## Phase 3: Verification and Archive

- [x] [serial] Run Cairn gates, tests, Tracey, clippy, and flake checks. r[cairn.supersede.evidence_export_bundle.gates]
- [x] [serial] Sync, archive, commit, and push the completed change. r[cairn.supersede.evidence_export_bundle.archive]
