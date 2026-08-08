## Context

The evidence pipeline now supports producer export, consumer verification, and local import. The remaining handoff gap is re-publication: CI jobs often need to mirror a verified/imported bundle into another cache or artifact directory without rerunning upstream replay and release-readiness checks.

## Goals

- Reuse the verified import receipt as the trust boundary.
- Copy only declared imported artifacts into a fresh output bundle.
- Preserve original artifact roles, relative paths, and hashes.
- Emit one deterministic republish receipt with portable path placeholders.
- Fail closed on tampering, missing files, unsafe paths, extra files, or destination collisions.

## Non-Goals

- No network upload, signing, or remote cache protocol.
- No regeneration of replay/release evidence from the local checkout.
- No acceptance of undeclared local files.

## Decisions

### Verification-first import receipt validation

`cairn evidence republish` reads `<imported>/evidence-import-receipt.json`, verifies its hash, checks the receipt is valid, and confirms every declared artifact still exists with the expected hash before copying.

### Declared-artifact copy semantics

The command copies only relative artifact paths declared in the import receipt. It rejects unsafe paths, path traversal, symlinks, duplicate paths, undeclared extra files, and pre-existing destination collisions. The republished directory receives the declared artifacts plus `evidence-republish-receipt.json`.

### Stable receipt shape

The receipt uses `imported = "<imported>"` and `out = "<out>"` instead of raw host paths. It records `import_receipt_hash`, `artifacts`, `valid`, `verdict`, `issues`, and `receipt_hash`.

## Risks / Trade-offs

- Extra-file rejection is strict; this is intentional so CI cache contents remain reviewable.
- The republish command trusts the import receipt only after re-hashing declared local artifacts, not after merely parsing JSON.
- Republish does not prove the original source repo is still clean; that evidence is already captured by the exported/imported chain.
