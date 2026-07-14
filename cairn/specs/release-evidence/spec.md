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

### Requirement: Release evidence CLI source split uses focused modules

r[release_evidence.cli_source_split.focused_modules] The system MUST split the release-evidence CLI implementation into focused source modules that preserve a small dispatcher facade and isolate cohesive responsibilities.

#### Scenario: Focused module boundaries are visible

r[release_evidence.cli_source_split.focused_modules.visible_boundaries]
- GIVEN the release-evidence source split is implemented
- WHEN a reviewer inspects the release-evidence CLI source tree
- THEN current-HEAD index generation, publication materialization, retention planning, and rotation execution are separated into named modules or equivalent focused files
- AND the dispatcher facade no longer contains the full implementation for every release-evidence family

#### Scenario: Index generation has a focused home

r[release_evidence.cli_source_split.index_module]
- GIVEN the release-evidence source split is implemented
- WHEN a reviewer inspects current-HEAD evidence index code
- THEN index option parsing, orchestration, receipt assembly, and rendering live in a focused module or clearly bounded source region
- AND the public `release-evidence index` behavior remains unchanged

#### Scenario: Publication materialization has a focused home

r[release_evidence.cli_source_split.publish_module]
- GIVEN the release-evidence source split is implemented
- WHEN a reviewer inspects local publication materialization code
- THEN publish option parsing, artifact copying, manifest assembly, and receipt generation live in a focused module or clearly bounded source region
- AND the public `release-evidence publish` behavior remains unchanged

#### Scenario: Retention planning has a focused home

r[release_evidence.cli_source_split.retention_module]
- GIVEN the release-evidence source split is implemented
- WHEN a reviewer inspects retention planning code
- THEN candidate inventory, plan decisions, label filtering, and plan receipt generation live in a focused module or clearly bounded source region
- AND the public `release-evidence retention-plan` behavior remains unchanged

#### Scenario: Rotation execution has a focused home

r[release_evidence.cli_source_split.rotate_module]
- GIVEN the release-evidence source split is implemented
- WHEN a reviewer inspects rotation execution code
- THEN saved-plan verification, dry-run mutation reporting, execute-mode local deletion, and rotation receipt generation live in a focused module or clearly bounded source region
- AND the public `release-evidence rotate` behavior remains unchanged

### Requirement: Release evidence CLI source split keeps bridge APIs narrow

r[release_evidence.cli_source_split.narrow_bridges] The system MUST expose only narrow shared helpers across release-evidence modules and avoid broad crate-root shims or unrelated helper movement.

#### Scenario: Shared helpers are limited to cross-family needs

r[release_evidence.cli_source_split.narrow_bridges.shared_helpers]
- GIVEN multiple release-evidence modules need common path, hash, issue, or JSON helper logic
- WHEN the implementation exposes shared helpers
- THEN helper visibility is limited to the release-evidence module tree wherever practical
- AND unrelated CLI families do not gain new broad public bridge APIs

### Requirement: Release evidence CLI source split remains behavior preserving

r[release_evidence.cli_source_split.behavior_preserved] The system MUST preserve release-evidence command behavior, deterministic receipts, issue codes, fixtures, and policy hash material while splitting source modules.

#### Scenario: Focused behavior checks pass

r[release_evidence.cli_source_split.behavior_preserved.focused_checks]
- GIVEN the source split is implemented
- WHEN the operator runs focused release-evidence tests
- THEN index, publish, retention, rotation, and dirty-worktree checks pass without intentional receipt, issue-code, fixture, or policy drift

#### Scenario: Repository checks pass

r[release_evidence.cli_source_split.repository_checks]
- GIVEN the source split is implemented
- WHEN the operator runs formatting, workspace tests, clippy, Cairn validation, Tracey coverage, and diff checks
- THEN all checks pass and the repository remains ready for commit

#### Scenario: Package can be archived after verification

r[release_evidence.cli_source_split.archive_ready]
- GIVEN the source split has passed focused and repository verification
- WHEN the operator syncs and archives the change package
- THEN accepted specs include the release-evidence source-split requirements
- AND no active package remains for this completed source-organization slice

### Requirement: Release evidence current-head staleness guard

r[release_evidence.current_head_staleness_guard.command] The system MUST provide a deterministic local checker for generated release-evidence index documents so stale current-head claims fail closed.

