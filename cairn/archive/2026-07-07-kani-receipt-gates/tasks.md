## Phase 1: Receipt profile and eligibility

- [x] [serial] r[cairn.kani_receipt_gates.receipt_profile] Add a Kani receipt profile with required fields for requirement id, source scope, hashes, toolchain identity, status, contract/unwind state, stubs, and non-claims.
- [x] [serial] r[cairn.kani_receipt_gates.eligibility] Define pure eligibility checks for using Kani receipts as scoped behavioral verification artifacts.
- [x] [serial] r[cairn.kani_receipt_gates.default_no_execution] Preserve the default gate boundary where gates verify saved receipts and do not run Kani.

## Phase 2: Gate integration and fixtures

- [x] [parallel] r[cairn.kani_receipt_gates.positive_fixtures] Add positive fixtures for bounded and contract-backed Kani receipts.
- [x] [parallel] r[cairn.kani_receipt_gates.negative_fixtures] Add negative fixtures for stale content, failed verdict, missing identity, unsupported unbounded claim, unverified-stub promotion, and weakened non-claims.
- [x] [serial] r[cairn.kani_receipt_gates.gate_diagnostics] Emit deterministic diagnostics that name the rejected eligibility condition.

## Phase 3: Documentation and validation

- [x] [serial] r[cairn.kani_receipt_gates.operator_docs] Document how repositories produce Kani receipts before Cairn gates verify them.
- [x] [serial] r[cairn.kani_receipt_gates.eligibility] Run focused receipt tests, gate fixture tests, policy export check, and Cairn validation/gates before archive.

## Verification Coverage

| Scenario | Planned Evidence |
|---|---|
| Passing bounded Kani receipt is eligible for scoped claim | Positive fixture test |
| Passing contract-backed receipt is eligible for contract claim | Positive fixture test |
| Stale content hash fails | Negative fixture test |
| Failed verdict fails | Negative fixture test |
| Missing identity fails | Negative fixture test |
| Unsupported unbounded claim fails | Negative fixture test |
| Unverified-stub promotion fails | Negative fixture test |
| Gates remain no-execution by default | CLI/gate fixture test |
