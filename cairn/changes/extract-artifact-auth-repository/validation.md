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

## Cairn Cargo/Nix source cutover

Cairn now selects both standalone packages and the Nix source through the exact SSH repository `ssh://git@github.com/OnixResearch/artifact-auth.git` at revision `799459346d5416fbd7b9f55840a7371441b55afa`.

- `Cargo.toml` pins `artifact-auth-core` and `artifact-auth-ed25519` to the same revision; `Cargo.lock` contains exactly one `0.1.0` record for each package with the complete immutable Git source identity.
- `flake.nix` declares the same revision as a non-flake source input, avoiding the standalone repository's validation-only transitive inputs and any product dependency cycle. Nix-generated `flake.lock` binds that source to `sha256-nEgz2FtVuDesX95yyxidp0vhjxL4INB6Ve8rkpLyJk0=`.
- `nix/artifact-auth-source-pin.nix` is a pure validator over parsed Cargo manifests, Cargo lock data, Nix lock data, resolved Nix source identity, standalone manifests, and the checked release profile. `flake.nix` is the thin shell that supplies those inputs and turns issues into a failing check.
- Positive validation requires exact Cargo/Nix revision parity, one lock record per package, explicit Cairn workspace wiring, `MIT OR Apache-2.0`, `publish = false`, the reviewed package set, the immutable Git distribution channel, and no sibling-path or floating source declarations.
- Negative self-tests reject a floating Cargo branch, mismatched Nix revision, duplicate Cargo package record, incompatible standalone package/profile license, and workspace-relative Cargo source.

Passing checks:

- pre-change and post-change focused `cairn-core` and `cairn-cli` authenticated-input test filters;
- `cargo check -p cairn-core -p cairn-cli` after Cargo generated the lock update;
- `cargo fmt --check`;
- `nix build .#checks.x86_64-linux.artifact-auth-source-pin --no-link -L`;
- `nix build .#cairn --no-link -L`.

SSH credentials are retrieval authority only. They do not change the reviewed source revision, package license, release profile, authentication semantics, or bounded non-claims, and these checks do not prove credential availability for another operator.

## Pure Cairn adapter

`crates/cairn-core/src/verified/artifact_auth/adapter/` now owns a pure two-step compatibility adapter:

1. `map_cairn_artifact_auth` converts already measured Cairn policy/input observations into an `artifact-auth-core` policy, scope, signer-specific statement, raw public-key bytes, and compatibility identities.
2. The Cairn shell must verify the new statement bytes explicitly, normally with `artifact_auth_ed25519::verify_statement`; `evaluate_cairn_artifact_auth` accepts that standalone cryptographic observation and projects the bounded decision into a distinct `CairnArtifactAuthReport`.

The selected mapping binds `cairn.authenticated-evidence.v1/<evidence-kind>` as the domain, the existing purpose, recomputed receipt profile and BLAKE3 identity as subject, required parent BLAKE3 identities as typed parents, and the measured Valence integrity receipt as verifier/currentness context. Cairn's one-key behavior maps to threshold one and generation one; a non-revoked or revoked Cairn policy observation maps to `Current` or `Revoked` without claiming freshness beyond the supplied context.

The adapter decodes configured public-key hex and dual-records Cairn's legacy BLAKE3-of-lowercase-hex identity beside the standalone BLAKE3-of-raw-key identity. Unsupported Cairn source schemas or signature/key algorithms, malformed keys, missing configured signers, and legacy-key-identity mismatch fail mapping with stable diagnostics that exclude raw key material.

### Approach and adversarial audit

| Family | Result |
| --- | --- |
| Reuse the legacy `signature.verified` boolean for the new statement | Rejected: it authenticates Cairn's historical canonical JSON, not `artifact-auth.statement.v1`, and would widen the cryptographic claim. |
| Put filesystem/canonicalization/verification orchestration in `cairn-core` | Rejected: it would move consumer authority and break the existing functional-core / imperative-shell boundary. |
| Pure map plus explicit exact-statement cryptographic observation | Selected: the adapter owns deterministic translation/projection while the shell retains measurement and verifier provenance. |

