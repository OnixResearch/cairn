## Phase 1: Contract

- [x] [serial] Add the standalone `no_std + alloc` request, result, identity, and admission core. r[cairn.gate_interchange.contract] r[cairn.gate_interchange.request] r[cairn.gate_interchange.result]
- [x] [serial] Add coherent pass and block handling with fail-closed mismatch errors. r[cairn.gate_interchange.result] r[cairn.gate_interchange.reconciliation]
- [x] [serial] Add positive and negative unit tests for bounds, ordering, identity drift, applicability drift, cohort drift, and verdict contradictions. r[cairn.gate_interchange.conformance]

## Phase 2: Producer artifacts

- [x] [serial] Add the typed Nickel contract and positive and negative fixtures. r[cairn.gate_interchange.contract] r[cairn.gate_interchange.conformance]
- [x] [serial] Add deterministic passing and blocked producer fixtures with freshness tests. r[cairn.gate_interchange.conformance]
- [x] [serial] Document shell ownership, applicability, compatibility, authority, reconciliation, and non-claims. r[cairn.gate_interchange.boundary]

## Phase 3: Verification and lifecycle

- [x] [serial] Add focused Nix checks for contract tests, host-independent compilation, Nickel fixtures, and fixture freshness. r[cairn.gate_interchange.conformance]
- [x] [serial] Run formatting, workspace tests, strict Clippy, Slotcar, policy, Cairn, traceability, and full Nix checks. r[cairn.gate_interchange.conformance]
- [x] [serial] Record verification evidence, sync accepted specifications, archive the change, and rerun validation. r[cairn.gate_interchange.conformance] r[cairn.gate_interchange.boundary]
