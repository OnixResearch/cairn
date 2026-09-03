# Validation evidence

## Baseline

The generated policy was fresh before implementation.

The focused artifact-workflow baseline passed:

- `cairn-core`: 7 tests passed
- `cairn-cli` workflow profiles: 5 tests passed

The repository development shell did not start. Its pinned Octet toolchain returned a `rust-src` fixed-output hash mismatch.

The baseline Cargo tests ran through explicit Nixpkgs Rust and linker packages with `RUSTC_WRAPPER` empty.

## Focused implementation checks

The focused suites passed after implementation:

- artifact workflow and outcome-machine core: 13 tests passed
- generated policy and policy rejection: 47 tests passed
- outcome-machine lifecycle: 4 tests passed
- existing workflow-profile compatibility: 5 tests passed
- agent completion fixtures: 5 tests passed
- lifecycle plan goldens: 1 focused test passed

The lifecycle suite covers these paths:

- exact profile scaffold
- delivery, foundation, and bounded-exception route admission
- hidden session state rejection
- repository completion authority
- planned state passing proposal, design, and tasks gates
- planned state blocking sync
- BLAKE3-bound completion evidence
- changed evidence blocking sync
- observed evidence allowing sync and archive
- existing profile compatibility

The generated policy freshness check passed. Cairn validation and the proposal, design, and tasks gates passed.

Focused Clippy passed with `-D warnings` for `cairn-core` and `cairn-cli` through `--no-deps` scope.

## Broad checks and blockers

The workspace test suite passed all tests through the evidence-registry test. Release readiness then failed because traceability reports the new active delta requirement IDs as dangling.

This state is expected before sync and archive. Cairn traceability scans accepted specifications, while implementation markers already reference the active delta requirements.

Remote-builder Slotcar and full Nix attempts stopped on the existing `rust-src` fixed-output hash mismatch:

```text
specified: sha256-q/gu/3mAuLgNfJlxV/Sw1jttbi4PIBjN+XH0bGmB5NQ=
got:       sha256-WTRv7eyiu+VOfb8+90cALNJrUa3uLwRFIaeEr+tAIjQ=
```

A local-only Slotcar run bypassed the transfer mismatch and exited successfully. It emitted 25,001 Octet findings across existing workspace and interoperability sources.

This warning-only result is not strict Octet acceptance evidence.

A local-only full Nix run reached the traceability check. It failed only because the active delta requirement IDs remain outside accepted specifications.

A full workspace Clippy attempt also found six `unnecessary_sort_by` findings in untouched `cairn-radicle-cob` files. Focused no-dependency Clippy passed for the changed crates.

## Lifecycle completion

Cairn validated the completed task set without issues.

The sync dry-run admitted all eight added requirements. The executed sync wrote accepted specification hash `b8f2612f536cbac65198f987e593fc797a050a43b9f6718560e635dfc3dba16a` with mutation manifest `6e1a295bf522735428c9ffed7396c1b4445d446006b5d465aa716fce783ab648`.

## Post-archive verification

The first post-archive trace run found one missing documentation marker. The workflow-profile documentation test already checked the required content. Adding its requirement marker made traceability pass with `2289/2289` references.

The next full Nix runs found two stale expected Preserves hashes for the changed positive agent receipt fixture. Cairn independently emitted `fa40e4628a96d8fd3f7b580b77d2fba90e1ec73d3cd5eeb76d230336299d3fa5`. Both focused deterministic hash tests passed after updating those bound expectations.

The final local-only `nix flake check path:$PWD -L --option builders ''` passed. The remote-builder `rust-src` transfer mismatch remains an environment-specific blocker for that route.

The archive mutation manifest is `39b7fa353fcd63bc1660479e50af03a569f40ce70e478238c5d61b8961f94124`.

## Claim boundary

These checks establish deterministic profile behavior and local compatibility for the tested paths. They do not prove that authored outcome claims or referenced evidence are true.