A secondary local-model review was attempted but unavailable (`fetch failed`), so deterministic source and test evidence remains authoritative. Independent audit found a promotion hazard in an initial projection that embedded the legacy `AuthenticatedEvidenceReport`: callers could have passed it directly to `evaluate_authenticated_release_gate`. The final `CairnArtifactAuthReport` is a distinct DTO with `decision_passed`, standalone identities, stable issues, mandatory non-claims, and an explicit boundary; it is not accepted by the legacy release-gate API.

Positive and negative adapter tests prove field mapping, explicit-observation control independent of the legacy boolean, passing decision identity/non-claims, malformed-key redaction, legacy/standalone key-identity separation, invalid standalone signature rejection, and revoked-key rejection. The module tree contains no filesystem, environment, process, clock, path, JSON, or presentation APIs.

Passing focused checks:

- pre-change `cairn-core` and `cairn-cli` authenticated-input baselines;
- four adapter positive/negative tests;
- all 347 `cairn-core` tests;
- strict `cairn-core` all-target Clippy;
- `cargo fmt --all --check`;
- Slotcar consumer check over the tracked adapter source;
- `nix build .#cairn --no-link -L`.

These checks establish deterministic translation and a bounded report only. They do not establish dual-run parity, verifier correctness, currentness freshness, lifecycle conformance, authorization, readiness, or release eligibility.

## Dual-run compatibility and rollback

Cairn now dual-runs legacy and standalone authentication in `crates/cairn-cli/src/authenticated_inputs/dual_run.rs` from one measured `AuthenticatedEvidenceInput`. The existing `signature_hex` continues to verify Cairn's historical canonical JSON, while the optional `artifact_auth_signature_hex` verifies only the standalone length-delimited statement through `artifact_auth_ed25519::verify_statement`. The signatures share measured subject, parents, producer, key, purpose, trust, and currentness observations but cannot be reused across intentionally distinct preimages.

The pure comparator under `crates/cairn-core/src/verified/artifact_auth/adapter/compatibility/` emits an ordered classification for every observed dimension using `equal`, `intentionally_mapped`, or `blocking` dispositions:

- **preimage:** both BLAKE3 identities must be valid and distinct, recording the intentional schema/domain transition;
- **identity:** subject, producer, and key label must remain equal; legacy text-key and standalone raw-key identities plus decision-schema identities are explicitly mapped; standalone statement/policy/scope/decision/key identities must be internally consistent;
- **decision:** outcomes must match, reports must be internally well formed, and a rejected pair requires a shared measured, parent, purpose, or currentness cause rather than two unrelated generic signature failures;
- **issue:** reviewed legacy/standalone issue pairs, consumer-retained measurement/parent checks, and standalone cryptographic/threshold enrichment are explicit; every other issue is blocking;
- **non-claim:** the exact legacy boundary, Cairn adapter boundary, and all eleven standalone required non-claims must remain present without additions, omissions, or substitutions.

`AuthenticationComponent.report` remains `Option<AuthenticatedEvidenceReport>`, so the standalone report and compatibility DTO cannot enter `evaluate_authenticated_release_gate`. The shell exposes both under `artifact_auth_standalone` and `artifact_auth_compatibility`, but `check.passed` continues to follow the legacy result. Every compatibility report fixes `legacy_authoritative = true`, `standalone_authority_admitted = false`, and `rollback_available = true`; an absent/malformed dedicated signature, adapter failure, decision mismatch, unknown issue, malformed/same preimage, identity drift, or weakened non-claim makes `case_explained = false` with stable blockers.

### Approach registry and adversarial audit

