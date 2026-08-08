## Why

Cairn can now produce, verify, import, and republish deterministic evidence bundles. Downstream agents and CI still need a small deterministic catalog they can fetch first to identify what bundle they are looking at and which receipt hashes/artifact hashes to expect before copying or downloading the full bundle.

## What Changes

- Add a first-class evidence index command for exported, imported, and republished evidence directories.
- Add an index verifier that rehashes the referenced bundle and index receipt without relying on host paths.
- Emit stable success and failure receipts with portable placeholders and deterministic issue codes.
- Document the index workflow and add tests/gates covering index generation and verification.

## Scope

In scope: local filesystem index generation/verification, one bundle per command invocation, stable receipt schemas, deterministic issue handling.

Out of scope: network publishing, signing keys, remote registries, multi-bundle aggregation, or cryptographic trust beyond existing BLAKE3 receipt/hash validation.

## Verification

Run policy export checks, Rust tests, clippy, Cairn validation, Tracey, coverage audit, release-readiness, evidence export/verify/import/republish/index/index-verify smoke, whitespace checks, and Nix flake checks.
