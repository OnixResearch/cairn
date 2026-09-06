# Retrospective evidence for 0ea6ac9

Commit `0ea6ac9` (`Preserve native producer meaning across saved JSON imports`) lacked its final report when the prior pass timed out.
This report records a fresh replay before the mandatory round-three merge, not an inferred prior result.

## Observed replay

Private Pueue task 3287, group `finish3-evidence`, exited 0 on the unchanged commit.
The command ran in the Nix development shell with `RUSTC_WRAPPER=`, `CARGO_TARGET_DIR=target`, and `CARGO_INCREMENTAL=0`:

- `cargo test -p cairn-core check_evidence`: 26 passed.
- `cargo test -p cairn-cli --test check_evidence_stack`: 3 passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: exit 0.
- `cargo fmt --all -- --check`: exit 0.

Log: `/home/brittonr/git/OnixResearch/cairn/.pi/cairn-rest-1295/finish3-2746-evidence-prior.log`.

## Delivered surface

The commit adds Lattice, Kiln, and Valence admission, original Lattice/Mantle fixture intake, producer-source identity checking, and the Valence fixture license notice.
Core controls reject identity, ordering, native-state, claim, and applicability substitutions. Shell tests retain native success/cache/failure meaning without promoting metadata to executed tests.
All imported adapters remain completion-ineligible. Kiln calls its pinned original validator; Lattice and Valence use bounded consumer translations.
The commit also contains the producer-specific bounded policy reader and Mantle restricted package-discovery adapter. These exist in source; the focused replay above does not establish their full acceptance.

## Task state and remaining evidence

No checkbox changes in this report. Three tasks remain checked and seven open.
The full workspace suite and Nix battery still need fresh completion evidence. Mantle source-pin packaging must be built, not merely inspected.
Kiln producer-owned export intake and pinned completion-consumer replay remain distinct from the Cairn-authored workflow fixture.
Valence export freshness needs a repeatable exact-source check. Positive `cairn-local` loop replay still needs admitted fmt, validation, traceability, and duration facts.
The coordinator owns sync/archive. No sync, archive, push, or producer pin change occurred.
