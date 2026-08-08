## Context

The evidence pipeline supports produce → verify → consume → re-publish. Consumers still lack a deterministic first file that summarizes bundle type, artifact roles, artifact hashes, and receipt-chain bindings.

## Goals

- `cairn evidence index --root . --bundle DIR --out FILE` writes a portable JSON index for one evidence bundle.
- `cairn evidence index verify --root . --index FILE` validates that index against the indexed bundle.
- Support exported, imported, and republished bundle directories by detecting the canonical receipt file present.
- Preserve artifact role/path/hash information from the source receipt.
- Bind index receipts to export/verify/import/republish receipt hashes when present.
- Avoid raw host paths, absolute roots, environment values, credentials, and secrets.

## Decisions

### Bundle-kind detection

The index command detects bundle kind by canonical receipt files in priority order: republished (`evidence-republish-receipt.json`), imported (`evidence-import-receipt.json`), exported (`evidence-export-receipt.json`). Ambiguous or missing receipt combinations produce deterministic failures.

### Index schema

The index receipt includes stable fields: `command`, `root`, `bundle`, `out`, `valid`, `verdict`, `issues`, `bundle_kind`, `artifacts`, `receipt_chain`, `bundle_receipt_hash`, and `receipt_hash`. The written index file is the receipt itself.

### Verification

Index verification reads the index, validates its self hash, resolves the stable indexed bundle path relative to the index file when possible, re-detects/re-hashes the bundle, compares kind, artifacts, and receipt chain, and emits a deterministic verify receipt.

## Risks

- Policy/schema changes will stale goldens and agent receipt fixtures.
- Tracey refs may need accepted/archive spec repair after sync.
- Index verification must not trust stale index content when the bundle has been tampered.
