## ADDED Requirements

### Requirement: Runtime action record schema
r[cairn.runtime_evidence.records] The system MUST define a deterministic runtime action record schema for agent trust-boundary actions with subject identity, action metadata, authorization decision, capture origin, bounded input and outcome identities, integrity links, and explicit non-claims.

#### Scenario: Valid record carries bounded action evidence
r[cairn.runtime_evidence.records.valid]
- GIVEN a runtime action record with a subject, action class, authorization decision, capture origin, input hash, outcome hash, previous record hash, and record hash
- WHEN Cairn validates the record
- THEN the record is accepted as bounded runtime evidence
- AND the validation result states that the record does not prove agent correctness or policy enforcement.

#### Scenario: Malformed record fails closed
r[cairn.runtime_evidence.records.malformed]
- GIVEN a runtime action record with a missing subject, unsupported action class, malformed hash, or missing non-claim boundary
- WHEN Cairn validates the record
- THEN Cairn emits a deterministic failure issue
- AND the issue does not include raw prompts, raw tool arguments, credentials, or host-specific paths.

### Requirement: Subject-scoped runtime chains
r[cairn.runtime_evidence.subject_scope] The system MUST verify runtime action chains per subject so evidence for one subject is not silently mixed with evidence for another subject.

#### Scenario: Single-subject chain verifies
r[cairn.runtime_evidence.subject_scope.single]
- GIVEN a runtime action chain whose records all carry the same subject identity
- WHEN Cairn verifies the chain
- THEN the receipt includes the subject, chain root hash, record count, head hash, and integrity verdict.

#### Scenario: Mixed-subject chain is rejected
r[cairn.runtime_evidence.subject_scope.mixed]
- GIVEN a runtime action chain with records from multiple subjects and no policy-declared aggregation profile
- WHEN Cairn verifies the chain
- THEN Cairn rejects the chain with a stable mixed-subject issue code.

### Requirement: Capture origin classification
r[cairn.runtime_evidence.capture_origin] The system MUST classify runtime record capture origin without upgrading weaker evidence into stronger evidence.

#### Scenario: Boundary-captured record stays boundary-captured
r[cairn.runtime_evidence.capture_origin.boundary]
- GIVEN a record whose capture origin is `boundary_captured`
- WHEN Cairn summarizes the runtime chain
- THEN the summary counts that record as boundary-captured evidence.

#### Scenario: Ingested telemetry is not upgraded
r[cairn.runtime_evidence.capture_origin.ingested]
- GIVEN a record whose capture origin is `ingested_telemetry`, `wrapper_asserted`, or `unknown`
- WHEN Cairn summarizes the runtime chain
- THEN the summary preserves the weaker origin class
- AND it does not count that record as boundary-captured evidence.

### Requirement: Hash-only and redacted runtime content
r[cairn.runtime_evidence.hash_only] Runtime records MUST support hash-only input and outcome identities and MUST reject raw secret-bearing summaries in shareable evidence receipts.

#### Scenario: Hash-only record verifies
r[cairn.runtime_evidence.hash_only.valid]
- GIVEN a runtime record containing input and outcome hashes without human-readable summaries
- WHEN Cairn validates the record
- THEN the record is accepted as shareable hash-only evidence.

#### Scenario: Secret-bearing summary fails safely
r[cairn.runtime_evidence.hash_only.secret]
- GIVEN a runtime record summary containing credential-like, token-like, or raw private path-like data
- WHEN Cairn validates the record for a shareable evidence profile
- THEN Cairn rejects the record with a stable content-boundary issue
- AND the failure receipt does not echo the sensitive text.

### Requirement: External witness checkpoint schema
r[cairn.runtime_evidence.external_witness] The system MUST define a deterministic witness checkpoint schema that binds a subject, chain root hash, record count, head hash, witness identity metadata, and checkpoint hash.

