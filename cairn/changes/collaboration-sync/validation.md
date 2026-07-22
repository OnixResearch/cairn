# Validation: Collaboration Sync

## Result

Cairn now exposes one supported preview-first known-peer pull:

```bash
rad seed RID --no-fetch
cairn collaboration sync CHANGE \
  --backend radicle.v1 \
  --rad-home PATH \
  --rid RID \
  --control-socket ABSOLUTE_PATH \
  --peer-id NID \
  --peer-address HOST:PORT \
  [--root PATH] [--execute]
```

`cairn radicle sync` is the compatibility entry point. The portable backend rejects network sync explicitly.

Preview loads the explicit profile and local storage, validates typed provider inputs, captures the exact pre-state, and emits connect/fetch/reload actions without opening the node control socket. Execution requires operator-configured seeding policy, acquires the existing import, update/resolution, and checkout lock paths, rechecks the previewed state, verifies the control-socket node ID, performs one non-persistent bounded connection and one bounded fetch, and then requires a typed change reload, the requested signed namespace in the fetch result, and the exact custom COB object ref under that peer namespace.

Conflicts remain explicit metadata and are never resolved by sync. No working files are modified.

## Success Evidence

- Equivalent explicit snapshots produce identical ordered connect/fetch/reload plans and BLAKE3 plan identities.
- Malformed repository IDs, peer IDs, addresses, self peers, empty homes, and relative control sockets are rejected before execution.
- Execution rejects an alternate Radicle home even if a caller could present equivalent key material.
- All three cooperating per-change lock paths serialize another sync and are released on successful completion.
- Stale local state and wrong-profile control sockets stop before peer contact.
- Unseeded policy is surfaced as a provider blocker; Cairn does not change seeding policy implicitly.
- Unavailable peers fail within the named connection deadline without discovery fallback.
- Fetch success requires the requested signed namespace, exact peer custom COB ref, and typed named change state; unrelated repository refs cannot satisfy the command.
- Ref updates and namespaces are sorted and deduplicated before receipt hashing.
- A zero-update fetch is a no-op only when exact peer and typed-state postconditions still pass.
- Concurrent authorized heads remain visible in full sorted conflict metadata, and explicit resolution still occurs only through `collaboration resolve`.

## Portfolio Search and Adversarial Audit

| Family | Mechanism | Result |
|---|---|---|
| `direct-git-copy` | Copy repository storage or namespace refs | Rejected: bypasses node transport and signed-reference fetch validation |
| `ambient-rad-cli` | Spawn `rad sync` or `rad fetch` from PATH | Rejected: ambient executable/version/output and implicit policy behavior violate the explicit adapter boundary |
| `background-discovery` | Discover seeds and synchronize broadly | Rejected: exceeds the proven known-peer mechanism and cannot support bounded convergence claims |
| `explicit-control-fetch` | Typed Radicle handle over one explicit socket, peer, address, and repository | Validated: smallest mechanism matching the convergence harness and command contract |

The surviving candidate was audited for stale previews, Radicle-home/socket substitution, self peers, unsigned or unrelated namespaces, missing exact custom refs, no-op false completion, lock leaks, conflict erasure, nondeterministic evidence, and receipt overclaim.

An advisory review suggested disconnecting the peer after fetch. That recommendation was rejected because the command cannot prove ownership of a possibly pre-existing node session; disconnecting it could disrupt unrelated operator work. `persistent=false` prevents creating a retry lease, while the receipt claims only the completed command observation. Advisory output was used to add tests and was not treated as authoritative evidence.

## Defects Found and Fixed

The first explicit bootstrap attempt failed with `cannot fetch ... as it is not seeded`. Pinned Radicle behavior requires local seeding policy even for direct known-peer fetch. The contract, documentation, and live gate now require a separate operator `rad seed RID --no-fetch` step and verify that Cairn reports the unseeded failure without changing policy.

The first full flake replay exposed an intermittent `timed out reading from control socket` during repeated peer handshakes. The original three-second connection window was valid but too brittle under the sandbox schedule. The named connection and fetch windows were increased to ten and fifteen seconds. The actual live public-command scenario then passed fifty consecutive repetitions, the dedicated Nix check, and the full flake check. No automatic retry or alternate-peer fallback was added.

The audit also strengthened success from “typed state plus an existing peer ref” to require that the current fetch result explicitly identify the requested signed namespace. This prevents a stale local peer ref plus unrelated current fetch evidence from satisfying the command.

## Deterministic Validation

- Pre-change baseline: `cargo test -p onix-cairn-radicle` and `cargo test -p cairn-cli` passed.
- Focused pure/CLI suites: sync planner/evaluator, lock, argument, receipt, portable rejection, and adapter/CLI Clippy checks passed.
- Explicit live command: the ignored two-node test passed with `CAIRN_BIN` and `CAIRN_RADICLE_NODE_BIN` supplied explicitly.
- Repetition audit: the live public-command test passed fifty consecutive runs after timeout hardening.
- Named Nix gate: `nix build path:$PWD#checks.x86_64-linux.radicle-two-peer-convergence --no-link -L` passed.
- Full `cargo test --workspace`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Policy freshness, Cairn validation, proposal/design/tasks gates, and scoped verified-core preservation passed.
- Traceability coverage passed with `1717/1717` accepted requirement and scenario IDs referenced.
- Release-readiness and `checks.x86_64-linux.self-dogfood` passed.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`; `aarch64-linux` was omitted as incompatible.

## Budget and Stop Condition

Four mechanism families, three architecture/implementation rounds, focused Cairn and pinned Radicle source retrieval, two advisory audits, one fifty-run repetition audit, and the repository's Rust/Cairn/Nix validators were used. Search stopped because the explicit-control-fetch route satisfied every observable completion criterion and passed the authoritative full flake gate.

## Bounded Claims

A successful receipt proves only the recorded local profile/home, control socket and node ID, explicit peer/address, repository/change, fetched signed namespaces and ref updates, exact peer custom COB ref, typed local reload, adapter/daemon versions, and deadlines. It does not prove publication, push, discovery, arbitrary-peer trust, peer-retained state equality, long-term availability, eventual convergence, partition recovery, conflict correctness, lifecycle approval, evidence truth, working-tree materialization, fsync-level durability, release eligibility, or resistance to unrelated processes with equivalent host authority.
