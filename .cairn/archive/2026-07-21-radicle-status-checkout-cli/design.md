# Design: Radicle Status and Checkout CLI

## Goal and Success Contract

The exact goal is a typed read-only status command and a preview-first checkout command that materializes one conflict-free Cairn change COB into exact native active-change bytes without weakening current authority or overwrite safety.

Completion requires:

- typed in-process object lookup by validated Cairn change ID;
- deterministic status over object/state/content identities, lifecycle/evidence condition, current profile authority, policy drift, and local byte equivalence;
- no filesystem or Radicle mutation during status or checkout preview;
- checkout refusal for conflicts, divergent or unsupported local content, non-delegate profiles, and preview/execution drift;
- current canonical delegate validation immediately before filesystem mutation;
- prepared exact-byte writes, verification, atomic directory rename into an absent destination, and post-checkout byte equivalence; and
- positive and negative tests plus focused, workspace, lifecycle, purity, traceability, and Nix checks.

False completion includes reading raw refs without typed evaluation, hiding reducer conflicts, treating historical operation authority as current authority, writing into an existing divergent change, partially materializing files, claiming network synchronization, or treating checkout as lifecycle/gate approval.

## Approach Registry

| Family | Mechanism | State | Reason |
|---|---|---|---|
| typed projection | Enumerate exact Cairn COB refs, typed-load the unique matching change, and project pure status/checkout facts | active | Preserves reducer validation and existing duplicate-object detection |
| raw Git materialization | Decode blobs or refs directly into lifecycle files | falsified | Bypasses typed reduction, authorization history, schema checks, and conflicts |
| prepared directory checkout | Write and verify a capability-bounded prepared tree, then rename it into an absent destination | active | Prevents visible partial active changes during normal execution |
| in-place overlay | Write expected files directly into an existing active change | falsified | Can overwrite independent edits, preserve unsupported entries, or expose partial state |
| historical-authority checkout | Accept the identity documents that authorized historical operations as current mutation authority | falsified | A retired delegate could mutate the current filesystem |
| caller-asserted authority | Trust a CLI boolean or delegate list | falsified | The mutation authority must come from the current canonical repository identity |

## Retrieval and Audit Budget

The implementation budget is three review rounds, four distinct read/materialization mechanisms, one adversarial secondary review, six targeted Cairn/Radicle API reads before broader retrieval, focused tests before and after core changes, one full workspace rail, and one Nix rail. Audit risks are duplicate object ambiguity, conflicted reducer state, stale lifecycle/evidence, policy drift, current-identity drift, local destination races, symlink/path escape, unsupported entries, partial writes, stale prepared directories or locks, receipt overclaiming, and accidental signer requirements for read-only status.

## Decisions

### Decision: status is typed and read-only

`cairn radicle status <change> --root PATH --rad-home PATH --rid RID [--policy PATH]` loads the explicit profile public identity and repository storage but never loads a signer. It resolves exactly one matching custom object through typed evaluation. The receipt reports current profile delegate authority without treating non-authority as a read blocker.

The pure core computes state identity, semantic content identity when conflict-free, lifecycle condition, current evidence count, sorted materialized file descriptors, and local destination classification. Content conflicts remain visible status rather than becoming a partial file plan.

### Decision: checkout is preview-first and create-only

`cairn radicle checkout <change>` uses the same explicit context and emits its plan by default. `--execute` is required for mutation. An absent destination is creatable; a byte-equivalent destination is an idempotent no-op; any divergent, partial, symlinked, non-UTF-8, oversized, or unsupported destination is blocked. This slice intentionally has no force/replace option because no trusted local base binding exists yet.

### Decision: recheck current authority while serialized

Before filesystem mutation, the adapter reloads the explicit profile, current canonical identity, unique typed object, and state. It requires the profile actor to be a delegate, loads the matching signer, signs and verifies a challenge bound to the repository, object, canonical identity, and typed state hash, acquires a per-repository/per-change checkout lock under the explicit Radicle home, and repeats the repository checks while the lock is held. The CLI compares the guarded inspection with the preview before touching the repository and holds the guard through materialization. Read-only status does not load or use the signer.

The check is point-in-time rather than a lease. A later identity change does not rewrite history. The local lock serializes cooperating Cairn checkout processes but is not a sandbox against another process with the same host filesystem authority.

### Decision: prepare, verify, and atomically expose

The shell writes exact planned bytes below a capability-bounded prepared directory keyed by repository, change, and state hash. It verifies every prepared file's length and BLAKE3 hash, checks the destination again, and renames the complete directory into `cairn/changes/<change>`. Existing prepared state fails closed for deliberate recovery. The post-mutation local scan must classify the destination as byte-equivalent.

### Decision: receipts bind observations without promoting claims

Status and checkout receipts bind repository/object/state/content identities, actor/current identity, policy observation, lifecycle/evidence condition, local classification, file hashes, blockers, execution mode, and mutation result. They do not include Radicle home paths in portable hash material and do not claim peer publication, gate validity, dependency readiness, source correctness, archive execution, or release eligibility.

## Functional Core / Imperative Shell

```text
cairn-core
  typed COB state + local byte snapshot -> deterministic status/checkout plan
  conflict classification, exact comparison, file hashes, state/content identities

onix-cairn-radicle
  explicit profile/storage loading
  unique typed object inspection
  current canonical authority and serialized checkout guard

cairn-cli
  arguments, policy and capability-bounded local scan
  prepared writes, verification, atomic rename, post-check
  deterministic JSON receipts
```

## Non-Claims

Passing status proves only that the selected local Radicle storage reduced the stated object under the versioned Cairn reducer. Passing checkout proves only that the stated local bytes were materialized from that local reduced state under the recorded point-in-time authority observation. Neither proves network publication, peer availability, remote trust, gate or evidence truth, dependency readiness, future authority, source correctness, archive execution, release eligibility, or resistance to an unrelated process with equivalent host filesystem authority.
