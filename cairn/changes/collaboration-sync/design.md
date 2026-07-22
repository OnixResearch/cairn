## Context

The archived `radicle-two-peer-convergence` change proves that two independent Nix-pinned Radicle nodes can explicitly connect and fetch Cairn's custom change COB, preserve concurrent conflict heads, and converge after complete-basis resolution. That harness is evidence for one known-peer pull mechanism, not discovery, publication, background replication, or arbitrary network convergence.

The public collaboration surface is preview-first. Provider-neutral semantics stay in `cairn-core`; Radicle node IDs, addresses, control sockets, signed refs, storage locks, fetch results, and typed COB reload remain in the Radicle adapter and CLI shell.

## Success Contract

**Goal:** expose exactly the bounded known-peer pull already validated by the convergence harness.

**Completion evidence:** preview performs no node-control command; execution rechecks the preview under cooperating locks; the local node ID matches the explicit profile; one bounded connect/fetch succeeds; the requested typed change reloads; the fetched peer namespace contains that exact change object ref; deterministic receipts bind before/after state and conflicts; actual two-node CLI tests pass.

**False completion:** direct Git ref copying, spawning an ambient `rad` binary, returning success after connect without typed reload, treating any local change as peer evidence, silently resolving conflict, mutating seeding/follow policy, announcing refs, changing working files, or claiming publication/eventual convergence.

**Audit risks:** stale previews, socket/profile substitution, self-fetch, peer/address mismatch, no-op fetches, partial network failure, concurrent local import/update/checkout/resolution, missing custom refs, malicious or invalid signed operations, conflict erasure, nondeterministic ref ordering, and overbroad sync claims.

**Budget:** at most four mechanism families, three architecture rounds, twenty focused repository/pinned-source retrievals, one advisory model audit, and the repository's deterministic Rust/Cairn/Nix validators. Allowed terminal states are validated, blocked with an exact provider limitation, exhausted, or user-decision-required.

## Approach Registry

| Family | Mechanism | Claim | State |
|---|---|---|---|
| `direct-git-copy` | Copy namespaces or repository storage directly | Local refs become similar | Falsified: bypasses node transport and signed-reference fetch validation |
| `ambient-rad-cli` | Spawn `rad sync`/`rad fetch` from PATH | Reuse upstream CLI behavior | Falsified: ambient executable/version/output and policy side effects weaken the explicit adapter boundary |
| `background-discovery` | Ask the node to discover seeds and synchronize broadly | Eventual peer convergence | Falsified: exceeds known-peer evidence and cannot support bounded convergence claims |
| `explicit-control-fetch` | Typed Radicle library handle over an explicit control socket, one non-persistent connect, one direct fetch, typed change reload | Bounded observed pull from one known peer | Active; smallest mechanism matching the proven harness |

## Decisions

### Decision: expose sync as one explicit pull

**Choice:** `collaboration sync` requires `--rad-home`, `--rid`, `--control-socket`, `--peer-id`, and `--peer-address`. It performs at most one connect and one fetch from that peer. `portable-json.v1` reports that transport is unsupported.

**Rationale:** the convergence harness proves explicit known-peer fetch. It does not prove discovery, multi-peer selection, push, or background replication. Naming the bounded operation in the receipt prevents the broader word “sync” from becoming a convergence claim.

### Decision: preview binds local state without contacting the node

**Choice:** preview loads only the explicit profile and storage, validates typed identifiers, captures an exact local-state summary, and emits fixed connect/fetch/reload actions and named deadlines. It does not open the control socket, connect, fetch, seed, follow, announce, fork, checkout, or write.

**Rationale:** preview must be inspectable and side-effect free. A peer's future refs cannot be precomputed honestly, so preview binds actions and local preconditions rather than inventing a result hash.

### Decision: keep pure planning and evaluation in the Radicle adapter

**Choice:** deterministic functions derive a sync plan from typed snapshots and evaluate explicit execution observations. The imperative shell loads profiles/storage, acquires locks, talks to the local node, and reloads the COB.

**Rationale:** peer IDs, addresses, fetch results, and signed namespaces are provider facts, but their validation and receipt classification remain testable without a daemon.

### Decision: serialize against cooperating change operations

**Choice:** execution acquires the existing per-change import, update/resolution, and checkout lock paths before the final local-state recheck and holds them through fetch/reload.

**Rationale:** fetch mutates Radicle storage and must not race cooperating import, update, resolution, or checkout commands. Existing lock paths preserve compatibility and avoid adding a lock that older operations would ignore.

### Decision: require exact post-fetch peer evidence

**Choice:** success requires a typed reload of the named change and an exact `refs/namespaces/<peer>/refs/cobs/<cairn-type>/<object>` ref. Ref updates and fetched namespaces are sorted before receipt generation. A zero-update fetch is a valid no-op only when the peer ref and typed state still validate.

**Rationale:** a repository fetch or unrelated ref update is not evidence that the requested collaboration object came from the requested peer.

### Decision: conflicts are an observed result, not a blocker

**Choice:** sync reports post-fetch conflict metadata and state/content hashes but does not choose heads, modify working files, or invoke resolution.

**Rationale:** transport and semantic resolution remain separate explicit operations.

## Risks / Trade-offs

- A direct fetch can partially change Radicle storage before a provider failure is reported. Receipts compare pre/post observations and do not claim transactional network mutation.
- Explicit peer addresses may become stale between preview and execution; execution fails within fixed deadlines rather than discovering alternatives.
- The command does not alter seeding policy. Operators remain responsible for long-lived Radicle policy and availability.
- Cooperating local locks do not exclude unrelated processes with equivalent host authority.
- A successful one-peer pull proves only the recorded operation and resulting local state, not remote state equality or future convergence.
