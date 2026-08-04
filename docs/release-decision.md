# Release-decision collaboration

Cairn represents a release decision with the `xyz.onixresearch.release-decision` profile over the shared transport-neutral collaboration machinery.

## Commands

```bash
release-decision create     --store DIR --decision-id ID --base-revision git_oid:OID \
                            --source-revision S --artifact-manifest M --artifact-identity A \
                            --target-label T --release-label R \
                            --proposers P --reviewers Q --approvers A --withdrawers W --invalidators I [--execute]
release-decision status     --store DIR --decision-id ID
release-decision evidence   --store DIR --decision-id ID --actor ACTOR --evidence-id E --schema S \
                            --verification-role R --verdict V --external-identity X [--execute]
release-decision finding    --store DIR --decision-id ID --actor ACTOR --finding-id F [--blocker-class B] [--execute]
release-decision vote       --store DIR --decision-id ID --actor ACTOR --kind approve|reject [--execute]
release-decision withdraw|invalidate|supersede --store DIR --decision-id ID --actor ACTOR [--reason R] [--execute]
release-decision conflict   --store DIR --decision-id ID
release-decision export     --store DIR --decision-id ID
```

Every mutation is preview-first. Preview writes nothing.

## Candidate binding

A decision binds one immutable candidate descriptor (source revision, artifact manifest, artifact identity, target label, release label). The candidate root stays separate from the decision ID, provider object ID, release label, and artifact identity. Candidate substitution fails closed; a replacement candidate requires a distinct decision ID and an explicit supersession relation.

## Roles and votes

Proposer, reviewer, approver, withdrawer, and invalidator role sets are immutable at the root. A proposer cannot approve a candidate. Every vote binds one exact review-basis root derived from the candidate, policy, required evidence, findings, and blocker dispositions.

## Evidence and blockers

Evidence links preserve verification role and claim scope; recorded-only evidence cannot satisfy a property slot. Unresolved policy-blocking findings block approval. Concurrent blocker dispositions remain visible as explicit conflicts.

## Verdicts and lifecycle

Approval requires sufficient current approve votes with no reject vote or unresolved blocker; reject requires the reverse; both present is an explicit decision conflict. Withdrawal, invalidation (preserving the prior approval and review basis), and supersession (reported per-object, not atomic) follow policy rules.

## Direct COB backend

The `xyz.onixresearch.release-decision` profile is stored through `radicle-cob` in the shared direct adapter. Existing `cairn-change`, `rfd`, `adr`, and `verification-obligation` COB bytes stay unchanged.

## Claim boundary

An approved release-decision receipt does not execute a release, move a tag, or deploy. It proves only the recorded neutral state, configured role checks, evidence validation summaries, review-basis and vote identities, deterministic reduction, and a bounded lifecycle fact. It does not prove artifact publication, synchronization, remote trust, source correctness, or release eligibility.
