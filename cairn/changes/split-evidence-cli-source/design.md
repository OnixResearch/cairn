## Design

### Boundary

This is a behavior-preserving source/test organization change. The split MUST move cohesive code from `crates/cairn-cli/src/evidence.rs` (~2,039 lines) into focused modules without changing public command names, flags, output fields, issue codes, mutation behavior, fixture contents, deterministic hashes, or Tracey coverage semantics.

### Proposed module shape

Use `evidence/mod.rs` as dispatch/facade plus focused `export.rs`, `verify.rs`, `import.rs`, `republish.rs`, `index.rs`, and shared `common.rs` helpers.

The final implementation MAY adjust exact file names if dependency inspection shows a smaller or safer seam, but the result MUST keep a narrow facade/dispatch layer and avoid broad crate-root re-export shims.

### Verification strategy

1. Run the smallest focused target for evidence export, verify, import, republish, and index command families after the extraction compiles.
2. Run the relevant full integration target(s) that previously covered the moved behavior.
3. Run repository-level checks: `cargo fmt --check`, `cargo check -p cairn-cli` or affected package check, `cargo clippy -p cairn-cli --all-targets -- -D warnings` when CLI code changes, `cairn validate --root .`, `cairn tracey coverage --root .`, and `git diff --check`.
4. If new files are added, stage them before flake-backed checks so Nix sees the same source tree that will be committed.

### Non-goals

- No public behavior changes.
- No fixture/golden updates except those strictly required by Rust module/test-target naming, and any such update must be justified in the implementation notes.
- No broad cleanup outside the named target family.
