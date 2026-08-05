# Policy-exception collaboration

Cairn represents a policy exception with the `xyz.onixresearch.policy-exception` profile over the shared transport-neutral collaboration machinery.

## Commands

```bash
policy-exception request       --store DIR --exception-id ID --base-revision git_oid:OID \
                               --rules r1 --targets t1 --rationale X --controls c \
                               --requesters R --reviewers Q --approvers A --revokers W --observers O [--execute]
policy-exception status        --store DIR --exception-id ID
policy-exception approve|reject|revoke|expire|close --store DIR --exception-id ID --actor ACTOR [--reason R] [--execute]
policy-exception supersede     --store DIR --exception-id ID --actor ACTOR --successor ID [--execute]
policy-exception observe-validity --store DIR --exception-id ID --actor ACTOR --observed-epoch N --finite-valid-until N [--execute]
policy-exception conflict      --store DIR --exception-id ID
policy-exception export        --store DIR --exception-id ID
```

Every mutation is preview-first. Preview writes nothing.

## Scope and request binding

A request binds one exact policy identity, an explicit typed scope (known rule IDs and safe selectors), rationale, and compensating controls. Implicit wildcards, unsafe selectors, and unknown rule IDs fail closed. The request root is derived deterministically; a request revision advances it and makes prior review and approval facts inapplicable.

## Roles and decisions

Requester, reviewer, approver, revoker, and validity-observer role sets are immutable at the root. A requester cannot approve an exception. Approval binds one exact request root; conflicting decisions block applicability.

## Validity

Validity is admitted as explicit finite-bound observations from authorized observers, with no ambient clock access. An expired or revoked exception is inapplicable; a stale older observation cannot reverse an earlier expiration.

## Direct COB backend

The `xyz.onixresearch.policy-exception` profile is stored through `radicle-cob` in the shared direct adapter. Existing `cairn-change`, `rfd`, `adr`, `verification-obligation`, and `release-decision` COB bytes stay unchanged.

## Claim boundary

An approved policy-exception receipt does not grant authority. It proves only the recorded request, scope, identity on controls, configured role checks, decisions, explicit validity observations, and deterministic applicability classification. It does not prove policy truth, enforcement, real-world access, or release eligibility.
