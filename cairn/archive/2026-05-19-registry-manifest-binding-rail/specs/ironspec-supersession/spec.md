# ironspec-supersession delta: registry manifest binding rail

## ADDED Requirements

### Requirement: Registry Manifest Artifact Roles

The system MUST support stable evidence manifest artifact roles for registry artifacts and registry verification or consumption receipts.

r[cairn.supersede.registry_manifest_binding.roles]

#### Scenario: Registry roles are stable and portable

- GIVEN a registry file, registry verification receipt, registry selection receipt, and registry fetch-plan receipt
- WHEN the operator generates an evidence manifest that includes those artifacts
- THEN the manifest SHALL record stable registry artifact roles, BLAKE3 hashes, schema versions, and portable metadata without raw absolute paths or secret values.

### Requirement: Registry Manifest Generation

The system MUST allow evidence manifest generation to bind an already-built evidence registry and its verification/consumption receipts into the manifest artifact index.

r[cairn.supersede.registry_manifest_binding.generate]

#### Scenario: Generate manifest with registry evidence

- GIVEN an exported evidence bundle, a published evidence index, an evidence registry, and registry verification output
- WHEN the operator runs manifest generation with registry inputs
- THEN the command SHALL emit a deterministic evidence manifest containing both the existing evidence artifacts and the registry artifacts.

### Requirement: Registry Manifest Verification

The system MUST verify registry artifacts referenced by evidence manifests using existing registry verification semantics before accepting the manifest.

r[cairn.supersede.registry_manifest_binding.verify]

#### Scenario: Verify manifest-bound registry

- GIVEN an evidence manifest containing a registry artifact and registry verification receipt
- WHEN the operator verifies the manifest
- THEN the command SHALL rehash manifest artifacts, verify the registry, and reject mismatched or tampered registry evidence with deterministic issue codes.

### Requirement: Registry Manifest Failure Receipts

The system MUST emit deterministic, secret-safe failure receipts when registry manifest binding validation fails.

r[cairn.supersede.registry_manifest_binding.failure]

#### Scenario: Tampered registry fails manifest verification

- GIVEN a manifest-bound registry whose content no longer matches the manifest artifact hash or registry verification semantics
- WHEN the operator verifies the manifest
- THEN the command SHALL report deterministic issues without leaking raw absolute roots, credentials, environment values, or secret material.

### Requirement: Registry Manifest Policy Coverage

The system MUST keep registry manifest binding schemas and verification requirements covered by Rust/Nickel gates and Tracey references.

r[cairn.supersede.registry_manifest_binding.policy]

#### Scenario: Policy and Tracey remain fresh

- GIVEN registry manifest binding changes to manifest schema or public receipt fields
- WHEN policy export, strict validation, and Tracey coverage run
- THEN the generated policy SHALL be fresh and every registry manifest binding requirement SHALL be referenced by implementation or verification evidence.
