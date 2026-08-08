## Tasks

- [x] [serial] Define the Preserves evidence envelope ingest manifest contract. r[cairn.preserves_evidence_ingest.contract]
- [x] [serial] Add positive fixtures for canonical opaque and adapter-backed Preserves evidence envelopes. r[cairn.preserves_evidence_ingest.fixtures.positive]
- [x] [serial] Add negative fixtures for non-canonical bytes, unknown schema, stale digest, unsupported role, and semantic overclaim. r[cairn.preserves_evidence_ingest.fixtures.negative]
- [x] [serial] Implement pure ingest validation and thin canonical-byte measuring shell. r[cairn.preserves_evidence_ingest.validation]
- [x] [serial] Document opacity modes and lifecycle non-claims. r[cairn.preserves_evidence_ingest.docs]
- [x] [serial] Run focused tests and Cairn validation/gates. r[cairn.preserves_evidence_ingest.final_validation]

## Verification evidence

- `cargo test -p cairn-core preserves_evidence_ingest` passed 7 focused positive and negative tests.
- `cargo fmt --all -- --check` passed after formatting the ingest hardening.
- `cairn validate --root .` and the proposal, design, and tasks gates passed.
- The focused purity scan found no filesystem, environment, process, clock, async, printing, or JSON-rendering effects in `preserves_evidence_ingest.rs`.
- Sync exposed the known full-spec delta shell behavior; the reviewed requirement text was merged into `cairn/specs/preserves-evidence-ingest/spec.md`, all expected IDs are present, repository validation passed, and traceability coverage reported `1243/1243` referenced.
