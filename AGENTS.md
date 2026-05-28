# Agent Notes

## Release
- crates.io names `cairn`, `cairn-cli`, and `cairn-core` are already occupied by unrelated owners. For Rust registry publishing, use an Onix-prefixed package name or another owned namespace; keep binary name `cairn` if desired.
- `v0.1.0` is tagged at `2e6ca1f` with post-tag remote install smoke docs. Current `main` can be ahead of that tag, so generate fresh release evidence before tagging another package release.

## Verification
- Use `nix run path:$PWD#cairn -- policy export --check` for policy freshness. `nix develop -c cairn ...` does not put the packaged CLI on PATH and can collide with the top-level `cairn/` directory.
