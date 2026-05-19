# Tasks: Evidence Manifest Generation

## Phase 1: Specification

- [x] [serial] Define manifest generation requirements and scenarios. r[cairn.supersede.evidence_manifest_generate.cli]
- [x] [serial] Gate proposal, design, and tasks for the active change. r[cairn.supersede.evidence_manifest_generate.gates]

## Phase 2: Implementation

- [x] [serial] Add `evidence-manifest generate` argument parsing and deterministic manifest output. r[cairn.supersede.evidence_manifest_generate.cli]
- [x] [serial] Add canonical role ordering and artifact hashing. r[cairn.supersede.evidence_manifest_generate.canonical]
- [x] [serial] Add public metadata JSON support. r[cairn.supersede.evidence_manifest_generate.metadata]
- [x] [serial] Add deterministic failure receipts for missing/unreadable/invalid inputs. r[cairn.supersede.evidence_manifest_generate.failures]

## Phase 3: Documentation and Verification

- [x] [serial] Document generation flow and generate → verify usage. r[cairn.supersede.evidence_manifest_generate.docs]
- [x] [serial] Add positive round-trip and negative path/secret-safe tests. r[cairn.supersede.evidence_manifest_generate.tests]
- [x] [serial] Sync, archive, run final gates, commit, and push. r[cairn.supersede.evidence_manifest_generate.archive]
