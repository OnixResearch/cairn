# Radicle Status and Checkout CLI Specification

## MODIFIED Requirements

### Requirement: Deterministic exact checkout planning
r[cairn.radicle_status_checkout.checkout_plan] The system MUST deterministically compare the complete conflict-free materialized COB view with a capability-bounded local active-change snapshot before checkout execution without requiring profile or identity-document authority.

#### Scenario: Absent or exact destination is classified safely
r[cairn.radicle_status_checkout.checkout_plan.safe]
- GIVEN the local active-change destination is absent or byte-identical to the materialized COB view
- WHEN Cairn plans checkout
- THEN it MUST classify absent state as checkout-ready and exact state as an idempotent no-op.

#### Scenario: Divergent or unsafe destination is blocked
r[cairn.radicle_status_checkout.checkout_plan.divergent]
- GIVEN local files are partial, divergent, oversized, non-UTF-8, symlinked, or unsupported
- WHEN Cairn plans checkout
- THEN it MUST report deterministic blockers and MUST NOT plan overlay or destructive replacement.

## REMOVED Requirements

### Requirement: Read-only typed Radicle status
r[cairn.radicle_status_checkout.status] This profile-backed status requirement MUST be removed in favor of COB-only status.

### Requirement: Current-authority checkout guard
r[cairn.radicle_status_checkout.checkout_authority] This identity-document checkout authority requirement MUST be removed.

### Requirement: Preview-first atomic checkout
r[cairn.radicle_status_checkout.checkout_execution] This `cairn radicle` execution requirement MUST be removed in favor of COB-only checkout.

### Requirement: Deterministic status and checkout receipts
r[cairn.radicle_status_checkout.receipts] This identity-document receipt requirement MUST be removed in favor of COB-only receipts.

### Requirement: Explicit claim boundary
r[cairn.radicle_status_checkout.non_claims] This profile-specific claim requirement MUST be removed in favor of the COB-only claim boundary.
