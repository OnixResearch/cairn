# Napkin

## Corrections
| Date | Source | What Went Wrong | What To Do Instead |
|------|--------|----------------|-------------------|

## User Preferences
- Avoid AI writing tells in docs/responses: pompous adverbs, "delve/leverage/robust", tapestry/landscape/framework filler, "not X but Y" reframes, rhetorical question reveals, repeated punchy fragments, filler transitions, patronizing analogies, false suspense, grandiose stakes, em-dash overuse, bold-first bullets, unicode decoration, signposted conclusions.

## Patterns That Work
- (approaches that succeeded)

## Patterns That Don't Work
- Running multiple `nix run path:$PWD#cairn -- gate ...` commands in parallel can contend on the Nix eval cache and time out; run Cairn gates sequentially or use one pueue task.
- Running `nix run path:$PWD#cairn ...` in foreground can exceed a short shell timeout while building; use pueue for first-use or derivation-building Cairn CLI commands.
- For focused Cargo test replays, prefer `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test ...`; shared `/home/brittonr/.cargo-target` plus sccache can produce stale missing-rlib/proc-macro noise.
- Assuming `cairn init` writes `.gitkeep` files into empty lifecycle dirs is wrong; smoke output lists only files created by later template/scaffold commands.

## Domain Notes
- crates.io names `cairn`, `cairn-cli`, and `cairn-core` are already taken by unrelated owners; publish under an Onix-prefixed name (e.g. `onix-cairn*`) or use Nix/GitHub release channels.
- `v0.1.0` tag exists and remote install smoke docs point to commit `2e6ca1f`; current `main` may be ahead and needs fresh release evidence for a new tag.

- 2026-05-29: Cairn native `sync --execute` for newly added specs can create only the accepted spec shell (`# ...`, Purpose, Requirements) instead of copying full delta requirements. After syncing/archiving, copy full archived delta spec into `cairn/specs/<domain>/spec.md` before running `tracey coverage`, or Tracey sees implementation refs as dangling.
- 2026-05-29: Probe file/receipt/output paths must reject absolute and `..` components; plain `root.join(path)` lets probe declarations or `--out` escape repo bounds.
- 2026-05-29: `release-readiness` can fail because `fixtures/agent-receipts/positive.json` has stale `replay_policy_hash`/`receipt_hash` after policy changes; regenerate fixture hashes plus plan goldens before trusting MCP smoke evidence.
