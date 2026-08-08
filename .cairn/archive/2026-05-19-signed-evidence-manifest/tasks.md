# Tasks: Signed Evidence Manifest

## Phase 1: Specification

- [x] [serial] Define manifest verification requirements and scenarios. r[cairn.supersede.signed_evidence_manifest.verify]
- [x] [serial] Gate proposal, design, and tasks for the active change. r[cairn.supersede.signed_evidence_manifest.gates]

## Phase 2: Implementation

- [x] [serial] Add the read-only `evidence-manifest verify` CLI command and deterministic receipt generation. r[cairn.supersede.signed_evidence_manifest.verify]
- [x] [serial] Add stable public receipt schema and generated policy updates. r[cairn.supersede.signed_evidence_manifest.schema]
- [x] [serial] Compose existing artifact validators for bundle, replay, replay-verification, agent receipt, and release-readiness roles. r[cairn.supersede.signed_evidence_manifest.chain]
- [x] [serial] Validate optional signer/anchor metadata as public deterministic metadata only. r[cairn.supersede.signed_evidence_manifest.metadata]

## Phase 3: Documentation and Verification

- [x] [serial] Document manifest format, CLI usage, stable fields, and non-goals. r[cairn.supersede.signed_evidence_manifest.docs]
- [x] [serial] Add positive and negative integration coverage, including secret/path-safe failure receipts. r[cairn.supersede.signed_evidence_manifest.tests]
- [x] [serial] Sync, archive, run final gates, commit, and push. r[cairn.supersede.signed_evidence_manifest.archive]
