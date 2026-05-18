## ADDED Requirements

### Requirement: Canonical Cairn Hermes workflow skill
r[cairn.supersede.hermes_skill.canonical_cairn] The system MUST maintain the repository workflow skill under the canonical Cairn identity rather than an OpenSpec or Ironspec primary name.

#### Scenario: Canonical repo skill path
r[cairn.supersede.hermes_skill.canonical_cairn.path]
- GIVEN the repository carries a copyable Hermes workflow skill source
- WHEN a future agent inspects `docs/hermes/`
- THEN the canonical artifact is named `cairn-workflow-skill.md`

#### Scenario: Canonical live skill
r[cairn.supersede.hermes_skill.canonical_cairn.live]
- GIVEN a Hermes session is working in `/home/brittonr/git/cairn`
- WHEN it loads a project workflow skill
- THEN the skill name is `cairn-workflow`
- AND the skill describes Cairn-native lifecycle, MCP, receipt, release-readiness, Tracey, Slotcar, and Nix verification contracts

### Requirement: Legacy workflow skill redirect
r[cairn.supersede.hermes_skill.redirect] The system MUST preserve a safe compatibility path from older Ironspec/OpenSpec workflow skill names to the canonical Cairn workflow skill.

#### Scenario: Older skill points to Cairn
r[cairn.supersede.hermes_skill.redirect.local]
- GIVEN a future session loads the older `ironspec-workflow` skill while working on Cairn
- WHEN the skill body is read
- THEN it directs the agent to use `cairn-workflow` for Cairn repository work
- AND it does not present OpenSpec or Ironspec as the primary Cairn identity
