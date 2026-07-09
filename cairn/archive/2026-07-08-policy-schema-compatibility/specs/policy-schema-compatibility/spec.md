# Policy Schema Compatibility Specification

## Purpose

Defines compatibility handling for generated Cairn policy JSON consumed by downstream repos.

## Requirements

### Requirement: Policy schema metadata
r[cairn.policy_schema_compatibility.metadata] Cairn generated policy JSON MUST include enough metadata to identify the policy schema version, producer identity, field-set identity, and refresh command.

#### Scenario: Current policy metadata passes
r[cairn.policy_schema_compatibility.fixtures.positive]
- GIVEN generated policy JSON includes current schema metadata and required fields
- WHEN Cairn validates the policy metadata
- THEN validation MUST classify the policy as current.

#### Scenario: Missing metadata fails closed
r[cairn.policy_schema_compatibility.fixtures.negative]
- GIVEN generated policy JSON omits schema metadata or required field-set identity
- WHEN Cairn validates the policy metadata
- THEN validation MUST fail with deterministic diagnostics naming the missing field.

### Requirement: Compatibility classifier
r[cairn.policy_schema_compatibility.classifier] Cairn MUST classify known generated policy shapes as current, compatible-with-warning, refresh-required, or unsupported before validating lifecycle content.

#### Scenario: Refresh-required policy is actionable
r[cairn.policy_schema_compatibility.diagnostics]
- GIVEN a downstream repo has a known stale generated policy that lacks fields required by the current Cairn binary
- WHEN validation runs
- THEN Cairn MUST report `policy-refresh-required` with the policy path and refresh command instead of reporting only a parse failure.

#### Scenario: Unsupported policy fails closed
r[cairn.policy_schema_compatibility.classifier.unsupported]
- GIVEN a generated policy declares an unknown or unsupported schema version
- WHEN validation runs
- THEN Cairn MUST fail closed with an unsupported-policy diagnostic.

### Requirement: Validation evidence
r[cairn.policy_schema_compatibility.validation] The change MUST include positive and negative policy fixtures plus focused validation evidence before archive.

#### Scenario: Fixture matrix covers compatibility states
r[cairn.policy_schema_compatibility.validation.fixtures]
- GIVEN current, compatible, refresh-required, missing-field, and unsupported-version fixtures
- WHEN focused validation runs
- THEN each fixture MUST produce the expected classification and receipt hash.
