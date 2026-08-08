# Change: Evidence Export Bundle

## Why

Cairn can now produce and verify coverage-audit, replay, replay-verification, release-readiness, agent bundle, and evidence manifest receipts. Agents and CI still have to assemble those artifacts with wrapper-specific paths and ordering, which leaves room for publication drift.

## What Changes

- Add a first-class `cairn evidence export --root . --out <dir>` command.
- Generate a deterministic evidence directory containing coverage audit, replay, replay verification, release-readiness, agent evidence bundle, bundle verification, evidence manifest, and manifest verification artifacts.
- Emit a deterministic export receipt listing stable artifact roles, relative paths, hashes, and the verified manifest hash.
- Fail with deterministic, path/secret-safe issues when required inputs are missing, artifact generation fails, or verification fails.

## Scope

In scope:

- CLI orchestration and deterministic artifact writing under a caller-provided output directory.
- Policy/public receipt schema updates and docs.
- Tests proving stable artifact roles, hash linkage, verification, and deterministic repeated exports.

Out of scope:

- Network upload, signing key management, transparency-log anchoring, or remote publishing.
- Replacing lower-level commands; export composes existing receipts.

## Verification

- Policy export check.
- Focused CLI tests for export bundle generation and deterministic failure behavior.
- Full Cairn validation, Tracey, workspace tests, clippy, and flake check.
