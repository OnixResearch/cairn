# Napkin

## Corrections
| Date | Source | What Went Wrong | What To Do Instead |
|------|--------|----------------|-------------------|

## User Preferences
- (accumulate here as you learn them)

## Patterns That Work
- (approaches that succeeded)

## Patterns That Don't Work
- Running multiple `nix run path:$PWD#cairn -- gate ...` commands in parallel can contend on the Nix eval cache and time out; run Cairn gates sequentially or use one pueue task.

## Domain Notes
- crates.io names `cairn`, `cairn-cli`, and `cairn-core` are already taken by unrelated owners; publish under an Onix-prefixed name (e.g. `onix-cairn*`) or use Nix/GitHub release channels.
- `v0.1.0` tag exists and remote install smoke docs point to commit `2e6ca1f`; current `main` may be ahead and needs fresh release evidence for a new tag.
