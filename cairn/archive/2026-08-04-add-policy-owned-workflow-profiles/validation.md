# Validation: Policy-owned workflow profiles

## Result

The change passed its focused tests, repository gates, lifecycle checks, and Nix checks on 2026-08-04.

## Behavior evidence

- Nickel policy exports three typed profiles: `spec-driven`, `assurance`, and `no-spec-delta`.
- Existing changes use the read-only `spec-driven` compatibility fallback.
- Explicit profile creation records immutable profile identity and creates the exact policy-owned scaffold.
- Assurance gates require risk, verification, and an approved review that binds current BLAKE3 artifact hashes.
- No-spec gates require rationale, local `a[...]` criteria, linked tasks, and current review evidence.
- No-spec sync emits an explicit empty action list and does not create or change accepted specs.
- Sync and archive recheck profile identity before mutation.
- CLI and MCP profile creation and status tests agree on profile, effect, and identity fields.

## Negative evidence

Tests reject unknown and late profile selection, ambient selection authority, duplicate or cyclic profile policy, conflicting no-spec deltas, malformed or duplicate acceptance IDs, unlinked tasks, stale review hashes, and mutation after invalid preflight.

## Commands

The following checks passed:

```text
nix run path:$PWD#cairn -- policy export --check
nix develop -c cargo fmt --all -- --check
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test --workspace
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo clippy --workspace --all-targets -- -D warnings
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- gate proposal add-policy-owned-workflow-profiles --root .
nix run path:$PWD#cairn -- gate design add-policy-owned-workflow-profiles --root .
nix run path:$PWD#cairn -- gate tasks add-policy-owned-workflow-profiles --root .
nix run path:$PWD#cairn -- traceability coverage --root .
nix run path:$PWD#cairn -- tracey coverage --root .
nix run path:$PWD#cairn -- agent-receipt smoke-mcp --root .
nix run path:$PWD#cairn -- release-readiness --root .
nix build path:$PWD#checks.x86_64-linux.self-dogfood --no-link -L
nix build path:$PWD#checks.x86_64-linux.tracey-coverage --no-link -L
nix build path:$PWD#checks.x86_64-linux.slotcar --no-link -L
nix build path:$PWD#checks.x86_64-linux.radicle-cob-format-compatibility --no-link -L
nix flake check path:$PWD -L
```

Traceability passed at `1855/1855`. Release readiness returned `valid: true` and `verdict: pass`. The optional `onix_release_ready` field remained false because this change did not request the authenticated release profile.

## Claim boundary

These checks establish deterministic profile selection, artifact linkage, policy conformance, bounded mutation behavior, and receipt identity. They do not prove zero behavior effect, risk-analysis truth, test sufficiency, code correctness, or release eligibility.
