# Post-merge validation

Merged `origin/main` `70154ef` (owner-scoped producer-job schemas) into the
change branch at `b79bf76`. The accepted convergence spec keeps both
`cairn.convergence.inputs` (their MODIFIED requirement) and
`cairn.convergence.mcp_inspect` (this change). No conflict remained.

Commands on the merged tree, inside
`env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix --option min-free 0 develop -c`:

| Command | Exit | Result |
| --- | --- | --- |
| `cargo build --bin cairn` | 0 | merged binary |
| `cairn policy export --check` | 0 | `policy fresh: cairn-policy/generated/cairn-policy.json` |
| `cargo test -p cairn-cli --test convergence --test convergence_pilot --test mcp_cli` | 0 | 25 + 2 + 30 passed, 0 failed |
| `cairn validate --root .` | 0 | valid |
| `cairn traceability coverage --root .` | 0 | `2491/2491 referenced (profile cairn-default)` |

The flake check recorded in `flake-check.md` ran before this merge; the
orchestrator runs the final-tree check.