#### Scenario: Matching checkpoint proves witnessed prefix
r[cairn.runtime_evidence.external_witness.match]
- GIVEN a verified runtime chain and a witness checkpoint for the same subject, chain root, record count, and head hash
- WHEN Cairn compares the checkpoint with the chain
- THEN the receipt reports a witnessed prefix verdict and includes the matching checkpoint hash.

#### Scenario: Wrong-subject checkpoint fails
r[cairn.runtime_evidence.external_witness.subject_mismatch]
- GIVEN a verified runtime chain and a checkpoint for a different subject
- WHEN Cairn compares the checkpoint with the chain
- THEN Cairn reports a deterministic subject-mismatch issue.

### Requirement: Completeness verdicts distinguish local integrity from witnessed completeness
r[cairn.runtime_evidence.completeness_verdicts] The runtime evidence verifier MUST emit deterministic verdict classes that distinguish local chain integrity from externally witnessed completeness evidence.

#### Scenario: No checkpoint is integrity-only
r[cairn.runtime_evidence.completeness_verdicts.integrity_only]
- GIVEN a valid runtime action chain with no supplied external checkpoint
- WHEN Cairn verifies runtime evidence
- THEN the receipt reports an integrity-only verdict
- AND it states that completeness was not witnessed.

#### Scenario: Truncated chain fails against checkpoint
r[cairn.runtime_evidence.completeness_verdicts.truncated]
- GIVEN a witness checkpoint whose record count is greater than the presented chain length
- WHEN Cairn compares the checkpoint with the chain
- THEN the receipt reports a truncated-chain verdict and fails closed.

#### Scenario: Altered prefix fails against checkpoint
r[cairn.runtime_evidence.completeness_verdicts.altered]
- GIVEN a witness checkpoint whose head hash does not match the presented record at the checkpoint count
- WHEN Cairn compares the checkpoint with the chain
- THEN the receipt reports an altered-before-checkpoint verdict and fails closed.

### Requirement: Runtime evidence CLI
r[cairn.runtime_evidence.cli] The system MUST expose runtime evidence chain and checkpoint verification through a deterministic CLI command.

#### Scenario: CLI verifies valid runtime evidence
r[cairn.runtime_evidence.cli.valid]
- GIVEN a valid runtime action chain and matching witness checkpoint artifacts
- WHEN the operator runs `cairn runtime-evidence verify` with those artifacts
- THEN Cairn emits a deterministic JSON receipt with validity, verdict, subject, chain root hash, head hash, checkpoint hash, issue list, and receipt hash.

#### Scenario: CLI failure remains path-safe
r[cairn.runtime_evidence.cli.path_safe]
- GIVEN malformed runtime evidence paths or artifacts containing host-specific strings
- WHEN the CLI reports a verification failure
- THEN the receipt uses stable issue codes and classes
- AND it does not include raw absolute paths, raw environment values, credentials, or secrets.

### Requirement: Policy-owned runtime evidence schemas
r[cairn.runtime_evidence.policy_schema] Runtime evidence receipt fields, allowed capture-origin values, checkpoint fields, hash algorithm labels, and issue codes MUST be declared in Nickel-owned policy instead of hidden CLI literals.

#### Scenario: Public schemas include runtime evidence fields
r[cairn.runtime_evidence.policy_schema.public]
- GIVEN generated Cairn policy
- WHEN public receipt schemas are inspected
- THEN the runtime evidence verification receipt schema lists stable top-level fields and artifact fields.

#### Scenario: Unknown policy value fails closed
r[cairn.runtime_evidence.policy_schema.unknown]
- GIVEN a runtime record with a capture-origin value not declared by policy
- WHEN Cairn validates the record
- THEN Cairn emits a deterministic unknown-capture-origin issue.

### Requirement: Runtime evidence bundle and manifest roles
r[cairn.runtime_evidence.manifest_roles] Agent evidence bundles and evidence manifests SHOULD support optional runtime-chain and witness-checkpoint artifact roles while preserving existing bundle and manifest validation semantics.

