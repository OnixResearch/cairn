# Hash Algorithm Policy Specification

## Purpose

Defines BLAKE3 defaults and explicit hash algorithm exceptions for stack evidence.

## Requirements

### Requirement: Hash policy contract
r[cairn.hash_algorithm_policy.contract] Cairn MUST define a hash policy contract with BLAKE3 as the default for stack-owned content identity and explicit exception rows for non-BLAKE3 algorithms.

#### Scenario: BLAKE3 default passes
r[cairn.hash_algorithm_policy.fixtures.positive]
- GIVEN an evidence schema uses BLAKE3 for stack-owned content identity fields
- WHEN hash policy validation runs
- THEN validation MUST pass without requiring an exception row.

#### Scenario: Undeclared SHA256 fails
r[cairn.hash_algorithm_policy.fixtures.negative]
- GIVEN an evidence schema introduces a SHA256 identity field without a declared interop or legacy exception
- WHEN hash policy validation runs
- THEN validation MUST fail with an undeclared-hash diagnostic.

### Requirement: Hash policy validation
r[cairn.hash_algorithm_policy.validation] Cairn MUST validate configured schema/evidence roots against the hash policy and emit deterministic diagnostics.

#### Scenario: Declared exception is bounded
r[cairn.hash_algorithm_policy.validation.exception]
- GIVEN a SHA256 field is declared as a legacy or interop exception with owner, reason, scope, and non-claims
- WHEN validation runs
- THEN the exception MUST be reported as accepted only for the declared scope.

### Requirement: Hash policy documentation
r[cairn.hash_algorithm_policy.docs] Cairn documentation MUST state that hash exceptions preserve identity compatibility only and do not prove artifact truth or semantic correctness.

#### Scenario: Non-claim is visible
r[cairn.hash_algorithm_policy.docs.non_claims]
- GIVEN a report accepts a non-BLAKE3 exception
- WHEN the supported claim is rendered
- THEN it MUST state that the exception is scope-bound and not a general approval of weaker hashing for new stack-owned identity.

### Requirement: Final validation
r[cairn.hash_algorithm_policy.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers hash policy
r[cairn.hash_algorithm_policy.final_validation.fixtures]
- GIVEN BLAKE3, declared SHA256, undeclared SHA256, malformed digest, and missing-owner fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.
