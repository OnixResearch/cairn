# Tasks

## Phase 1: Pure Import Plan

- [x] [serial] Add a typed pure import snapshot and validated lossless mapping into `ChangeCobInit`, with positive exact-byte and negative malformed-ID/limit tests. r[cairn.radicle_import_cli.lossless_plan]

## Phase 2: Current Authority Adapter

- [x] [serial] Add explicit Radicle profile loading, canonical current-identity delegate inspection, and fail-closed authority results without trusting historical operation authority. r[cairn.radicle_import_cli.current_authority]
- [x] [serial] Add typed COB creation and reload that binds the checked identity OID and rejects unauthorized actors or semantic-root drift. r[cairn.radicle_import_cli.signed_creation]

## Phase 3: CLI and Evidence

- [x] [serial] Add preview-first `cairn radicle import` parsing, bounded active-change reads, deterministic receipts, and `--execute` mutation behavior with positive and negative CLI tests. r[cairn.radicle_import_cli.command] r[cairn.radicle_import_cli.receipt]
- [x] [serial] Document the operator flow and bounded claims; run focused tests, workspace tests, Clippy, formatting, purity, lifecycle gates, traceability, and Nix validation. r[cairn.radicle_import_cli.non_claims]

## Verification Evidence

- Pure import and existing reducer tests passed with positive exact-byte and negative malformed/limit fixtures.
- Radicle adapter tests passed for current canonical authority, signed typed create/reload, duplicate rejection, unauthorized no-mutation, and concurrent-import locking.
- CLI tests passed for explicit context, bounded reads, deterministic receipts, blockers, and non-claims.
- Workspace tests, formatting, workspace Clippy, policy freshness, lifecycle validation/gates, and verified-core purity passed.
- Traceability reported `1611/1611` referenced.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`.
- Live-node publication and peer synchronization remain explicit non-claims because no external `rad` CLI/node was available.
