## Source admission

The owning Git repository is `radicle-cob-git-store` at commit `af38dae7e0454b1206512794420a3076ea5a8d1f`; `git archive --format=tar` at that commit has BLAKE3 `ff1392cab83cb76123c56b0a70ab50fa0893788f5a876e4daf614003905a38e0`. `tools/import-radicle-cob-git-store.sh` accepts an explicit local checkout with that object, checks the full archive before extracting, stages the Cargo package plus original license texts inside Cairn's `vendor/` filesystem, refuses modified existing snapshots, and treats an identical snapshot as no-op. Cargo uses only the tracked in-repo path and lockfile; no sibling path or new remote is needed at build time.

## Adapter cutover

Cairn domain evaluators use `Evaluate<GitCobStore>` directly. Existing `CobError` converts `LoadSignerError` into the prior symlink/filesystem/keystore variants; other domain mappers retain their existing signer-unavailable error for keystore failures and map symlinks to their existing symlink variant. Domain-specific reducers, postcondition checks, per-change locks, actor authorization, and high-level create/update/inspect remain local. The CLI and tests import the generic types from the vendor package directly. No compatibility alias, facade, or obsolete local storage file remains.

## Verification and non-claims

Exercise signed round trips and negative refs/signatures in the generic package, Cairn adapter and independent fixture tests, an actual COB CLI path, import refusal/idempotency, formatting/lint, lifecycle gates and relevant Nix checks. These establish only local bounded observations; no transport, Radicle node, actor trust, or release claim transfers with the storage mechanism.
