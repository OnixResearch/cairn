## Phase 1: Runtime evidence core

- [x] [serial] r[cairn.runtime_evidence.records] Add pure runtime action record DTOs and canonical BLAKE3 identity helpers in `cairn-core`, with positive tests for valid records and negative tests for malformed subject, action, identity, and integrity fields.
- [x] [serial] r[cairn.runtime_evidence.subject_scope] Implement subject-scoped chain validation that rejects mixed-subject chains unless policy explicitly declares a multi-subject aggregation profile.
- [x] [serial] r[cairn.runtime_evidence.capture_origin] Implement capture-origin classification without upgrading ingested, wrapper-asserted, or unknown records to boundary-captured evidence.
- [x] [serial] r[cairn.runtime_evidence.hash_only] Implement bounded content checks that accept hash-only input/outcome identities and reject raw secret-bearing summaries.

## Phase 2: Witness checkpoints

- [x] [serial] r[cairn.runtime_evidence.external_witness] Add witness checkpoint DTOs and pure checkpoint comparison over subject, chain root, record count, and head hash.
- [x] [serial] r[cairn.runtime_evidence.completeness_verdicts] Emit deterministic completeness verdict classes for no checkpoint, matching witnessed prefix, wrong subject, truncation, and alteration before a checkpoint.
- [x] [serial] r[cairn.runtime_evidence.external_witness] Add positive and negative fixtures for intact witnessed chains, tampered records, dropped records, stale checkpoints, wrong-subject checkpoints, malformed checkpoints, and local-only integrity chains.

## Phase 3: CLI and policy integration

- [x] [serial] r[cairn.runtime_evidence.cli] Add a `runtime-evidence verify` CLI shell that reads chain/checkpoint artifacts, delegates all decisions to pure core, and emits a path-safe deterministic receipt.
- [x] [serial] r[cairn.runtime_evidence.policy_schema] Add Nickel-owned public receipt schemas, allowed capture-origin values, hash algorithm declarations, checkpoint fields, and stable issue codes.
- [x] [serial] r[cairn.runtime_evidence.manifest_roles] Add optional runtime-chain and witness-checkpoint artifact roles to agent evidence bundles and evidence manifests without weakening existing validators.
- [x] [serial] r[cairn.runtime_evidence.docs] Document operator flow, privacy rules, Halo/AARM compatibility boundaries, and the integrity-versus-completeness non-claim in README and agent-run receipt docs.

## Phase 4: Verification and lifecycle

- [x] [serial] r[cairn.runtime_evidence.tests] Add CLI, core, bundle, manifest, and release-readiness tests covering valid and invalid runtime evidence flows.
- [x] [serial] r[cairn.runtime_evidence.gates] Run focused validation: policy export check, formatting, core/CLI tests, Cairn validation, runtime evidence fixtures, and release-readiness.
- [x] [serial] r[cairn.runtime_evidence.archive] Sync and archive the completed change only after validation evidence and Tracey coverage are clean.
