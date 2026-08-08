## Why

Cairn can export, verify, and import deterministic evidence bundles, but downstream CI mirrors and reviewers still need a deterministic way to re-publish imported evidence without regenerating it from the local repository. Without a first-class republish rail, hand-written copy scripts can lose artifact roles, skip import verification, include undeclared files, or leak local paths.

## What Changes

- Add `cairn evidence republish --root . --imported <dir> --out <dir>`.
- Verify the imported evidence directory before copying any artifacts.
- Re-publish only artifacts declared by `evidence-import-receipt.json`.
- Emit deterministic `evidence-republish-receipt.json` with stable placeholder paths.
- Reject missing, tampered, duplicate, extra, or unsafe imported artifacts with deterministic issue codes.

## Scope

In scope: deterministic local filesystem republish from a prior Cairn import, receipt schema, docs, tests, lifecycle archive, and final verification gates.

Out of scope: network publishing, signing, external storage APIs, mutable cache eviction, or trusting imported evidence without verification.

## Verification

Run policy export checks, Rust tests/clippy/fmt, Cairn validate, Tracey coverage, coverage audit, release-readiness, evidence export/verify/import/republish smoke, `git diff --check`, and `nix flake check path:$PWD -L`.
