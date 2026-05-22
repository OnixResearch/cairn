# Design: release evidence publish materialization

## Context

Cairn already has a complete local evidence chain: release-readiness receipts, evidence bundles, bundle verification, import/republish, evidence indexes, registry/mirror/fetch rails, and the newer `release-evidence index` orchestration. The missing producer-side seam is a deterministic local publication tree that an operator can inspect, archive, or hand to an external transport without Cairn claiming that transport is trusted.

## Goals

- Materialize current-HEAD release evidence into a deterministic local directory layout.
- Preserve existing validator authority by consuming verified release-evidence/evidence-index outputs rather than reimplementing their checks.
- Emit a producer receipt/manifest with stable relative paths, artifact roles, BLAKE3 hashes, payload commit, source index hash, and retention labels.
- Fail closed on dirty or mismatched source evidence, unsafe output paths, hash drift, incomplete artifacts, or malformed retention metadata.
- Keep local publication distinct from network upload, signing, credential handling, tag movement, and public release claims.

## Proposed shape

Add a narrow operator command, likely:

```text
cairn release-evidence publish \
  --root PATH \
  --index FILE \
  --artifact-dir DIR \
  --publish-dir DIR \
  [--channel stable] \
  [--release YYYY-MM-DD] \
  [--retention LABEL]
```

The command should:

1. Reopen and verify the supplied current-HEAD release evidence index and its required artifacts using the existing release-evidence/evidence-index validators.
2. Require all copied material to stay under a safe local `--publish-dir` using normalized relative paths.
3. Materialize a deterministic directory layout such as:
   - `index.md`
   - `artifacts/<role-or-stable-name>`
   - `manifest.json`
   - `receipt.json`
4. Hash every materialized artifact after writing and compare it with the source evidence hash before emitting a passing receipt.
5. Sort artifact and receipt entries by stable role/path identifiers.
6. Record channel/release/retention labels only as policy-validated local metadata, not as a remote-publication or tag claim.

## Non-goals

- No network upload or remote mirror validation.
- No credential discovery, secret handling, or signing-key access.
- No tag creation or movement.
- No change to existing evidence bundle/index semantics beyond additive producer publication metadata.
- No public unattended production-readiness assertion.

## Risks and mitigations

- **Overclaiming publication:** receipt and docs must say the command only materializes a local producer tree.
- **Path traversal / unsafe copies:** reject absolute artifact refs, parent traversal, symlinks when ambiguous, and outputs outside `--publish-dir`.
- **Stale evidence:** verify source index/artifacts before copying and again after writing.
- **Receipt churn:** use stable ordering, portable relative paths, and BLAKE3 hashes; avoid timestamps unless policy owns them.
- **Validator duplication:** reuse existing release-evidence/evidence-index validation helpers through narrow bridge APIs.
