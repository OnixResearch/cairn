# Bootstrap contract and core increment

Cairn maintainers own the bootstrap contracts and direct core tests.
This increment adds checked project-declaration, ownership, and upgrade shapes to the existing cohort and recovery contracts.
Project declarations restrict their locator vocabulary but grant no filesystem authority.
Cap Root admission and capability-relative observation remain shell obligations.

The first-pass core already owns admission, compatibility, ownership, exact plan identity, and recovery decisions.
This increment adds declaration admission and byte-identity accessors without I/O.
The complete-manifest field is an expected identity, not evidence of installation.

## Observed checks

The commands ran from the assigned worktree with this prefix:

```text
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop --option min-free 0 -c
```

- `cargo test -p cairn-core consumer_bootstrap`: 10 passed. The pre-edit baseline had 9 passing tests.
- `cargo test -p cairn-cli --test consumer_bootstrap_controls`: 2 passed, including the expanded Nickel/Rust fixture matrix.
- `cargo clippy -p cairn-core --all-targets -- -D warnings`: passed.
- `cargo fmt --all`: completed.

Accepted fixtures cover cohort, project declaration, ownership, upgrade, policy selection, and recovery shapes.
Rejected fixtures cover excessive bounds, parent escape, and malformed ownership identity.
Core tests cover unsupported cohorts and upgrades, invalid explicit policy, unmanaged or modified files, stale plans, interruption, and unknown durability.

This evidence completes the contract and pure-core tasks only. No installation shell, policy resolver, or doctor integration is claimed.
