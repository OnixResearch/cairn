# ADR document collaboration

Cairn represents an Architecture Decision Record (ADR) with the `xyz.onixresearch.adr` profile over the shared transport-neutral document core.

## Commands

```bash
adr import   --store DIR --document-id ID --base-revision git_oid:OID \
             --authors A --reviewers R --approvers Q --publishers P \
             --decision decision.md [--execute]
adr status   --store DIR --document-id ID
adr update   --store DIR --document-id ID --actor ACTOR --text T [--execute]
adr accept   --store DIR --document-id ID --actor APPROVER [--execute]
adr deprecate --store DIR --document-id ID --actor ACTOR [--execute]
adr supersede --store DIR --document-id ID --actor ACTOR --replaces ADR_ID [--execute]
adr conflict --store DIR --document-id ID
adr resolve  --store DIR --document-id ID --actor ACTOR --text T [--execute]
adr number   --store DIR --document-id ID --number N --registry SNAPSHOT_ID --publisher ACTOR
adr publish  --store DIR --document-id ID --out DIR
```

Every mutation is preview-first. Preview writes nothing.

## Roles and immutability

Author, reviewer, approver, and publisher role sets are immutable at the root. An approver performs acceptance. Once a decision is accepted its source artifacts are frozen: any later artifact edit is rejected as an `ImmutableDecision`.

## Identity and numbering

The ADR document ID is independent of any provider object ID. A publication number is a set-once publisher action bound to an explicit local registry snapshot. Local number validation does not claim global uniqueness.

## Lineage and supersession

Typed links record supersession, reciprocal lineage expectations, RFD origin, related decisions, and implementation evidence. Superseding an ADR does not claim cross-object atomicity.

## Direct COB backend

The `xyz.onixresearch.adr` profile is stored through `radicle-cob` in the shared direct adapter. Existing `xyz.onixresearch.cairn-change` and `xyz.onixresearch.rfd` refs, commits, and bytes remain unchanged.

## Claim boundary

A passing ADR receipt proves only the recorded neutral state, selected local backend observations, configured role checks, deterministic reduction, and a bounded materialization plan. It does not prove remote publication, synchronization, global number uniqueness, semantic review quality, link truth, source correctness, or release eligibility.
