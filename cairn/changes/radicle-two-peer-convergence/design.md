## Context

The production Radicle adapter uses typed `radicle-cob` operations, canonical identity documents, signed creator/delegate authority, and provider-assigned Git operation IDs. Existing tests exercise real profile and storage APIs but never transfer a custom Cairn COB between independent Radicle namespaces. Radicle 0.24 exposes two relevant mechanisms:

1. `radicle::test::fetch` performs an actual Git fetch between separate storages through Radicle's registered test transport and validates fetched refs, but its own documentation says it does not run real nodes.
2. The packaged `radicle-node` 1.9.1 service exposes loopback peer connection and explicit fetch over separate control sockets and storage homes.

A file copy, shared storage, reducer-only replay, or duplicated fixture is false completion because it bypasses signed namespace transport and peer-owned refs.

## Success Contract

The exact goal is to validate that one Cairn change COB can be transported between two independent Radicle profiles, diverge through authorized concurrent updates, converge to the same visible conflict after bidirectional transfer, and converge to the same conflict-free state after one explicit complete-basis resolution.

Observable completion evidence is:

- distinct home paths, public keys, storage paths, and peer namespaces;
- successful Radicle reference fetches containing namespaced updates rather than direct filesystem copies;
- identical conflict head metadata and state hashes after concurrent exchange;
- one resolution operation whose basis names every competing head;
- identical final state/content hashes and no conflicts after resolution transfer;
- a live-node Nix check using two actual `radicle-node` processes, bounded socket readiness, explicit connect/fetch, and deterministic teardown; and
- positive and negative assertions plus repository lifecycle/release gates.

Excluded outputs include mock-only success described as network publication, two handles to one storage, same-key profiles, unbounded sleeps/retries, skipped live-node evidence, direct repository/ref copying, and tests that only compare reducer inputs constructed in memory.

## Decisions

### Decision: Use two complementary transport rails

**Choice:** Keep an always-on deterministic signed-reference test using Radicle's local test transport and add a separate live-node Nix check using packaged `radicle-node` processes on loopback.

**Rationale:** The local rail gives fast, reproducible semantic regression coverage. The live rail checks process lifecycle, control sockets, peer connection, upload-pack/fetch, independent storage, and signed namespace visibility. Neither rail is mislabeled as the other.

### Decision: Reuse production import, update, inspection, and resolution APIs

**Choice:** Build the scenario through `create_change_import`, `create_change_update`, `inspect_change_status`, `inspect_change_cob_conflicts`, `plan_change_cob_resolution`, and `create_change_resolution` instead of writing raw refs or replaying only the pure reducer.

**Rationale:** This exercises the exact custom COB mapping and current authority checks that supported commands use while keeping transport orchestration in the integration shell.

### Decision: Make both peers current delegates before divergence

**Choice:** Establish a canonical identity document containing both distinct peer DIDs before importing the Cairn change, with a one-signature threshold suitable for the bounded test fixture.

**Rationale:** Concurrent updates must be independently signed and authorized. Reusing one key on both peers would not prove peer-owned namespace convergence; allowing an outsider would bypass the production authority contract.

### Decision: Keep live transport as a dedicated Nix gate

**Choice:** Mark the live process test ignored during ordinary Cargo runs and execute it explicitly from a named flake check with `radicle-node` available. The deterministic local transport test remains part of ordinary workspace tests.

**Rationale:** Ordinary Cargo environments do not promise the external daemon binary. The flake gate makes the required dependency and execution explicit rather than silently skipping based on ambient PATH.

### Decision: Use bounded process orchestration

**Choice:** Allocate loopback ports through the operating system, poll control sockets until a named deadline, use explicit fetch timeouts, capture child logs, and terminate/reap both daemons on every return path.

**Rationale:** Fixed ports, unbounded sleeps, detached children, and ambient homes create flaky or leaking tests. The shell must own all process and timing effects; application convergence assertions remain deterministic over loaded states.

## Approach Registry

| Family | Mechanism | State | Evidence / blocker |
|---|---|---|---|
| `reducer-replay` | Apply the same operation set in memory | Rejected | Proves reducer order independence only; no signed refs or peer storage |
| `storage-copy` | Copy bare repository files or refs between homes | Rejected | Bypasses Radicle fetch validation, namespaces, and transport |
| `local-radicle-fetch` | Radicle registered transport between independent storages | Active | Deterministic and real Git fetch, but explicitly not node networking |
| `live-loopback-nodes` | Two `radicle-node` processes with explicit connect/fetch | Active | Strongest bounded transport evidence available in repository/Nix scope |

## Risks / Trade-offs

- Loopback success does not prove public-network routing, NAT traversal, discovery, long-term availability, or eventual convergence.
- The live check depends on the Nix-pinned `radicle-node` package while Cairn's Rust adapter uses the workspace-pinned `radicle` crate; the gate must report both versions and fail on incompatible behavior rather than claiming protocol universality.
- Process tests can leak children or collide on ports. RAII cleanup, OS-assigned ports, explicit readiness deadlines, and captured diagnostics are mandatory.
- A one-signature two-delegate fixture tests independent authority but not multi-signature identity quorum behavior.
- No `collaboration sync` command is added by this change. Product synchronization semantics remain deferred until the harness is stable.
