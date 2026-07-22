## Context

The maintained direct tests prove standard signed commits, manifests, refs, CRDT heads, and Cairn reduction using `CobRepository`. Producer and consumer behavior still share Cairn-owned integration code, so an encoder or discovery error repeated on both sides could look compatible. The new evidence must be static, deterministic, provenance-bound, runnable offline, and unable to add `radicle`, `radicle-node`, profiles, identities, nodes, or networking to product dependencies.

## Search Contract

The exact goal is two-way checked compatibility between Cairn's COB-only provider and a materially separate producer/consumer path. Completion requires a hash-bound static Git fixture produced without Cairn crates, successful Cairn load plus append, successful independent direct-consumer evaluation of Cairn output, negative fixture rejection, and a Nix dependency audit. False completion includes two wrappers around `CobRepository`, fixture generation during normal tests, an unpinned or unauthenticated fixture, a live Radicle node, or claims beyond the pinned format cohort.

The bounded search used four mechanism families, one local advisory audit, repository and installed crate sources, and two implementation/audit rounds. Network discovery and live-node setup were excluded because the installed pinned sources were sufficient and runtime behavior is outside the goal.

## Decisions

### Decision: Use an upstream-storage static producer fixture

**Choice:** Generate the committed bundle once with a standalone reference producer that depends on upstream `radicle` 0.24.0, calls `radicle::storage::git::Repository` plus direct `radicle-cob`, manually authors Cairn action JSON, and imports no Cairn crate. Commit the generator source as evidence, not as a workspace package or release dependency.

**Rationale:** This exercises Radicle's own COB ref writer and storage adapter while keeping the umbrella crate out of Cairn's Cargo graph and normal Nix build inputs. A static bundle is deterministic, offline, reviewable, and cannot invoke profiles, nodes, discovery, or transport during validation.

### Decision: Use a separate read-only direct consumer

**Choice:** Implement a test-only consumer with its own namespaced-ref discovery, direct `radicle-cob` change loading, and minimal independent JSON reducer. It MUST NOT call `CobRepository`, `CairnChangeCob`, `inspect_change`, or Cairn core reduction when evaluating Cairn-produced output.

**Rationale:** Reusing the format and crypto libraries is the compatibility target, while separate storage discovery and action reduction distinguish the consumer mechanism from the product adapter. This is stronger than merely inspecting the commit tree and weaker than a separately maintained implementation, so claims remain bounded to the pinned cohort.

### Decision: Bind provenance with Nickel and BLAKE3

**Choice:** Keep a typed Nickel provenance source and deterministic generated JSON. Tests recompute BLAKE3 for the bundle and producer source, bind exact producer/library versions and fixture IDs, and reject drift. The Nix gate checks that Nickel export equals the committed JSON.

**Rationale:** The fixture is evidence only if its origin and expected object graph are explicit and tampering is detected before semantic evaluation.

## Approach Registry

| Family | Mechanism | State | Reason |
|---|---|---|---|
| upstream-storage-static | Upstream Radicle Git storage writes a committed bundle from manual Cairn JSON | selected | Different ref/storage adapter, offline runtime, exact provenance |
| raw-git-independent | Hand-build signed commit bytes and verify signatures without `radicle-cob` | deferred | Stronger independence but duplicates subtle SSH-signature and commit grammar; a bug could create false incompatibility |
| direct-git2-library | Raw `git2::Repository` plus `radicle-cob` on both sides | rejected | Too close to the existing test mechanism |
| live-radicle-runtime | Profile/node-backed generation or synchronization | excluded | Reintroduces the runtime boundary intentionally removed from Cairn |

The advisory response proposed nonexistent commands and artifacts and was rejected without use. Deterministic source inspection and repository tests remain authoritative.

## Risks / Trade-offs

- The producer and consumer still share pinned `radicle-cob` and crypto semantics, so this does not prove compatibility with arbitrary Radicle releases or an independently specified implementation.
- A committed Git bundle is opaque in ordinary review. Typed provenance, source retention, exact BLAKE3 hashes, and deterministic regeneration instructions make its identity reviewable.
- The reference producer mentions the umbrella `radicle` crate, but it is plain fixture evidence rather than a Cargo workspace member, Nix input, runtime dependency, or maintained executable.
- Deterministic test keys are public fixtures and MUST never be interpreted as secrets, operator identities, or production authority.
- Positive load/update evidence does not claim publication, availability, synchronization, convergence, remote trust, or whole-repository durability.
