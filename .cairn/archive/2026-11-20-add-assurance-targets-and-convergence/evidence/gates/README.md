# WP0 lifecycle gates

Packaged Cairn CLI built with
`nix --option min-free 0 build 'git+file:///home/brittonr/git/OnixResearch/cairn?rev=965591480848b144bd7e4e7288d3c17675f2ea31#cairn'`
(store path `/nix/store/n2xkxzcmdkip84w75pavlby0jzw25js6-cairn-0.2.1`, `cairn 0.2.1`).
`--option min-free 0` only skips the client auto-GC wait on this host; it does not change the build.

Run from the change worktree root on 2026-11-19:

| Command | Exit | Result | Transcript |
| --- | --- | --- | --- |
| `cairn gate proposal add-assurance-targets-and-convergence --root .` | 0 | `PASS` | `proposal.json` |
| `cairn gate design add-assurance-targets-and-convergence --root .` | 0 | `PASS` | `design.json` |
| `cairn gate tasks add-assurance-targets-and-convergence --root .` | 0 | `PASS` | `tasks.json` |
| `cairn validate --root .` | 0 | `valid: true` | `validate.json` |

The gates check lifecycle structure and policy conformance only.
