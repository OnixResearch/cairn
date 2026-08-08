## Why

Cairn can now publish a deterministic index for one evidence bundle. Release channels, CI mirrors, and downstream agents still need one stable registry that aggregates several bundle indexes before deciding what to fetch or cache.

## What Changes

- Add `cairn evidence registry build --root PATH --index FILE [--index FILE ...] --out FILE`.
- Add `cairn evidence registry verify --root PATH --registry FILE`.
- Registry receipts are deterministic, path-normalized, and bind each indexed bundle by index hash, bundle kind, bundle receipt hash, receipt chain, policy hash, and artifact hashes.
- Verification recomputes every referenced index and rejects missing, tampered, duplicate, unsafe, or stale registry entries with deterministic issue codes.

## Verification

Run proposal/design/tasks gates, policy export, Rust tests/clippy/fmt, Cairn validate, Tracey, coverage audit, release-readiness, evidence export/verify/import/republish/index/registry smoke, `git diff --check`, and `nix flake check`.
