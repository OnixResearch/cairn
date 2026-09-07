# Bootstrap dependency admission

Cairn maintainers own this consumer boundary and its conformance tests.
The current consumer is the disposable Cairn bootstrap fixture.
The target outcome is bounded, ownership-aware project installation.
The shell must retain installation policy, locks, transactions, recovery, and receipt meaning.

## Published sources

| Package | Published Git endpoint | Reviewed revision | Notices |
|---|---|---|---|
| `cap-root-core` | `https://seed.radicle.garden/z3cWBCK7VPbEmbL5MAvKJWKjCE745.git` | `8991e2572578c16e590c2f1c0915db6c46c5d9cb` | `LICENSE`, AGPL-3.0-or-later |
| `durable-file-publication` | `https://seed.radicle.garden/z3tAR4For7qw8ZirkJzoDw1VNDDLM.git` | `951c27f59003cea9bfdb40ed4d89653d50fada1f` | `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, and referenced provenance |
| `nickel-export-core` | `https://github.com/OnixResearch/nickel-export` | `257fafc1c746f1faf156207043a4c826bfb16d49` | Existing unchanged dependency and notices |

The Radicle RIDs are the repository names in the two seed URLs, without `.git`.
The seed is a transport endpoint. It does not grant source authority or indefinite availability.
Both sources were acquired from the published endpoint and checked out at their exact reviewed revisions.
No producer implementation was copied or changed. No ambient sibling path enters the dependency graph.
Cargo and Nix retain the complete source notices. The packaged CLI also installs the licenses, publication notice, and referenced provenance under `share/licenses/cairn/`.

The owner authorized these exact component dependencies for this change.
This authorization does not change public licenses or grant rights to unrelated recipients.
The CLI keeps its existing AGPL-3.0-or-later license. The functional core does not import Cap Root or the publication shell.

## Contracts

Cap Root owns lexical relative-path and leaf admission through `RootRelativePath` and `AdmissionMode`.
It does not open roots, reject actual symlink substitution, or authorize I/O.
Cairn must supply those operations in its shell.

Durable File Publication owns one-file mechanics through `publish_one_file`, `PublicationIo`, and `StageNameSource`.
Its Linux adapter takes an already-opened directory handle.
Cairn must keep `CommittedDurabilityUnknown` separate from durable completion.
A failed stage cleanup requires explicit consumer handling.
The producer does not supply project locks, installation manifests, recovery, or multi-file atomicity.

Nickel Export retains its existing pin and adapter in `crates/cairn-cli/src/nickel_export_boundary.rs`.
Normal lifecycle commands must consume checked JSON without evaluator fallback.

## Repeatability checks

From the Cairn checkout, run:

```text
nix build path:$PWD#checks.x86_64-linux.bootstrap-source-pin --no-link -L
nix build path:$PWD#packages.x86_64-linux.cairn.cargoDeps --no-link -L
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop --option min-free 0 -c cargo test -p cairn-cli --test bootstrap_publication_contract --test consumer_bootstrap_controls
```

`nix/bootstrap-source-pin.nix` binds Cargo declarations, lock entries, Nix inputs, exact revisions, NAR identities, and Cargo source hashes.
Its negative cases reject revision mismatch, floating references, sibling paths, missing packages, and content-hash mismatch.
The existing `nickel-export-cutover` gate retains the export boundary checks.

The consumer tests cover accepted and rejected lexical paths, synchronized publication, no-replace conflicts, symlink targets, interrupted writes, failed cleanup, and unknown durability.
These tests establish mechanism conformance only. They do not establish installed-consumer setup, recovery success, source correctness, or release eligibility.
