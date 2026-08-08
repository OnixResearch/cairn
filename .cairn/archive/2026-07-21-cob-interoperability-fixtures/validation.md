# Validation: COB Interoperability Fixtures

## Result

Cairn now carries a deterministic, provenance-bound signed Git COB bundle produced through upstream `radicle::storage::git::Repository` without any Cairn crate. Production Cairn code loads that root and appends an authorized delegate update. A separate test-only storage reader and minimal action reducer then evaluates Cairn-produced create/update history without importing Cairn's COB repository, evaluator, inspection, or reducer implementation.

The evidence remains local and offline. The standalone producer is excluded from Cairn's root workspace, product lockfile, Nix dependency inputs, released executables, and runtime behavior.

## Fixture and Provenance Evidence

- `fixtures/radicle-cob-interoperability/external-cairn-cob.bundle` is a complete Git SHA-1 bundle with `refs/heads/main` and one standard namespaced Cairn COB ref.
- The retained standalone producer pins `radicle` 0.24.0, `radicle-cob` 0.20.0, `radicle-crypto` 0.17.0, `radicle-oid` 0.2.0, and `git2` 0.20.4 in its own lockfile.
- The producer manually authors Cairn init JSON and writes through upstream Radicle Git storage; it imports no Cairn package.
- Two independent producer runs emitted byte-identical metadata and Git bundles before the fixture was accepted.
- Typed `provenance.ncl` and generated `provenance.json` bind BLAKE3 identities for the bundle, producer source, standalone manifest, and standalone lockfile, plus exact versions, actor/delegate keys, base/object/operation IDs, type, ref, repository ID, proposal, and non-claims.
- The dedicated Nix gate exports Nickel and byte-compares the generated JSON, verifies the Git bundle, checks the producer formatting, audits the product dependency tree, and runs both direct format and interoperability suites.

## Inward Compatibility Evidence

- A fresh bare repository fetches only the bundle's explicit branch and namespaced refs without retaining a Git remote.
- Cairn's production `inspect_change` recovers the declared type, change ID, repository ID, typed base, creator, immutable delegate, proposal, root operation, and signed namespaced ref.
- The deterministic public test seed reproduces the declared delegate key.
- Production `update_change` appends a delegate-signed proposal revision, matches the planned content root, retains the external root operation and commit, and leaves the repository without configured remotes.

## Outward Compatibility Evidence

- `ReferenceRepository` independently discovers namespaced refs, validates namespace/signed-author equality, and delegates only low-level Git change loading to the direct format library.
- `ReferenceState` independently decodes the minimal Cairn init and `set_artifact` JSON, checks type/version/schema, related base, immutable authority, linear current-head basis, and signed actor authorization.
- Static source assertions reject accidental imports of `cairn_core`, `CobRepository`, `CairnChangeCob`, `inspect_change`, or `apply_change_cob_operation` into either consumer component.
- The direct consumer recovers exact schema, IDs, authority, operations, and updated proposal from a Cairn-created and Cairn-updated object.

## Negative Evidence

- A one-byte bundle mutation fails BLAKE3 identity before import.
- Redirecting the declared namespaced ref to the ordinary unsigned base commit fails as `InvalidCobChange`.
- Removing the declared namespaced ref fails as `ChangeMissing`.
- Replacing the loaded root manifest with a non-default version fails with the explicit `VersionMismatch` diagnostic.
- The standalone producer lock has no `radicle-node`, while Cairn's maintained product lock and package dependency tree contain neither `radicle` nor `radicle-node`.

## Portfolio Search and Adversarial Audit

Four mechanism families were considered. The selected upstream-storage static fixture changes the storage/ref writer while preserving offline validation. A hand-built raw Git/SSH encoder was deferred because duplicating commit and signature grammar could create false incompatibility. Another raw-`git2` wrapper around the same existing path was rejected as too correlated. A live profile/node route was excluded because it contradicts the product boundary.

The first producer build selected `radicle-oid` 0.2.1, which pulled `git2` 0.21 and exposed an upstream cohort incompatibility. The standalone producer now pins the same exact `radicle-oid` 0.2.0 cohort as Cairn. The first bundle import used a non-bare repository and Git correctly refused to fetch into its checked-out branch; fixture materialization now uses a fresh bare repository and verifies that no remote remains.

The first post-sync traceability run found that markers in README and `flake.nix` are outside the configured evidence roots. Release-boundary markers now live beside the executable Rust checks, after which accepted traceability and evidence export passed. Pre-sync workspace evidence export failed as expected while the new active requirements were not yet accepted; the full workspace suite passed after bounded sync.

The advisory audit produced generic concerns and one useful discriminator: bind declared IDs and versions to observations rather than only hashes. Tests now verify the exact ref target, absence of remotes, all pinned producer lock versions, and observed Cairn state. Unsupported advisory commands were not used as evidence.

## Deterministic Rails

- Root and standalone-producer formatting checks passed.
- Four positive/negative interoperability tests passed; the existing three adapter unit tests and three format-compatibility tests also passed.
- `cargo test --workspace` passed, including both 73-test CLI binary suites and 391 core tests.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Proposal, design, and tasks gates plus strict Cairn validation passed.
- Accepted traceability passed with `1694/1694` requirement and scenario IDs referenced and no missing or dangling markers.
- Evidence export and release-readiness passed after accepted-spec sync.
- The dedicated `radicle-cob-format-compatibility` Nix check passed against the final fixture implementation.
- `self-dogfood` passed.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`; incompatible `aarch64-linux` checks were omitted.

## Bounded Claims

This proves compatibility only for the pinned upstream storage/format cohort, Cairn's declared custom type, and the exercised linear histories. Producer and consumer still share `radicle-cob` signature/change semantics, so the evidence is not an independent formal specification and does not cover arbitrary versions or COB types.

BLAKE3 provenance proves retained artifact identity, not that a third party independently attested generation. The two deterministic regeneration runs are bounded local observations. Public deterministic fixture keys provide no production authority.

No test publishes, discovers, seeds, announces, connects, fetches from a network, synchronizes peers, or proves convergence. Passing evidence does not prove remote trust, transport correctness, crash durability, source-code correctness, semantic merge correctness, or universal release eligibility.
