# Check Evidence Producer Specification

## Purpose

Defines the `check-evidence-producer` capability.

## Requirements

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

### Requirement: Observed dependency applicability
r[cairn.check_evidence_producer.dependency_observation] The local producer MUST observe a bounded, explicit dependency-file projection using capability-relative no-follow reads and BLAKE3 identities that bind each path and its observed bytes.
The producer MUST require exact equality with the plan's dependency identity set before execution, around each child, before final publication, and during read-only manifest verification.
The producer MUST NOT replace a declared dependency set with an empty set or treat supplied digest assertions as observed file evidence.

#### Scenario: Current dependencies reach the original consumer
- GIVEN a reviewed source projection, dependency projection, completion contract, and explicit execution authority
- WHEN the selected checks pass and all observed dependency identities remain equal
- THEN the producer can publish current completion rows and a manifest accepted by the original consumers
- AND the receipt makes no dependency-closure or remote-source attestation claim

#### Scenario: Missing or changed dependencies remain blockers
- GIVEN a missing projection, invalid path, symbolic link, unavailable member, exceeded bound, or dependency identity drift
- WHEN the producer observes applicability
- THEN it rejects the affected execution or withholds complete publication
- AND a changed dependency invalidates read-only manifest verification

### Requirement: Native Nix build payload admission
r[cairn.check_evidence_producer.native_nix_build] The local Nix producer MUST execute a fixed offline, no-link JSON build of the consumer-owned expression and independently admit the native derivation/output payload before a successful process can satisfy a build obligation.
The producer MUST retain the admitted native payload for independent manifest admission, preserve build-only meaning with zero executed-test claims, and reject unsupported, empty, malformed, truncated, or incomplete payloads.
The producer MUST NOT promote an evaluated value, failing payload, process exit alone, or opaque store-path realization into a passed derivation build.

#### Scenario: A real successful build reaches completion
- GIVEN a real Nix derivation build, a complete supported native JSON payload, unchanged applicability, and complete teardown
- WHEN the producer normalizes and durably publishes the attempt
- THEN the original completion and manifest consumers accept the exact required build result
- AND neither consumer treats the build as executed-test coverage or proof of fresh builder execution

#### Scenario: Process success without native build evidence remains incomplete
- GIVEN zero exit with an empty, unsupported, incomplete, or non-build value, or a failed, timed-out, cancelled, or overflowing build
- WHEN the producer evaluates and publishes the attempt
- THEN the affected completion row is non-passing and no complete manifest is admitted

### Requirement: Exact maintained consumer command profiles
r[cairn.check_evidence_producer.maintained_consumer_commands] The producer MUST support only the closed additional vectors `cargo test --workspace --all-targets --locked --no-fail-fast`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo fmt --all -- --check`, and `nix flake check -L`, with immutable explicit executable identities, explicit environment, empty features and unchanged applicability, dependency, prerequisite and publication bindings.
Formatting, Clippy and flake checks MUST retain process-check-only evidence, false native payload admission and zero executed tests. Workspace tests MUST retain strict test-summary admission. Existing native Nix build admission MUST remain unchanged.
Only the new flake-check kind MAY admit the maintained 10,800,000 ms deadline; every other kind MUST retain the 3,600,000 ms maximum. Legacy streams MUST retain 1 MiB maxima; only the new workspace-test kind MAY admit 8 MiB per stream, Clippy 8 MiB stderr, and flake-check 64 MiB stderr. All other streams and teardown limits MUST remain unchanged. Only the exact reviewed four-line NIX_CONFIG value MAY carry newlines, only on the flake-check kind; no host configuration is modified.

#### Scenario: Maintained checks reach completion without invented evidence
- GIVEN exact reviewed profiles and a fully observed, unchanged source and dependency set
- WHEN each bounded command succeeds with complete natural-exit and teardown observations
- THEN completion and independent manifest admission preserve command identities and role distinctions
- AND process checks supply no native build payload or test-coverage claims

#### Scenario: Unsupported commands, observations and limits fail closed
- GIVEN altered arguments, nonempty features, forged payload or coverage, incomplete process observations, a failed process, or exceeded kind-specific bounds
- WHEN planning, normalization or manifest verification admits the attempt
- THEN the affected command cannot satisfy completion

