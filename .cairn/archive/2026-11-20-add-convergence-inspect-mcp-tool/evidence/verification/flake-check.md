# Full flake check

Command:

```console
nix --option min-free 0 --option builders '' flake check path:$PWD -L --keep-going
```

Ran once on the committed tree at `6083eec` (implementation, merge with
`origin/main` `f20bcb4`, archive and test transcript), before the second merge
with `origin/main` `70154ef`.

- Exit status: 0
- Last lines: `all checks passed!`, `warning: The check omitted these
  incompatible systems: aarch64-linux`
- Full transcript: 32,894,147 bytes, BLAKE3
  `f39747bad8d406a91b32033a5101d13a6e5da0823bc339451e132c13eba43c10`
  (`/tmp/flake-check2.log` on britton-desktop; too large to commit)

A first attempt failed during evaluation because a file was copied into the
worktree while Nix was hashing the source
(`store path ... contents have changed ... Make sure files do not change during
evaluation`). The file was committed, the tree was frozen, and the check above
is the clean rerun.

Per the orchestrator, the final `origin/main` merge (`70154ef`) is validated
with the focused `convergence`, `convergence_pilot` and MCP suites plus
`policy export --check`, `validate` and `traceability coverage`; the orchestrator
runs the final-tree flake check.
