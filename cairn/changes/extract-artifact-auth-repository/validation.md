# Validation evidence

## Completion contract

The change is complete only when a product-neutral repository exists at an immutable published revision, positive and negative conformance passes, Cairn pins the same Cargo/Nix revision, dual-run differences are explained, accepted specs are synced, and the change archives. A local crate, plausible design, unchecked task boxes, synthetic cryptographic observations, sibling-path dependency, or passing signature without bounded non-claims is not completion.

## Fixed source cohort

- Cairn `cc4639be24475d80844b091cdec605eb2fef9ff0`
- Mantle `26c6880c08cd730c9e9420f92ea91a87212e492d`
- Molten `70590459a218dc8e66948ab6f305a7c54142b710`
- Valence `8eccac35fcd8aae8ef64b3d1021e7f1a53656180`

Exact reviewed file BLAKE3 SRI identities, semantic classifications, retained authority, and compatibility limits are recorded in `../artifact-auth/docs/source-inventory.md` in the sibling workspace checkout.

## Pre-change baselines

- `nix develop -c cargo test -p cairn-core authenticated_inputs`
- `nix develop -c cargo test -p cairn-cli authenticated_inputs`
- `nix develop -c cargo test -p crunch-action-result-core`
- `nix develop -c cargo test -p crunch-attestation-core`
- `nix develop -c cargo test -p crunch-build signing::tests`

All baseline commands passed. The Mantle signing filter ran fifteen tests with no failures. Existing unrelated Mantle worktree changes were not modified.

## Approach registry

| Family | Mechanism | Evidence | State |
| --- | --- | --- | --- |
| Cairn-first | Generalize typed authenticated-input observations. | Clean functional-core/shell boundary, but one-key and lifecycle/Valence-specific schema. | Falsified as sole source. |
| Mantle-first | Move OCI or action-result trust implementation. | Strong distinct-full-key threshold and canonical normalization, but OCI/Nix/build/signing authority is product-specific. | Falsified as unmodified move. |
| Contract-first synthesis | Define one typed length-delimited statement and reuse the strongest pure invariants. | Preserves Cairn observations, Mantle full-key thresholds, Molten purpose/generation/currentness, and Valence authorization separation. | Selected and implemented. |

## Standalone implementation

Local repository: `../artifact-auth`

Published immutable Git revision: `799459346d5416fbd7b9f55840a7371441b55afa`

Remote source: `https://github.com/OnixResearch/artifact-auth` at `refs/heads/main`; `main@origin` and `git ls-remote` both resolve to the exact revision above.

The repository contains:

- `artifact-auth-core`: `#![no_std]` plus `alloc`, versioned length-delimited statements, typed BLAKE3/SHA-256 references, deterministic policy/currentness/threshold decisions, and required non-claims;
- `artifact-auth-ed25519`: pure raw-key Ed25519 verification and a composed verify-and-evaluate path;
- typed Nickel repository, release, and per-consumer mapping profiles with checked JSON exports and mapping-loss rejection;
- frozen legacy Cairn/Mantle canonical statement bytes and BLAKE3 comparison identities, verified by the conformance suite;
- native Cairn policy/spec lifecycle and traceability evidence;
- source grant, exact provenance, preserved attribution, consumer mappings, migration guidance, and positive/negative conformance fixtures;
- generated Cargo and Nix lockfiles produced by their owning tools.

## Deterministic checks

Passing focused checks:

- `cargo test --workspace`
- `cargo check -p artifact-auth-core --no-default-features --target wasm32-unknown-unknown`
- `cargo check -p artifact-auth-ed25519 --no-default-features --target wasm32-unknown-unknown`
- `cargo clippy --workspace --all-targets -- -D warnings`
- Nickel typecheck/export freshness for repository and release profiles
- Cairn validation and traceability coverage: six requirements, six referenced, verdict `pass`

`nix flake check --option secret-key-files '' -L` passed all ten checks. The default signing-key option was cleared because this host configuration references an absent `/run/secrets/vars/nix-signing-key/key`; the repository checks themselves passed. The flake covered Cargo tests, Clippy, formatting, host and Wasm `no_std` builds, Nickel export freshness, Cairn validation/traceability, and the authority audit. Negative Nickel fixtures reject invalid standalone licensing, absent or false owner authorization, mismatched fixed source provenance, and weakened non-claims; the authority audit also rejects product dependencies and verifies package-license metadata plus the recorded fixed source cohort.

