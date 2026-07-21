# Tasks: Backend-Agnostic Change Collaboration

- [x] [serial] Generalize collaborative change schema, context, hashing, import planning, and checkout planning while preserving explicit Radicle v1 identities. r[cairn.collaboration_backend.semantic_core]
- [x] [serial] Add deterministic portable envelope construction and tamper validation with positive and negative core tests. r[cairn.collaboration_backend.portable_state]
- [x] [serial] Add capability-bounded preview-first portable import, read-only status, and preview-first exact checkout with serialized drift checks. r[cairn.collaboration_backend.portable_shell]
- [x] [serial] Add the backend-neutral collaboration CLI dispatcher while retaining the Radicle compatibility command. r[cairn.collaboration_backend.cli]
- [x] [serial] Emit backend-labeled portable receipts with explicit authority and lifecycle non-claims. r[cairn.collaboration_backend.receipts]
- [x] [serial] Document the backend boundary and run focused, workspace, lifecycle, traceability, Clippy, formatting, purity, and Nix validation before archive. r[cairn.collaboration_backend.non_claims]

## Verification Evidence

- Neutral core, portable envelope, portable CLI, legacy compatibility, and real Git-backed Radicle adapter positive/negative tests passed.
- A portable import/status CLI smoke passed without Radicle context; exact checkout round-trip, tamper, stale-lock, symlink, divergence, and cross-backend lock fixtures passed.
- Full workspace tests, Clippy with warnings denied, formatting, policy freshness, verified-core purity, lifecycle validation/gates, and sync passed.
- Traceability reported `1643/1643` referenced.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`.
