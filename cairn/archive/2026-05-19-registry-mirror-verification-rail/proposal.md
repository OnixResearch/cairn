# registry-mirror-verification-rail

## Why

Cairn registries can be built, verified, selected from, planned against, and bound into evidence manifests. Agents and CI still need a deterministic offline proof that a mirrored directory contains the exact registry and declared indexes they intend to consume before any fetch execution or downstream selection.

## What Changes

- Add a read-only registry mirror verifier that checks a registry file against a local mirror directory.
- Verify the mirrored registry hash, every declared index reachable through safe `index_relative` paths, and the registry's existing index/bundle/hash/label invariants.
- Emit a portable, secret-safe receipt with stable issue codes and normalized mirror/root fields.
- Document the mirror verifier as an offline proof step before future fetch execution.

## Out of Scope

- Network fetching, URL trust, credentials, or remote transport semantics.
- Mutating or repairing mirrors.
- Real cryptographic signatures beyond existing hash/manifest binding semantics.

## Verification

- Targeted CLI tests cover valid mirrors, registry tampering, missing indexes, and index hash mismatches.
- Docs/schema tests include the new receipt fields.
- Full Rust/Nickel gates pass: fmt, tests, clippy, policy freshness, validate strict, Tracey coverage, flake check, and no checked-in Python/shell helpers.