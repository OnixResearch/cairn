# Release Evidence Retention/Rotation Delta

## Requirements

### Requirement: Release evidence retention and rotation has a stable local CLI contract

r[release_evidence.retention_rotation.cli_contract] The system MUST provide a first-class local contract for planning retention, supersession, and rotation decisions over materialized release evidence publication trees.

#### Scenario: CLI contract names explicit roots and labels

r[release_evidence.retention_rotation.cli_contract.inputs_outputs]
- GIVEN an operator has one or more local release evidence publication trees
- WHEN the operator invokes the retention planning command
- THEN the command accepts explicit repository root, publication-root, output plan, and optional channel or retention labels
- AND it documents that labels are local policy metadata rather than remote-publication, tag, or credential claims

### Requirement: Release evidence retention planning composes publication validators

r[release_evidence.retention_rotation.composes_publication_validators] The system MUST verify candidate publication trees through existing release-evidence publication manifest and receipt semantics rather than duplicating independent release validators.

#### Scenario: Publication manifests remain authoritative

r[release_evidence.retention_rotation.composes_publication_validators.manifest_receipt]
- GIVEN the retention planner inventories a candidate publication tree
- WHEN it decides whether the tree can be retained, superseded, or pruned
- THEN it reopens and verifies the candidate `manifest.json` and `receipt.json` before making a passing decision
- AND it preserves existing issue semantics for invalid publication evidence wherever practical

### Requirement: Release evidence retention planning emits deterministic plan and receipt artifacts

r[release_evidence.retention_rotation.plan_receipt] The system MUST emit deterministic retention plan and receipt artifacts that bind publication manifest hashes, receipt hashes, safe relative tree paths, labels, actions, reason codes, and verdict.

#### Scenario: Retention decisions are reviewable

r[release_evidence.retention_rotation.plan_receipt.decisions]
- GIVEN the planner completes successfully
- WHEN a reviewer inspects the plan and receipt
- THEN every candidate publication tree is listed with a stable relative path, manifest hash, receipt hash, normalized labels, and action such as `keep`, `supersede`, or `prune_candidate`
- AND the receipt records the plan hash, policy hash, input root, and verdict without embedding secrets or absolute temporary paths

#### Scenario: Planning uses deterministic ordering

r[release_evidence.retention_rotation.plan_receipt.ordering]
- GIVEN multiple candidate trees are present
- WHEN the planner emits decisions
- THEN decisions are sorted by policy-owned labels and stable relative paths
- AND the command does not rely on filesystem walk order, wall-clock timestamps, or ambient environment values for retention ordering

### Requirement: Release evidence rotation execution is explicit and receipt-backed

r[release_evidence.retention_rotation.rotation_execution] The system MUST default to read-only planning and require explicit execution plus plan verification before mutating local publication trees.

#### Scenario: Rotation execution verifies the saved plan before mutation

r[release_evidence.retention_rotation.rotation_execution.verify_before_mutation]
- GIVEN an operator asks to execute a saved rotation plan
- WHEN the rotation command runs
- THEN it verifies the plan hash, referenced publication manifest hashes, and referenced receipt hashes before any mutation
- AND it emits a deterministic mutation manifest for every touched relative path when local deletion or movement occurs

### Requirement: Release evidence retention and rotation fails closed on unsafe states

r[release_evidence.retention_rotation.fail_closed] The system MUST reject unsafe retention or rotation states including unsafe paths, symlinks, missing manifests, changed hashes, malformed labels, ambiguous duplicate current releases, and execute requests without a valid plan.

#### Scenario: Unsafe or stale trees do not produce passing plans

r[release_evidence.retention_rotation.fail_closed.unsafe_or_stale]
- GIVEN a candidate tree has an unsafe relative path, missing manifest, changed artifact hash, malformed label, or duplicate current-release identity
- WHEN retention planning or rotation execution validates the tree
- THEN the command exits nonzero or emits a deterministic non-passing receipt
- AND the failure avoids leaking credentials, secrets, raw environment values, or unnecessary absolute paths

### Requirement: Release evidence retention and rotation documents local-only non-claims

r[release_evidence.retention_rotation.operator_docs] The system MUST document that retention and rotation govern local producer-side evidence directories only and do not imply remote deletion, mirror trust, signing, tag movement, credential validation, or public release-page changes.

#### Scenario: Operator documentation prevents retention overclaims

r[release_evidence.retention_rotation.operator_docs.non_claims]
- GIVEN an operator reads the retention and rotation documentation
- WHEN they decide whether to keep, supersede, prune, or transfer a publication tree
- THEN the documentation states that Cairn only planned or executed local filesystem retention actions
- AND it states that remote mirrors, credentials, signatures, tags, public release pages, and consumer trust decisions require separate external processes