| Family | Result |
| --- | --- |
| Test-only comparison corpus | Rejected: runtime manifests would expose neither drift nor rollback state. |
| Reuse one signature across both canonical preimages | Rejected: one signature cannot authenticate two intentionally distinct byte contracts, and treating it as reusable would widen the verification claim. |
| Require a replacement standalone signature immediately | Rejected: existing manifests would fail before the legacy rollback could operate. |
| Optional dedicated standalone signature plus pure comparator | Selected: positive parity is cryptographically demonstrable, old manifests retain legacy authority, and missing or unexplained evidence blocks only cutover admission. |

A secondary local-model review was attempted but unavailable (`fetch failed`). Independent audit then falsified an initial rule under which two unrelated invalid signatures could appear compatible because both produced generic rejection classes. The final decision classifier requires a shared rejection context; a dedicated negative test proves unrelated generic failures remain blocking.

The compatibility corpus covers a passing pair of independently signed preimages, graph-byte tampering observed by both paths, missing standalone carrier, a valid-length signature over the wrong preimage, same/malformed preimage identity, producer drift, unknown standalone issue, weakened non-claims, unavailable standalone evaluation, and unrelated rejection causes. The graph tamper case remains explained while both decisions reject; missing or wrong standalone signatures leave the passing legacy report authoritative but produce cutover blockers.

Passing focused checks:

- pre-change adapter and CLI authenticated-input baselines;
- nine pure adapter/compatibility positive and negative tests;
- CLI dual-run baseline, rollback, wrong-preimage, and graph-tamper assertions;
- strict `cairn-core` and `cairn-cli` all-target Clippy;
- `cargo fmt --all --check`;
- pure-core I/O smoke;
- exact artifact-auth source-pin check;
- Slotcar consumer check over the tracked source;
- `nix build .#cairn --no-link -L`.

All 352 `cairn-core` unit tests pass. The seven-test CLI determinism suite also passes after repairing a pre-existing stale expected-order vector: policy, total-count, and coverage fixtures already included `agent_completion_validate`, while one vector still listed only the other eighteen cases.

The first broader CLI run exhausted the per-user `/tmp` quota through stale `nix-shell.*` trees; removing only completed owned shell temporaries reduced apparent `/tmp` use from 23.37 GiB to 1.63 GiB. After recovery, the full CLI suite advances to `evidence_export_materializes_complete_portable_chain` and fails for the expected mid-change Traceability boundary: `cairn-default` reads requirements only from `cairn/specs`, so the seven active-change implementation markers under `cairn.artifact_auth.repository_extraction.*` are dangling until accepted-spec sync in V4. That makes generated release-readiness and its bundle/manifest verification invalid. The exact dangling IDs are `conformance`, `consumer_authority`, `consumer_authority.cairn`, `consumer_authority.no_promotion`, `cutover`, `cutover.negative`, and `cutover.positive` under the `cairn.artifact_auth.repository_extraction` prefix; this is not an I12 authentication mismatch. V3 and V4 remain open and MUST rerun the full suite after accepted-spec sync.

These results classify the checked corpus and preserve rollback; they do not admit standalone authority, prove corpus completeness, prove verifier correctness, replace consumer lifecycle checks, or establish readiness/release eligibility.

## Consumer-owned adoption handoff

The immutable standalone revision `799459346d5416fbd7b9f55840a7371441b55afa` publishes typed Nickel profiles and checked JSON projections for Cairn, Mantle, Molten, and Valence under `config/consumers/` and `fixtures/consumers/`. The remote `refs/heads/main`, local standalone checkout, release profile, Cargo/Nix pins, and consumer paths all identify that same reviewed revision. Each profile records the frozen consumer source baseline, shared fields, explicit extensions, and retained product authority; it is review input rather than proof that a later consumer checkout remains compatible.

Consumer-owned active Cairn packages now exist without producer-side implementation edits:

| Consumer | Commit | Package | Retained boundary |
| --- | --- | --- | --- |
| Mantle | `52a0c657` | `cairn/changes/adopt-artifact-auth` | OCI/action-result semantics, repository/registry/credential/signing, cache/build/release authority |
| Molten | `e406b84` | `cairn/changes/adopt-artifact-auth` | entropy, keys/rotation, capability/federation authority, Preserves/Iroh, runtime/evidence authority |
| Valence | `bbcc5a10` | `cairn/changes/adopt-artifact-auth` | Radicle verification, actor authorization, Evidence IR/COB/Git/SSH, lifecycle/release interpretation |

