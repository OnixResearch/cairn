# Design: Radicle Import CLI

## Goal and Success Contract

The exact goal is a preview-first operator path that losslessly imports one existing native Cairn active change into `xyz.onixresearch.cairn-change`, and that refuses execution unless the signing actor is a delegate in the repository's current canonical Radicle identity document.

Completion requires:

- an exact pure import plan over proposal, design, tasks, metadata, and keyed delta-spec bytes;
- an explicit Radicle home and repository ID, with no hidden `RAD_HOME` discovery;
- no Radicle ref mutation during preview or blocked execution;
- current canonical identity and delegate checks immediately before signed creation;
- a created COB that can be loaded through the typed store and has the previewed semantic content root; and
- positive and negative tests plus focused, workspace, lifecycle, purity, traceability, and Nix checks.

False completion includes checking only the operation's historical identity document, trusting a caller-supplied delegate boolean, invoking an external `rad` helper, mutating during preview, emitting an object ID without loading the typed result, or claiming live peer synchronization.

## Approach Registry

| Family | Mechanism | State | Reason |
|---|---|---|---|
| in-process canonical authority | Read `canonical_identity_head`, load that document, and test the profile actor before opening a signed typed store | active | Uses repository-owned current authority and is testable without a subprocess |
| operation identity only | Rely on the identity document embedded in the new operation | falsified | A retired delegate can cite a historical document; replay authority is not current mutation authority |
| external `rad` helper | Spawn a CLI/helper to create or inspect a COB | falsified | The local CLI is unavailable, subprocess bounds are weaker, and helper repository context is unsuitable |
| caller assertion | Accept an `is_delegate` flag or delegate list from command input | falsified | Self-asserted authorization is not repository authority |

## Retrieval and Audit Budget

The implementation budget is three review rounds, two materially distinct authority mechanisms, one adversarial secondary review, five targeted Radicle/Cairn API reads before broader retrieval, focused tests before and after core changes, one full workspace rail, and one Nix rail. Audit risks are stale identity selection, check/write races, actor/signer mismatch, path traversal in delta-spec IDs, unbounded artifact reads, duplicate imports, policy drift, secret profile-path disclosure, and unsupported transport claims.

## Decisions

### Decision: make import preview-first

`cairn radicle import <change> --root PATH --rad-home PATH --rid RID [--policy PATH]` emits a plan by default. `--execute` is the only mode that loads a signer and creates a COB. Both modes inspect current repository identity and report whether the actor is currently authorized.

### Decision: require explicit Radicle context

The command requires `--rad-home` and `--rid`. It does not infer profile state from `RAD_HOME`, invoke `rad`, or derive a repository from a working-tree remote. This keeps authority and storage selection visible in automation.

### Decision: use canonical current authority

The adapter computes the repository's canonical identity head, loads that exact identity document, verifies the profile public key is a current delegate, and binds the checked identity OID into the signed root operation. The execute path performs this check inside the same adapter call immediately before opening the typed write store.

This is a point-in-time local check, not a lease. A concurrent identity update can occur after the check; the receipt records the checked identity OID and does not claim continuous authorization.

### Decision: keep file mapping pure

The CLI shell reads bounded UTF-8 artifacts through the repository capability boundary and passes a typed snapshot to `cairn-core`. The pure planner maps standard artifacts and stable delta-spec IDs into `ChangeCobInit`, validates protocol limits, and permits only exact native lifecycle artifact classes.

### Decision: bind preview and execution

The preview computes the semantic `content_root`, artifact hashes, policy hash, base Git OID, actor, repository ID, and current identity OID. Execution rebuilds the same plan from current inputs, performs an initial current-authority check, acquires an atomic per-repository/per-change lock under the explicit Radicle home, rechecks authority and duplicate state while serialized, creates the typed object, reloads it, and rejects any content-root mismatch before reporting success. A second local process fails before signed creation while that lock is held. An unclean process termination can leave a stale local lock that requires operator removal; Cairn fails closed rather than guessing that the owner is dead.

### Decision: inventory namespaced refs before typed reload

Radicle 0.24's generic `Store::all()` silently omitted a newly created valid custom object in the local repository test topology even though `Store::get(object_id)` loaded it. Duplicate detection therefore inventories exact `refs/namespaces/*/refs/cobs/xyz.onixresearch.cairn-change/<object-id>` refs through the in-process Git backend, deduplicates object IDs, and then uses the typed store to evaluate each candidate. Raw ref presence is discovery evidence only; only successful typed reduction supplies a Cairn change ID.

## Functional Core / Imperative Shell

```text
cairn-core
  typed import snapshot -> validated ChangeCobInit
  artifact mapping, limits, semantic content root

onix-cairn-radicle
  explicit profile loading
  canonical identity lookup and delegate check
  signer acquisition, typed Store creation, typed reload

cairn-cli
  argument parsing, capability-bounded file reads
  policy loading, preview/execute orchestration
  deterministic JSON receipt
```

## Non-Claims

A passing import receipt proves that the stated local bytes were accepted into a locally stored signed typed COB under the recorded point-in-time canonical identity document. It does not prove gate validity, evidence truth, dependency readiness, network publication, peer availability, future authority, source correctness, archive execution, or release eligibility.
