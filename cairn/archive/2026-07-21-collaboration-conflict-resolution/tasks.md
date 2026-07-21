# Tasks: Explicit Collaboration Conflict Resolution

## Phase 1: Pure Inspection and Planning

- [x] [serial] Add deterministic conflict-head inspection and bounded exact single-head disclosure for artifact and lifecycle conflicts. r[cairn.collaboration_resolution.inspection]
- [x] [serial] Add complete history-preserving resolution planning, explicit lifecycle reset, deterministic operation/result identities, and stale-plan application. r[cairn.collaboration_resolution.plan]
- [x] [serial] Add positive and negative core tests for complete selection, exact bases, lifecycle reset, unauthorized/stale/malformed inputs, no-conflict input, and plan drift. r[cairn.collaboration_resolution.inspection] r[cairn.collaboration_resolution.plan]

## Phase 2: Provider Execution

- [x] [serial] Add preview-first portable resolution with provider/repository locking, full input rechecks, validated prepared replacement, and exact reload verification. r[cairn.collaboration_resolution.portable_execution]
- [x] [serial] Add preview-first signed Radicle resolution with current identity authority, signer possession, serialized state rechecks, typed reload, and exact result verification. r[cairn.collaboration_resolution.radicle_execution]
- [x] [serial] Add positive and negative provider tests for inspection, preview, successful resolution, drift, locks, unsafe stores, missing objects, and unauthorized actors. r[cairn.collaboration_resolution.portable_execution] r[cairn.collaboration_resolution.radicle_execution]

## Phase 3: CLI, Receipts, and Validation

- [x] [serial] Expose backend-neutral `collaboration conflicts|resolve` plus compatibility `radicle conflicts|resolve` with strict explicit selectors. r[cairn.collaboration_resolution.receipts]
- [x] [serial] Emit deterministic backend-labeled inspection/resolution receipts with head metadata, choices, complete bases, identities, authority observations, blockers, execution mode, mutation result, and bounded non-claims. r[cairn.collaboration_resolution.receipts] r[cairn.collaboration_resolution.non_claims]
- [x] [serial] Update public docs and run focused, workspace, lifecycle, traceability, formatting, Clippy, purity, and Nix validation before sync/archive. r[cairn.collaboration_resolution.non_claims]

## Verification Evidence

- Focused collaboration core, Radicle adapter, and portable CLI baselines passed before implementation.
- Pure resolver tests passed for sorted inspection, bounded one-head disclosure, complete multi-artifact selection, deletion choice, exact complete bases, lifecycle reset, stable hashes, unauthorized/stale/incomplete/duplicate choices, no-conflict input, and plan drift.
- Portable provider tests passed for conflicted-envelope round trip, preview, exact execution/reload, incomplete choices, wrong actor, stale lock, stale prepared state, symlink storage, strict arguments, and non-claims.
- Radicle provider tests passed for signed typed resolution/reload, stale-plan rejection, current-identity rejection, and real explicit profile/signer execution.
- Exact standalone CLI dispatch smoke passed for `collaboration conflicts` and blocked no-conflict `collaboration resolve`.
- Full workspace tests, formatting, and Clippy with warnings denied passed after implementation.
- Policy freshness, Cairn validation, proposal/design/tasks gates, and verified-core purity passed.
- Traceability coverage passed with `1674/1674` accepted requirement and scenario IDs referenced.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`; `aarch64-linux` was omitted as incompatible.
