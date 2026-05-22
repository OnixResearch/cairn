# Release evidence retention and rotation policy

## Why

Cairn can now generate a current-HEAD release evidence index and materialize a deterministic local publication tree, but retention is still an operator convention. Without a first-class retention/rotation rail, producers can keep stale evidence indefinitely, prune required artifacts by hand, or publish successor trees without deterministic evidence that the older tree was superseded rather than silently lost.

## What Changes

- Specify a local `release-evidence retention-plan` / `release-evidence rotate` style rail for planning retention, supersession, and pruning decisions over materialized publication directories.
- Define deterministic retention metadata, rotation plans, and receipts that bind publication manifest hashes, channel/release/retention labels, selected keep/prune/supersede decisions, and non-claims.
- Require fail-closed behavior for unsafe paths, missing manifests, malformed labels, hash drift, ambiguous duplicate releases, and attempts to delete without an explicit reviewed plan.
- Keep remote deletion, credential handling, mirror trust, signing, Git tag movement, and public release-page updates out of scope.

## Scope

In scope: local/offline retention policy contract, deterministic plan and receipt schemas, safe relative publication-tree inventory, policy-owned label validation, docs, tests, Tracey refs, and explicit non-claims.

Out of scope: network transport, remote mirror cleanup, credential discovery, destructive deletion without an explicit execute flag, signing-key management, GitHub release mutation, tag movement, or public unattended-release claims.

## Verification

Validate this package with native Cairn proposal/design/tasks gates now. Implementation should add focused positive/negative tests, policy export checks, Tracey coverage, workspace tests, clippy, and a clean smoke that plans retention over two local `release-evidence publish` trees without mutating by default.
