# Determinism Specification Delta

## ADDED Requirements

### Requirement: Preserves workflow docs tests are isolated from corpus behavior tests
r[cairn.determinism.preserves_workflow_test_split.docs_target] Cairn MUST keep Preserves workflow documentation coverage in a focused test surface separate from corpus mutation behavior checks.

#### Scenario: Corpus tests remain behavior-focused
r[cairn.determinism.preserves_workflow_test_split.corpus_focus]
- GIVEN Preserves corpus update integration tests
- WHEN documentation workflow assertions are split out
- THEN corpus tests SHALL continue to cover clean, stale, execute, and malformed corpus behavior without docs-only coupling.

#### Scenario: Split tests preserve coverage
r[cairn.determinism.preserves_workflow_test_split.tests]
r[cairn.determinism.preserves_workflow_test_split.validation]
- GIVEN the split Preserves workflow tests
- WHEN focused tests, Cairn validation, Tracey coverage, and diff checks run
- THEN all previous docs/runbook assertions and live Tracey references SHALL remain covered.
