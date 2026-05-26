# Determinism Specification Delta

## ADDED Requirements

### Requirement: Preserves corpus freshness is policy-inventoried
r[cairn.determinism.preserves_corpus_policy_inventory.surface] Cairn MUST list the Preserves corpus freshness rail in policy-owned deterministic surface inventory.

#### Scenario: Corpus freshness has a non-mutating replay case
r[cairn.determinism.preserves_corpus_policy_inventory.replay_case]
- GIVEN the checked-in Preserves canonicalization corpus
- WHEN Cairn exports deterministic replay policy
- THEN policy SHALL include a non-mutating corpus freshness case
- AND replay SHALL evaluate the dry-run freshness path without `--execute`.

#### Scenario: Corpus policy inventory is verified
r[cairn.determinism.preserves_corpus_policy_inventory.tests]
r[cairn.determinism.preserves_corpus_policy_inventory.validation]
- GIVEN the Preserves corpus policy inventory case
- WHEN policy export, coverage audit, replay selection, Cairn validation, and Tracey coverage run
- THEN the surface and replay case SHALL be covered by durable verification.
