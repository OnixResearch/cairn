## Why

The stack defaults to BLAKE3 for owned content identity, but some surfaces still use other hashes for protocol or legacy reasons, such as ChaosControl SHA256 snapshot refs. Those exceptions should be declared rather than rediscovered during review.

## What Changes

- Add a hash algorithm policy contract for stack-owned evidence.
- Record default BLAKE3 requirements and explicit exceptions with owner, reason, scope, and migration status.
- Add diagnostics for undeclared SHA/SHA256 or mixed-hash fields in new evidence schemas.

## Impact

- Reviewers can distinguish required interop hashes from accidental drift.
- Evidence consumers can normalize identity fields safely.
- Cairn release readiness can reject undeclared non-BLAKE3 identity surfaces.
