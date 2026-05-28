# Design: Package policy export runtime

## Context

`crates/cairn-cli/src/policy.rs` intentionally invokes `nickel export --format json` for policy export, while Rust validates semantic invariants and compares generated JSON. This preserves the Rust/Nickel ownership boundary, but the Nix package currently installs plain binaries without adding `nickel` to their runtime environment.

## Decisions

### 1. Wrap the packaged binaries

**Choice:** Add `makeWrapper` to the `cairn` package and wrap both installed binaries with a PATH prefix containing `pkgs.nickel`.

**Rationale:** The runtime dependency is narrow, explicit, and local to the packaged command surface. It fixes `nix run path:$PWD#cairn -- policy export --check` without embedding Nickel execution into Rust core or requiring operators to enter a dev shell.

### 2. Add a stripped-environment check

**Choice:** Add a flake check that copies the source, clears ambient environment with `env -i`, and runs the packaged `cairn policy export --check`.

**Rationale:** Existing policy checks include Nickel in the check environment, so they do not prove the package carries its own runtime dependency. The stripped check fails if the wrapper or runtime dependency regresses.

## Risks / Trade-offs

- Package closure grows by Nickel and its dependencies.
- The legacy `ironspec` alias receives the same wrapper so compatibility behavior stays identical.
- The wrapper only fixes packaged execution; direct target binaries built by Cargo still require Nickel in the caller environment for policy export.
