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

Current immutable local Git revision: `d7f77787fa8a88e843ddde7161c981175badb4b4`

The repository contains:

- `artifact-auth-core`: `#![no_std]` plus `alloc`, versioned length-delimited statements, typed BLAKE3/SHA-256 references, deterministic policy/currentness/threshold decisions, and required non-claims;
- `artifact-auth-ed25519`: pure raw-key Ed25519 verification and a composed verify-and-evaluate path;
- typed Nickel repository and release profiles with checked JSON exports;
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

The full `nix flake check --option secret-key-files '' -L` result must be recorded before marking the standalone release/conformance task complete.

## Adversarial audit

The secondary reviewer proposed collision, order, duplicate-threshold, stale-generation, and observation-forgery hypotheses. Deterministic inspection rejected the first four: fields are length-delimited, parents are sorted before encoding and scope comparison, verified identities are held in a `BTreeSet`, and generation/currentness are explicit policy observations. Signature bytes are intentionally outside the statement preimage because they authenticate rather than define the statement.

The observation-forgery hypothesis identified a useful misuse boundary. `artifact-auth-ed25519::verify_and_evaluate` was added as the normal path so policy admission is composed directly with canonical-byte verification. Protocol adapters may still supply an observation to the core, but verifier correctness and observation provenance are now explicit non-claims. Statement bytes are public and make no confidentiality claim.

## Exact blocker

`git ls-remote git@github.com:OnixResearch/artifact-auth.git HEAD` reports `Repository not found`. Push behavior requires an explicit user request, so the local repository has not been created or pushed remotely. Until a remote immutable revision exists:

- standalone publication task I9 remains open;
- Cairn Cargo/Nix pinning and cutover tasks I10-I12 remain open;
- consumer adoption handoff I13 remains open;
- final Cairn compatibility/full-rail and archive tasks remain open.

The next discriminating action is remote repository creation followed by an explicitly authorized push of local bookmark `main`, then pinning the resulting revision without sibling paths.