#### Scenario: Parser extracts generated payload fields

r[release_evidence.current_head_staleness_guard.parser]
- GIVEN a generated release-evidence Markdown index
- WHEN the checker reads the payload block
- THEN it extracts the payload commit, branch, clean marker, upstream, and ahead/behind fields
- AND missing or malformed payload fields are reported as deterministic validation issues

#### Scenario: Git checks bind payload to repository state

r[release_evidence.current_head_staleness_guard.git_checks]
- GIVEN a parsed release-evidence index and repository root
- WHEN the checker validates the payload commit
- THEN it verifies the commit exists locally
- AND it verifies the payload commit is an ancestor of the live HEAD
- AND current-head mode additionally requires the payload commit to equal the live HEAD

#### Scenario: Stale current-head claims fail closed

r[release_evidence.current_head_staleness_guard.tests]
r[release_evidence.current_head_staleness_guard.fail_closed]
- GIVEN a checked index whose path or mode claims current-head status
- WHEN its payload commit differs from live HEAD
- THEN the checker emits `valid=false`, `verdict=fail`, and a deterministic stale-current-head issue

#### Scenario: Release readiness includes checked-in evidence freshness

r[release_evidence.current_head_staleness_guard.release_readiness]
- GIVEN the conventional checked-in release evidence page exists
- WHEN `release-readiness` runs
- THEN the readiness receipt includes the index freshness check
- AND stale current-head claims contribute to a failing release-readiness verdict

#### Scenario: Operator docs distinguish current artifacts from audited snapshots

r[release_evidence.current_head_staleness_guard.operator_docs]
- GIVEN release evidence documentation is reviewed
- WHEN it describes generated current-head evidence and checked-in evidence pages
- THEN it says fresh current-head evidence is generated into local or CI artifacts
- AND checked-in evidence pages are audited payload snapshots unless explicitly refreshed for the current payload

#### Scenario: Package can be archived after verification

r[release_evidence.current_head_staleness_guard.archive_ready]
- GIVEN the checker, tests, readiness wiring, and docs pass focused and repository verification
- WHEN the operator syncs and archives the package
- THEN the accepted specs include the staleness guard requirements
- AND no active package remains for this slice

### Requirement: Current-HEAD release evidence snapshot
r[release_evidence.current_head_snapshot.generate] The system MUST support generating a local current-HEAD release evidence snapshot using existing release-evidence rails.

#### Scenario: Generate current-head evidence into target artifacts
r[release_evidence.current_head_snapshot.check_index]
- GIVEN a clean synced Cairn checkout
- WHEN the operator generates a release-evidence index into `target/release-evidence-current-head/`
- THEN the generated index SHALL bind the live HEAD and local artifacts without committing generated receipts
- AND the existing check-index rail SHALL validate the index in current-head mode.

#### Scenario: Snapshot verification is durable and non-self-referential
r[release_evidence.current_head_snapshot.validation]
r[release_evidence.current_head_snapshot.docs_boundary]
- GIVEN current-HEAD evidence changes after any subsequent commit
- WHEN the operator records or reports the snapshot
- THEN the evidence SHALL distinguish verified code commit from evidence-recording commit
- AND release-readiness, Cairn validation, Tracey coverage, and diff checks SHALL pass.

### Requirement: Release-readiness summarizes Preserves evidence
r[release_evidence.preserves_readiness_summary.output] The system MUST summarize internal Preserves evidence readiness in release-readiness receipts without replacing JSON public receipt semantics.

#### Scenario: Preserves readiness is reported without mutation
r[release_evidence.preserves_readiness_summary.fail_closed]
- GIVEN release-readiness runs in a Cairn checkout with Preserves corpus fixtures
- WHEN it evaluates Preserves readiness
- THEN it SHALL compose existing non-mutating rails and SHALL NOT run corpus update with `--execute`
- AND it SHALL emit deterministic issues when freshness cannot be proven.

#### Scenario: Preserves readiness summary is verified
r[release_evidence.preserves_readiness_summary.tests]
r[release_evidence.preserves_readiness_summary.validation]
- GIVEN the release-readiness Preserves summary
- WHEN focused tests and lifecycle validation run
- THEN schema expectations, failure issues, Cairn validation, and Tracey references SHALL pass.

