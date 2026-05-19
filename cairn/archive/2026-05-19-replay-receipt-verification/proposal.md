## Why

Replay receipts now carry policy hashes, selected cases/groups, entropy inventory, and environment attestation, but saved receipts cannot yet be verified by a first-class Cairn command. Agents and CI need a stable way to validate persisted replay evidence before attaching it to agent receipts or release gates.

## What changes

Add deterministic replay receipt verification:

- Introduce `cairn determinism verify-receipt <receipt.json> --root <root>`.
- Validate replay receipt schema/version, `receipt_hash`, current replay policy hash, selected cases/groups, entropy policy hash, and environment attestation hash.
- Emit a stable pass/fail verification receipt with deterministic issue codes.
- Keep verification read-only and secret-safe: do not re-run replay, do not expose raw environment values, and avoid absolute path leakage in hash material.
- Document agent/CI wrapper flow: run replay, save receipt, verify saved receipt, then attach replay evidence.

## Impact

Operators gain a cheap, portable replay-evidence gate. Older replay receipts fail deterministically with actionable issue codes instead of being accepted implicitly or requiring ad hoc JSON inspection.