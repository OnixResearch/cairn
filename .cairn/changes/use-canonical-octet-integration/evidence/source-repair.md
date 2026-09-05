# Gate-contract source repair

## Outcome and scope

The `cairn-gate-contract` crate passes the pinned Octet deny-all hook across all targets and all features.
The producer reports `Status: clean`, `Findings: 0`, `Warnings: 0`, and `Errors: 0`.
This result covers the library, unit tests, integration tests, and fixture generator.
It does not complete the workspace check or the utility-change drain.

The source repair separates bounded scalar admission from wire records.
Concrete owner paths replace private imports.
The private identity module becomes `binding`, but the root function names remain unchanged.
The producer fixture keeps its exact bytes and identities.
No Cargo lock, flake lock, producer pin, archived receipt, or recorded producer fixture changed.

Cairn maintainers own this repair and its repeatable verification helper.
The current consumer is the utility-change drain.
The helper calls the published producer hook rather than copying its lint catalog.
It is diagnostic evidence, not a replacement for the normal workspace completion gate.

## Producer and source exceptions

The producer remains `OnixResearch/octet` at `2f8213a88781d9d38bb81319d06a50805d48ccd3`.
The check uses `nightly-2026-03-21-x86_64-unknown-linux-gnu`.
Some catalog lints default to `Allow`. Thus, `-D warnings` alone is not the full deny-all gate.
The helper runs the pinned `hooks/octet-deny-all.sh` with the installed pinned runner.

Narrow source exceptions preserve closed-enum decisions and operations that cannot run in const evaluation.
Each exception names the Cairn gate-contract owner and its reason.
`const-probe.rs` compiles without `const_candidate` and fails with it on the repository development compiler.
The negative probe reports `E0379` for the non-const trait method and `E0015` for string comparison, prefix matching, and iteration.
`const-probe-toolchain.txt` and `const-probe-negative.log` retain these observations.
The BLAKE3 frame helpers call the non-const `blake3::Hasher::update` API.
No blanket exception, disabled lint, or finding budget is present.

## Checks

- Before edits, nine unit tests and one producer-fixture test passed.
- After edits, eleven unit tests and six integration tests passed.
- Positive cases cover canonical values, exact producer values, and all closed enum spellings.
- Negative cases cover malformed values, byte limits, unknown fields, unknown enum values, identity drift, and contradictory verdicts.
- Clippy passed for the package, all targets, and all features with `-D warnings`.
- Formatting passed for the package.
- The fixture generator output matched the checked-in producer fixture with `cmp`.
- The official deny-all hook passed for the package, all targets, and all features.
- The unchanged `cairn-core` test baseline passed 561 tests.

The read-only subagent review exceeded its time limit and supplied no acceptance evidence.
The current diagnostic and review passes are correlated.

## Remaining workspace blocker

The full workspace hook now reaches `cairn-core`.
It fails with 4,506 library diagnostics and 5,157 library-test diagnostics.
These counts overlap. They do not represent 9,663 distinct defects or complete coverage of downstream crates.
The failures include file decomposition, owner paths, assertions, and further catalog checks.
This repair does not suppress those findings or mark workspace verification complete.

The original warning-only full flake result does not establish strict acceptance.
Broad Rust, Clippy, and formatting checks also need a fresh replay after the source repair.
The earlier local store-integrity and all-system Nickel observations remain separate from this scoped result.
No archive, push, main integration, or worktree removal occurred for this repair.

## Reproduction

From the change worktree, run the focused source checks:

```sh
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test -p cairn-gate-contract --all-targets --all-features
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo clippy -p cairn-gate-contract --all-targets --all-features -- -D warnings
```

Run the saved helper for the package:

```sh
nix build --option min-free 0 --impure \
  -f .cairn/changes/use-canonical-octet-integration/evidence/strict-check.nix \
  --argstr root "git+file://$PWD" \
  --arg packages '[ "cairn-gate-contract" ]' --no-link -L
```

Omit `--arg packages` for the full workspace replay.
The `min-free` option avoids automatic collection waits. It does not change source validation, hashes, pins, or lints.

## Retained evidence

The local log directory is the primary checkout's `.pi/cairn-drain-resume-20260905/`.
These paths locate local evidence. They are not product dependencies.

| Log | BLAKE3 |
|---|---|
| `gate-contract-before.log` | `0f78880af82ca8a4b815a8929564bc3543a2e40d89bd70a9c88e2e274e169b08` |
| `gate-contract-source-final.log` | `56e3f088f4f49dc6b7e2fcc631cb3c98d1453fc4907ccb19504e7dffbf34f590` |
| `gate-contract-deny-all-replay.log` | `c75fbfa07de647407205c394f16feb8ee627626eb88ee08ff8eabd7ed554f734` |
| `workspace-deny-all-replay.log` | `59c453bda7002b628cfd4d8a204369c0decec49dc797061c0525e455b6222a22` |
| `core-before.log` | `f3e034f7cd9ac28bbd623d95fb6d5acedff342b350366255f52ae05a1e1415c2` |