### Requirement: Post-tag remote install smoke
r[release_evidence.post_tag_remote_install_smoke.build] The system MUST support a post-tag smoke workflow that verifies the tagged remote flake package can build outside the local checkout.

#### Scenario: Remote tag package builds
r[release_evidence.post_tag_remote_install_smoke.lifecycle]
- GIVEN a published Cairn release tag
- WHEN an operator builds `github:OnixResearch/cairn/<tag>#cairn` and runs the resulting binary against a temporary lifecycle root
- THEN `init`, `status`, and `validate` SHALL complete successfully without requiring repository-local target artifacts.

#### Scenario: Smoke evidence stays bounded
r[release_evidence.post_tag_remote_install_smoke.docs]
r[release_evidence.post_tag_remote_install_smoke.validation]
- GIVEN the post-tag smoke completes
- WHEN the operator records evidence
- THEN the report SHALL name the tag, commands, verdict, and non-claims without preserving credentials, absolute temp paths, or claiming remote production readiness.

### Requirement: v0.1.0 release evidence publication snapshot
r[release_evidence.v010_publication.index] The system MUST support generating and verifying a local release-evidence index for the v0.1.0 release payload without mutating the release tag.

#### Scenario: v0.1.0 evidence index is verified
r[release_evidence.v010_publication.publish]
- GIVEN a clean checkout at the v0.1.0 payload commit
- WHEN the operator generates, verifies, and materializes release evidence
- THEN the index and publication receipts SHALL be valid and bind to the evaluated payload commit.

#### Scenario: v0.1.0 evidence summary is auditable
r[release_evidence.v010_publication.docs]
r[release_evidence.v010_publication.validation]
- GIVEN generated release evidence artifacts exist under `target/`
- WHEN the operator records checked-in evidence
- THEN only a compact audited summary SHALL be committed and it SHALL distinguish local/trusted evidence from remote publication, signature, credential, or production-readiness claims.

### Requirement: aarch64 release support boundary is explicit
r[release_evidence.aarch64_support_boundary.docs] The system SHOULD document the architecture support boundary for release evidence when a full multi-system Nix check was not performed.

#### Scenario: aarch64 non-claim is visible
r[release_evidence.aarch64_support_boundary.non_claims]
- GIVEN release-readiness evidence was produced on x86_64 and Nix omitted incompatible aarch64 checks
- WHEN an operator reads release evidence notes
- THEN the notes SHALL state that aarch64 release support still requires builder-backed validation before being claimed.

#### Scenario: Future validation path is concrete
r[release_evidence.aarch64_support_boundary.validation]
r[release_evidence.aarch64_support_boundary.priority]
- GIVEN a suitable aarch64 builder or evaluator is available
- WHEN an operator wants to expand release support
- THEN the notes SHALL name the future command path and SHALL keep unavailable builder noise separate from current product-local release proof.

### Requirement: Workspace contract matrix schema
r[cairn.workspace_contract_matrix.schema] Cairn MUST define a workspace contract matrix schema for provider/consumer repo edges, artifact identities, validation commands, fixture coverage, owners, and non-claim boundaries.

#### Scenario: Complete matrix row passes
r[cairn.workspace_contract_matrix.positive]
- GIVEN a matrix row names an edge ID, provider repo, consumer repo, artifact family, schema or contract ID, artifact identity, validation command, positive fixture, negative fixture or failure class, owner, and required non-claim boundary
- WHEN the matrix validator evaluates the row
- THEN validation MUST pass and preserve the edge ID in the receipt.

#### Scenario: Malformed matrix row fails closed
r[cairn.workspace_contract_matrix.negative]
- GIVEN a matrix row omits an owner, uses an unsafe path, lacks negative coverage, declares a stale artifact identity, or weakens required non-claims
- WHEN the matrix validator evaluates the row
- THEN validation MUST fail with deterministic diagnostics naming the invalid edge.

### Requirement: Matrix validation receipt
r[cairn.workspace_contract_matrix.validation] Cairn MUST emit a deterministic BLAKE3-bound receipt for workspace contract matrix validation.

