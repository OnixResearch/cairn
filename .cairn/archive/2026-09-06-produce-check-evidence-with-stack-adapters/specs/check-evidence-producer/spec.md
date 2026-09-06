# Check evidence producer

## ADDED Requirements

### Requirement: Repository-owned producer selection
r[cairn.check_evidence_producer.contract] The system MUST bind production to a repository-owned required-check contract, exact subject, policy, tool cohort, target, feature set, and selected command identities.

#### Scenario: An admitted plan preserves the required set
- GIVEN a current completion contract and reviewed check profile
- WHEN the producer derives a plan
- THEN the plan contains the exact required checks and dependency order with a BLAKE3 identity

#### Scenario: Producer rows cannot weaken requirements
- GIVEN an unknown check, duplicate check, missing prerequisite, or producer-supplied replacement selection
- WHEN the producer admits the request
- THEN it rejects the request without execution or completion evidence

### Requirement: Explicit bounded local execution
r[cairn.check_evidence_producer.local_run] The system MUST execute local checks only through reviewed argv-based profiles with explicit authority, executable identity, environment, directory, limits, and teardown policy.

#### Scenario: Authorized checks produce bounded observations
- GIVEN an unchanged reviewed plan and an admitted executable
- WHEN the operator explicitly authorizes the local run
- THEN Bounded Exec supplies bounded process observations for the exact requested commands

#### Scenario: Denial and uncertain teardown cannot report success
- GIVEN a changed executable, denied authority, timeout, output overflow, or failed teardown
- WHEN the producer processes the attempt
- THEN it records the applicable non-passing outcome and creates no successful completion row

### Requirement: Producer-owned meaning survives translation
r[cairn.check_evidence_producer.adapters] The system MUST translate only supported pinned producer contracts while preserving native status, verification role, coverage scope, and non-claims.

#### Scenario: Current producer evidence reaches its existing consumer
- GIVEN admitted Kiln, Lattice, Mantle, or Octet observations with matching contract identities
- WHEN the selected adapter creates Cairn result rows
- THEN the rows preserve producer meaning and pass the applicable existing Cairn consumer checks

#### Scenario: A build or pending workflow cannot become a passed test
- GIVEN a build-only receipt, queued workflow, warning-only lint report, or zero exit with a failing native payload
- WHEN the adapter evaluates a required test or strict-check obligation
- THEN it retains a non-passing classification for that obligation

### Requirement: Current applicability and cache provenance
r[cairn.check_evidence_producer.applicability] The system MUST reject mismatched subjects, policies, targets, features, dependency identities, and unsupported cache claims before a result can satisfy completion.

#### Scenario: Exact executed evidence remains applicable
- GIVEN complete passing observations for the final subject and selected cohort
- WHEN the existing completion validator consumes the generated results
- THEN it admits the exact selected-check set without a producer-defined exemption

#### Scenario: Stale or unproven cached results remain blockers
- GIVEN a source change or a cache claim without the required execution provenance
- WHEN the producer imports prior evidence
- THEN it rejects applicability or records a non-passing result without a fabricated current run

### Requirement: Durable safe evidence publication
r[cairn.check_evidence_producer.publication] The system MUST preserve attempt identity, publication state, redaction, and explicit uncertainty when it publishes bounded result artifacts.

#### Scenario: Complete artifacts publish before their final manifest
- GIVEN a complete observed attempt and a safe authorized output root
- WHEN the producer publishes its evidence set
- THEN the final manifest references only published artifacts with exact BLAKE3 identities

#### Scenario: Partial output cannot masquerade as a complete attempt
- GIVEN interrupted publication, permission failure, secret-like diagnostics, or changed destination bytes
- WHEN the producer finalizes the attempt
- THEN it withholds complete success and preserves explicit failure or recovery evidence

### Requirement: Independent producer conformance
r[cairn.check_evidence_producer.conformance] The system MUST retain positive and negative local-run and saved-import fixtures plus an independent consumer replay against existing completion and loop validators.

#### Scenario: Fresh consumer needs no bespoke receipt script
- GIVEN the pinned producer and maintained Cairn or Kiln consumer fixture
- WHEN the fixture produces and consumes successful check evidence
- THEN the original validators accept the generated artifacts without hand-authored result rows

#### Scenario: Failure controls remain incomplete
- GIVEN failed, skipped, cancelled, malformed, truncated, missing, duplicated, or substituted producer results
- WHEN the same consumer replay evaluates those controls
- THEN completion remains blocked and no failed status becomes passing
