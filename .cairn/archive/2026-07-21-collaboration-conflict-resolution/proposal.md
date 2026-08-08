# Explicit Collaboration Conflict Resolution

## Why

Cairn preserves concurrent artifact and lifecycle heads and correctly blocks ordinary update and checkout, but operators cannot inspect each candidate through the CLI or publish an explicit history-preserving resolution. Reading provider storage directly is backend-specific, while treating local bytes as a winner would silently weaken conflict semantics.

## What Changes

- Add deterministic backend-neutral conflict inspection that lists every current artifact and lifecycle head and can disclose one explicitly requested artifact head without dumping all bounded content. r[cairn.collaboration_resolution.inspection]
- Add a pure resolution planner that requires one explicit current-head choice for every conflicted artifact, supersedes every current head, and conservatively resets conflicted lifecycle state to `open` only when explicitly requested. r[cairn.collaboration_resolution.plan]
- Add preview-first portable and signed Radicle execution with provider locks, current state/policy/authority rechecks, exact plan replay, and typed reload verification. r[cairn.collaboration_resolution.portable_execution] r[cairn.collaboration_resolution.radicle_execution]
- Add backend-labeled BLAKE3 receipts, CLI documentation, and explicit non-claims for local resolution versus merge correctness or peer convergence. r[cairn.collaboration_resolution.receipts] r[cairn.collaboration_resolution.non_claims]

## Impact

- **Files**: verified collaboration core, portable and Radicle provider shells, CLI dispatch, public docs, accepted collaboration specification, and positive/negative tests.
- **Compatibility**: existing state schemas and identities remain unchanged; resolution uses the existing typed `SetArtifact` and `SetLifecycle` operation forms.
- **Testing**: baseline/focused core, provider, and CLI tests plus workspace, lifecycle, traceability, purity, Clippy, and Nix rails.