#### Scenario: Receipt binds matrix and artifact identities
r[cairn.workspace_contract_matrix.validation.receipt]
- GIVEN a matrix file and referenced artifact identities
- WHEN Cairn validates the matrix
- THEN the receipt MUST include validity, verdict, issues, matrix hash, checked edge count, stale edge count, policy hash, and receipt hash.

#### Scenario: Receipt avoids raw host paths
r[cairn.workspace_contract_matrix.validation.privacy]
- GIVEN matrix validation runs from a local workspace
- WHEN Cairn emits the receipt
- THEN it MUST use safe repo-relative paths or stable labels and MUST NOT require raw host paths for public review.

### Requirement: Release-readiness integration
r[cairn.workspace_contract_matrix.release_readiness] Cairn release readiness SHOULD support disabled, optional, and required modes for workspace contract matrix receipts.

#### Scenario: Optional matrix records context
r[cairn.workspace_contract_matrix.release_readiness.optional]
- GIVEN matrix mode is optional and a valid matrix receipt is supplied
- WHEN release readiness runs
- THEN Cairn SHOULD include the matrix receipt hash as context without broadening upstream correctness claims.

#### Scenario: Required matrix fails stale edge
r[cairn.workspace_contract_matrix.release_readiness.required]
- GIVEN matrix mode is required and a configured edge is missing, stale, or invalid
- WHEN release readiness runs
- THEN Cairn MUST fail readiness with diagnostics naming the edge and failure class.

### Requirement: Provider and consumer boundaries
r[cairn.workspace_contract_matrix.docs] Cairn documentation MUST state that provider repos own local contract semantics and consumer repos own integration checks, while Cairn owns only the cross-repo edge inventory and validation receipt.

#### Scenario: Matrix non-claim is visible
r[cairn.workspace_contract_matrix.docs.non_claims]
- GIVEN a matrix receipt or release-readiness report mentions a passing edge
- WHEN the supported claim is rendered
- THEN it MUST state that the matrix proves declared edge health only and does not prove provider correctness, consumer behavioral correctness, verifier soundness, or release eligibility by itself.

### Requirement: Workspace contract matrix final validation
r[cairn.workspace_contract_matrix.final_validation] The change MUST include positive and negative tests, policy freshness, and Cairn validation evidence before archive.

#### Scenario: Fixture suite covers edge health
r[cairn.workspace_contract_matrix.final_validation.fixtures]
- GIVEN positive and negative matrix fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, and release-readiness mode coverage MUST be present.

### Requirement: Formal proof readiness fixture contract
r[cairn.release_evidence.formal_proof_readiness_fixture.contract] Cairn MUST define a frozen formal-proof readiness fixture contract for Valence stack readiness inputs derived from Octet formal-proof exports.

#### Scenario: Fixture contract names upstream evidence and Cairn receipt fields
r[cairn.release_evidence.formal_proof_readiness_fixture.contract.fields]
- GIVEN a formal-proof readiness fixture is declared
- WHEN Cairn validates the fixture contract
- THEN the fixture MUST name the Valence input identity, upstream Octet proof identity, verification-role summary, policy hash, input hash, readiness receipt hash, and required non-claim boundary.

### Requirement: Positive formal-proof readiness fixture
r[cairn.release_evidence.formal_proof_readiness_fixture.positive] Cairn MUST accept a readiness fixture with one eligible property record backed by Valence stack evidence and an Octet-owned formal-proof receipt while preserving recorded-only/manual-review context.

#### Scenario: Only eligible property evidence counts as verified
r[cairn.release_evidence.formal_proof_readiness_fixture.positive.property_count]
- GIVEN a Valence readiness input contains one eligible property record and additional recorded-only or manual-review records
- WHEN Cairn validates the readiness fixture
- THEN the readiness receipt MUST count only the eligible property record as verified and MUST report recorded-only or manual-review records separately.

### Requirement: Negative formal-proof readiness fixtures
r[cairn.release_evidence.formal_proof_readiness_fixture.negative] Cairn MUST reject readiness fixtures with stale Valence input, digest drift, role promotion, failed upstream proof status, missing hashes, missing non-claims, or overclaim text.

#### Scenario: Recorded-only evidence cannot satisfy readiness proof count
r[cairn.release_evidence.formal_proof_readiness_fixture.negative.recorded_only]
- GIVEN a readiness input counts recorded-only, boundary, imported Trellis, or manual-review evidence as verified property evidence
- WHEN Cairn validates the readiness fixture
- THEN validation MUST fail with a deterministic verification-role-promotion diagnostic.

