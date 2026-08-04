# Validation: Typed agent guidance context

Validation date: 2026-08-04.

## Baseline

The baseline policy, validation, proposal gate, design gate, tasks gate, and release-readiness checks passed before implementation.

## Focused checks

These checks passed:

```console
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test -p cairn-core verified::agent_guidance
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test -p cairn-cli --test agent_guidance
```

The focused suites cover these cases:

- valid project and lifecycle source roots
- profile, artifact, and operation selection
- policy, fact, and advisory ordering
- CLI and MCP parity
- workflow instruction composition
- stale project projections
- missing required and optional files
- symlinks, invalid UTF-8, and oversized text
- unknown selectors and unsafe paths
- undeclared and secret-like text
- checkout-independent portable identity
- raw portable-text rejection
- empty-guidance compatibility

## Rust checks

These checks passed:

```console
nix develop -c cargo fmt --all -- --check
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test --workspace
env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo clippy --workspace --all-targets -- -D warnings
```

The focused purity search found no file, environment, process, clock, network, or output effects in `agent_guidance.rs`.

## Policy and lifecycle checks

The generated policy is fresh.

The current policy hash is `2f8b371f0be4a1a339fc46e7808c04c8faed4555d4793a821b772f53bfa7daaf`.

The current policy field-set digest is `e7896d6058231d3440fc31936ed7b5baaf39530026d7c482cdd8d49d8a9d46bb`.

These packaged checks passed:

```console
nix run path:$PWD#cairn -- policy export --check
nix run path:$PWD#cairn -- validate --root $PWD
nix run path:$PWD#cairn -- gate proposal add-agent-guidance-context --root $PWD
nix run path:$PWD#cairn -- gate design add-agent-guidance-context --root $PWD
nix run path:$PWD#cairn -- gate tasks add-agent-guidance-context --root $PWD
nix run path:$PWD#cairn -- traceability coverage --root $PWD --json
nix run path:$PWD#cairn -- agent-receipt smoke-mcp --root $PWD
nix run path:$PWD#cairn -- release-readiness --root $PWD
```

The tasks gate reports 16 completed tasks and no open tasks.

Traceability covers 1893 of 1893 requirements. It reports no missing or dangling references.

MCP smoke reports `valid: true` and `verdict: pass`.

Release readiness reports `valid: true` and `verdict: pass`. Optional Onix release evidence remains absent, so `onix_release_ready` is false.

## Nix checks

These checks passed:

```console
nix build .#checks.x86_64-linux.slotcar --no-link -L
nix flake check path:$PWD -L
```

The remote builder was unavailable. Nix completed the builds locally.

## Claim boundary

These results prove the listed checks for the observed source and policy.

They do not prove context truth, secrecy, agent compliance, implementation correctness, or universal release fitness.