#### Scenario: Bundle links runtime evidence artifacts
r[cairn.runtime_evidence.manifest_roles.bundle]
- GIVEN an agent evidence bundle with runtime-chain and witness-checkpoint artifact roles
- WHEN Cairn verifies the bundle
- THEN the bundle verifier validates the referenced runtime evidence receipt and records its receipt hash.

#### Scenario: Manifest links runtime evidence artifacts
r[cairn.runtime_evidence.manifest_roles.manifest]
- GIVEN an evidence manifest with runtime-chain and witness-checkpoint artifact roles
- WHEN Cairn verifies the manifest
- THEN the manifest verifier validates the declared artifact hashes and records the runtime evidence verification receipt hash.

#### Scenario: Invalid runtime evidence does not weaken existing validators
r[cairn.runtime_evidence.manifest_roles.invalid]
- GIVEN a bundle or manifest with invalid runtime evidence artifacts
- WHEN Cairn verifies the bundle or manifest
- THEN Cairn reports deterministic runtime evidence issues
- AND it does not bypass replay, release-readiness, bundle, or manifest validators.

### Requirement: Runtime evidence documentation
r[cairn.runtime_evidence.docs] Cairn MUST document runtime evidence workflows, privacy rules, capture-origin meanings, Halo/AARM compatibility boundaries, and the integrity-versus-completeness non-claim.

#### Scenario: Operator can distinguish evidence claims
r[cairn.runtime_evidence.docs.claims]
- GIVEN Cairn's README and agent-run receipt documentation
- WHEN an operator reads the runtime evidence workflow
- THEN the docs explain that local chains prove integrity only
- AND the docs explain that independently controlled witness checkpoints are required for bounded completeness evidence.

#### Scenario: Compatibility boundary is explicit
r[cairn.runtime_evidence.docs.compatibility]
- GIVEN documentation for imported Halo/AARM-style runtime records
- WHEN an operator reads the compatibility section
- THEN the docs state that external hash algorithms and canonical forms are preserved as imported metadata
- AND Cairn still computes BLAKE3 identities for Cairn-owned receipts.

### Requirement: Runtime evidence tests
r[cairn.runtime_evidence.tests] Cairn MUST test valid and invalid runtime evidence records, chains, checkpoints, CLI receipts, bundle roles, and manifest roles.

#### Scenario: Tests cover positive and negative runtime evidence
r[cairn.runtime_evidence.tests.coverage]
- GIVEN Cairn's core and CLI test suites
- WHEN runtime evidence tests run
- THEN they cover valid chains, malformed records, mixed subjects, hash-only records, secret-bearing summaries, matching checkpoints, wrong-subject checkpoints, truncated chains, altered prefixes, stale checkpoints, and invalid bundle or manifest roles.

### Requirement: Runtime evidence gates
r[cairn.runtime_evidence.gates] Cairn MUST pass focused validation gates before runtime evidence behavior is treated as complete.

#### Scenario: Focused checks pass
r[cairn.runtime_evidence.gates.focused]
- GIVEN the runtime evidence implementation is complete
- WHEN policy export, formatting, core tests, CLI tests, Cairn validation, runtime fixtures, and release-readiness run
- THEN all checks pass or any pre-existing unrelated failures are explicitly recorded.

### Requirement: Runtime evidence lifecycle archive
r[cairn.runtime_evidence.archive] Cairn MUST sync and archive the runtime evidence change only after implementation, validation evidence, and Tracey coverage are complete.

#### Scenario: Archive preserves runtime evidence requirements
r[cairn.runtime_evidence.archive.ready]
- GIVEN runtime evidence implementation and verification are complete
- WHEN the change is synced and archived
- THEN accepted specs retain the runtime evidence requirements
- AND Tracey coverage passes for the new requirement IDs.
