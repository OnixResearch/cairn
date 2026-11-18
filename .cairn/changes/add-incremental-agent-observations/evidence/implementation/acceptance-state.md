# Incremental observation acceptance state

Status: **one behavioural failure outstanding; not archived.**

## Defects found and fixed during integration

1. `flate.nix` conflict between the incremental and targeted policy-fresh
   derivations left unresolved conflict markers in the committed file, so the
   flake failed to parse (`syntax error, unexpected '<', expecting 'inherit'`).
   Both derivations are now present and complete.
2. `usage.rs` lost measured-benchmark usage text during the same merges; restored.
3. `ensure_complete` returned `&ObservationSnapshot` borrowed from either
   parameter without a lifetime; bound to `snapshot`.
4. A core test bound `let unchanged = unchanged(...)`, shadowing the helper and
   failing with `E0618`; renamed to `comparison`.
5. **Real CLI defect**: `cairn observe` stored every option as a JSON string, so
   `--max-bytes`, `--max-entries`, `--page-entries` and `--page-bytes` always
   failed with "must be a non-negative integer". Numeric targets are now parsed
   into JSON numbers.
6. Three pagination loops re-specified `--page-entries` together with `--cursor`,
   which the implementation correctly rejects ("a continuation cursor owns the
   page entry and byte limits"). The loops now send limits only on the first
   request.

After these fixes: `cargo build -p cairn-cli --bin cairn` succeeds and
`cargo test -p cairn-core --lib incremental` is **14/14**. The CLI suite is
**9/10**.

## Outstanding failure

`crates/cairn-cli/tests/incremental_observations.rs:423`
(`context_entries_are_exact_and_stable_across_content_edits`) expects
`diff["changed_count"] == 1` but receives `2` after editing the `demo.a`
requirement block in a two-selector context observation (`demo.a`, `demo.b`).

`changed_count` is computed as `changed.len()`, so the difference genuinely
classifies both entries as changed. The likely cause is that a context entry's
content identity includes whole-document state rather than the selected block and
its span, which would make every selector change when any block in the file
changes. That contradicts the change's own requirement that context entries be
exact and stable across content edits.

This must be diagnosed in `difference`/entry identity before archive: either the
entry content is too coarse, or the test expectation is wrong. No archive, sync
or completion claim is made here.
