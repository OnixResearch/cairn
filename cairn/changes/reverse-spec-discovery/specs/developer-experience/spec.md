# Developer Experience Specification

## Purpose

Defines deterministic reverse-spec discovery features that help operators bootstrap Cairn lifecycle content from existing repository evidence without automatically accepting inferred requirements as truth.

## Requirements

### Requirement: Reverse-spec discovery policy
r[cairn.developer_experience.reverse_spec_discovery.policy] The system MUST own reverse-spec discovery scopes, limits, ignore classes, and public receipt fields in reviewed policy.

#### Scenario: Load discovery policy
r[cairn.developer_experience.reverse_spec_discovery.policy.load]
- GIVEN a generated Cairn policy bundle with reverse-spec discovery metadata
- WHEN the discovery command starts
- THEN the command uses policy-owned file count limits, byte limits, ignored path classes, redaction classes, and receipt field declarations
- AND missing or malformed policy produces a deterministic failure receipt

### Requirement: Reverse-spec discovery planner
r[cairn.developer_experience.reverse_spec_discovery.planner] The system MUST derive candidate requirements from explicit caller-provided repository evidence snapshots deterministically.

#### Scenario: Plan candidates from source snapshots
r[cairn.developer_experience.reverse_spec_discovery.planner.candidates]
- GIVEN sorted source, test, and documentation snapshots within policy limits
- WHEN Cairn plans reverse-spec discovery
- THEN the planner emits sorted candidate capabilities with stable identifiers, evidence references, review-needed status, and input hashes
- AND the planner performs no filesystem, environment, process, time, stdout/stderr, network, or JSON presentation effects

#### Scenario: Enforce discovery limits
r[cairn.developer_experience.reverse_spec_discovery.planner.limits]
- GIVEN repository evidence that exceeds policy-owned file count or byte limits
- WHEN Cairn plans reverse-spec discovery
- THEN the planner emits deterministic limit issues
- AND no candidate is marked accepted or verified

### Requirement: Reverse-spec discovery CLI
r[cairn.developer_experience.reverse_spec_discovery.cli] The system MUST expose reverse-spec discovery as a read-only command with optional draft artifact output that never mutates accepted specs.

#### Scenario: Emit discovery receipt
r[cairn.developer_experience.reverse_spec_discovery.cli.receipt]
- GIVEN explicit include paths in a repository
- WHEN the operator runs reverse-spec discovery
- THEN Cairn emits a deterministic receipt with selected paths, skipped paths, candidate requirements, policy hash, input hash, and receipt hash
- AND the command does not modify repository files unless an explicit output path for draft artifacts is provided

#### Scenario: Write draft artifacts only under explicit output
r[cairn.developer_experience.reverse_spec_discovery.cli.out]
- GIVEN a valid discovery plan and an explicit safe output path
- WHEN the operator requests draft artifact output
- THEN Cairn writes only the draft files declared in the receipt
- AND it does not modify `cairn/specs/` or promote candidates to accepted requirements

### Requirement: Reverse-spec discovery documentation
r[cairn.developer_experience.reverse_spec_discovery.docs] The system MUST document reverse-spec discovery as candidate generation requiring human or agent review.

#### Scenario: Docs state review-needed semantics
r[cairn.developer_experience.reverse_spec_discovery.docs.review_needed]
- GIVEN an operator reads reverse-spec discovery documentation
- WHEN they inspect the workflow
- THEN the documentation states that candidates are not accepted specs, proof of implementation, or verification evidence until reviewed through normal Cairn lifecycle gates

### Requirement: Reverse-spec discovery verification
r[cairn.developer_experience.reverse_spec_discovery.verification] The system MUST regression-test reverse-spec discovery for deterministic output, limits, redaction, and read-only behavior.

#### Scenario: Verify discovery fixtures
r[cairn.developer_experience.reverse_spec_discovery.verification.fixtures]
- GIVEN reviewed discovery fixtures with source, test, docs, oversized input, and sensitive-looking content
- WHEN the workspace verification suite runs
- THEN valid inputs produce stable candidate receipts
- AND oversized or redacted inputs produce stable issue codes without accepting inferred specs
