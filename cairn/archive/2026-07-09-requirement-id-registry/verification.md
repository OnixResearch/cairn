# Verification

## Baseline

Transcript: `/tmp/onix-requirement-registry-baseline-20260709.txt`

- Proposal, design, and tasks gates passed before implementation.
- `nix run path:$PWD#cairn -- validate --root .` passed.
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-req-registry nix develop -c cargo test -p cairn-core requirement_id_registry -- --nocapture` passed with `0` matching pre-change tests.

## Focused implementation checks

Transcript: `/tmp/onix-requirement-registry-focused-20260709.txt`

- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-req-registry nix develop -c cargo fmt`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-req-registry nix develop -c cargo fmt --check`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-req-registry nix develop -c cargo test -p cairn-core requirement_id_registry -- --nocapture`
- `nix run path:$PWD#cairn -- requirement-registry verify --input fixtures/requirement-id-registry/positive/active-alias.json`
- `nix run path:$PWD#cairn -- requirement-registry verify --input fixtures/requirement-id-registry/negative/unknown-id.json`

Results: formatting passed; focused core tests passed (`5` tests); positive CLI fixture passed; negative CLI fixture failed closed with `requirement_registry.unknown_requirement`.

## Fixture matrix

Transcript: `/tmp/onix-requirement-registry-fixture-matrix-20260709.txt`

All positive registry fixtures passed and all negative fixtures failed closed:

- positive: `owned-id`, `active-alias`
- negative: `unknown-id`, `wrong-owner`, `stale-alias`, `namespace-overreach`

## Archive and final validation

Transcripts:

- `/tmp/onix-requirement-registry-gate-sync-archive-20260709.txt`
- `/tmp/onix-requirement-registry-post-archive-repair-20260709.txt`
- `/tmp/onix-requirement-registry-final-validation-20260709.txt`

Results:

- Proposal, design, and tasks gates passed.
- Cairn validation passed before sync, after sync, after archive, after accepted-spec repair, and in final validation.
- Sync initially created an empty accepted spec wrapper; `cairn/specs/requirement-id-registry/spec.md` was repaired from the archived spec delta.
- Final `cargo fmt --check`, focused core tests, scoped clippy for `cairn-core`/`cairn-cli`, full registry fixture matrix, Cairn validation, and traceability coverage all passed.
- Scoped clippy first exposed a pre-existing `manual_contains` lint in policy schema compatibility; the final validation includes the repair.

## Non-claims

This proves registry shape, owner/reference linkage, alias status behavior, consumer admission, deterministic receipts, and fixture coverage. It does not prove requirement satisfaction, implementation correctness, test coverage completeness, or release eligibility.