#### Scenario: Stale upstream input fails closed
r[cairn.release_evidence.formal_proof_readiness_fixture.negative.stale]
- GIVEN the Valence graph input hash or sidecar digest does not match the readiness fixture declaration
- WHEN Cairn validates the readiness fixture
- THEN validation MUST fail with a deterministic stale-input diagnostic.

### Requirement: Formal-proof readiness validation core
r[cairn.release_evidence.formal_proof_readiness_fixture.validation] Cairn MUST validate formal-proof readiness fixtures through a pure deterministic core over parsed fixture values and policy metadata.

#### Scenario: Pure core owns readiness fixture decisions
r[cairn.release_evidence.formal_proof_readiness_fixture.validation.core]
- GIVEN parsed readiness fixture values and policy metadata
- WHEN the readiness fixture validator runs
- THEN it MUST return pass/fail diagnostics without reading files, invoking processes, inspecting environment, or contacting sibling repositories.

### Requirement: Readiness ownership documentation
r[cairn.release_evidence.formal_proof_readiness_fixture.docs] Cairn documentation MUST state that formal-proof readiness fixtures prove Cairn receipt handling under policy only.

#### Scenario: Readiness non-claim is visible
r[cairn.release_evidence.formal_proof_readiness_fixture.docs.non_claims]
- GIVEN a formal-proof readiness fixture passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that Cairn does not prove upstream source correctness, verifier soundness, Valence correctness, Octet correctness, whole-program correctness, semantic equivalence, or release eligibility outside the configured readiness policy.

### Requirement: Final validation evidence
r[cairn.release_evidence.formal_proof_readiness_fixture.final_validation] The change MUST include positive and negative fixtures, focused core tests, policy freshness checks, Cairn validation/gates, and release-readiness evidence before archive.

#### Scenario: Readiness fixture validation is reviewable
r[cairn.release_evidence.formal_proof_readiness_fixture.final_validation.fixtures]
- GIVEN valid and invalid formal-proof readiness fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, and saved evidence MUST bind fixture, policy, input, and receipt identities.

### Requirement: Lifecycle receipt core contract
r[cairn.release_evidence.lifecycle_receipt_core.contract] Cairn MUST define pure lifecycle receipt core contracts for verdicts, diagnostic rows, evidence summaries, command identity, hash fields, receipt hash profiles, and non-claim boundaries.

#### Scenario: Valid receipt envelope validates
r[cairn.release_evidence.lifecycle_receipt_core.fixtures.positive]
- GIVEN a receipt envelope has a supported verdict, required diagnostic shape, evidence summary, command identity, hash fields, receipt hash profile, and non-claim boundary
- WHEN lifecycle receipt core validation runs
- THEN validation MUST accept the envelope and preserve the receipt family identity.

#### Scenario: Invalid receipt envelope fails closed
r[cairn.release_evidence.lifecycle_receipt_core.fixtures.negative]
- GIVEN a receipt envelope has a malformed verdict, missing hash, stale receipt hash, missing diagnostic field, weakened non-claim, or release/lifecycle overclaim
- WHEN lifecycle receipt core validation runs
- THEN validation MUST fail with deterministic diagnostics naming the invalid receipt field.

### Requirement: Receipt hash profiles are explicit
r[cairn.release_evidence.lifecycle_receipt_core.hash_profiles] Cairn MUST require named receipt hash profiles for shared envelope validation.

#### Scenario: Self-hash field exclusion is profile-bound
r[cairn.release_evidence.lifecycle_receipt_core.hash_profiles.self_hash]
- GIVEN a receipt contains its own receipt hash field
- WHEN receipt hash material is computed
- THEN only fields declared by the selected receipt hash profile MUST be excluded from hash material.

### Requirement: Lifecycle receipt validation is pure
r[cairn.release_evidence.lifecycle_receipt_core.validation] Cairn MUST implement lifecycle receipt envelope validation as pure deterministic logic over in-memory receipt values, hash profiles, and policy metadata.

