## Design

### Intent

This change is a behavior-preserving source-organization slice. It should make future release-evidence work easier to review without changing the operator-facing contract for:

- `cairn release-evidence index`
- `cairn release-evidence publish`
- `cairn release-evidence retention-plan`
- `cairn release-evidence rotate`

### Module boundary

Use a facade layout under `crates/cairn-cli/src/release_evidence/` or an equivalent focused module tree:

- `mod.rs` or `release_evidence.rs`: public dispatcher and minimal shared imports.
- `index`: current-HEAD evidence index orchestration/rendering.
- `publish`: local publication materialization and publish receipt helpers.
- `retention`: retention-plan inventory and plan receipt helpers.
- `rotate`: rotation plan verification/execution and mutation receipt helpers.
- `common`: shared issue, safe-path, JSON/hash, and directory-hash helpers that are needed by more than one family.

The exact file names can differ if implementation ergonomics require it, but each moved family should have one obvious home.

### Bridge and visibility rules

- Keep `run_release_evidence(args)` as the dispatcher entry consumed by `main.rs`.
- Expose only narrow `pub(super)` / `pub(crate)` functions needed across sibling modules.
- Avoid broad crate-root re-exports or moving unrelated support helpers.
- Prefer direct sibling imports over a large shared prelude.
- Keep filesystem/process execution in the CLI crate; do not move release-evidence orchestration into `cairn-core`.

### Behavior preservation

- Existing receipt shapes, policy-hash material, issue codes, label validation, path safety, and command usage should remain stable.
- Existing tests should continue to pass without golden or fixture drift.
- If a bug is found during the split, fix it only when it is necessary for the split to remain sound and add/retain focused tests that make the behavior explicit.

### Verification strategy

Run focused release-evidence tests before broader checks:

1. `nix develop -c cargo test -p cairn-cli release_evidence -- --nocapture`
2. `nix develop -c cargo test --workspace`
3. `nix develop -c cargo clippy --workspace --all-targets -- -D warnings`
4. `nix develop -c cargo fmt --check`
5. `nix develop -c cargo run -p cairn-cli --bin cairn -- validate --root .`
6. `nix develop -c cargo run -p cairn-cli --bin cairn -- tracey coverage --root .`
7. `git diff --check`

### Non-goals

- No new release-evidence command.
- No policy schema change unless implementation proves an existing schema inconsistency.
- No remote publication, credential, signing, tag, mirror-trust, or public release-page claim changes.
- No broad cleanup of other CLI source hotspots.
