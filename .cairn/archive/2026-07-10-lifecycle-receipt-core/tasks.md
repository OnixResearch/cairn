## Tasks

- [x] [serial] Define shared lifecycle receipt envelope, verdict, diagnostic, evidence summary, hash, and non-claim contracts. r[cairn.release_evidence.lifecycle_receipt_core.contract]
- [x] [serial] Add positive fixtures for valid Cairn and downstream receipt-shaped evidence envelopes. r[cairn.release_evidence.lifecycle_receipt_core.fixtures.positive]
- [x] [serial] Add negative fixtures for malformed verdicts, missing hashes, stale receipt hashes, missing diagnostics, weakened non-claims, and overclaims. r[cairn.release_evidence.lifecycle_receipt_core.fixtures.negative]
- [x] [serial] Implement pure receipt envelope validation and hash profile checks in `cairn-core`. r[cairn.release_evidence.lifecycle_receipt_core.validation]
- [x] [serial] Add compatibility adapters for existing Cairn receipt families and downstream receipt-shaped evidence. r[cairn.release_evidence.lifecycle_receipt_core.adapters]
- [x] [serial] Document shared-envelope and Cairn-policy ownership boundaries. r[cairn.release_evidence.lifecycle_receipt_core.docs]
- [x] [serial] Run focused receipt fixture tests, Cairn validation/gates, and policy freshness checks. r[cairn.release_evidence.lifecycle_receipt_core.final_validation]
