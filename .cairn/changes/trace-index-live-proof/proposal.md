# Change: Prove the trace index on Cairn itself

## Why

The content-bound trace-index trial passes isolated tests, but Cairn cannot export its own requirement registry. The registry parser mistakes bare marker examples in requirement prose for requirement identities. This blocks a live export, verification, and query receipt.

## What Changes

- Make requirement-row identity come from the leading declaration marker, not marker-like prose in the remaining requirement body.
- Preserve fail-closed handling for missing, malformed, and duplicated declaration markers.
- Add a repository-scale Nix check that exports the current requirement registry and trace index, verifies the index, and queries one accepted requirement.
- Keep the generated proof artifacts in the Nix store so operators can inspect the receipts.

## Impact

- Fixes content-bound requirement registry export for accepted specifications that discuss marker syntax.
- Adds durable end-to-end evidence for the trace-index command chain.
- Does not change the declared-linkage claim boundary or treat tests as correctness proof.
