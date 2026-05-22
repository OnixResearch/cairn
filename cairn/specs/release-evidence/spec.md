# Release Evidence Specification

## Purpose

Defines the `release-evidence` capability.

## Requirements

### Requirement: Create current-HEAD release evidence index uses focused boundaries

r[release_evidence.current_head_evidence_index.focused_modules] The system MUST address current clean `main` HEAD and existing release/readiness evidence artifacts with focused boundaries that reduce review surface while preserving existing behavior.

#### Scenario: Focused boundaries are visible

r[release_evidence.current_head_evidence_index.focused_modules.visible_boundaries]
- GIVEN the ROI slice is implemented
- WHEN a reviewer inspects the affected source, tests, docs, or evidence
- THEN existing tag, harness, receipt, policy, Tracey, and release-readiness evidence are indexed with stable paths and non-self-referential wording
- AND the original target no longer remains the sole large review surface for that family

#### Scenario: Bridge APIs stay narrow

r[release_evidence.current_head_evidence_index.focused_modules.narrow_bridges]
- GIVEN another module or operator workflow still consumes the affected behavior
- WHEN the implementation exposes cross-module functions, shared helpers, or evidence pointers
- THEN only the narrow bridge surface needed by those consumers is exposed
- AND broad crate-root shims, unrelated helper moves, and tag or receipt churn are avoided

### Requirement: Create current-HEAD release evidence index remains behavior preserving

r[release_evidence.current_head_evidence_index.behavior_preserved] The system MUST preserve existing public behavior, deterministic evidence, fixture expectations, and issue codes while draining this ROI slice.

#### Scenario: Focused checks pass

r[release_evidence.current_head_evidence_index.behavior_preserved.focused_checks]
- GIVEN the ROI slice is implemented
- WHEN the operator runs evidence index review, release-readiness receipt generation/verification, clean worktree status, and diff checks
- THEN the focused checks pass without intentional receipt, issue-code, policy, fixture, or hash drift

#### Scenario: Repository checks pass

r[release_evidence.current_head_evidence_index.behavior_preserved.repository_checks]
- GIVEN the ROI slice is implemented
- WHEN the operator runs native Cairn validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit

### Requirement: Automated current-HEAD release evidence index has a stable CLI contract

r[release_evidence.automated_current_head_index.cli_contract] The system MUST provide a first-class operator command for generating a deterministic current-HEAD release evidence index from a local Cairn checkout without requiring manual hash transcription.

#### Scenario: CLI contract names explicit inputs and outputs

r[release_evidence.automated_current_head_index.cli_contract.inputs_outputs]
- GIVEN an operator wants to generate current-HEAD release evidence
- WHEN the operator invokes the command
- THEN the command accepts explicit root, index output, and generated-artifact/work directory arguments
- AND the command documents that generated bulky receipts remain outside source control unless the operator explicitly chooses otherwise

### Requirement: Automated current-HEAD release evidence index composes existing rails

r[release_evidence.automated_current_head_index.composes_existing_rails] The system MUST compose the existing release-readiness, evidence export, verify, import, republish, index, and index-verification rails rather than duplicating their validation logic.

#### Scenario: Existing receipt validators remain authoritative

r[release_evidence.automated_current_head_index.composes_existing_rails.authoritative_validators]
- GIVEN the command generates a release evidence index
- WHEN it validates intermediate artifacts
- THEN it uses the existing receipt validators and issue semantics for those artifacts
- AND it does not introduce a second implementation of release-readiness or evidence-index verification rules

### Requirement: Automated current-HEAD release evidence index binds generated output to the payload HEAD

r[release_evidence.automated_current_head_index.generated_index] The system MUST render deterministic index output that binds the payload commit, worktree state, generated artifact roles, relative artifact paths, receipt verdicts, and receipt hashes.

#### Scenario: Generated index records payload HEAD without self-reference

r[release_evidence.automated_current_head_index.generated_index.payload_head]
- GIVEN the command runs on a clean checkout
- WHEN it renders the release evidence index
- THEN the index records the payload commit from the checkout being evaluated
- AND it avoids claiming the final commit hash of any later documentation commit that may carry the generated index

#### Scenario: Generated index records required artifact and receipt hashes

r[release_evidence.automated_current_head_index.generated_index.required_hashes]
- GIVEN the command completes successfully
- WHEN a reviewer inspects the index
- THEN the index lists the required release-readiness, evidence export, evidence verification, import, republish, evidence index, and index-verification receipt hashes
- AND it lists required bundle artifact roles with stable relative paths and BLAKE3 hashes

### Requirement: Automated current-HEAD release evidence index fails closed on unsafe evidence states

r[release_evidence.automated_current_head_index.fail_closed] The system MUST reject or report deterministic failure evidence for dirty worktrees, mismatched payload HEADs, invalid generated receipts, missing required artifact roles, and incomplete evidence chains.

#### Scenario: Dirty or mismatched checkout does not produce a passing index

r[release_evidence.automated_current_head_index.fail_closed.dirty_or_mismatched]
- GIVEN the checkout is dirty or the generated index payload commit does not match the evaluated HEAD
- WHEN the command validates the generated evidence
- THEN the command exits nonzero or emits a deterministic non-passing receipt
- AND the failure avoids leaking secrets, absolute temporary paths, or credential values

### Requirement: Automated current-HEAD release evidence index remains operator-readable

r[release_evidence.automated_current_head_index.operator_docs] The system MUST document the generated index workflow, scope boundary, non-claims, verification commands, and expected source-control treatment for generated artifacts.

#### Scenario: Operator documentation prevents release overclaims

r[release_evidence.automated_current_head_index.operator_docs.non_claims]
- GIVEN an operator reads the generated index workflow documentation
- WHEN they decide whether to cite the evidence
- THEN the documentation states that the index is local/trusted release evidence only
- AND it does not claim remote mirror trust, credential validation, public unattended production readiness, or tag movement

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
