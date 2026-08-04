# RFD document collaboration

Cairn can represent an RFD as a transport-neutral collaborative document with an `xyz.onixresearch.rfd` profile.

The pure document-collaboration core keeps a stable document ID (separate from any provider object ID), exact artifacts, typed links, bounded discussion, lifecycle, evidence, revision DAGs, explicit conflicts, and immutable author/participant/reviewer/publisher role sets.

## Preview-first commands

```bash
rfd import   --store DIR --document-id ID --base-revision git_oid:OID \
             --authors A --participants P --reviewers R --publishers Q \
             --spec spec.md [--execute]
rfd status   --store DIR --document-id ID
rfd update   --store DIR --document-id ID --actor ACTOR --kind discussion|artifact --text T [--execute]
rfd conflict --store DIR --document-id ID
rfd resolve  --store DIR --document-id ID --actor ACTOR --text T [--execute]
rfd number   --store DIR --document-id ID --number N --registry SNAPSHOT_ID --publisher ACTOR
rfd checkout --store DIR --document-id ID --out DIR [--execute]
rfd publish  --store DIR --document-id ID --out DIR
```

Every mutation is preview-first. Preview writes nothing. Execution rechecks state and roles before mutation.

## Role authority

Roles come only from immutable root role sets. An author can edit artifacts. A participant can add discussion. A reviewer performs lifecycle transitions. A publisher assigns a number and publishes. Ambient keys never grant a role.

## Identity and numbering

The RFD document ID is independent of the provider object ID. A publication number is a set-once publisher action bound to an explicit local registry snapshot, the current content root, the policy identity, and the publisher actor. Local number validation does not claim global uniqueness.

## Conflicts

Concurrent artifact edits preserve all current heads as an explicit conflict. A resolution must name every current head. A conflicted RFD cannot publish.

## Claim boundary

A passing RFD receipt proves only the recorded neutral state, selected local backend observations, configured role checks, deterministic reduction, and a bounded materialization plan. It does not prove publication, synchronization, global number uniqueness, remote trust, semantic review quality, link truth, source correctness, or release eligibility.

<!-- r[impl cairn.document_collaboration.semantic_core.docs] -->
<!-- r[impl cairn.document_collaboration.policy] -->
<!-- r[impl cairn.document_collaboration.policy.deterministic] -->
<!-- r[impl cairn.document_collaboration.policy.invalid] -->
<!-- r[impl cairn.document_collaboration.radicle_adapter.boundary] -->
<!-- r[impl cairn.document_collaboration.validation.negative] -->
<!-- r[impl cairn.rfd_cob.interop] -->
<!-- r[impl cairn.rfd_cob.materialization.unsafe] -->
<!-- r[impl cairn.rfd_cob.profile] -->
<!-- r[impl cairn.rfd_cob.receipts] -->
<!-- r[impl cairn.rfd_cob.receipts.acceptance] -->
<!-- r[impl cairn.rfd_cob.receipts.backend] -->
<!-- r[impl cairn.rfd_cob.validation] -->
<!-- r[impl cairn.rfd_cob.validation.release] -->
<!-- r[impl cairn.document_collaboration.reducer.docs] -->
<!-- r[impl cairn.document_collaboration.portable.docs] -->
<!-- r[impl cairn.rfd_cob.profile.docs] -->
<!-- r[impl cairn.rfd_cob.authority.docs] -->
<!-- r[impl cairn.rfd_cob.identity.docs] -->
<!-- r[impl cairn.rfd_cob.cli.docs] -->
<!-- r[impl cairn.rfd_cob.materialization.docs] -->
<!-- r[impl cairn.rfd_cob.receipts.docs] -->
<!-- r[impl cairn.document_collaboration.validation.docs] -->
