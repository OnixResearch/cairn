# Lifecycle Review Diff Pilot Delta

## ADDED Requirements

### Requirement: The diff candidate remains test-only

r[cairn.review_diff_pilot.boundary] The pilot MUST NOT change default CLI, view, sync, archive, receipt, hash, or review output behavior.

#### Scenario: The pilot runs

- GIVEN two admitted lifecycle text artifacts and one pilot profile
- WHEN candidate comparison runs
- THEN it MUST use test-only paths and MUST NOT mutate lifecycle state

#### Scenario: Pilot success requests default use

- GIVEN the pilot meets its selected policy
- WHEN default adoption is proposed
- THEN a separate Cairn MUST define compatibility, selection, migration, rollback, and output versioning

### Requirement: The baseline binds current review output

r[cairn.review_diff_pilot.baseline] Every corpus case MUST bind the exact source pair, current baseline implementation, profile, and output identity.

#### Scenario: A complete baseline is admitted

- GIVEN source, implementation, profile, and output identities are current
- WHEN baseline admission runs
- THEN it MUST admit one comparable baseline result

#### Scenario: Baseline output is stale

- GIVEN the source or implementation identity changed
- WHEN comparison begins
- THEN it MUST reject the stored baseline

### Requirement: Policy owns corpus, limits, and decisions

r[cairn.review_diff_pilot.policy] Typed Nickel policy MUST own required fixtures, input and output limits, structural thresholds, owner review, and decision rules.

#### Scenario: Policy is complete

- GIVEN every required bound and decision field is present
- WHEN policy admission runs
- THEN it MUST admit the exact policy identity

#### Scenario: A required limit is absent

- GIVEN the policy omits an input, output, hunk, line, word, or runtime bound
- WHEN policy admission runs
- THEN it MUST reject the policy

### Requirement: Candidates preserve exact changed bytes

r[cairn.review_diff_pilot.candidates] Patience matching and word refinement MAY change presentation. They MUST preserve every exact changed span and MUST remain deterministic.

#### Scenario: Unique anchors improve locality

- GIVEN a lifecycle document has stable unique lines around one local change
- WHEN patience matching runs
- THEN it MAY produce a more local hunk with complete exact changes

#### Scenario: Word refinement hides a change

- GIVEN word refinement omits or normalizes an unequal byte range
- WHEN exactness validation runs
- THEN it MUST reject the candidate output

### Requirement: Numeric tolerance is prohibited

r[cairn.review_diff_pilot.exactness] The pilot MUST compare numbers exactly. It MUST NOT use approximate or tolerance-based numeric equality.

#### Scenario: One numeric digit changes

- GIVEN two lifecycle sources differ by one numeric digit
- WHEN any candidate runs
- THEN the output MUST identify that exact changed digit or containing span

#### Scenario: Numeric tolerance is requested

- GIVEN a profile or implementation enables numeric tolerance
- WHEN admission runs
- THEN it MUST reject the candidate

### Requirement: The corpus is lifecycle-specific and bounded

r[cairn.review_diff_pilot.corpus] The corpus MUST cover requirements, scenarios, tasks, moved sections, tables, code blocks, identifiers, numbers, whitespace, line endings, binary input, invalid text, and bounds.

#### Scenario: The complete corpus runs

- GIVEN all required positive and negative fixtures are present
- WHEN corpus validation runs
- THEN every case MUST produce its declared output or rejection class

#### Scenario: Input exceeds policy

- GIVEN an artifact exceeds a declared byte, line, word, or hunk bound
- WHEN candidate admission runs
- THEN it MUST return resource exhaustion without partial trusted output

### Requirement: Metrics remain deterministic and bounded

r[cairn.review_diff_pilot.metrics] The pilot MUST record changed-span completeness, hunk stability, locality, output size, and bounded resource observations.

#### Scenario: Equivalent runs repeat

- GIVEN equal source, candidate, profile, and corpus identities
- WHEN the pilot runs again
- THEN deterministic outputs and structural metrics MUST be equal

#### Scenario: Clock data enters the core

- GIVEN candidate meaning depends on an ambient clock observation
- WHEN architecture validation runs
- THEN it MUST reject that dependency from the pure comparator

### Requirement: Owner review is bounded and non-authoritative

r[cairn.review_diff_pilot.review] Owner review MAY record readability preferences. It MUST NOT replace exactness, determinism, bound, or negative-case gates.

#### Scenario: Review prefers an inexact candidate

- GIVEN owner review prefers a candidate that fails exactness
- WHEN the pilot decision runs
- THEN it MUST reject the candidate despite the preference

### Requirement: The pilot returns a closed decision

r[cairn.review_diff_pilot.decision] The pilot MUST return adopt-candidate, reject-candidate, or insufficient-evidence with exact policy and evidence identities.

#### Scenario: All selected conditions pass

- GIVEN exactness, determinism, corpus, bounds, metrics, and review conditions pass
- WHEN the decision runs
- THEN it MAY return adopt-candidate without changing defaults

#### Scenario: Evidence is incomplete

- GIVEN a required fixture, metric, review, identity, or negative case is missing
- WHEN the decision runs
- THEN it MUST return insufficient-evidence

### Requirement: Verification preserves non-claims

r[cairn.review_diff_pilot.verification] Focused validation MUST cover all decisions and MUST state that corpus results do not prove universal review quality.

#### Scenario: Focused pilot gates run

- GIVEN policy, candidates, oracle, corpus, metrics, review, and decisions are present
- WHEN focused validation runs
- THEN every positive and negative case MUST pass with bounded non-claims
