## ADDED Requirements

### Requirement: Evidence Registry Rail

Cairn MUST provide first-class deterministic evidence registry build and verification commands for aggregating published evidence indexes.

r[cairn.supersede.evidence_registry_rail.build]

#### Scenario: Registry build aggregates indexes

- GIVEN one or more deterministic evidence index receipt files
- WHEN an operator runs `cairn evidence registry build --root . --index <file> --out <file>`
- THEN Cairn emits a registry receipt with stable command, root, out, validity, verdict, issues, sorted entries, entry count, index hashes, bundle kinds, bundle receipt hashes, receipt chains, artifact hashes, and receipt hash fields

r[cairn.supersede.evidence_registry_rail.verify]

#### Scenario: Registry verification recomputes indexed evidence

- GIVEN a saved evidence registry receipt
- WHEN an operator runs `cairn evidence registry verify --root . --registry <file>`
- THEN Cairn recomputes every referenced index, validates the registry hash, compares each entry with current index evidence, and emits a deterministic verification receipt

r[cairn.supersede.evidence_registry_rail.failures]

#### Scenario: Registry commands reject unsafe or stale evidence

- GIVEN missing indexes, duplicate index hashes, malformed registry content, stale index entries, unsafe paths, or tampered artifacts behind an index
- WHEN registry build or verify runs
- THEN Cairn fails with deterministic issue codes and does not leak raw host paths, environment values, credentials, or secrets

r[cairn.supersede.evidence_registry_rail.gates]

#### Scenario: Registry rail gates pass

- GIVEN the registry implementation, policy schemas, docs, tests, and fixtures
- WHEN the required verification gates run
- THEN policy export, formatting, tests, clippy, validation, Tracey, coverage audit, release-readiness, evidence smoke commands, whitespace checks, and Nix flake checks pass

r[cairn.supersede.evidence_registry_rail.archive]

#### Scenario: Registry rail is archived with deterministic evidence

- GIVEN the registry rail tasks are complete
- WHEN the change is synced and archived
- THEN the accepted specification and dated archive retain the registry requirements, scenarios, and verification references
