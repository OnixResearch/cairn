# Cairn Policy Reuse License Specification

## ADDED Requirements

### Requirement: Named policy source and projection offer a permissive choice

r[cairn.license_boundary.policy_reuse_grant] Cairn MUST offer its `cairn-policy/default.ncl`, `contracts.ncl`, `lifecycle-store-contracts.ncl`, `nominal-evidence-identities.ncl`, and `generated/cairn-policy.json` under `AGPL-3.0-or-later OR MIT OR Apache-2.0`, both at the current revision and as exact copies from commit `b6759012db4dab0158936e995b7bad39ac10465b`.

#### Scenario: A consumer copies historical policy content

- GIVEN a consumer has byte-for-byte copies of the five named files from the pinned Cairn commit
- WHEN the consumer inspects Cairn's policy license grant
- THEN it MAY choose MIT for those copies and redistribute them with the applicable terms and notices
- AND the consumer does not need to change the original Git blobs to receive that license choice.

### Requirement: The policy grant does not extend to the Cairn CLI

r[cairn.license_boundary.policy_reuse_scope] The grant MUST identify the exact covered paths and historical blobs, ship its license texts, and MUST NOT claim to relicense `cairn-cli`, the exporter/toolchain, other Cairn policy files, or third-party content.

#### Scenario: A recipient reviews the licensing boundary

- GIVEN the grant and Cairn's license inventory are present
- WHEN the recipient compares covered paths with the pinned Git objects and CLI metadata
- THEN the five exact policy objects SHALL remain identifiable and the CLI SHALL retain its `AGPL-3.0-or-later` license
- AND license inventory validation SHALL reject a missing or wrongly licensed grant.
