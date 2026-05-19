## ADDED Requirements

### Requirement: Registry Channel Metadata Labels
The system MUST support deterministic, policy-validated channel metadata on evidence registry entries.

r[cairn.supersede.registry_channel_metadata.labels]

#### Scenario: Stable labels are recorded
- GIVEN a registry build command with channel, release, platform, and compatibility-class labels
- WHEN the registry is built
- THEN each registry entry records the stable labels without raw host paths, environment values, credentials, or secrets

#### Scenario: Invalid labels are rejected
- GIVEN a registry build command with a label outside policy constraints
- WHEN the registry is built
- THEN the command fails with a deterministic issue code and emits a portable failure receipt

### Requirement: Registry Channel Metadata Policy
The system MUST define registry label constraints in policy-owned data rather than hidden helper scripts or ad hoc host configuration.

r[cairn.supersede.registry_channel_metadata.policy]

#### Scenario: Policy defines allowed labels
- GIVEN generated policy
- WHEN registry label validation runs
- THEN allowed channel, platform, and compatibility-class values are read from policy-owned data

### Requirement: Registry Channel Metadata Build Semantics
The system MUST bind valid channel metadata into registry build and verify receipts while preserving deterministic duplicate/conflict behavior.

r[cairn.supersede.registry_channel_metadata.build]

#### Scenario: Duplicate channel tuple conflict
- GIVEN two registry entries with the same non-empty channel, release, platform, and compatibility-class tuple but different index hashes
- WHEN the registry is built
- THEN the command rejects the registry with a deterministic conflict issue

### Requirement: Registry Channel Metadata Consumption
The system MUST allow registry select and fetch-plan commands to filter by stable channel metadata.

r[cairn.supersede.registry_channel_metadata.consume]

#### Scenario: Select by channel metadata
- GIVEN a verified registry with one entry matching requested channel metadata
- WHEN registry select runs with matching channel selectors
- THEN it copies exactly the selected index and emits a portable receipt

#### Scenario: Fetch-plan by channel metadata
- GIVEN a verified registry with entries matching requested channel metadata
- WHEN registry fetch-plan runs with matching channel selectors
- THEN it emits deterministic metadata-only fetch and verify instructions

### Requirement: Registry Channel Metadata Verification
The system MUST test and document channel metadata behavior through Rust/Nickel gates and Tracey-covered requirements.

r[cairn.supersede.registry_channel_metadata.verify]

#### Scenario: Verification gates cover metadata behavior
- GIVEN the registry channel metadata implementation
- WHEN the targeted and final gates run
- THEN policy freshness, schema coverage, Tracey coverage, Rust tests, clippy, and flake checks pass without checked-in Python or shell helper scripts
