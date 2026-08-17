# Design: Adopt Nickel 1.17 for lifecycle policy

## Context

Cairn uses Nickel as the source language for lifecycle policy and contracts. Rust consumes checked-in generated policy artifacts.

## Decisions

### Decision: Pin Nickel without a broad package-set update

**Choice:** Select Nickel `1.17.0` from commit `1320a983e6c3d1e2fb53dd2464b084b4903b1426`.

Use a narrow upstream input when the current Nix package set lacks the target.

**Rationale:** The change must isolate Nickel behavior from unrelated Nixpkgs changes.

### Decision: Keep runtime free of Nickel execution

**Choice:** Nickel remains a build-time policy source. Runtime Rust continues to consume checked-in generated JSON.

**Rationale:** The version update must not add runtime evaluator effects.

### Decision: Prove policy freshness and rejection behavior

**Choice:** Regenerate policy with the new evaluator. Run valid fixtures and malformed, stale, unknown, and type-mismatched negative fixtures.

Regenerate `flake.lock` only with Nix lock commands.

**Rationale:** Policy freshness and fail-closed behavior are separate required facts.

### Decision: Record bounded evaluator evidence

**Choice:** Relevant policy evidence records the exact Nickel version and source identity.

The record does not prove lifecycle correctness or source correctness.

## Risks and trade-offs

- Generated JSON ordering or diagnostics can change.
- Self-dogfood can reveal policy assumptions that older Nickel accepted.
- A passing export does not prove every CLI path correct.
