# Current compact entry guidance delta

## ADDED Requirements

### Requirement: Source-bound entry guidance
r[cairn.agent_entry_guidance.generation] The system MUST derive a bounded entry guide from checked layout, workflow profile, command, policy, generator, and adapter contracts.

#### Scenario: Current contracts produce exact entry routes
- GIVEN an admitted catalog for the current native lifecycle layout and supported profiles
- WHEN Cairn renders the entry guide
- THEN it names those exact surfaces and routes stage details to existing workflow instruction commands

#### Scenario: Unknown or conflicting contracts cannot emit authoritative instructions
- GIVEN an unsupported command contract or inconsistent layout identity
- WHEN generation admits the source
- THEN it fails with bounded diagnostics rather than inserting guessed commands or legacy fallback paths

### Requirement: Lazy stage guidance with preserved constraints
r[cairn.agent_entry_guidance.scope] The system MUST keep entry text separate from selected stage details while preserving required authority, safety, and evidence constraints.

#### Scenario: One selected stage receives its required instructions
- GIVEN an admitted entry guide and a selected artifact stage
- WHEN the agent follows its typed instruction route
- THEN Cairn returns that stage's current constraints and prerequisites without unrelated workflow manuals

#### Scenario: A small text budget cannot omit mandatory boundaries
- GIVEN a configured guide bound below the required boundary and routing content
- WHEN the renderer evaluates the guide
- THEN it rejects the bound instead of removing mutation or completion constraints

### Requirement: Existing installation and freshness authority
r[cairn.agent_entry_guidance.install] The system MUST extend existing manifest and preview-first installation semantics to entry guides without user-global writes or silent unmanaged-file replacement.

#### Scenario: An unchanged reviewed plan installs current guidance
- GIVEN an admitted adapter, safe project-relative destination, and exact current plan hash
- WHEN the operator explicitly executes installation
- THEN the existing shell records source identities and publishes the planned guide through its guarded installation path

#### Scenario: A modified guide blocks replacement
- GIVEN a changed destination, modified generated guide, symlink, stale manifest, or interrupted installation
- WHEN installation or freshness inspection runs
- THEN it reports the existing conflict or recovery state and does not overwrite unrelated bytes

### Requirement: Maintained examples and copied-guide diagnostics
r[cairn.agent_entry_guidance.examples] The system MUST validate maintained guidance examples against current command contracts and report stale explicitly selected copied guides without rewriting them.

#### Scenario: Supported examples remain current across adapters
- GIVEN Agent Skills, Claude, and Codex fixtures from the same catalog
- WHEN deterministic contract checks run
- THEN supported layouts, profiles, routes, and semantic constraints agree across rendered examples

#### Scenario: Old layout guidance stays a report-only finding
- GIVEN an explicitly selected copied guide that uses an obsolete native layout
- WHEN the checker compares its declared source identity or maintained fixture
- THEN it reports stale guidance without user-home discovery, global installation, or inferred edit authority

### Requirement: Host-visible guidance acceptance
r[cairn.agent_entry_guidance.conformance] The system MUST compare host-exposed entry and stage guidance with the legacy workflow baseline under positive and negative scenario oracles.

#### Scenario: The declared host loads only the selected guidance
- GIVEN an explicit host adapter that records actual loaded instruction content
- WHEN the paired benchmark runs the same complete tasks
- THEN it measures entry and follow-up stage tokens with equal oracle outcomes

#### Scenario: A small file is not proof of a smaller prompt
- GIVEN the host loads both old and new guides or omits required stage instructions
- WHEN the benchmark evaluates the run
- THEN it reports the actual exposure and rejects unsupported savings or correctness claims
