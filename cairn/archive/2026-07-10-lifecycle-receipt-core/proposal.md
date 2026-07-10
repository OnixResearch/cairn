## Why

Cairn, Octet, Mantle, and Valence all emit receipt-shaped evidence with verdicts, diagnostics, evidence summaries, policy hashes, input hashes, receipt hashes, and explicit non-claims. Cairn owns lifecycle policy, but the reusable receipt envelope pieces are broader than Cairn and should not be repeatedly reimplemented by stack repos.

This change defines a Cairn-owned lifecycle receipt core that can be extracted as pure DTOs and validation predicates while keeping lifecycle decisions and mutation policy inside Cairn.

## What Changes

- Define reusable receipt DTOs for verdicts, diagnostic rows, evidence summaries, hash fields, command identity, and non-claim boundaries.
- Add compatibility adapters for Cairn gate/validate/archive/sync receipts and downstream Octet/Mantle/Valence receipt-shaped evidence.
- Add positive fixtures for valid pass/warn/fail-style receipts and bounded advisory receipts.
- Add negative fixtures for malformed verdicts, missing hashes, stale receipt hashes, missing diagnostics, missing non-claims, and release/lifecycle overclaims.
- Keep policy evaluation, gate decisions, sync/archive mutation, and release-readiness decisions Cairn-owned.

## Impact

- **Cairn** owns the reusable lifecycle receipt envelope and validation core.
- **Stack repos** can share receipt DTOs without delegating domain evidence or release decisions.
- **Validation** focuses on pure in-memory receipt structures; CLI shells continue to own filesystem and JSON output.
