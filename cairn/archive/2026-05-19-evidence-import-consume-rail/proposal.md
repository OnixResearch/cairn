## Why

Cairn can export and verify deterministic evidence bundles, but CI and reviewers still need a safe way to consume a verified bundle as local evidence without hand-written copy scripts. Ad-hoc import steps can accidentally preserve extra files, unsafe paths, host-specific locations, or unverified artifacts.

## What Changes

- Add `cairn evidence import --root . --bundle <dir> --into <dir>` as a deterministic consumer rail.
- Verification runs before import, using the existing exported-bundle verifier.
- Only declared exported artifacts are copied into the import directory.
- Emit `evidence-import-receipt.json` with stable paths, artifact roles, hashes, verification hash, and deterministic issue codes.
- Reject unsafe output locations, unsafe bundle artifact paths, undeclared extra files, and failed bundle verification without leaking host-specific paths or secrets.

## Scope

In scope: local deterministic import/copy, receipt schema, docs, tests, policy/golden updates.

Out of scope: network fetch, signing, remote trust policy, package registry publishing, compression formats, and long-term cache eviction.

## Verification

- Proposal/design/tasks gates.
- Policy export freshness.
- CLI integration tests for happy path and deterministic failure modes.
- Workspace tests, clippy, validate, Tracey, coverage audit, release-readiness, export/import smoke, diff check, flake check.
