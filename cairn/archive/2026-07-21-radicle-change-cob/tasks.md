# Tasks

## Phase 1: Lossless Core

- [x] [serial] Add the versioned lossless active-change state, validated artifact identifiers, protocol bounds, BLAKE3 content/state identities, and exact native materialization plan in `cairn-core`. r[cairn.radicle_change_cob.lossless_change_state] r[cairn.radicle_change_cob.materialized_view]
- [x] [serial] Add the pure operation reducer with explicit revision bases, deterministic head derivation, creator/delegate authority facts, lifecycle transitions, and append-only evidence. r[cairn.radicle_change_cob.deterministic_reducer] r[cairn.radicle_change_cob.evidence_boundary]
- [x] [serial] Add positive and negative core tests for lossless materialization, disjoint convergence, same-artifact conflicts, resolution, stale or unknown bases, unauthorized edits, stale evidence/transitions, malformed identifiers/hashes, duplicate actions, and protocol limits. r[cairn.radicle_change_cob.lossless_change_state] r[cairn.radicle_change_cob.deterministic_reducer] r[cairn.radicle_change_cob.evidence_boundary] r[cairn.radicle_change_cob.materialized_view]

## Phase 2: Radicle Adapter

- [x] [serial] Add `crates/cairn-radicle` with the `xyz.onixresearch.cairn-change` action encoding and native Radicle `Cob`, `CobWithType`, and `Evaluate` implementations over the pure reducer. r[cairn.radicle_change_cob.radicle_adapter]
- [x] [serial] Add positive and negative adapter tests for canonical action round trips, stable type identity, root-shape rejection, missing identity documents, and delegate/creator authority mapping where the Radicle test harness supports deterministic fixtures. r[cairn.radicle_change_cob.radicle_adapter]

## Phase 3: Evidence and Validation

- [x] [serial] Document the COB/Git identity split, aggregate dependency-validation boundary, historical-versus-current authority boundary, resource limits, and explicit non-claims in the public README. r[cairn.radicle_change_cob.evidence_boundary] r[cairn.radicle_change_cob.radicle_adapter]
- [x] [serial] Run focused core and adapter tests, formatting, Clippy, Cairn validation, proposal/design/tasks gates, traceability coverage, and the verified-core purity smoke check; record exact blockers rather than promoting untested peer-sync claims. r[cairn.radicle_change_cob.lossless_change_state] r[cairn.radicle_change_cob.deterministic_reducer] r[cairn.radicle_change_cob.evidence_boundary] r[cairn.radicle_change_cob.radicle_adapter] r[cairn.radicle_change_cob.materialized_view]

## Verification Evidence

- Baseline: `cargo test -p cairn-core structured_lifecycle` passed `6/6` before core changes.
- Focused core: `cargo test -p cairn-core radicle_change` passed all positive and negative reducer tests.
- Radicle adapter: `cargo test -p onix-cairn-radicle` passed `7/7`, including a real Git-backed generic `Store` create/update/get flow, identity-document delegate mapping, unauthorized update rejection, and missing-identity rejection.
- Workspace: `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` passed after explicit canonical JSON ordering removed `serde_json/preserve_order` hash drift.
- Replay regression: focused determinism tests passed after cross-root copies stopped copying derived Cargo `target` trees.
- Lifecycle: policy export freshness, Cairn validation, proposal/design/tasks gates, and traceability coverage passed; coverage reported `1596/1596` referenced.
- Boundary: the verified-core purity search found no filesystem, environment, process, clock, async, or output effects in `radicle_change.rs`.
- Broad rail: `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`.
- Non-claim: the test harness validates local signed Git-backed COB storage and reduction, not live-node transport, multi-peer availability, current-authority execution, evidence truth, archive truth, or release readiness.
