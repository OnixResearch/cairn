# Refuse a malformed consumer-selected release gate list

## Why

`release-readiness` lets a consuming repository replace Cairn's maintained gate
list through `release_readiness_policy.required_external_commands` in its
admitted policy. When that list was present but malformed (not an array, empty,
or holding a non-string entry), Cairn silently reported the maintained-local
gates instead. A consumer that mistyped its policy therefore received a passing
receipt that listed gates it never selected, with nothing in the receipt saying
its own selection had been ignored. Duplicate and blank entries were accepted
as written.

Lattice pins this selection path, and Kiln and Lattice need to align on one
Cairn `main` revision, so the selection contract should be explicit before
consumers move to it.

## What Changes

- An absent `release_readiness_policy.required_external_commands` keeps the
  maintained-local gates, as before.
- A present list must be a non-empty array of unique, non-blank strings, and
  `release_readiness_policy` must be an object. Anything else reports
  `repository_gate_summary.policy` as `invalid-consumer-selected-gates`, a null
  `required_commands`, and an `issue` diagnostic naming the field and entry
  index. The `repository_gate_policy_listed` check fails with the same issue,
  and the receipt verdict is `fail`.
- Negative CLI tests cover the wrong type (string and null), a non-object
  `release_readiness_policy`, an empty list, a duplicate entry, a non-string
  entry and a blank entry. A positive test covers a `release_readiness_policy`
  without the list.
- `docs/cairn-mcp.md` and `CHANGELOG.md` describe the selection contract.

## Impact

- **Files**: `crates/cairn-cli/src/tracey.rs`,
  `crates/cairn-cli/src/release_readiness.rs`,
  `crates/cairn-cli/tests/release_receipts/release_readiness.rs`,
  `docs/cairn-mcp.md`, `CHANGELOG.md`.
- **Compatibility**: receipts for policies without the list and for well-formed
  lists are byte-identical. No in-repository policy sets the list. Lattice's
  authored list (three unique, non-blank strings) and ChaosControl's (four) are
  accepted. Lattice's Nickel contract already refuses every malformed form;
  ChaosControl's refuses empty lists and empty or non-string entries but not
  whitespace-only or duplicate entries. Kiln's policy does not set the list and
  keeps the maintained-local gates.
- **Testing**: `cargo test --workspace`, Clippy, `policy export --check`,
  `cairn validate`, the change gates and `nix flake check`.

## Out of Scope

- A closed vocabulary of command names. Required commands are free-form
  operator command lines, so Cairn has no command set to check against.
- Executing the listed commands. The receipt still records the list only.
- Typing `release_readiness_policy` in Cairn's own Nickel policy contract.
