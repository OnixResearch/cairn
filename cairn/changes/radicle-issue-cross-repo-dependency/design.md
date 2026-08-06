# Design: Radicle issue cross-repo linkage

## Context

Cairn has a cross-repo dependency model. A change declares rows in `cross_repo_dependencies`. Each row names a repo, an upstream change, a required receipt kind, an expected status, optional hash constraints, and non-claims. The evaluator in `cairn-core/src/verified/cross_repo_dependencies.rs` blocks a change until matching explicit evidence exists.

The `SUPPORTED_RECEIPT_KINDS` allowlist has only Cairn lifecycle kinds. There is no way to link a change to a Radicle issue COB today.

## Goals

- Link a Cairn change to a Radicle issue as a first-class cross-repo dependency.
- Keep issue evidence as a bounded read-only observation.
- Reuse the existing cross-repo evaluator, blocker, and non-claim machinery.
- Require explicit evidence inputs with no implicit transport.

## Non-Goals

- Model the Radicle-native issue COB as Cairn lifecycle state.
- Let Cairn author or reduce an issue.
- Add node, peer, discovery, sync, or convergence behavior.
- Change local `depends_on` ordering.

## Decisions

### Decision: Add a `radicle-issue.v1` receipt kind

**Choice:** Add `radicle-issue.v1` to `SUPPORTED_RECEIPT_KINDS` in `cross_repo_dependencies.rs`. The evaluator already handles new kinds generically once admission accepts them.

**Rationale:** The evaluator validates the kind against the allowlist. Adding the kind is the smallest change that makes issue evidence a first-class receipt.

### Decision: Define a bounded issue receipt contract

**Choice:** Define `cairn.radicle-issue-receipt.v1`. A receipt binds the upstream repo identity, issue id, COB object id, observed status, subject hash, expected status, and receipt hash.

The contract reuses the existing `CrossRepoReceiptSummary` shape so the core evaluator needs no new identity logic.

**Rationale:** A typed contract keeps the linkage deterministic and auditable without new evaluator paths.

### Decision: Keep the issue adapter read-only

**Choice:** Add a `radicle_issue` decoder to `cairn-radicle-cob`. It decodes a supported issue action history in a local COB store and emits a bounded observation. It does not reduce the issue into lifecycle state and it does not write an update.

**Rationale:** Cairn does not own the issue reducer. A read-only observation respects the claim boundary.

### Decision: Require explicit evidence inputs

**Choice:** The CLI loads issue receipts only from explicit `--cross-repo-evidence` files or imported bundles. It does not start a node, connect to a peer, or fetch a remote by default.

**Rationale:** This matches the existing no-fetch rule for cross-repo evidence.

### Decision: Reuse existing blockers and non-claims

**Choice:** Unsatisfied issue rows surface through the existing read-only blocker list and diagnostic path. The issue receipt carries the same class of non-claims as other cross-repo evidence.

**Rationale:** Cairn already treats receipt linkage separately from upstream correctness, remote trust, and release eligibility.

## Rejected Alternatives

### Model the issue as a Cairn-native COB type

This would duplicate the issue reducer and claim authority Cairn does not own. The read-only adapter stays.

### Add the full `radicle` crate dependency

This would pull profiles, identity documents, and node types across the existing boundary. The adapter stays COB-only.

### Allow implicit issue fetching

This would add node and transport behavior that Cairn explicitly does not provide. Evidence stays explicit.

## Test Design

Positive core tests cover a matching open issue receipt and a matching closed issue receipt for the new kind. Positive shell tests cover explicit receipt loading through `--cross-repo-evidence`.

Negative core tests cover missing, stale, wrong-repo, wrong-issue, wrong-kind, unsupported-kind, malformed, and hash-mismatch issues. Negative shell tests confirm no mutation and the no-fetch default.

## Risks and Trade-offs

- A new receipt kind widens the allowlist. The evaluator still rejects every other unknown kind.
- An issue status maps to the receipt expected status. The adapter must map the observed Radicle state to an explicit status.
- The word "issue" is a Radicle COB type name, not a Cairn validation finding. Documentation keeps these meanings separate.

## Claim Boundary

A satisfied `radicle-issue.v1` row proves only that explicit typed evidence matches a declared linkage.

It does not prove issue correctness, closure authority, peer convergence, remote trust, downstream integration correctness, or release eligibility.

## Source Adaptation

The change reuses the accepted `change-dependency-graph` cross-repo dependency contract. It extends the receipt-kind allowlist and adds a bounded read-only adapter without changing existing receipt kinds, hashes, or lifecycle mutation behavior.
