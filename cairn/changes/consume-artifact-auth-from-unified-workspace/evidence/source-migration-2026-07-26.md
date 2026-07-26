# Cairn Artifact workspace source migration

Observed: 2026-07-26

## Source identities

Historical rollback source:

- Repository: `ssh://git@github.com/OnixResearch/artifact-auth.git`
- Revision: `799459346d5416fbd7b9f55840a7371441b55afa`

Active unified source:

- Repository: `ssh://git@github.com/OnixResearch/onix-artifact.git`
- Revision: `e41340bec587b6d049b5cc518ec7db925dde84be`
- Nix NAR hash: `sha256-2cM912L2YXVnVX9LquwhAPKyjPP/z/oFQRe7Qq9bHHE=`
- Source archive BLAKE3: `8fc65f3d7968d76af419f08ac972955514c8256f8aa5ed90815d8bdcced28113`

`git ls-remote` resolved remote `refs/heads/main` to the full unified revision. This proves remote object availability at the observation time only.

## Authentication source parity

Recursive comparison found no differences in either authentication crate directory.

- `artifact-auth-core/src/lib.rs`: `839ea3e35429e2a1252020dd779b46ebd185b6911674036820da118d487d1d21`
- `artifact-auth-ed25519/src/lib.rs`: `7fd3987fe3ddc6d30efca37878952135f19905a00b52f349db237054be6b3633`

The source migration changes repository identity and revision only. It does not change authentication package bytes, APIs, features, or Cairn adapters.

## Package boundaries

The unified source profile and workspace contain exactly:

1. `artifact-auth-core`
2. `artifact-auth-ed25519`
3. `artifact-transfer-core`

Cairn's generated Cargo lock contains exactly the two authentication packages from this source. It contains no `artifact-transfer-core` or artifact-binding package.

The pure source-pin validator checks the complete source workspace and the narrower consumer graph separately.

## Positive and negative validation

Before the migration:

- nine focused `cairn-core` artifact-auth tests passed;
- the exact historical source-pin Nix check passed.

After the migration:

- Cargo generated the lock update and checked `cairn-core` plus `cairn-cli`;
- Nix generated the renamed non-flake input lock;
- nine focused `cairn-core` artifact-auth tests passed;
- the focused shell dual-run test covers passing, missing standalone signature, wrong standalone signature, and tampered measured input cases;
- the exact unified source-pin Nix check passed.

Negative source-pin self-tests reject:

- a floating Cargo branch;
- a mismatched Nix revision;
- duplicate authentication lock entries;
- transfer-package entry into Cairn's consumer graph;
- an incomplete source profile;
- incomplete source workspace membership;
- incompatible source licensing;
- a workspace-relative Cargo dependency;
- authentication entry-point byte drift.

## Authority and rollback

Cairn retains filesystem measurement, canonical JSON reconstruction, lifecycle policy, trusted-producer selection, currentness inputs, Valence linkage, readiness composition, diagnostics, exit behavior, and release decisions.

The historical source revision remains an explicit rollback coordinate. It is not an active Cargo or Nix dependency.

This migration does not prove artifact truth, trust-root correctness, revocation freshness, verifier correctness, authorization, lifecycle conformance, release readiness, Radicle availability, or whole-stack GitHub independence.
