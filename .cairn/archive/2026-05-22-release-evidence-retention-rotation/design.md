# Design: release evidence retention and rotation policy

## Context

`cairn release-evidence index` creates deterministic current-HEAD evidence, and `cairn release-evidence publish` materializes that evidence into a local producer-side tree with `index.md`, artifacts, `manifest.json`, and `receipt.json`. Operators still need a deterministic way to decide which local publication trees remain current, which are superseded, and which can be pruned without conflating local retention with remote trust or credential-backed deletion.

## Goals

- Provide a first-class local retention/rotation planning contract for materialized release evidence directories.
- Bind every retention decision to verified publication manifests/receipts and stable channel/release/retention labels.
- Emit deterministic plan and receipt artifacts with safe relative paths, BLAKE3 hashes, keep/prune/supersede actions, and a verdict.
- Default to read-only planning; require an explicit execute mode for any local deletion or mutation.
- Keep the rail local/offline and avoid remote mirror, credential, tag, signing, or public release-page claims.

## Proposed shape

Add a narrow producer command family, likely:

```text
cairn release-evidence retention-plan \
  --root PATH \
  --publish-root DIR \
  --out FILE \
  [--channel LABEL] \
  [--keep-latest N] \
  [--retain LABEL]

cairn release-evidence rotate \
  --root PATH \
  --plan FILE \
  [--execute]
```

The planning command should:

1. Inventory candidate local publication trees below `--publish-root` using only safe relative paths.
2. Reopen each candidate `manifest.json` and `receipt.json` and compose the existing release-evidence publication validation semantics rather than duplicating release validators.
3. Normalize and policy-validate retention labels such as channel, release, retention class, and superseded-by references.
4. Emit a deterministic retention plan listing each tree as `keep`, `supersede`, `prune_candidate`, or `invalid`, with reason codes and source manifest/receipt hashes.
5. Avoid wall-clock defaults; if dates or ordering are needed, derive them from explicit release labels or manifest metadata that was already reviewed.

The rotation command should:

1. Verify the saved plan hash and all referenced publication tree hashes before any mutation.
2. Default to dry-run/read-only mode.
3. Require `--execute` for local mutations and emit a deterministic mutation manifest with before/after hashes and touched relative paths.
4. Refuse unsafe paths, symlinks, missing files, changed hashes, ambiguous duplicate current releases, or malformed labels.

## Non-goals

- No remote upload or remote deletion.
- No credential discovery, secret handling, or signing-key access.
- No Git tag creation or movement.
- No public release-page mutation.
- No claim that pruning local evidence changes consumer trust state elsewhere.

## Risks and mitigations

- **Accidental data loss:** make planning read-only and require explicit `--execute`, plan hash verification, and mutation manifests before deleting local files.
- **Overclaiming retention:** receipt/docs must state the rail only governs local producer-side evidence directories.
- **Path traversal:** reject absolute paths, parent traversal, unsafe symlinks, and outputs outside declared roots.
- **Stale or tampered evidence:** verify source publication manifests/receipts before planning and again before executing rotation.
- **Nondeterministic ordering:** sort by policy-owned labels and stable relative paths, never filesystem walk order.
