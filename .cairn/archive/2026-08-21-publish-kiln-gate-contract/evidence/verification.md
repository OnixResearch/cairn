# Verification evidence

Date: 2026-08-21

## Baseline

Before implementation, the existing gate core tests passed: 21 passed and 0 failed. The existing gate receipt schema test passed.

## Contract checks

- `cargo test -p cairn-gate-contract --all-targets`: pass, including 9 unit tests and the producer-fixture test.
- `cargo clippy -p cairn-gate-contract --all-targets -- -D warnings`: pass.
- `cargo check -p cairn-gate-contract --lib --target wasm32-unknown-unknown`: pass.
- Producer fixture regeneration and byte comparison: pass.
- Positive Nickel request, passing result, and blocked result fixtures: pass.
- Negative Nickel schema, identity, and evidence-order fixtures: rejected as required.
- Focused Nix contract, WebAssembly, and Nickel checks: pass.

## Repository checks

- `cargo fmt --all -- --check`: pass.
- `cargo test --workspace --all-targets --features redb-registry`: pass.
- `cargo clippy --workspace --all-targets --features redb-registry -- -D warnings`: pass.
- `cairn policy export --check`: pass.
- `cairn validate --root .`: pass.
- `cairn gate tasks publish-kiln-gate-contract --root .`: pass; receipt `7c2a65317b93c98982ee6a764d336c74bee8dfd50850cdc069e332247a1c21b8`.
- `cairn traceability coverage --root . --json`: 2281 of 2281 referenced; receipt `d2105571e88e676627f357b03d0eacfa316db1b0dce86b28d61328739c440991`.
- `nix flake check path:$PWD -L`: pass before and after archive, including Slotcar, self-dogfood, policy, Rust, Nickel, lifecycle, traceability, and the new contract checks.
- Archive execution: pass; receipt `8d4a50bafb166d4c072eceade19417f234f85d42a3139b40c08dc55d60fffcc2`.
- `git diff --check`: pass.

The first repository-wide test run stopped at `evidence_export_materializes_complete_portable_chain`. The active requirement markers were intentionally dangling before the accepted-spec sync. After sync, focused evidence-registry tests and the complete workspace suite passed.

## Architecture and authority

The new crate is a pure `no_std + alloc` component. It does not import `cairn-cli` or perform host effects. It binds external request facts to a captured native gate observation without changing Cairn gate semantics.

The consumer retains executable verification, repository and policy path authority, process limits, persistence, retries, reconciliation, gate composition, and merge or release authority.

## Non-claims

Passing these checks does not prove candidate truth, source correctness, test completeness, process reliability, merge authority, release execution, deployment success, or future applicability.
