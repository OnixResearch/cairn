## MODIFIED Requirements

### Requirement: Determinism replay oracle

The system MUST provide a deterministic replay oracle for agent and CI preflight evidence.

#### Scenario: Policy-owned entropy inventory

- GIVEN generated policy with replay cases and groups
- WHEN replay receipts are emitted
- THEN the receipts expose a stable policy-owned entropy inventory, entropy assumptions, and an entropy policy hash

r[cairn.supersede.determinism_entropy_inventory.policy]
r[cairn.supersede.determinism_entropy_inventory.receipt]

#### Scenario: Entropy inventory schema

- GIVEN public replay receipt schemas
- WHEN policy is exported and schema tests run
- THEN replay receipt stable fields include `entropy_inventory`, `entropy_assumptions`, and `entropy_policy_hash`

r[cairn.supersede.determinism_entropy_inventory.schema]

#### Scenario: Rejected entropy failures

- GIVEN a replay-relevant rejected entropy source is present in the environment
- WHEN `cairn determinism replay` runs
- THEN replay emits a deterministic nonzero failure receipt without exposing the source value

r[cairn.supersede.determinism_entropy_inventory.failures]

#### Scenario: Entropy inventory documentation and tests

- GIVEN agent and CI operators inspect Cairn determinism docs
- WHEN they read replay guidance or run focused replay tests
- THEN clock/date, locale/timezone, cwd/root paths, environment variables, filesystem metadata/order, external commands, and Nix/build context handling are documented and covered

r[cairn.supersede.determinism_entropy_inventory.docs]
r[cairn.supersede.determinism_entropy_inventory.tests]
r[cairn.supersede.determinism_entropy_inventory.verification]
r[cairn.supersede.determinism_entropy_inventory.archive]
