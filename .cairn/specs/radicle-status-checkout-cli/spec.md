# Radicle Status Checkout Cli Specification

## Purpose

Defines the `radicle-status-checkout-cli` capability.

## Requirements

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
