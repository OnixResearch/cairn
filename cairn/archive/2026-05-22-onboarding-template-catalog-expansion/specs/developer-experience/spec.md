# Developer Experience Specification

## Purpose

Improve first-use onboarding by expanding the reviewed template catalog with concrete docs-only and release-evidence profiles.

## Requirements

### Requirement: Expanded onboarding template catalog
r[cairn.developer_experience.onboarding_template_catalog_expansion.catalog] The system MUST expose additional reviewed onboarding templates for docs-only and release-evidence repository profiles.

#### Scenario: List expanded templates
r[cairn.developer_experience.onboarding_template_catalog_expansion.catalog.list]
- GIVEN an operator runs the template listing command
- WHEN Cairn renders the onboarding catalog
- THEN docs-only and release-evidence profile identifiers are present with stable descriptions
- AND existing template identifiers remain available

### Requirement: Expanded template application
r[cairn.developer_experience.onboarding_template_catalog_expansion.apply] The system MUST apply expanded templates through the existing preview-first safe write semantics.

#### Scenario: Preview expanded template
r[cairn.developer_experience.onboarding_template_catalog_expansion.apply.preview]
- GIVEN an operator previews a new template
- WHEN Cairn emits the apply receipt
- THEN the receipt lists planned files and content hashes
- AND no filesystem mutation occurs without `--execute`

### Requirement: Expanded onboarding documentation
r[cairn.developer_experience.onboarding_template_catalog_expansion.docs] The system MUST document the expanded template catalog and its non-goals.

#### Scenario: Read expanded onboarding docs
r[cairn.developer_experience.onboarding_template_catalog_expansion.docs.read]
- GIVEN an operator reads Cairn's onboarding command examples
- WHEN templates are described
- THEN docs-only and release-evidence examples are included
- AND the text avoids remote publishing, hosted UI, and signing claims

### Requirement: Expanded onboarding verification
r[cairn.developer_experience.onboarding_template_catalog_expansion.verification] The system MUST verify the expanded template catalog with deterministic regression coverage.

#### Scenario: Run expanded template tests
r[cairn.developer_experience.onboarding_template_catalog_expansion.verification.run]
- GIVEN focused developer-experience CLI tests
- WHEN template listing and apply behavior are exercised
- THEN the new templates produce deterministic receipts
- AND unknown-template failure behavior remains fail-closed
