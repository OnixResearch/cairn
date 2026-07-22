## Phase 1: Fixture evidence

- [x] [serial] Add the standalone upstream-storage producer source, static Git bundle, typed Nickel provenance, deterministic JSON export, and BLAKE3 identity checks. r[cairn.cob_interop.fixture]
- [x] [serial] Add negative fixture checks for hash drift, unsigned history, incompatible manifest version, and missing declared refs. r[cairn.cob_interop.negative]

## Phase 2: Two-way interoperability

- [x] [serial] Prove Cairn loads the external fixture and appends an authorized signed update without losing the reference-produced root. r[cairn.cob_interop.inward]
- [x] [serial] Implement a test-only independent direct consumer and prove it evaluates Cairn-produced create/update history without Cairn storage or reducer code. r[cairn.cob_interop.outward]
- [x] [serial] Extend the dedicated Nix dependency/compatibility gate and operator documentation without adding runtime dependencies or claims. r[cairn.cob_interop.boundary]

## Phase 3: Verification

- [x] [serial] Run focused tests, fixture freshness, dependency audit, formatting, and Clippy before sync; then run the full workspace, lifecycle, traceability, evidence, release-readiness, self-dogfood, and flake rails before archive. r[cairn.cob_interop.validation]
