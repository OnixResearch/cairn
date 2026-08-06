## Why

Cairn already links a change to an upstream receipt through `cross_repo_dependencies`. The current `SUPPORTED_RECEIPT_KINDS` allowlist contains only Cairn lifecycle kinds such as `proposal_gate`, `sync_receipt`, and `archive_receipt`. An "issue" kind is not present, so a change cannot gate on a Radicle issue today.

Teams track work in Radicle issue COBs. To make Cairn changes honest about that work, a change must be able to declare "this change requires Radicle issue #3 to be open (or closed)" and gate on a typed receipt.

## What Changes

- Add a `radicle-issue.v1` receipt kind to the cross-repo dependency allowlist.
- Define a typed `cairn.radicle-issue-receipt.v1` contract for the issue receipt.
- Add a bounded, read-only Radicle issue adapter that emits the issue receipt from a local COB store.
- Keep the issue adapter out of Cairn lifecycle reduction and out of any write path.
- Gate an issue linkage through the existing cross-repo dependency evaluator and blocker machinery.
- Add positive and negative core fixtures and CLI integration coverage.

## Impact

- **Core**: Add the receipt kind, the issue receipt schema, validation, and pure evaluator support in `cross_repo_dependencies.rs`.
- **Adapter**: Add a read-only `radicle_issue` decoder in `cairn-radicle-cob` that emits issue receipts with explicit non-claims.
- **CLI**: Load issue evidence only from explicit operator-supplied files or imported bundles.
- **Documentation**: Document issue linkage metadata, the receipt schema, non-claims, and the no-fetch boundary.
- **Testing**: Add positive and negative fixtures for satisfied, unsupported, stale, mismatched, and missing issue evidence.

## Out of Scope

- Reading or writing the Radicle-native issue COB as Cairn lifecycle state.
- Authoring updates to a Radicle issue from Cairn.
- Node, peer, discovery, sync, or convergence behavior.
- Replacing local `depends_on` dependency ordering.
- Treating an issue receipt as proof of issue correctness, closure authority, remote trust, or release eligibility.
