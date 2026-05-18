## Why

Cairn now has portable receipt hashes and deterministic lifecycle mutation evidence, but the regression surface is still mostly test-local. Agents and CI need a first-class replay oracle they can invoke against a checkout to prove that Cairn's public use surfaces remain deterministic across repeated runs and distinct temporary roots.

Without a maintained operator command, determinism can regress in places that tests do not cover: newly added receipt fields, MCP tool outputs, lifecycle dry-run plans, release-readiness bundles, or environment-sensitive failure paths. A workspace replay oracle turns those invariants into a single receipt that agents can run before trusting Cairn evidence.

## What Changes

- Add a workspace-level replay oracle surface that runs configured deterministic Cairn commands repeatedly and across copied roots.
- Define stable comparison semantics: full normalized JSON equality for repeat runs, portable hash equality across roots, and explicit allowlists for display-only path differences.
- Emit a deterministic oracle receipt summarizing checked commands, normalized-output hashes, portable evidence hashes, mismatches, skipped commands, and nondeterminism failures.
- Extend MCP reproducibility coverage so MCP read-only calls can be included in the same oracle plan.
- Add negative rails for hidden entropy and mutation attempts during replay.

## Capabilities

### Modified Capabilities

- `ironspec-supersession`: strengthens `determinism_replay` from a test-maintained invariant into an operator-facing replay oracle suitable for agents and CI.

## In Scope

- Read-only and dry-run commands: status, validate, gate, sync dry-run, archive dry-run, migration dry-run, release-readiness, MCP describe, and selected MCP read-only calls.
- Workspace fixtures copied to at least two distinct roots.
- Deterministic receipt schema and BLAKE3 evidence for the oracle itself.
- Bounded, policy-declared normalization of display-only path fields.

## Out of Scope

- Executing destructive lifecycle mutations inside the replay oracle by default.
- Proving external tools are deterministic beyond explicitly captured availability/version evidence.
- Distributed or multi-machine replay; this change targets one machine/CI worker with multiple roots.

## Verification

- Add focused tests that run the oracle against fixture repositories twice and across two temp roots.
- Add negative tests for hidden entropy, mutation attempts, and undocumented path differences.
- Update receipt schema docs/goldens and Tracey links.
- Validate with `nix run path:$PWD#cairn -- validate --root .`, gate checks, `cargo test -p cairn-cli`, and release-readiness.
