## Context

`cairn evidence export` materializes a stable local evidence directory and emits an export receipt. The receipt lists stable artifact roles, relative paths, hashes, manifest hash, manifest verification receipt hash, and a portable receipt hash. Existing lower-level verifiers can validate replay receipts, agent evidence bundles, and evidence manifests, but consumers should not have to reassemble that sequence by hand.

## Goals

- Provide a single read-only verifier for exported evidence directories.
- Keep verification deterministic and portable across checkout/output locations.
- Reuse existing verifier logic rather than duplicating bundle/manifest/replay semantics.
- Report every local consistency failure with stable machine-readable issue codes.

## Non-Goals

- Fetching bundles from remote storage.
- Validating cryptographic signatures or transparency logs beyond existing manifest metadata fields.
- Mutating or repairing evidence directories.

## Design

### Command

`cairn evidence verify --root . --bundle <dir>` reads `<dir>/evidence-export-receipt.json`, then verifies the exported artifact chain.

The command is read-only. It accepts `--root` and `--bundle` in either order consistent with other Cairn path parsers.

### Verification steps

1. Read and parse `evidence-export-receipt.json`.
2. Confirm the export receipt has the expected command and a valid self hash.
3. For each declared artifact, require a relative path, reject traversal/absolute paths, rehash file content, and compare to the declared hash.
4. Require canonical roles for the exported evidence chain: coverage audit, replay receipt, replay verification, agent receipt, release-readiness, agent evidence bundle, bundle verification, evidence manifest, and manifest verification.
5. Inspect persisted verification receipts and require `valid=true` for replay verification, bundle verification, manifest verification, coverage audit, and release-readiness.
6. Re-run existing validators where practical: manifest verification over `evidence-manifest.json` and bundle verification over `agent-evidence-bundle.json`.
7. Emit a deterministic verification receipt with stable issue objects and a portable `receipt_hash`.

### Receipt shape

The receipt includes `command`, `root`, `bundle`, `valid`, `verdict`, `issues`, `artifacts`, `export_receipt_hash`, `manifest_hash`, `manifest_verification_receipt_hash`, and `receipt_hash`. The `bundle` field is a stable placeholder (`<bundle>`) so temp/output directories do not affect receipt hashes or leak host paths.

### Failure handling

Failures are accumulated and sorted deterministically. Issue codes are stable, for example `evidence_verify.export_receipt_missing`, `evidence_verify.artifact_hash_mismatch`, `evidence_verify.required_role_missing`, `evidence_verify.bundle_verification_invalid`, and `evidence_verify.manifest_verification_mismatch`. Issue payloads must not include absolute bundle paths or secret values.