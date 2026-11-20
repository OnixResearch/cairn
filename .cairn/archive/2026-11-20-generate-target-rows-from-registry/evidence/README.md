# Evidence

Transcripts from the verification run.

- `gates/`: `cairn validate` and the proposal, design and tasks gates.
- `checks/assurance-targets-contracts.txt`: the flake check that re-exports every
  fixture, requires `registry-derived/targets.ncl` to equal
  `octet-example/targets.json` byte for byte, and requires every negative to fail.
- `checks/cargo-test-assurance-targets.txt`: the unchanged Rust admission tests.
- `checks/policy-export-check.txt`: generated policy freshness.
- `checks/traceability-coverage-pre-archive.txt`: coverage before sync/archive.

The Cairn commands ran the packaged CLI built from this worktree by
`nix run path:$PWD#cairn`:

`/nix/store/xbm6dqspcqz6idirz6hkjzwb66mzkdig-cairn-0.2.1/bin/cairn`

(`cairn 0.2.1`, built from the `q9v85lm34yjm42bfvbdasmznhfmlypic` derivation).
