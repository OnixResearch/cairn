# Determinism Specification Delta

## ADDED Requirements

### Requirement: Preserves corpus has CI drift guards
r[cairn.determinism.preserves_corpus_ci_guard] Cairn MUST expose CI-visible checks that make Preserves canonicalization corpus drift fail closed.

#### Scenario: Clean corpus passes CI guard
r[cairn.determinism.preserves_corpus_ci_guard.clean_check]
- GIVEN the checked-in Preserves canonicalization corpus
- WHEN CI runs the Preserves corpus freshness guard
- THEN the guard runs the dry-run corpus update command
- AND the guard passes only when no stale hashes are detected

#### Scenario: Stale corpus fails CI guard
r[cairn.determinism.preserves_corpus_ci_guard.stale_detection]
- GIVEN a Preserves corpus with a deliberately stale expected hash
- WHEN CI runs the stale-detection guard
- THEN the dry-run corpus update command exits nonzero
- AND the guard passes only if stale drift is detected without mutation

#### Scenario: Preserves corpus guard validation is repeatable
r[cairn.determinism.preserves_corpus_ci_guard.validation]
- GIVEN the Preserves corpus CI guard checks
- WHEN Cairn validates the repository and runs Tracey coverage
- THEN the guard requirements have durable implementation and verification references
- AND the checks remain deterministic and secret-free
