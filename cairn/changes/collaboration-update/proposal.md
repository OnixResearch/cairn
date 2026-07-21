# Preview-First Collaboration Update

## Why

Cairn can import, inspect, and check out collaborative active changes, but operators cannot publish subsequent exact artifact edits through the collaboration interface. The reducer already models explicit revision bases and conflict-preserving artifact updates, so the missing shell leaves collaboration state effectively static after import.

## What Changes

- Add a pure backend-neutral planner that compares one exact local active-change snapshot with one conflict-free collaboration state and derives explicit artifact updates or an idempotent no-op. r[cairn.collaboration_update.plan]
- Add preview-first `portable-json.v1` update execution with serialized input rechecks and atomic validated store replacement. r[cairn.collaboration_update.portable_execution]
- Add preview-first signed Radicle update execution with current creator-or-delegate authority, signer possession, serialized drift checks, and typed reload verification. r[cairn.collaboration_update.radicle_execution]
- Add backend-labeled BLAKE3-bound receipts that expose revision bases, changed artifacts, content identities, blockers, and mutation results without claiming conflict resolution or peer publication. r[cairn.collaboration_update.receipts] r[cairn.collaboration_update.non_claims]

## Impact

- **Files**: collaboration update core, portable and Radicle shells/adapters, CLI dispatch, accepted collaboration specs, README/workflow docs, and positive/negative fixtures.
- **Testing**: focused core/adapter/CLI tests, workspace tests, formatting, Clippy, policy freshness, Cairn gates, traceability, verified-core purity, and Nix flake checks.
