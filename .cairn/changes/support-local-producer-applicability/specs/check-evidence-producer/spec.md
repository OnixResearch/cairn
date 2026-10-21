# Check evidence producer delta

## ADDED Requirements

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
Only the new flake-check kind MAY admit the maintained 10,800,000 ms deadline; every other kind MUST retain the 3,600,000 ms maximum, and all stream and teardown limits MUST remain unchanged. Only the exact reviewed four-line NIX_CONFIG value MAY carry newlines, only on the flake-check kind; no host configuration is modified.

#### Scenario: Maintained checks reach completion without invented evidence
- GIVEN exact reviewed profiles and a fully observed, unchanged source and dependency set
- WHEN each bounded command succeeds with complete natural-exit and teardown observations
- THEN completion and independent manifest admission preserve command identities and role distinctions
- AND process checks supply no native build payload or test-coverage claims

#### Scenario: Unsupported commands, observations and limits fail closed
- GIVEN altered arguments, nonempty features, forged payload or coverage, incomplete process observations, a failed process, or exceeded kind-specific bounds
- WHEN planning, normalization or manifest verification admits the attempt
- THEN the affected command cannot satisfy completion

