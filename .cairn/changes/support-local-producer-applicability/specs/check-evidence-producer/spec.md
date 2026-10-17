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
