# Change: Evidence Manifest Generation

## Why

Cairn can verify persisted evidence manifests, but agents and CI still need wrapper glue to assemble canonical manifest JSON and compute artifact hashes. That reintroduces nondeterministic JSON ordering, role spelling, stale hashes, and secret/path leakage risks at the packaging seam.

## What changes

Add a first-class read-only generation command:

```bash
cairn evidence-manifest generate \
  --bundle bundle.json \
  --agent-receipt agent.json \
  --replay replay.json \
  --replay-verification replay-verify.json \
  --release-readiness release.json \
  --out manifest.json \
  --root .
```

The command will produce canonical manifest JSON that `cairn evidence-manifest verify` accepts without wrapper-side hashing logic.

## Scope

- Generate deterministic manifest JSON from role-specific artifact paths.
- Compute stable artifact hashes using Cairn's existing canonical file hashing semantics.
- Sort artifacts by stable role order.
- Accept optional public signer/anchor metadata JSON files.
- Write to `--out <path>` or stdout with `--out -`.
- Emit deterministic failure receipts for missing inputs, unreadable artifacts, invalid metadata JSON, or unsupported argument combinations.
- Add tests proving generate → verify round trips and failures remain path/secret safe.

## Non-goals

- Cryptographic signing.
- Remote anchoring or transparency-log publication.
- Secret management.
- Replacing `evidence-manifest verify`; generation complements verification.
