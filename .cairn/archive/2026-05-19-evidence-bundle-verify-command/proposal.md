## Why

Cairn can now export a deterministic evidence directory, but reviewers and CI still need to know the internal artifact chain to verify a received bundle. That leaves a consumption gap: producers have a single `evidence export` command, while consumers must manually rehash and invoke lower-level verifiers.

## What Changes

- Add `cairn evidence verify --root . --bundle <dir>` as the first-class consumer command for exported evidence directories.
- Verify `evidence-export-receipt.json`, all declared artifact hashes, manifest verification, bundle verification, replay verification, release-readiness, and coverage audit linkage.
- Emit one deterministic, path-safe verification receipt with stable issue codes and a receipt hash.
- Document the publish/consume workflow: producer runs `evidence export`; reviewer/CI runs `evidence verify`.

## Scope

In scope: local directory verification, deterministic issue codes, policy receipt schema, docs, tests, and archived spec evidence.

Out of scope: network downloads, transparency-log lookup, cryptographic signature validation beyond existing metadata hashing, and remote artifact publication.

## Verification

Run Cairn gates, policy freshness, Rust tests/clippy, Tracey coverage, release-readiness, evidence export, evidence verify, diff checks, and flake checks.