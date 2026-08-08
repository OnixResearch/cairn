# Stack Provenance Gates Specification

## Purpose

Normalize function-address readiness input digests without weakening role or domain validation.

## Requirements

### Requirement: Supported digest spellings are producer-specific
r[cairn.stack_provenance_gates.function_address_digest_normalization.contract] Cairn MUST document supported digest spellings for Valence, Mantle, Kamacite, and graph function-address readiness fields.

#### Scenario: Supported spelling is normalized
r[cairn.stack_provenance_gates.function_address_digest_normalization.positive]
- GIVEN a readiness input field uses its documented digest spelling
- WHEN Cairn parses the function-address readiness inputs
- THEN Cairn MUST normalize the digest deterministically before readiness evaluation.

### Requirement: Digest spelling normalization stays outside role validation
r[cairn.stack_provenance_gates.function_address_digest_normalization.domain] Cairn MUST validate readiness field roles and identity domains separately from digest spelling normalization.

#### Scenario: Cross-field substitution fails
r[cairn.stack_provenance_gates.function_address_digest_normalization.negative]
- GIVEN a syntactically valid BLAKE3 digest appears in the wrong readiness field
- WHEN function-address readiness evaluation runs
- THEN Cairn MUST fail or report the evidence with a deterministic role/domain issue.

### Requirement: Malformed digest spellings fail closed
r[cairn.stack_provenance_gates.function_address_digest_normalization.shell] Cairn MUST reject malformed function-address readiness digests, including unsupported prefixes, wrong length, non-hex content, and unsupported case variants.

#### Scenario: Smoke validates normalized inputs
r[cairn.stack_provenance_gates.function_address_digest_normalization.validation]
- GIVEN the stack smoke uses documented producer spellings
- WHEN Cairn release-readiness runs
- THEN readiness MUST accept valid normalized inputs and reject invalid optional-present or required evidence.
