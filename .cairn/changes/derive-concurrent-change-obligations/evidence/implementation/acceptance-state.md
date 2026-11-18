# Concurrent change footprints acceptance state

Status: **authored source only; no check was executed in this worktree.**

## Authored surface

- `crates/cairn-core/src/verified/concurrent_change_footprints.rs`: versioned
  footprint, derivation, obligations, outcomes, explanations, invalidation,
  bounded finite evaluation.
- `crates/cairn-core/src/verified/concurrent_change_footprints/tests.rs`:
  positive and negative core cases.
- `crates/cairn-cli/src/concurrent_change_footprints.rs`: read-only inspection.
- `crates/cairn-cli/tests/footprints.rs`: positive and negative CLI cases.
- `cairn-policy/concurrent-change-footprints.ncl`: policy source.
- `crates/cairn-core/src/verified/mod.rs`, `crates/cairn-core/src/lib.rs`,
  `crates/cairn-cli/src/main.rs`: module registration and command routing.
- `crates/cairn-cli/src/support/usage.rs`: command usage line.

## Not performed here

No build, test, formatter, Clippy, lifecycle validation, traceability, policy
export or Nix command was run. The parent regenerates the policy projection,
executes the focused and workspace checks, and retains current-source evidence
before any completion claim.

## Parent commands

```console
nix develop -c nickel export --format json \
  cairn-policy/concurrent-change-footprints.ncl \
  | jq -S . > cairn-policy/generated/concurrent-change-footprints.json
cargo test -p cairn-core --lib concurrent_change_footprints
cargo test -p cairn-cli --test footprints
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
nix run path:$PWD#cairn -- policy export --check
nix run path:$PWD#cairn -- validate --root .
nix flake check path:$PWD -L
```

## Scope reminder

The analysis covers only declared footprints over one finite semantics profile.
It does not prove behavioral independence, merge permission, implementation
correctness, readiness or release eligibility, and it does not discharge or
refresh any native obligation, receipt or lifecycle decision.