Cairn's current extraction package is its own consumer-owned adoption change. Every downstream package records exact standalone source/profile identity, requires a current local baseline, keeps effects in the consumer shell, mandates positive and negative mapping-loss fixtures, rejects unrelated-failure false parity, and preserves a bounded legacy rollback. All three downstream repositories pass native Cairn validation and proposal/design/tasks gates for the new package; each package intentionally has seven unchecked implementation tasks.

Only the four lifecycle files in each new package were committed. No downstream Cargo/Nix dependency, lock, runtime, policy, source, readiness, or release implementation was changed. Pre-existing Mantle bootstrap work and Molten's untracked eBPF lifecycle package were left unstaged and untouched. SSH credentials remain retrieval authority only.

## Final Cairn verification

Executed sync promoted all 30 extraction requirement/scenario IDs into `cairn/specs/artifact-auth-repository/spec.md`. The first post-sync Traceability run correctly rejected 23 standalone-owned IDs that had no accepted evidence link. `crates/cairn-core/src/verified/artifact_auth/mod.rs` now records those as `verify` links to the exact standalone release/source-pin/conformance rail and explicitly states that linkage is not Cairn-local proof. Traceability then passed with 1,577/1,577 accepted IDs referenced and no missing or dangling IDs.

The complete V3 rail passes:

- `cargo fmt --all --check` in the repository dev shell;
- full `cargo test --workspace`, including all authenticated-input, compatibility, determinism, release-receipt, lifecycle, and negative tests;
- strict `cargo clippy --workspace --all-targets -- -D warnings`;
- first-party `cargo tigerstyle check --output-format json --artifact-dir artifacts/slotcar-run`;
- policy export freshness;
- Cairn validation plus proposal, design, and tasks gates;
- Traceability coverage at 1,577/1,577;
- exact `artifact-auth-source-pin`, including synthetic floating/revision/duplicate/license/workspace-relative negative mutations;
- Slotcar consumer validation;
- `nix build path:$PWD#cairn --no-link -L`; and
- full `nix flake check path:$PWD -L`, including self-dogfood, with `all checks passed` on the supported `x86_64-linux` system.

Two environmental/build-closure discriminators were resolved rather than suppressed. First, the initial full Cargo run filled a stale per-user `/tmp` tree (235.18 GiB apparent) and three receipt tests failed only with `Disk quota exceeded`; after deleting that completed job's disposable tree, rerunning with a home-scoped temporary directory passed the entire workspace. A targeted Nix build also created the conventional ignored `result` symlink; the determinism suite rejected it as intended, and removing only that generated symlink restored the passing run. Second, the first full flake check found three structured-lifecycle tests unable to spawn the separately pinned `nickel-export` executable: host tests inherited it from the dev shell, while `rust-tests.nativeCheckInputs` named only Git and Nickel. Adding the exact `nickel-export` package to that Nix test closure made the isolated `rust-tests` derivation and full flake rail pass. No lockfile was edited for either repair.

The operator guide now records the exact SSH revision, distinct legacy/standalone signatures, diagnostic compatibility fields, source update procedure, runtime/dependency rollback, retained Cairn authority, and mandatory non-claims. These passing rails establish current source, test, lifecycle, and build evidence only; they do not admit standalone release authority or widen authentication claims.

## Next cutover boundary

Standalone publication I9, exact source pinning I10, pure Cairn adapter I11, dual-run compatibility I12, consumer-owned handoff I13, accepted-spec sync, operator documentation, and final full-rail verification are complete. Only V4 archive mutation remains. The change may be archived after marking the closeout task complete, confirming a clean current tasks gate and archive dry run, executing the native Cairn archive, and committing the resulting archive state.