#### Scenario: Shell owns receipt collection
r[cairn.release_evidence.lifecycle_receipt_core.validation.shell]
- GIVEN a CLI collects repository snapshots, policy files, command inputs, or receipt JSON from disk
- WHEN lifecycle receipt core validation is invoked
- THEN filesystem access, JSON presentation, stdout/stderr, process exits, and mutation planning MUST remain outside the receipt core.

### Requirement: Compatibility adapters preserve receipt families
r[cairn.release_evidence.lifecycle_receipt_core.adapters] Cairn SHOULD expose adapters for existing Cairn receipt families and downstream receipt-shaped evidence without changing their public claim boundaries.

#### Scenario: Existing receipt family adapts
r[cairn.release_evidence.lifecycle_receipt_core.adapters.family]
- GIVEN an existing Cairn gate, validate, sync, archive, runtime-evidence, or release-evidence receipt uses its current public JSON shape
- WHEN the compatibility adapter validates it
- THEN the adapter SHOULD preserve the public receipt shape while routing shared envelope checks through the lifecycle receipt core.

### Requirement: Lifecycle receipt core remains bounded
r[cairn.release_evidence.lifecycle_receipt_core.docs] Passing lifecycle receipt core validation MUST NOT prove release eligibility, lifecycle readiness, policy satisfaction, evidence truth, source-code correctness, verifier soundness, or downstream behavioral correctness.

#### Scenario: Boundary is visible
r[cairn.release_evidence.lifecycle_receipt_core.final_validation]
- GIVEN a lifecycle receipt core validation report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that validation proves only receipt envelope syntax, hash consistency, diagnostics shape, and non-claim presence.

### Requirement: Cairn evidence filesystem authority is capability-rooted
r[cairn.release_evidence.cap_std_boundary.root_wrappers] Cairn MUST represent repository, evidence bundle, registry, release, replay, and scan filesystem authority as typed capability roots opened by the CLI shell.

#### Scenario: Valid evidence path stays under the declared root
r[cairn.release_evidence.cap_std_boundary.tests.positive]
- GIVEN Cairn has opened an operator-declared repository or artifact root
- WHEN evidence, registry, release, replay, or scan code reads or writes a valid relative path through the typed root
- THEN the operation MUST remain confined to that declared root and preserve existing valid-input behavior.

#### Scenario: Invalid evidence path cannot escape the root
r[cairn.release_evidence.cap_std_boundary.tests.negative]
- GIVEN input names `../` traversal, an absolute path, a missing capability root, or a symlink escape attempt
- WHEN Cairn resolves the path through the typed root
- THEN Cairn MUST fail closed with deterministic diagnostics before reading, writing, scanning, or replaying outside the declared root.

### Requirement: Capability dependency stays outside Cairn core
r[cairn.release_evidence.cap_std_boundary.dependency] Cairn MUST add `cap-std` only to crates or modules that own CLI/support filesystem effects.

#### Scenario: Core remains pure
r[cairn.release_evidence.cap_std_boundary.conversion]
- GIVEN `cairn-core` validation, policy, lifecycle, and receipt logic is reviewed
- WHEN cap-std adoption is complete
- THEN `cairn-core` MUST receive loaded inputs or validated relative locators and MUST NOT depend on `cap-std` or open ambient filesystem paths.

### Requirement: Capability boundary is documented without evidence overclaims
r[cairn.release_evidence.cap_std_boundary.docs] Cairn MUST document that capability roots bound local filesystem authority only and do not prove artifact truth, replay completeness, lifecycle correctness, or release readiness.

#### Scenario: Closeout evidence includes boundary validation
r[cairn.release_evidence.cap_std_boundary.validation]
- GIVEN the cap-std evidence-boundary change is ready to archive
- WHEN focused evidence, registry, release, replay, scan, Cairn validation, Tracey coverage, and gate checks run
- THEN the evidence MUST include positive and negative filesystem-boundary coverage and visible non-claim documentation.

### Requirement: Durable release-run ledger model
r[cairn.release_evidence.durable_run_ledger.model] Cairn MUST define a pure durable release-run ledger model with run identity, lifecycle root identity, policy hash, command identity, profile, step rows, artifact references, diagnostics, blockers, resume hints, and non-claim boundaries.

