## Tasks

- [x] [serial] Define pure sync/archive dry-run plan types in `cairn-core/src/verified/`.
- [x] [serial] Implement deterministic plan builders from existing `ChangeStatus`/policy inputs.
- [x] [serial] Render stable plan JSON in `cairn-cli` without adding mutation side effects.
- [x] [parallel] Add positive fixture-backed CLI tests for sync/archive dry-run JSON.
- [x] [parallel] Add negative tests for blocked plans and missing change evidence.
- [x] [serial] Verify with `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo tigerstyle check`, and `nix flake check`.
