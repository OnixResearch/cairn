# Ears Vocabulary Specification

## Purpose

Defines the `ears-vocabulary` capability.

## Requirements

### Requirement: EARS response vocabulary classification
r[cairn.ears_vocabulary] The system MUST classify EARS requirement responses against a fixed vocabulary of pure-boolean predicates, returning `Resolved` or `RecordedOnly`.

#### Scenario: Non-negative response resolves
r[cairn.ears_vocabulary.non_negative]
- GIVEN the response "produce non-negative call prices"
- WHEN classified
- THEN it resolves to the predicate `is_non_negative`

#### Scenario: Unresolvable response is recorded-only
r[cairn.ears_vocabulary.unresolvable]
- GIVEN the response "make users happy"
- WHEN classified
- THEN it is `RecordedOnly`

### Requirement: Strict mode fails on unresolved responses
r[cairn.ears_vocabulary.strict] In strict mode, the coverage report MUST be invalid if any response is unresolved.

#### Scenario: Strict mode fails with unresolved
r[cairn.ears_vocabulary.strict.fail]
- GIVEN two responses where one resolves and one does not
- WHEN checked in strict mode
- THEN the report is invalid and the unresolved response is listed

#### Scenario: Strict mode passes when all resolved
r[cairn.ears_vocabulary.strict.pass]
- GIVEN two responses that both resolve
- WHEN checked in strict mode
- THEN the report is valid

### Requirement: Vocabulary includes c-earchin nine and trellis-shaped predicates
r[cairn.ears_vocabulary.predicates] The vocabulary MUST include the c-earchin nine predicates plus trellis-shaped predicates for systems domains.

#### Scenario: Vocabulary is stable
r[cairn.ears_vocabulary.predicates.stable]
- GIVEN the vocabulary predicates list
- WHEN inspected
- THEN it includes `is_non_negative`, `is_monotone_increasing`, `preserves_invariant`, `no_deadlock`, and `linearizable`

### Requirement: Non-claim boundary
r[cairn.ears_vocabulary.boundary] The coverage report MUST carry a non-claim boundary stating that unresolved responses are recorded_only and prove nothing.

#### Scenario: Boundary present
r[cairn.ears_vocabulary.boundary.present]
- GIVEN an EarsVocabularyReport
- WHEN inspected
- THEN it carries a boundary field stating "records which requirement responses resolve"