## Adversarial audit

The secondary reviewer proposed collision, order, duplicate-threshold, stale-generation, and observation-forgery hypotheses. Deterministic inspection rejected the first four: fields are length-delimited, parents are sorted before encoding and scope comparison, verified identities are held in a `BTreeSet`, and generation/currentness are explicit policy observations. Signature bytes are intentionally outside the statement preimage because they authenticate rather than define the statement.

The observation-forgery hypothesis identified a useful misuse boundary. `artifact-auth-ed25519::verify_and_evaluate` was added as the normal path so policy admission is composed directly with canonical-byte verification. Protocol adapters may still supply an observation to the core, but verifier correctness and observation provenance are now explicit non-claims. Statement bytes are public and make no confidentiality claim.

## Published release identities

The reviewed standalone source is published at immutable Git revision `799459346d5416fbd7b9f55840a7371441b55afa`. Distribution remains Git/Nix only: both Cargo packages set `publish = false`, so no crates.io registry-name claim is made.

- Cargo packages: `artifact-auth-core@0.1.0` and `artifact-auth-ed25519@0.1.0`; workspace manifest BLAKE3 SRI `blake3-SDBr9Q2T52DaPDvyhu1Qoy/IyzT2ZQpMCSXJSQI3sP8=`; core manifest `blake3-aQ+q76b7WemahBOFfPrl3+jBqXe7rLBwbTUhdjmRLTs=`; Ed25519 manifest `blake3-Ad05hGHX4sYWUR4bwpzbBm/622j0IiNhEeYEgXqTA0c=`; Cargo lock `blake3-ccVDoXb4N88xE/rua84Ezm+zuLRZs7FpCa8pSdFaDj4=`.
- Nix source: `github:OnixResearch/artifact-auth/799459346d5416fbd7b9f55840a7371441b55afa`; Nix-prefetched source hash `sha256-nEgz2FtVuDesX95yyxidp0vhjxL4INB6Ve8rkpLyJk0=`. SHA-256 is retained here because Nix source addressing requires its SRI hash; repository-owned file identities remain BLAKE3. Flake source BLAKE3 SRI is `blake3-bV5MHa5FlGde3D4yada/AV/k1tkHjTvx5eZJOuVcxiQ=` and flake lock BLAKE3 SRI is `blake3-S9ksqG51Huqkw/joT9z8ncVHTCy2rkuoe/cDApqA8Go=`.
- License and provenance: `MIT OR Apache-2.0`; MIT text `blake3-SDK+A1UmXBhOt6L8fyTk0i01O8oqg/YwQW6pxU+ZQmI=`; Apache-2.0 text `blake3-g8s6L8+Cm2E44JWwgwFsNN3N+ge2jTh4JyLBT8+FrOY=`; owner grant `blake3-0e8U9uP546z7UqjDOFJJwsjuXL7lRBxZMnXxw1qB/is=`; source inventory `blake3-t1wWzYhnjo/BZMs3hrmZc24f3yNpfgYKIoaLyU+dzbg=`.
- Release profile: typed Nickel source `blake3-28d/KoqVm4WUnpT+m2jyqhER9sMvCfklrJMYo/PRWAc=`; checked JSON projection `blake3-XkAoYxjPfwKNs8i/KT4Km0+3xvXudd2pfbYTD8nl804=`.
- Conformance corpus: positive and negative cases `blake3-kZahH760kdhvPAsEUrmS7WyCYq8X8H5ROSYDPJdxCvI=`.

These identities bind the published source and reviewed release inputs; they do not add artifact truth, authorization, verifier-correctness, or downstream release claims.

## Next cutover boundary

Standalone publication task I9 is complete. Cairn Cargo/Nix pinning and cutover tasks I10-I12, consumer-owned adoption handoff I13, final full-rail verification, accepted-spec sync, and archive remain open. The next discriminating action is to pin the exact published revision in Cairn without a sibling path, verify Cargo and Nix source parity, and reject floating or mismatched inputs.
