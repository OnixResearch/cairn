# Developer Experience Specification

## Purpose

Defines user-facing Cairn workflow improvements that make deterministic lifecycle work easier to start without weakening policy ownership, native layout semantics, or receipt evidence.

## Requirements

### Requirement: Onboarding template catalog
r[cairn.developer_experience.onboarding_templates.catalog] The system MUST expose a reviewed onboarding template catalog for supported first-use repository profiles.

#### Scenario: List reviewed templates
r[cairn.developer_experience.onboarding_templates.catalog.list]
- GIVEN a Cairn policy bundle with reviewed onboarding template metadata
- WHEN the operator asks Cairn to list onboarding templates
- THEN the CLI emits a deterministic receipt containing sorted template identifiers, descriptions, supported files, and policy hash
- AND the receipt does not depend on filesystem traversal order, current time, network access, or agent-specific environment values

### Requirement: Onboarding template application
r[cairn.developer_experience.onboarding_templates.apply] The system MUST apply onboarding templates only through explicit safe preview or execute flows.

#### Scenario: Preview template materialization
r[cairn.developer_experience.onboarding_templates.apply.preview]
- GIVEN a repository root and a supported onboarding template identifier
- WHEN the operator previews template application
- THEN the CLI emits a deterministic plan receipt listing every file that would be created or skipped
- AND no filesystem mutation occurs

#### Scenario: Execute template materialization
r[cairn.developer_experience.onboarding_templates.apply.execute]
- GIVEN a repository root and a reviewed template plan with no blockers
- WHEN the operator executes template application
- THEN the CLI creates only safe root-relative template paths
- AND the mutation receipt records before and after BLAKE3 hashes for touched files

#### Scenario: Reject unsafe or conflicting template paths
r[cairn.developer_experience.onboarding_templates.apply.reject_unsafe]
- GIVEN a template expansion contains traversal, absolute paths, or an overwrite conflict without an explicit allowed policy
- WHEN the operator previews or executes the template
- THEN Cairn emits a deterministic failure receipt
- AND no unsafe path is written

### Requirement: Onboarding quickstart documentation
r[cairn.developer_experience.onboarding_templates.quickstart] The system MUST document a concise template-based first-use path for new Cairn repositories.

#### Scenario: Follow documented quickstart
r[cairn.developer_experience.onboarding_templates.quickstart.follow]
- GIVEN a user follows the documented onboarding template quickstart in a clean test repository
- WHEN they run the listed Cairn commands
- THEN the repository has a native `cairn/` lifecycle layout and at least one reviewable change package
- AND the documented validation command emits a passing deterministic receipt

### Requirement: Onboarding template verification
r[cairn.developer_experience.onboarding_templates.verification] The system MUST regression-test onboarding templates with deterministic positive and negative coverage.

#### Scenario: Verify template receipts
r[cairn.developer_experience.onboarding_templates.verification.receipts]
- GIVEN positive and negative onboarding template fixtures
- WHEN the workspace verification suite runs
- THEN supported templates produce stable passing receipts
- AND unsupported identifiers, unsafe paths, and overwrite conflicts produce stable failure issue codes
