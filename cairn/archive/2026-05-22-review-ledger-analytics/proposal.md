## Why

Cairn can store deterministic receipts, but it lacks a trust-ledger summary over semantic review outcomes. Rothrock’s analysis becomes useful because gate outcomes can be aggregated by gate, decision, error type, and recovery. Cairn should produce local deterministic analytics over checked-in or supplied review receipts.

## What Changes

- Add a local ledger command that reads review receipts from configured paths.
- Aggregate approvals, rejections, taxonomy distribution, gate kind, and reviewer/source metadata.
- Emit deterministic analytics receipts for evidence bundles and trend review.

## Impact

- **Files**: Cairn policy, pure core receipt/profile validators, CLI validation/analytics commands, fixtures, tests, and docs as applicable.
- **Testing**: Positive/negative receipt fixtures, policy export checks, focused CLI tests, validation/gate receipts, and Tracey coverage refs.
