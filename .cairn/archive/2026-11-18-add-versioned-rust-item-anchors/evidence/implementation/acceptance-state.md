# Rust item anchor acceptance state

Status: **implemented and committed; one traceability self-reference to clear.**

## Verified

- `cargo build -p cairn-cli --bin cairn` succeeds after regenerating the main
  policy from `cairn-policy/default.ncl` (the change adds the `rust_item_v1`
  traceability profile through `contracts.ncl`).
- `cairn validate --root .` reports 0 issues.
- A completion canary run on `fixtures/basic` plus the agent-completion fixture
  reports `completion: ready`, so the regenerated policy does not stale the
  completion fixture in the way an added top-level policy key would.

`cairn policy export --check` could not be exercised directly: the packaged
binary returns `error: No such file or directory (os error 2)` for that
subcommand, which also happened on an unrelated branch and needs its own
diagnosis. Freshness is therefore asserted only by construction (the generated
file was produced from the Nickel source in this tree), not by the checker.

## Outstanding

`cairn traceability coverage --root .` reports one dangling requirement:
`demo.item`. It comes from Rust **test fixtures** in
`crates/cairn-cli/src/trace_index/rust_items.rs` — ten occurrences of the literal
marker `r[impl demo.item]` inside fixture strings. The scanner parses source
bytes, so those fixtures register as live claims for a requirement that is
correctly undeclared.

This is the same class of defect previously found and fixed in
`selected_work/tests.rs`. The fix is to stop the exact marker sequence appearing
in the source, for example by assembling it with `concat!("r[im", "pl demo.item]")`
and using that constant in the fixture strings and in the `match_indices` /
`find` calls. The traceability policy has no path-exclusion facility
(`traceability_policy` exposes only `default_profile` and `profiles`), so the
fixture source itself must change.

No archive, sync or completion claim is made here.
