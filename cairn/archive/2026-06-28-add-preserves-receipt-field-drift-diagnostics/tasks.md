# Tasks

## 1. Pure core drift helper

- [x] I1 [serial] Add `receipt_field_drift(baseline, candidate, source_map)` returning sorted, stable `DriftEntry { field_path, class, expected, actual, source_id }` in `crates/cairn-core/src/verified/`. r[cairn.determinism.preserves_receipt_field_drift.command]
- [x] I2 [serial] Add a policy-owned path→source-id table in Nickel policy and export it into generated policy JSON; load it in the shell. r[cairn.determinism.preserves_receipt_field_drift.source_id_mapping]
- [x] I3 [serial] Prove ordering-insensitive object comparison and a `canonical_bytes_differ` boolean from the two Preserves hashes. r[cairn.determinism.preserves_receipt_field_drift.deterministic]

## 2. Read-only CLI shell

- [x] I4 [serial] Add `cairn receipt-preserves-diff --baseline <baseline.json> --candidate <candidate.json>` as a read-only deterministic command emitting both input hashes, both Preserves hashes, sorted drift with source IDs, policy/policy hashes, and `receipt_hash`. r[cairn.determinism.preserves_receipt_field_drift.read_only]

## 3. Verification

- [x] V1 [serial] Add positive (identical → empty drift), negative (field drift → correct paths/source ids), malformed (non-object/non-JSON → fail closed), and boundary (float change, key reordering, scalar↔object type change) tests. r[cairn.determinism.preserves_receipt_field_drift.tests]
- [x] V2 [serial] Add a determinism replay case and a policy-inventory entry following the ROI-03 precedent. r[cairn.determinism.preserves_receipt_field_drift.validation]
- [x] V3 [serial] Run focused gates: `validate`, `gate proposal|design|tasks`, `preserves-corpus-fresh`, Slotcar purity smoke, clippy, fmt, and `release-readiness`. r[cairn.determinism.preserves_receipt_field_drift.validation]
