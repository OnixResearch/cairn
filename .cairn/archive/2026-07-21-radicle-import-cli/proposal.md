# Proposal: Radicle Import CLI

## Why

Cairn can reduce and store `xyz.onixresearch.cairn-change` objects, but operators cannot create one from a native active change. The missing shell also leaves the previously documented current-authority obligation unenforced: a historical operation identity document is sufficient for replay, but it is not sufficient authority for a new Git-backed mutation.

## What Changes

- Add a pure import planner that maps every present native active-change artifact into the lossless COB root schema.
- Add preview-first `cairn radicle import <change>` behavior with explicit `--rad-home` and `--rid` inputs.
- Re-check the actor against the repository's current canonical identity document immediately before signed COB creation.
- Emit a BLAKE3-bound receipt for preview, blocked, and executed outcomes.
- Keep checkout, update, archive execution, network transport, and live peer synchronization outside this slice.

## Impact

The CLI gains a new mutation-capable command. `cairn-core` gains only pure import planning; Radicle profile loading, identity reads, signing, storage mutation, lifecycle file reads, and JSON output remain in shell crates.
