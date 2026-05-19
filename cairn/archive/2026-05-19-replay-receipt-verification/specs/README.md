# replay-receipt-verification spec deltas

This change modifies deterministic replay requirements to add a first-class read-only verifier for saved replay receipts. The verifier checks receipt and attestation hashes, current policy compatibility, selected cases/groups, and emits deterministic verification receipts for agent/CI wrappers.
