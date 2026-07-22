## Context

The semantic reducer is already pure, but the provider shell is implemented through `radicle 0.24`: profile homes, canonical identity documents, Radicle storage repositories, device signers, and node connect/fetch. The direct `radicle-cob 0.20` library provides signed change entries, object/history evaluation, object and change storage traits, and create/get/list/update operations without requiring Radicle node or profile APIs. Its `git2::Repository` implementation supplies change-entry storage; Cairn must supply the small object-ref storage implementation.

## Decisions

### Decision: Depend on COB primitives, not the Radicle runtime

**Choice:** The adapter will link direct COB/crypto/OID/Git/ref-format crates only. The umbrella `radicle` crate, `radicle-node`, profile homes, repository IDs, identity documents, seeding policy, control sockets, and peer transport are excluded from the supported build and command surface.

**Rationale:** This is the narrowest dependency family that retains the requested signed Git-backed COB mechanism. Keeping the umbrella library but hiding node commands would preserve compile-time and semantic coupling.

### Decision: Use one explicit Git repository and COB ref namespace

**Choice:** A thin `CobRepository` shell will wrap one operator-supplied Git repository, delegate change-entry storage to `radicle-cob`'s Git implementation, and own deterministic `refs/namespaces/<actor>/refs/cobs/<type>/<object>` enumeration and updates. Read operations require only `--git-repository`; signed mutations additionally require `--keystore`.

**Rationale:** The store remains wire-shaped as a Radicle COB without importing Radicle storage, profile, or network policy. Explicit paths avoid ambient profile discovery.

### Decision: Bind authority into the root state

**Choice:** New root state records a bounded immutable set of delegate public keys. The signed root author is the creator. Evaluation derives creator/delegate facts only from the signed entry author and root authority set. Empty authority remains omitted from historical serialization so existing legacy semantic hashes remain stable; histories that require external Radicle identity-document delegation fail closed rather than silently gaining authority.

**Rationale:** External mutable authority snapshots would make historical evaluation depend on ambient state. Creator-only authority would make ordinary collaboration and approval workflows unusable. Immutable root authority is deterministic and available during every replay.

### Decision: Remove transport instead of emulating it

**Choice:** `collaboration sync`, `cairn radicle`, live-node tests, signed-reference convergence fixtures, and the named `radicle-two-peer-convergence` Nix check are removed. The replacement compatibility command is local `cairn cob` only.

**Rationale:** Git remote transport can be selected separately by the operator. Cairn's COB layer should not own discovery, seeding, connection, fetch, publication, or convergence claims.

## Approach Registry

| Family | Mechanism | State | Reason |
|---|---|---|---|
| umbrella-hidden | Keep `radicle` but hide node commands | Rejected | Compile-time profile/node/storage/identity coupling remains |
| direct-cob-store | Direct COB primitives plus Cairn Git ref store | Selected | Retains signed COB semantics with the narrowest checked dependency surface |
| copied-cob-format | Reimplement COB commits and signatures without `radicle-cob` | Rejected | Creates avoidable format and signature drift risk |
| external-authority | Replay against mutable external delegate files | Rejected | Historical validity changes with ambient authority state |

## Risks / Trade-offs

- Existing objects whose accepted operations depended on Radicle identity-document delegates cannot be authorized from COB history alone; they fail closed and need an explicit migration rather than an invented authority result.
- `radicle-cob` still intentionally brings Radicle crypto/OID/DAG primitives and Git SHA-1 object identities required by that format. Cairn-owned semantic and receipt identities remain BLAKE3.
- Removing built-in sync means operators choose Git transport outside Cairn. A local COB receipt does not prove remote publication or convergence.
- Direct keystore loading initially supports unencrypted OpenSSH Ed25519 keys only; encrypted or agent-backed signing requires a separately reviewed explicit interface.
