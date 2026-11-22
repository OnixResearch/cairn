# Consumer survey (2026-11-22)

Searched for `release_readiness_policy` and `required_external_commands`,
excluding `target/` and `.git/`.

## Cairn (this repository)

- No policy source or generated policy sets `release_readiness_policy`.
  `cairn-policy/generated/cairn-policy.json` keeps the maintained-local gates.
- The only other hits are recorded release-readiness receipts
  (`.cairn/archive/2026-08-01-add-cli-machine-contract/post-archive-release-readiness.json`
  and `fixtures/function-address-stack-smoke/positive/cairn-readiness.json`),
  whose `required_external_commands` is the receipt's copy of the maintained
  list, not a policy selection. Their bytes are unchanged.

## Lattice (`OnixResearch/lattice`, pins Cairn `47ea1b86`)

`cairn-policy/default.ncl` and `cairn-policy/generated/cairn-policy.json`
declare:

```
release_readiness_policy = {
  required_external_commands = [
    "scripts/check.rs",
    "octet-precommit-check",
    "nix flake check -L --option secret-key-files ''",
  ],
  mcp_completion_contract = "target/release-readiness/mcp-contract.json",
  mcp_completion_results = "target/release-readiness/mcp-results.json",
}
```

The list is a non-empty array of three unique, non-blank strings, so it is
accepted and reported as `consumer-selected-gates`. Lattice's own
`cairn-policy/release-selection-contracts.ncl` already refuses an empty list,
blank or whitespace-only entries, non-string entries and duplicates at export
time. The test `release_readiness_uses_consumer_selected_gates` replays this
exact selection. Lattice's `docs/release-readiness-integration.md` still says
the producer falls back to its maintained list when the field is malformed;
that sentence becomes stale once Lattice moves to a revision with this change.

## Kiln (`kiln-int2` clone at `9173f49b`)

No policy sets `release_readiness_policy`. Every hit is under `.cairn/` and is a
recorded release-readiness receipt that lists the maintained-local gates. Kiln
keeps the maintained-local gates.
