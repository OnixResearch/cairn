# Validation: Radicle Two-Peer Convergence

## Result

Cairn now has two complementary two-peer convergence rails for its custom signed change COB.

The always-on deterministic rail creates separate Radicle homes, storage repositories, signing keys, and namespaces; transfers only through Radicle's validated signed-reference Git fetch; creates authorized concurrent proposal updates; exchanges both namespaces; verifies identical explicit conflicts; resolves once with a basis containing every competing head; and verifies identical conflict-free state and content identities after resolution transfer.

The dedicated `checks.x86_64-linux.radicle-two-peer-convergence` gate starts two actual `radicle-node` processes on OS-assigned loopback ports, waits for distinct control sockets and node IDs, exercises a bounded unavailable-peer failure, explicitly connects the peers, and fetches import, concurrent update, and resolution refs. Both child processes are killed and reaped through RAII on every return path.

No synchronization CLI was added.

## Success Evidence

- Distinct peer descriptors reject shared homes, storage paths, public keys, and mismatched repository IDs.
- Bob becomes a current canonical delegate before divergence; a non-delegate signer cannot plan a valid update.
- Initial import fetch returns namespaced custom COB ref updates, and the fetched `SignedRefs` identifies Alice's key and includes the custom COB reference.
- One-way pre-exchange states remain different and conflict-free, so stale state cannot be mislabeled as convergence.
- Bidirectional exchange yields byte-equal typed states, identical BLAKE3 state hashes, and exactly two conflict heads with distinct peer actors.
- The reviewed resolution action's basis equals the complete conflict-head set.
- Resolution transfer yields byte-equal typed states, identical BLAKE3 state/content identities, selected exact bytes, and no remaining conflicts.
- The live unavailable-peer attempt completes within the named deadline and leaves Bob without the repository before the valid fetch.
- The live gate reports `radicle-node 1.9.1 (5bd3569e120a6172d9df68e1d1d0eed15e8104b1)`; the Rust adapter remains locked to `radicle 0.24.0` by `Cargo.lock`.

## Portfolio Search and Adversarial Audit

| Family | Mechanism | Result |
|---|---|---|
| `reducer-replay` | Apply matching operation sets only in memory | Rejected: no signed refs, storage, transport, or peer namespaces |
| `storage-copy` | Copy bare repositories or custom refs directly | Rejected: bypasses Radicle fetch validation and is false transport evidence |
| `local-radicle-fetch` | Radicle's registered transport between independent storages | Validated for deterministic signed-reference and application convergence; explicitly not a running node |
| `live-loopback-nodes` | Two actual Nix-pinned daemons using control-socket connect/fetch | Validated as the strongest bounded local peer-transport evidence in scope |

The audit required signed-ref owner/ref assertions after fetch, exact state checks after every exchange stage, complete resolution bases, a stale one-way counterexample, an unavailable-peer path, explicit daemon dependency, and process cleanup. Advisory review output was used only to add checks and was not treated as authoritative evidence.

The first Nix live-gate run failed because the sandbox lacked the `git` executable used by Radicle repository initialization. Adding `pkgs.git` to the check's explicit native inputs fixed the dependency boundary; the next named check and full flake both passed. No ambient host tool was accepted as completion evidence.

The generic historical verified-core grep also matched pre-existing `to_json` methods unrelated to this slice. A scoped diff against `origin/main` proved this change modifies no `crates/cairn-core/src/verified` file, and the authoritative Slotcar/full flake rails passed. Existing unrelated matches were not churned.

## Deterministic Validation

- Pre-change baseline: `cargo test -p onix-cairn-radicle` passed `21` tests.
- Focused deterministic rail: `cargo test -p onix-cairn-radicle --test two_peer_convergence` passed.
- Explicit live rail: `CAIRN_RADICLE_NODE_BIN=... cargo test -p onix-cairn-radicle --test live_two_peer_convergence -- --ignored --nocapture` passed.
- Named Nix rail: `nix build path:$PWD#checks.x86_64-linux.radicle-two-peer-convergence --no-link -L` passed in the sandbox.
- Full `cargo test --workspace`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Policy freshness, Cairn validation, proposal/design/tasks gates, and scoped verified-core preservation passed.
- Traceability coverage passed with `1691/1691` accepted requirement and scenario IDs referenced.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`; `aarch64-linux` was omitted as incompatible.

## Bounded Claims

Passing the deterministic and live rails proves only the observed profiles, keys, repository, signed namespaces, custom COB operations, daemon/adapter versions, loopback addresses, deadlines, and fetch results in those runs. It does not prove public-network publication, peer discovery, NAT traversal, arbitrary-peer trust, long-term availability, eventual convergence, partition recovery, multi-signature identity quorum behavior, release eligibility, or resistance to unrelated processes with equivalent host authority. It does not add or imply a supported `cairn collaboration sync` command.
