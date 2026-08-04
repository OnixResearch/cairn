# Verification-obligation collaboration

Cairn represents a verification obligation with the `xyz.onixresearch.verification-obligation` profile over the shared transport-neutral workflow core.

## Commands

```bash
verification-obligation import    --store DIR --obligation-id ID --base-revision git_oid:OID \
                                  --submitters A --contributors C --verifiers V --invalidators I \
                                  --claim CLAIM_ID --decision claim.md [--execute]
verification-obligation status    --store DIR --obligation-id ID
verification-obligation update    --store DIR --obligation-id ID --actor CONTRIBUTOR --text T [--execute]
verification-obligation evidence  --store DIR --obligation-id ID --actor ACTOR --evidence-id E --evidence-kind K \
                                  --verification-role R --claim-scope S --external-identity X [--execute]
verification-obligation finding   --store DIR --obligation-id ID --actor ACTOR --finding-id F [--execute]
verification-obligation response  --store DIR --obligation-id ID --actor VERIFIER --finding-id F --response R [--execute]
verification-obligation discharge|reject|invalidate --store DIR --obligation-id ID --actor VERIFIER [--execute]
verification-obligation conflict  --store DIR --obligation-id ID
verification-obligation resolve   --store DIR --obligation-id ID --actor ACTOR --text T [--execute]
verification-obligation export    --store DIR --obligation-id ID
```

Every mutation is preview-first. Preview writes nothing.

## Roles and claim binding

Submitter, contributor, verifier, and invalidator role sets are immutable at the root. Discharge and rejection require the verifier role; invalidation requires the invalidator role. A submitter cannot discharge an obligation.

The obligation binds an exact claim root. A claim revision advances the claim-root revision, making a prior discharge inapplicable without deleting history.

## Findings and discharge

Findings carry status, blocker class, source span, response, and disposition. An unresolved blocking finding prevents discharge. Discharge requires a conflict-free state. A discharged obligation is immutable: later field edits are rejected.

## Evidence

Evidence references preserve their verification role and claim scope. Recorded-only evidence cannot be promoted to property or proof evidence.

## Direct COB backend

The `xyz.onixresearch.verification-obligation` profile is stored through `radicle-cob` in the shared direct adapter. Existing `cairn-change`, `rfd`, and `adr` COB refs, commits, and bytes stay unchanged.

## Claim boundary

A passing verification-obligation receipt proves only the recorded neutral state, selected local backend observations, configured role checks, deterministic reduction, and a bounded evidence/decision surface. It does not prove remote synchronization, evidence truth, proof soundness, semantic correctness, or release eligibility.
