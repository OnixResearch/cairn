# Local Radicle COB Git store adoption

Cairn is the current consumer of the product-neutral `onix-radicle-cob-git-store` crate, owned by the OnixResearch Radicle COB Git store maintainers. It supplies signed Radicle COB storage and explicit signer loading over one local Git repository. Cairn retains reducers, domain types, mutation locks, actor authorization, high-level CRUD, receipt semantics, and lifecycle policy. Nothing in this import supplies remote transport, Radicle node operation, key trust, distributed consistency, or release eligibility.

## Immutable source and local import

- Independent source Git commit: `af38dae7e0454b1206512794420a3076ea5a8d1f`.
- Full `git archive --format=tar` BLAKE3: `ff1392cab83cb76123c56b0a70ab50fa0893788f5a876e4daf614003905a38e0`.
- Cargo package: `onix-radicle-cob-git-store` at `vendor/radicle-cob-git-store/`, with original `LICENSE-MIT` and `LICENSE-APACHE` license texts.
- Source import script: `tools/import-radicle-cob-git-store.sh`.

From the Cairn checkout, with a local Git repository containing that exact source object:

```sh
tools/import-radicle-cob-git-store.sh /explicit/path/to/radicle-cob-git-store
```

The script verifies the complete immutable archive before extracting the package, stages on the same filesystem as the vendor destination, refuses to replace any modified vendor snapshot, and accepts an identical snapshot without rewriting it. The supplied path is an *operator import input*, never a Cargo/Nix sibling-path dependency. The archive includes Git archive metadata; a source-directory-only tar has a different digest. No public remote, RID, network fetch, or source availability is asserted. Cargo and Nix builds consume only the tracked vendor package and Cairn's lockfile; an operator needs access to the source Git object only to reproduce the import.

## Parity and maintenance

The source change retained the same Git refs and signed change format. Cairn tests cover signed update/readback, malformed refs, authority and interoperability with an independently generated fixture. The generic repository's tests additionally cover malformed signatures, stale ref updates, and signer symlinks. The import script's no-op and refuse-on-drift behaviors can be repeated from a different working directory. Replacing this pin requires a new exact source revision, archive BLAKE3, reviewed package and license import, updated Cairn lockfile, and refreshed behavior/gate evidence; never overwrite local vendor edits in place.
