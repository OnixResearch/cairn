# Release Evidence Delta

## ADDED Requirements

### Requirement: Producer release evidence publication has a stable local materialization contract

r[release_evidence.publish_materialization.cli_contract] The system MUST provide a first-class producer-side contract for materializing verified release evidence into a deterministic local publication tree without requiring operators to hand-copy artifacts.

#### Scenario: CLI contract names explicit source and destination inputs

r[release_evidence.publish_materialization.cli_contract.inputs_outputs]
- GIVEN an operator has generated current-HEAD release evidence and wants a producer publication tree
- WHEN the operator invokes the publication materialization command
- THEN the command accepts explicit root, source index, source artifact directory, and publish output directory arguments
- AND it documents optional channel, release, and retention metadata as local labels rather than remote-publication claims

### Requirement: Producer release evidence publication composes existing evidence validators

r[release_evidence.publish_materialization.composes_existing_validators] The system MUST verify source evidence and materialized outputs through existing release-evidence, evidence-index, and artifact hash validation semantics rather than duplicating independent release validators.

#### Scenario: Existing validators remain authoritative for source evidence

r[release_evidence.publish_materialization.composes_existing_validators.source_evidence]
- GIVEN the command receives a release evidence index and generated artifacts
- WHEN it validates source material before publication
- THEN it uses the existing evidence index and release-evidence validation semantics for the source evidence
- AND it does not introduce a second implementation of readiness, bundle, index, or hash verification rules

### Requirement: Producer release evidence publication emits a deterministic local manifest and receipt

r[release_evidence.publish_materialization.manifest_receipt] The system MUST emit deterministic local manifest and receipt artifacts that bind the payload commit, source index hash, materialized artifact roles, safe relative output paths, BLAKE3 hashes, label metadata, and verdict.

#### Scenario: Materialized output binds every copied artifact

r[release_evidence.publish_materialization.manifest_receipt.artifact_binding]
- GIVEN the command completes successfully
- WHEN a reviewer inspects the publication manifest and receipt
- THEN every materialized file is listed with a stable role or identifier, safe relative path, and BLAKE3 hash
- AND the receipt records the source index hash, payload commit, publish directory layout version, and verdict

#### Scenario: Publication metadata is deterministic and reviewable

r[release_evidence.publish_materialization.manifest_receipt.metadata]
- GIVEN the operator supplies channel, release, or retention labels
- WHEN the command emits the manifest and receipt
- THEN the labels are policy-validated, sorted or normalized, and included as local metadata
- AND omitted labels do not produce nondeterministic defaults such as wall-clock timestamps

### Requirement: Producer release evidence publication fails closed on unsafe materialization states

r[release_evidence.publish_materialization.fail_closed] The system MUST reject unsafe publication states including dirty or mismatched source evidence, invalid source receipts, unsafe relative paths, missing artifacts, post-copy hash drift, and malformed label metadata.

#### Scenario: Unsafe paths or hash drift do not produce a passing publication

r[release_evidence.publish_materialization.fail_closed.path_or_hash]
- GIVEN source evidence contains an unsafe artifact path or a copied artifact hash no longer matches the verified source hash
- WHEN the command validates the publication tree
- THEN the command exits nonzero or emits a deterministic non-passing receipt
- AND the failure avoids leaking absolute temporary paths, credentials, secrets, or environment values

### Requirement: Producer release evidence publication documents non-claims and retention boundaries

r[release_evidence.publish_materialization.operator_docs] The system MUST document that materialization is a local producer-side publication step with explicit retention boundaries and no remote transport, credential, mirror-trust, signing, tag, or public unattended-readiness claims.

#### Scenario: Operator documentation prevents publication overclaims

r[release_evidence.publish_materialization.operator_docs.non_claims]
- GIVEN an operator reads the publication materialization documentation
- WHEN they decide whether to cite or transfer the materialized tree
- THEN the documentation states that the tree is local verified evidence only until an external transport or trust process handles it
- AND it states that the command does not validate credentials, remote mirrors, signatures, public release pages, tag movement, or unattended production readiness
