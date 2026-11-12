# Contributing to Cairn

Cairn is a deterministic lifecycle engine with a Nickel-authored policy layer.
Changes must preserve the pure-core/imperative-shell boundary and the evidence
non-claims documented in `docs/`.

Read `AGENTS.md` before changing product behavior. It defines the lifecycle,
worktree, validation, and repository-local rules that apply to this checkout.

## Prerequisites

Use the repository Nix development shell. Plain Cargo can miss the pinned
toolchain, Nickel cohort, `nickel-export`, lifecycle checks, and Octet policy.

```bash
nix develop
```

## Fast local loop

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
nix run path:$PWD#cairn -- policy export --check
nix run path:$PWD#cairn -- validate --root .
```

Run the narrowest check that exercises your change first. Then run the
maintained gate for the touched surface:

```bash
nix flake check path:$PWD -L
```

`nix flake check` covers policy freshness, formatting, tests, lifecycle checks,
traceability, Clippy, Octet, and self-dogfood. Do not replace a failing
repository gate with a hand-written approximation.

## Lifecycle work

- Create or update a native `.cairn/` change package for behavior changes.
- Keep proposal, design, tasks, specs, and evidence consistent.
- Use a dedicated worktree and a unique branch from current `origin/main` for a
  Cairn change completion, as `AGENTS.md` requires.
- Do not add a second task system, hidden fallback layout, or Python/shell gates.

## Design constraints

- Pure core decisions stay in `cairn-core`; files, environment, processes,
  clocks, output, exits, and mutation stay in `cairn-cli`.
- Use BLAKE3 for stack-owned identities unless an interoperability surface
  requires another algorithm.
- Bound every new input and fail closed on malformed, stale, linked, or
  out-of-root data.
- Preserve existing receipt schemas and documented stable fields. Additive
  changes need a schema version and positive and negative fixtures.
- Do not edit generated policy JSON, lockfiles, or source pins by hand. Change
  the Nickel or manifest source and regenerate with the owning tool.
- Keep claim boundaries explicit. A receipt proves only what its schema says.

## Tests and evidence

- Add positive and negative tests together for each new validator, adapter,
  parser, policy check, or receipt verifier.
- Test behavior, boundaries, invariants, transitions, precedence, and errors.
  Do not test source text, wiring, or tautologies.
- Record the exact commands and results in the change package or pull request.
- A simulated or partial run must be labeled as such.

## Pull requests

- Keep each pull request focused and repository-local.
- Include the change package, verification commands, and known non-claims.
- Do not force-push a branch under review.
- Do not include unrelated formatting, generated churn, or vendor updates.
- Report integration blockers rather than silently weakening a gate.

## Licensing

Contributions are accepted under the license of the surface they touch:

- `crates/cairn-core`, `crates/cairn-gate-contract`,
  `crates/cairn-radicle-cob`, templates, and reusable examples:
  `MIT OR Apache-2.0`.
- `crates/cairn-cli`, repository documentation, and lifecycle material:
  `AGPL-3.0-or-later`.

See `docs/licensing.md` and `license-policy.tsv`. Do not relicense third-party
material or change a declared grant without an explicit reviewed decision.
