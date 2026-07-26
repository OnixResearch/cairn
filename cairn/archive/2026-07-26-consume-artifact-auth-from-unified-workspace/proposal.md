## Why

Cairn still pins the historical `OnixResearch/artifact-auth` repository even though the neutral authentication packages now live unchanged in the unified `OnixResearch/onix-artifact` workspace. Keeping the old source in Cairn preserves duplicate distribution and source-pin policy after the unified workspace became available at an immutable revision.

## What Changes

- Move Cairn's Cargo and Nix source pin to `OnixResearch/onix-artifact` revision `e41340bec587b6d049b5cc518ec7db925dde84be`.
- Bind the exact Nix NAR hash, source archive BLAKE3, source workspace package set, and unchanged authentication crate bytes.
- Keep Cairn's resolved crate graph limited to `artifact-auth-core` and `artifact-auth-ed25519`; do not admit `artifact-transfer-core` into Cairn.
- Strengthen positive and negative source-pin checks for workspace-package drift, consumer-graph widening, floating sources, local paths, license drift, and Cargo/Nix disagreement.
- Preserve the historical `artifact-auth` revision as a bounded rollback source without retaining it as an active dependency.
- Update the accepted source-ownership requirement to the unified workspace package boundary.

## Success Criteria

- Cargo and Nix resolve the same immutable unified-workspace revision.
- The authentication crate directories and source entry points match the historical source exactly.
- Focused positive and negative authentication tests pass before and after the source migration.
- Cairn retains measurement, canonical payload, lifecycle, authorization, evidence, and release authority.
- No consumer behavior, public API, or runtime authority changes.

## Non-Goals

- Publishing or changing the unified workspace remote.
- Adding transfer or binding behavior to Cairn.
- Removing historical repositories or rollback evidence.
- Claiming Radicle availability, whole-stack GitHub independence, artifact truth, authorization, or release readiness.
