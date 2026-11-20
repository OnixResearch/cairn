# Verification record

Worktree `/home/brittonr/wt/cairn-mcp-convergence`, branch
`add-convergence-inspect-mcp-tool`, base `231d61e`. All Rust commands ran inside
`env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix --option min-free 0 develop -c`.

| Command | Exit | Result |
| --- | --- | --- |
| `cargo fmt --check` | 0 | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | no warnings |
| `cargo check -p cairn-cli --all-targets` | 0 | compiles |
| `cargo test -p cairn-core --lib convergence` | 0 | 49 passed, 0 failed, 867 filtered out |
| `target/debug/cairn policy export --check` | 0 | `policy fresh: cairn-policy/generated/cairn-policy.json` |
| `target/debug/cairn validate --root .` | 0 | see `../gates/validate.json` |
| `cairn gate proposal add-convergence-inspect-mcp-tool --root .` | 0 | PASS |
| `cairn gate design add-convergence-inspect-mcp-tool --root .` | 0 | PASS |
| `cairn gate tasks add-convergence-inspect-mcp-tool --root .` | 0 | PASS |
| `cairn sync add-convergence-inspect-mcp-tool --root . --execute` | 0 | `blocked=false`, `mutated=true`; delta spec accepted |
| `cairn traceability coverage --root .` | 0 | `2491/2491 referenced (profile cairn-default)` |

`cairn-policy/generated/mcp-discovery.json` was regenerated the way the flake
does it: `nickel export --format json cairn-policy/mcp-discovery.ncl`. The
result adds `cairn_convergence_inspect` to the four profile listings and changes
no other entry.

Gate and validation receipts are under `../gates/`. `cargo test -p cairn-cli`
and the full `nix flake check` results are recorded below when they finish.
