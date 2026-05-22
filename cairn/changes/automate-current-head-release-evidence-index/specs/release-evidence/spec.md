# Release Evidence Delta

## ADDED Requirements

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
