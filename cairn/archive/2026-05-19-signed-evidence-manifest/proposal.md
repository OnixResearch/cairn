# Change: Signed Evidence Manifest

## Why

Cairn now validates agent, replay, replay-verification, bundle, and optional release-readiness artifacts, but agents and CI still need a stable artifact index to publish, archive, and audit the set as one unit. Filenames and wrapper convention are too easy to drift after generation.

## What Changes

- Add a first-class read-only command for verifying an evidence manifest, e.g. `cairn evidence-manifest verify <manifest.json> --root .`.
- Define a stable manifest JSON shape that lists evidence artifacts by role, hash, optional schema version, and optional secret-free signer/anchor metadata.
- Verify each referenced artifact through existing deterministic validators where applicable, including bundle verification, saved replay receipt verification, agent receipt validation, and optional release-readiness checks.
- Emit one deterministic manifest verification receipt with stable fields, issue codes, and portable hashes.

## Non-Goals

- No private key handling, cryptographic signing implementation, network anchoring, timestamp authority, or remote publication.
- No requirement that all consumers adopt a new bundle layout immediately.
- No raw absolute path, environment value, credential, or host-specific detail in stable receipts or issue text.

## Capabilities

### New Capabilities

- `cairn.supersede.signed_evidence_manifest`: CI/agents can verify a persisted evidence artifact index in one deterministic pass/fail receipt.

### Modified Capabilities

- Agent receipt and replay verification documentation describe the manifest as the outer publication/index layer over already-verified evidence bundles.

## Impact

- **Files**: `crates/cairn-cli/src/main.rs`, `crates/cairn-cli/tests/cli.rs`, `cairn-policy/default.ncl`, generated policy, README, agent receipt docs, lifecycle specs/archive.
- **APIs**: New CLI verification surface and stable public receipt schema.
- **Testing**: Add integration coverage for valid manifests, stale/tampered artifact hashes, invalid referenced evidence, optional release-readiness, signer/anchor metadata handling, and secret/path-safe failures.
