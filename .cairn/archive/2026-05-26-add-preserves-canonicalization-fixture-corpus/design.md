# Design

## Corpus format

The corpus is a checked-in JSON array under `fixtures/preserves-canonicalization/corpus.json`. Each case contains:

- `id`: stable snake-case identifier.
- `description`: human review hint.
- `value`: the JSON value converted through `preserves::value::to_value`.
- `expected_hash`: BLAKE3 hex of `canonical_preserves_bytes(value)`.

Keeping expected hashes with the fixture makes review diffs explicit when Preserves or serde conversion behavior changes.

## Case selection

The initial corpus covers:

- primitive values (`null`, booleans, strings, integers),
- Unicode strings,
- arrays with mixed primitive values,
- nested objects with object-key ordering sensitivity handled canonically,
- receipt-shaped object material resembling Cairn evidence payloads.

The corpus is intentionally small so it is cheap to run as part of focused core tests and broad workspace tests.

## Implementation boundary

Implementation stays in `cairn-core` test code around the existing `canonical_preserves_hash` helper. The helper API remains unchanged and public JSON receipt schemas remain unchanged.

## Verification

Focused verification runs `cargo test -p cairn-core preserves -- --nocapture`, then repository gates (`fmt`, `validate`, Tracey, clippy, workspace tests) before sync/archive.
