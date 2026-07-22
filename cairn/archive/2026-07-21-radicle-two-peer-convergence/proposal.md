## Why

Cairn now creates, updates, inspects, and explicitly resolves signed Radicle change COBs, but existing evidence stops at one local profile and one local Radicle storage. The implementation therefore cannot yet distinguish application-level convergence from accidental same-store behavior, and it explicitly does not prove that signed namespaces survive peer transport.

Before exposing any `collaboration sync` command, Cairn needs a bounded two-peer harness that transfers real signed Radicle references between independent profiles, creates concurrent updates, observes the same conflict on both peers, resolves it once, and verifies convergence after a second transfer. A deterministic local transport test and a dedicated live-node Nix gate are both required because Radicle's in-process test transport explicitly does not run node daemons.

## What Changes

- Add a reusable two-profile Radicle convergence fixture with independent homes, signers, storage, and repository namespaces.
- Add an always-on signed-reference transport test that imports a Cairn change, transfers it, creates concurrent artifact updates, exchanges both namespaces, and verifies identical conflict state.
- Resolve the conflict on one peer, transfer the resolution, and verify identical conflict-free state and content identities on both peers.
- Add a dedicated Nix check that starts two real `radicle-node` processes on loopback, uses bounded connect/fetch operations, and exercises the same custom COB convergence path without copying repositories or refs directly.
- Add negative coverage for unavailable peers, unauthorized mutation, stale pre-transfer state, timeout/process cleanup, and missing or invalid transfer results.
- Document that the harness proves only the observed local two-node run; it does not establish arbitrary-network availability, eventual convergence, remote trust, or a supported synchronization CLI.

## Impact

- **Files**: `crates/cairn-radicle/tests/`, `crates/cairn-radicle/Cargo.toml`, `flake.nix`, `README.md`, `docs/hermes/cairn-workflow-skill.md`, and native Cairn lifecycle artifacts.
- **Testing**: focused two-peer tests, dedicated live-node Nix check, workspace tests, warnings-denied Clippy, policy/lifecycle/traceability gates, verified-core purity, release readiness, and full `nix flake check`.
