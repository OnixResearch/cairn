## Why

Cairn can now prove two-peer Radicle convergence through its integration harness, but operators still have no supported command that performs the bounded known-peer fetch used by that evidence. Requiring users to assemble raw node-control calls would bypass Cairn's preview, context binding, typed reload, deterministic receipt, and claim boundary.

## What Changes

- Add preview-first `cairn collaboration sync <change> --backend radicle.v1` and compatibility `cairn radicle sync` commands for one explicit peer ID, peer protocol address, repository ID, Radicle home, and node control socket.
- Keep preview read-only with respect to the node: inspect and bind local profile/storage state, then report the bounded connect/fetch/reload actions without opening the control socket.
- On `--execute`, require operator-configured local seeding policy, acquire cooperating local change locks, recheck the exact preview context, verify the control socket belongs to the explicit profile, perform one bounded non-persistent connect and one bounded fetch, and typed-reload the requested Cairn change.
- Emit deterministic pre/post state, signed-namespace, ref-update, blocker, conflict, timeout, execution-mode, and non-claim evidence.
- Reject portable sync, malformed or self peers, stale local state, unavailable/mismatched nodes, failed fetches, missing peer change refs, and missing post-fetch typed change state without resolving conflicts or modifying the working tree.

## Impact

- **Files**: `crates/cairn-radicle/src/`, `crates/cairn-cli/src/collaboration.rs`, `crates/cairn-cli/src/radicle_change/`, Radicle/CLI tests, the live two-peer gate, README/workflow docs, and native Cairn lifecycle evidence.
- **Testing**: pre-change Radicle/CLI baselines; pure plan/evaluation positive and negative tests; argument/receipt tests; actual two-node command preview/execute/no-op/conflict tests; workspace format/test/Clippy; policy/lifecycle/traceability; named live-node, Slotcar, self-dogfood, release-readiness, and full flake gates.
