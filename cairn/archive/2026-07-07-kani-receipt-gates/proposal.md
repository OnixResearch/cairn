# Change: Kani receipt gates

## Why

Cairn already treats stronger behavioral claims as requiring stronger rails. Kani receipts can satisfy selected behavioral requirements when they are scoped, current, and explicit about bounds, contracts, stubs, and non-claims. Cairn needs gate semantics for accepting those receipts without running Kani inside gates.

## What Changes

- Add a Kani receipt profile for saved probe/evidence receipts.
- Let gates accept Kani receipts for scoped behavioral requirements only when status, source hash, policy hash, tool identity, property scope, unwind/contract status, stub classification, and non-claims pass validation.
- Reject stale receipts, failed verdicts, missing unwind evidence for unbounded claims, unverified-stub promotion, and weakened non-claims.
- Keep gate execution read-only: probes run Kani separately and gates verify saved receipts.

## Impact

- **Files**: receipt validation core, policy vocabulary, gate checks, fixtures, docs, and tutorials.
- **Consumers**: repositories can use Kani receipts as first-class Cairn verification artifacts.
- **Non-claims**: Cairn validates receipt consistency and policy eligibility; it does not certify Kani or rerun the verifier during gates.
