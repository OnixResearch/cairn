# Release evidence publish materialization

## Why

`cairn release-evidence index` now gives operators a deterministic current-HEAD evidence page and local artifact set, but the producer side still stops at ad hoc `target/...` paths. A release operator needs a first-class, reviewed materialization rail that lays out generated release evidence as a durable local publication tree without smuggling in network, credential, tag, or remote-trust claims.

## What Changes

- Specify a producer-side `release-evidence publish` or equivalent materialization command that copies or emits the current-HEAD evidence index and required artifacts into a deterministic local publish directory.
- Define a stable publication manifest/receipt that binds payload commit, source index hash, artifact roles, relative output paths, BLAKE3 hashes, retention labels, and explicit non-claims.
- Require safe relative paths, deterministic ordering, fail-closed validation, and composition of existing release-evidence/evidence-index validators instead of duplicating receipt semantics.
- Document retention/rotation boundaries for local publication directories while keeping network upload, credentials, remote mirror trust, and tag movement out of scope.

## Scope

In scope: public CLI contract, local materialization layout, producer receipt schema, retention metadata, docs, tests, policy/Tracey updates, and deterministic failure behavior.

Out of scope: remote uploads, signing keys, credential validation, GitHub release creation, tag movement, CDN/mirror trust, or production unattended-release claims.

## Verification

Validate with native Cairn proposal/design/tasks gates now; implementation later should add focused positive/negative CLI tests, policy export checks, Tracey coverage, workspace tests, clippy, and a clean local smoke that materializes a publish tree from generated current-HEAD evidence.