#### Scenario: Complete run ledger validates
r[cairn.release_evidence.durable_run_ledger.fixtures.positive]
- GIVEN a release-run ledger records a complete local release-readiness workflow with matching policy hash, command identity, step rows, artifact references, diagnostics, and non-claims
- WHEN ledger validation runs
- THEN Cairn MUST accept the ledger and preserve the run identity as a local evidence artifact.

### Requirement: Ledger validation fails closed
r[cairn.release_evidence.durable_run_ledger.validation] Cairn MUST reject release-run ledgers with stale identity, malformed state transitions, missing command identity, missing artifact references, or weakened boundaries.

#### Scenario: Invalid ledger is rejected
r[cairn.release_evidence.durable_run_ledger.fixtures.negative]
- GIVEN a release-run ledger has a stale policy hash, missing command identity, impossible step transition, weakened non-claim, cross-run artifact substitution, or remote publish overclaim
- WHEN ledger validation runs
- THEN Cairn MUST fail with deterministic diagnostics naming the rejected ledger field.

### Requirement: Status and explain projections are deterministic
r[cairn.release_evidence.durable_run_ledger.status_explain] Cairn SHOULD expose deterministic status and explain projections over already-loaded release-run ledgers.

#### Scenario: Blocked run explains next action
r[cairn.release_evidence.durable_run_ledger.shell]
- GIVEN a release-run ledger has blocked or resumable steps
- WHEN an operator asks for status or explain output
- THEN Cairn SHOULD report blockers, stale inputs, missing evidence, resumable steps, terminal verdict, and next recommended local action without mutating remote services.

### Requirement: Ledger validation remains local evidence only
r[cairn.release_evidence.durable_run_ledger.final_validation] Cairn MUST state that passing release-run ledger validation proves only ledger syntax, identity linkage, transition validity, and non-claim preservation.

#### Scenario: Ledger does not publish or promote
- GIVEN a release-run ledger validates
- WHEN release-readiness or evidence export consumes it
- THEN the ledger MUST NOT by itself publish artifacts, create tags, create GitHub releases, upload registry packages, sign artifacts, prove release eligibility, prove evidence truth, or trust external services.

### Requirement: Workspace contract identity maps
r[cairn.workspace_contract_matrix.iddqd_indexes.contract] Cairn MUST index workspace contract edges and observed artifacts by ids derived from the values rather than by duplicated external map keys.

#### Scenario: Unique edge observations pass
r[cairn.workspace_contract_matrix.iddqd_indexes.fixtures.positive]
- GIVEN each declared workspace contract edge id is unique and each observed artifact record names a matching unique edge id
- WHEN workspace contract validation builds identity indexes
- THEN validation MUST compare observed and declared artifact identities without changing public receipt shape.

#### Scenario: Duplicate identity fails closed
r[cairn.workspace_contract_matrix.iddqd_indexes.fixtures.negative]
- GIVEN declared edges or observed artifact records contain duplicate edge ids, missing observations, stale artifact identities, unsafe paths, or weakened non-claims
- WHEN workspace contract validation builds identity indexes
- THEN validation MUST fail closed with deterministic diagnostics and MUST NOT silently overwrite a prior record.

### Requirement: Receipt schema compatibility
r[cairn.workspace_contract_matrix.iddqd_indexes.receipt_compatibility] Cairn MUST preserve the existing workspace contract matrix, validation receipt, and release-readiness receipt schemas while using iddqd only inside the pure validation core.

#### Scenario: Public receipt fields remain stable
r[cairn.workspace_contract_matrix.iddqd_indexes.validation]
- GIVEN a valid workspace contract matrix and matching observed artifact identities
- WHEN validation emits a receipt
- THEN the receipt MUST keep the existing public fields, matrix hash behavior, checked edge list, stale edge count, and non-claim boundary.

### Requirement: Identity-map final validation evidence
r[cairn.workspace_contract_matrix.iddqd_indexes.final_validation] The change MUST include positive and negative workspace contract coverage plus focused Cairn validation evidence before archive.

#### Scenario: Identity-index fixtures cover duplicate handling
r[cairn.workspace_contract_matrix.iddqd_indexes.final_validation.fixtures]
- GIVEN valid and invalid workspace contract matrix fixtures
- WHEN focused workspace contract tests and Cairn gates run
- THEN unique edge identities MUST pass and duplicate or stale identities MUST fail closed.
