## ADDED Requirements

### Requirement: Repository carries only the native lifecycle root
r[cairn.supersede.repo_native_only] The Cairn repository MUST NOT keep a top-level legacy OpenSpec lifecycle root as source-controlled lifecycle content once equivalent native Cairn content exists.

#### Scenario: Native-only repository doctor
r[cairn.supersede.repo_native_only.doctor]
- GIVEN the Cairn repository contains its native lifecycle content under `cairn/`
- WHEN an operator runs `cairn doctor --root .`
- THEN the selected layout is `cairn`
- AND the layout selection check does not warn that legacy and native roots are both present

### Requirement: Cairn-owned labels use Cairn identity
r[cairn.supersede.repo_native_labels] The repository MUST describe Cairn-owned packages, checks, and normal workflow examples with Cairn-primary wording rather than OpenSpec-primary labels.

#### Scenario: Local validation check naming
r[cairn.supersede.repo_native_labels.check]
- GIVEN the repository flake defines local verification checks for Cairn
- WHEN those checks are reviewed
- THEN Cairn validation checks use Cairn-primary names
- AND OpenSpec naming is limited to explicit legacy migration inputs or historical compatibility references

### Requirement: Legacy import remains explicit compatibility
r[cairn.supersede.repo_native_verification] The system MUST preserve explicit legacy OpenSpec import compatibility while making the Cairn repository itself native-only.

#### Scenario: Migration bridge preserved
r[cairn.supersede.repo_native_verification.import]
- GIVEN an external repository still has legacy `openspec/` lifecycle content
- WHEN an operator explicitly runs `cairn migrate openspec --root <path>`
- THEN Cairn treats that content as a migration source
- AND normal runtime operations on the Cairn repository do not require a checked-in top-level `openspec/` root
