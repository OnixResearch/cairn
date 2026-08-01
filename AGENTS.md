# Agent Notes

## Release
- crates.io names `cairn`, `cairn-cli`, and `cairn-core` are already occupied by unrelated owners. For Rust registry publishing, use an Onix-prefixed package name or another owned namespace; keep binary name `cairn` if desired.
- `v0.1.0` is tagged at `2e6ca1f` with post-tag remote install smoke docs. Current `main` can be ahead of that tag, so generate fresh release evidence before tagging another package release.

## Cairn change completion
- Complete each Cairn change in a dedicated worktree on a unique new branch from current `origin/main`.
- Implement, validate, sync, archive, and commit the change only in that worktree.
- Push the verified change branch before you integrate it into `main`.
- Fetch `origin` again, fast-forward `main` to the verified branch, and push `main`.
- Verify that `origin/main` contains the completion commit before you remove the worktree.
- Do not open a pull request or force-push. Preserve unrelated work and report integration blockers.

## Verification
- Use `nix run path:$PWD#cairn -- policy export --check` for policy freshness. `nix develop -c cairn ...` does not put the packaged CLI on PATH and can collide with the top-level `cairn/` directory.
- `cairn init` creates empty lifecycle directories but does not materialize `.gitkeep` files; docs/examples should not list `.gitkeep` as expected output unless the command changes.
- When generated policy hash changes, refresh agent receipt fixture `replay_policy_hash`/`receipt_hash` values and plan goldens together; stale fixture hashes make `agent-receipt smoke-mcp` and `release-readiness` fail.
- For focused Cargo test replays, prefer `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test ...`; the shared `/home/brittonr/.cargo-target` plus sccache can emit stale missing-rlib or proc-macro feature noise unrelated to the patch.
