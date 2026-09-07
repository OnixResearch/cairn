# Kiln vendor packaging repair

## Outcome and boundary

The strict helper now reaches `cargo-octet` and reports actual `cairn-core` catalog diagnostics.
Kiln vendoring no longer loads absent manifests from unrelated producer check workspaces.
The repair changes consumer packaging only. It does not change Rust sources, Cargo dependencies, producer revisions, lint scope, or lint severity.
The existing Mantle adapter remains in place.

Cairn maintainers own this adapter and its source-pin checks.
The immediate consumer is the package-scoped Octet diagnostic helper.
The durable capability is repeatable vendoring of the admitted producer crates without unrelated workspace members.
The same adapter serves the normal Octet workspace check and the source-pin check.

## Published source and admitted files

The existing `kiln-core` flake input already supplies the exact published source:

- Repository: `https://seed.radicle.garden/z2wsvXm5S2sJGvuV1k5JHiwi1PbKE.git`
- Revision: `5d99f4e9bb954743443e340baece31256b4c53f1`
- Nix NAR identity: `sha256-ULVaEH8tOiR2SoHVkpB5dB7ZKrvu99VxQ8KBLytP+RM=`
- Cargo package: `kiln-core`, version `0.1.0`

No new input or lockfile change was necessary.
The source comes from the immutable Nix input, not a sibling checkout.
The synthetic workspace contains `crates/kiln-core`, one generated root `Cargo.toml`, and the two original license files.
The generated manifest projects the producer's resolver, package inheritance, lint inheritance, and required `serde` dependency without changing their values.
Crane still resolves inheritance and uses Cargo's package-file admission.
The complete copied crate matches the published crate before vendoring. The final vendored Rust source also matches byte-for-byte.

`LICENSE-APACHE` and `LICENSE-MIT` retain their exact bytes in both the synthetic workspace and the final package.
There is no separate root or crate-local `NOTICE` file in this pinned source.
The inherited license remains `MIT OR Apache-2.0`. This repair does not change license terms or copy producer semantics into Cairn.
Nix source-admission SHA-256 values are separate from the BLAKE3 identities of these evidence logs.

## Admission controls

The adapter admits Kiln only for the exact URL, revision, NAR identity, package name, and package version.
It rejects a mismatched package at the admitted source and a mismatched source for the admitted package.
Unrelated Git dependencies retain the normal Crane path.

The check includes positive source and packaging cases plus eleven negative controls:

- Substituted revision, NAR identity, Cargo source, package name, or version.
- Empty or duplicated package sets.
- Substitution of either license digest.
- Missing license notices.
- Substitution of the package name in the producer manifest.

The build also compares source and license bytes and requires exactly one vendored Kiln package.
Existing producer revision checks and Mantle packaging controls still run.

The package-scoped helper previously passed `cargoLock` directly to the producer helper.
That route bypassed the consumer adapter.
It now uses the same admitted vendor directory exposed by the workspace check.
The producer command, deny-all hook, selected packages, all-targets flag, and all-features flag remain unchanged.

## Verification

These commands ran from the repair worktree:

```sh
nixfmt --check flake.nix nix/producer-cargo-vendor.nix nix/check-producer-source-pins.nix .cairn/changes/use-canonical-octet-integration/evidence/strict-check.nix
nix build path:$PWD#checks.x86_64-linux.check-producer-source-pins --no-link -L
nix build path:$PWD#checks.x86_64-linux.octet-integration --no-link -L
nix build --impure --file .cairn/changes/use-canonical-octet-integration/evidence/strict-check.nix --argstr root "git+file://$PWD" --arg packages '[ "cairn-core" ]' --no-link -L
```

Formatting and both named checks passed. The integration check reused its existing output after successful evaluation.
The source-pin check built the Kiln adaptation and the complete locked vendor directory.
Cargo manifests, Cargo.lock, and Rust source files did not change, so focused Cargo tests and Clippy were not required for this packaging repair.
`git diff --cached --check` also passed.

The first strict attempt used a `path:` root that included an unrelated 11.3 GiB `target/` directory.
That attempt stopped during source transfer and supplies no lint evidence.
The completed attempt used the documented `git+file://` root after staging the repair.
This source selection excludes ignored build artifacts. It does not narrow Cargo package or target selection.

## Exact remaining blocker

The completed strict attempt failed with exit code 1 at the Nix command boundary.
The producer reported `Status: integration-failure`, 8,142 errors, and zero warnings.
Rust reported 3,786 library errors and 4,356 library-test errors.
These counts overlap and do not describe 8,142 distinct defects.

The remaining errors are Cairn catalog findings, not producer vendoring failures.
`kiln-core-strict-summary.txt` retains the tool identity, source example, complete per-lint counts, and derivation identity.
The largest categories are owner-path repetition, assertion density, borrowed argument types, missing const functions, and non-trait imports.
No catalog repair or suppression is part of this change.
The strict workspace check, lifecycle completion, and release eligibility remain unproven.

## Retained evidence

- `kiln-source-pins.log`: BLAKE3 `fe5a83eb2247528a398b186c9bbd1f5d5753c3ddf71548566576d45ef06ea01c`. One trailing space was removed for the Git whitespace check.
- `kiln-octet-integration.log`: empty successful cached-build log, BLAKE3 `af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262`.
- Worktree-local `.pi/cairn-rest-1295/kiln-core-strict.log`: BLAKE3 `6b56b89a70989d94751f405bd98489fea8bb9aa261d601e1da1f493a3273dbca`.

The full strict log remains local because it contains 132,496 lines of overlapping diagnostics.
The tracked summary is an excerpt, not a replacement for that log.
No push, sync, archive, rebase, merge abort, reset, or agent spawn occurred.
