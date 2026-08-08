# Design

`release-readiness` remains the operator-facing bundle. The CLI shell resolves a review-ledger path from `--review-ledger <path>` or a conventional repository path when present. It reads the ledger JSON and treats it as an immutable receipt artifact.

The check is deterministic:

- absent ledger: `passed=true`, `skipped=true`, with a clear reason;
- present ledger with `valid=true` and pass/APPROVED verdict: `passed=true`;
- present ledger with invalid/failing verdict: `passed=false`, with the receipt hash and issue summary.

The implementation stays in the CLI shell because it reads files. The semantic review core remains the receipt validator/analytics owner.
