# Developer Experience Specification

## Purpose

Defines user-facing Cairn workflow improvements that make deterministic lifecycle work easier to start without weakening policy ownership, native layout semantics, receipt evidence, or FCIS boundaries.

## Requirements

### Requirement: Onboarding template catalog
r[cairn.developer_experience.onboarding_templates.catalog] The system MUST expose a reviewed onboarding template catalog for supported first-use repository profiles.

#### Scenario: List reviewed templates
r[cairn.developer_experience.onboarding_templates.catalog.list]
- GIVEN a Cairn policy bundle with reviewed onboarding template metadata
- WHEN the operator asks Cairn to list onboarding templates
- THEN the CLI emits a deterministic receipt containing sorted template identifiers, descriptions, supported files, and policy hash
- AND the receipt does not depend on filesystem traversal order, current time, network access, or agent-specific environment values

### Requirement: Onboarding template application
r[cairn.developer_experience.onboarding_templates.apply] The system MUST apply onboarding templates only through explicit safe preview or execute flows.

#### Scenario: Preview template materialization
r[cairn.developer_experience.onboarding_templates.apply.preview]
- GIVEN a repository root and a supported onboarding template identifier
- WHEN the operator previews template application
- THEN the CLI emits a deterministic plan receipt listing every file that would be created or skipped
- AND no filesystem mutation occurs

#### Scenario: Execute template materialization
r[cairn.developer_experience.onboarding_templates.apply.execute]
- GIVEN a repository root and a reviewed template plan with no blockers
- WHEN the operator executes template application
- THEN the CLI creates only safe root-relative template paths
- AND the mutation receipt records before and after BLAKE3 hashes for touched files

#### Scenario: Reject unsafe or conflicting template paths
r[cairn.developer_experience.onboarding_templates.apply.reject_unsafe]
- GIVEN a template expansion contains traversal, absolute paths, or an overwrite conflict without an explicit allowed policy
- WHEN the operator previews or executes the template
- THEN Cairn emits a deterministic failure receipt
- AND no unsafe path is written

### Requirement: Onboarding quickstart documentation
r[cairn.developer_experience.onboarding_templates.quickstart] The system MUST document a concise template-based first-use path for new Cairn repositories.

#### Scenario: Follow documented quickstart
r[cairn.developer_experience.onboarding_templates.quickstart.follow]
- GIVEN a user follows the documented onboarding template quickstart in a clean test repository
- WHEN they run the listed Cairn commands
- THEN the repository has a native `.cairn/` lifecycle layout and at least one reviewable change package
- AND the documented validation command emits a passing deterministic receipt

### Requirement: Onboarding template verification
r[cairn.developer_experience.onboarding_templates.verification] The system MUST regression-test onboarding templates with deterministic positive and negative coverage.

#### Scenario: Verify template receipts
r[cairn.developer_experience.onboarding_templates.verification.receipts]
- GIVEN positive and negative onboarding template fixtures
- WHEN the workspace verification suite runs
- THEN supported templates produce stable passing receipts
- AND unsupported identifiers, unsafe paths, and overwrite conflicts produce stable failure issue codes

### Requirement: Prompt-to-change scaffold planner
r[cairn.developer_experience.prompt_to_change_scaffold.planner] The system MUST plan a complete draft change package from explicit operator-provided request material without network or LLM execution.

#### Scenario: Plan package artifacts from request text
r[cairn.developer_experience.prompt_to_change_scaffold.planner.request_text]
- GIVEN explicit request text, a change name, and an optional spec domain
- WHEN Cairn plans a prompt-to-change scaffold
- THEN the planner returns proposal, design, tasks, and delta spec artifact bodies
- AND the plan records a stable input hash over the provided request material

#### Scenario: Reject insufficient scaffold input
r[cairn.developer_experience.prompt_to_change_scaffold.planner.insufficient_input]
- GIVEN empty request material or an invalid change name
- WHEN Cairn plans a prompt-to-change scaffold
- THEN the planner returns deterministic issues
- AND no package artifacts are approved for execution

### Requirement: Prompt-to-change scaffold CLI
r[cairn.developer_experience.prompt_to_change_scaffold.cli] The system MUST expose prompt-to-change scaffolding through preview and explicit execute CLI flows.

#### Scenario: Preview scaffold writes
r[cairn.developer_experience.prompt_to_change_scaffold.cli.preview]
- GIVEN a valid scaffold request
- WHEN the operator runs the scaffold command without execute mode
- THEN Cairn emits a deterministic receipt listing the root-relative files that would be written
- AND no filesystem mutation occurs

#### Scenario: Execute scaffold writes
r[cairn.developer_experience.prompt_to_change_scaffold.cli.execute]
- GIVEN a valid scaffold request and an initialized native Cairn repository
- WHEN the operator runs the scaffold command in execute mode
- THEN Cairn writes the planned change package files under `.cairn/changes/<change>/`
- AND the mutation receipt records stable before and after file hashes

### Requirement: Gate-ready scaffold output
r[cairn.developer_experience.prompt_to_change_scaffold.gate_ready] The system MUST generate scaffolded task references and delta scenarios that are structurally gate-ready.

#### Scenario: Scaffolded task references match delta IDs
r[cairn.developer_experience.prompt_to_change_scaffold.gate_ready.refs]
- GIVEN a scaffolded change package
- WHEN Cairn validates the package and evaluates the proposal, design, and tasks gates
- THEN task checkboxes reference requirement IDs declared by the same delta spec or explicit dependency markers
- AND the gates do not fail because of unknown task references or missing EARS scenario clauses

### Requirement: Prompt-to-change operator documentation
r[cairn.developer_experience.prompt_to_change_scaffold.docs] The system MUST document prompt-to-change scaffolding as deterministic structure generation rather than built-in AI implementation.

#### Scenario: Docs state non-claims
r[cairn.developer_experience.prompt_to_change_scaffold.docs.non_claims]
- GIVEN an operator reads the prompt-to-change scaffold documentation
- WHEN they inspect the command description
- THEN the documentation states that Cairn consumes explicit local input and does not call remote LLMs, credentials, network services, or hidden agent state

### Requirement: Prompt-to-change scaffold verification
r[cairn.developer_experience.prompt_to_change_scaffold.verification] The system MUST test scaffold generation with positive and negative deterministic fixtures.

#### Scenario: Verify scaffold fixtures
r[cairn.developer_experience.prompt_to_change_scaffold.verification.fixtures]
- GIVEN reviewed scaffold input fixtures
- WHEN the workspace verification suite runs
- THEN valid fixtures produce stable artifact plans and receipts
- AND invalid inputs produce stable failure issue codes without writing files

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
- AND it does not modify `.cairn/specs/` or promote candidates to accepted requirements

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

### Requirement: Directory include expansion
r[cairn.developer_experience.reverse_spec_directory_inventory.expand] The system MUST expand explicit directory includes into deterministic safe relative file snapshots before reverse-spec planning.

#### Scenario: Expand source directory
r[cairn.developer_experience.reverse_spec_directory_inventory.expand.source_dir]
- GIVEN an operator includes a repository-relative directory containing source, docs, tests, hidden files, and build artifacts
- WHEN Cairn prepares reverse-spec discovery snapshots
- THEN only safe text-like files under reviewed directory classes are selected in sorted order
- AND hidden, build, vendor, and unsafe paths are skipped without reading them as candidate evidence

### Requirement: Directory inventory receipt evidence
r[cairn.developer_experience.reverse_spec_directory_inventory.receipt] The system MUST expose selected and skipped directory inventory paths in the normal discovery receipt.

#### Scenario: Review directory receipt
r[cairn.developer_experience.reverse_spec_directory_inventory.receipt.review]
- GIVEN directory includes produce selected and skipped paths
- WHEN Cairn emits the discovery receipt
- THEN the receipt includes stable selected_paths and skipped_paths arrays
- AND optional draft output remains review-needed candidate material, not accepted specification content

### Requirement: Directory inventory documentation
r[cairn.developer_experience.reverse_spec_directory_inventory.docs] The system MUST document directory include usage and non-claims for reverse-spec discovery.

#### Scenario: Read operator guidance
r[cairn.developer_experience.reverse_spec_directory_inventory.docs.read]
- GIVEN an operator reads Cairn's quickstart command list
- WHEN reverse-spec discovery is described
- THEN directory include examples are present
- AND the documentation states that generated candidates still require review

### Requirement: Directory inventory verification
r[cairn.developer_experience.reverse_spec_directory_inventory.verification] The system MUST verify directory inventory expansion with positive and negative regression coverage.

#### Scenario: Run directory inventory tests
r[cairn.developer_experience.reverse_spec_directory_inventory.verification.run]
- GIVEN fixture directories with eligible and ineligible files
- WHEN focused developer-experience tests run
- THEN eligible files produce deterministic candidates
- AND ineligible paths appear as skipped or absent without mutating accepted specs

### Requirement: Expanded onboarding template catalog
r[cairn.developer_experience.onboarding_template_catalog_expansion.catalog] The system MUST expose additional reviewed onboarding templates for docs-only and release-evidence repository profiles.

#### Scenario: List expanded templates
r[cairn.developer_experience.onboarding_template_catalog_expansion.catalog.list]
- GIVEN an operator runs the template listing command
- WHEN Cairn renders the onboarding catalog
- THEN docs-only and release-evidence profile identifiers are present with stable descriptions
- AND existing template identifiers remain available

### Requirement: Expanded template application
r[cairn.developer_experience.onboarding_template_catalog_expansion.apply] The system MUST apply expanded templates through the existing preview-first safe write semantics.

#### Scenario: Preview expanded template
r[cairn.developer_experience.onboarding_template_catalog_expansion.apply.preview]
- GIVEN an operator previews a new template
- WHEN Cairn emits the apply receipt
- THEN the receipt lists planned files and content hashes
- AND no filesystem mutation occurs without `--execute`

### Requirement: Expanded onboarding documentation
r[cairn.developer_experience.onboarding_template_catalog_expansion.docs] The system MUST document the expanded template catalog and its non-goals.

#### Scenario: Read expanded onboarding docs
r[cairn.developer_experience.onboarding_template_catalog_expansion.docs.read]
- GIVEN an operator reads Cairn's onboarding command examples
- WHEN templates are described
- THEN docs-only and release-evidence examples are included
- AND the text avoids remote publishing, hosted UI, and signing claims

### Requirement: Expanded onboarding verification
r[cairn.developer_experience.onboarding_template_catalog_expansion.verification] The system MUST verify the expanded template catalog with deterministic regression coverage.

#### Scenario: Run expanded template tests
r[cairn.developer_experience.onboarding_template_catalog_expansion.verification.run]
- GIVEN focused developer-experience CLI tests
- WHEN template listing and apply behavior are exercised
- THEN the new templates produce deterministic receipts
- AND unknown-template failure behavior remains fail-closed

### Requirement: Developer-experience CLI focused modules
r[cairn.developer_experience.cli_modules.focused_modules] The system MUST keep developer-experience CLI command families in focused source modules with clear responsibilities.

#### Scenario: Review focused command modules
r[cairn.developer_experience.cli_modules.focused_modules.review]
- GIVEN the developer-experience CLI implementation is inspected
- WHEN a maintainer reviews template, scaffold, discovery, and shared helper logic
- THEN each command family is located in a focused module
- AND shared filesystem helper code is separated from command dispatch

### Requirement: Developer-experience CLI façade boundary
r[cairn.developer_experience.cli_modules.facade_boundary] The system MUST preserve the existing developer-experience CLI façade used by top-level dispatch.

#### Scenario: Dispatch through stable façade
r[cairn.developer_experience.cli_modules.facade_boundary.dispatch]
- GIVEN `main.rs` dispatches `template`, `scaffold`, and `discover-specs` commands
- WHEN the developer-experience implementation is split into submodules
- THEN the façade still exposes `run_template`, `run_scaffold`, and `run_discover_specs`
- AND top-level CLI dispatch does not need command-specific submodule knowledge

### Requirement: Developer-experience CLI behavior preservation
r[cairn.developer_experience.cli_modules.behavior_preservation] The system MUST preserve developer-experience CLI receipts and mutation semantics across the module split.

#### Scenario: Verify unchanged command receipts
r[cairn.developer_experience.cli_modules.behavior_preservation.receipts]
- GIVEN existing focused developer-experience CLI tests
- WHEN the module split is verified
- THEN template, scaffold, and discovery commands keep their receipt fields, policy IDs, safe path checks, and mutation manifest semantics
- AND no new user-facing command syntax is required

### Requirement: Developer-experience CLI archived design context
r[cairn.developer_experience.cli_modules.archived_design] The system MUST archive the module-boundary rationale for future maintainability review.

#### Scenario: Review archived split context
r[cairn.developer_experience.cli_modules.archived_design.review]
- GIVEN the split change is archived
- WHEN a maintainer reviews the archive
- THEN the proposal, design, tasks, and spec delta describe the façade and submodule boundaries
- AND the archive records that the split is behavior-preserving rather than a new capability

### Requirement: Canonical Git remote is documented for operators
r[developer_experience.git_remote_relocation_cleanup.configure] The system SHOULD document the canonical Cairn Git remote so operators can remove relocation-warning noise from future pushes.

#### Scenario: Remote configuration is verified
r[developer_experience.git_remote_relocation_cleanup.verify]
- GIVEN an operator updates the local `origin` URL
- WHEN they inspect `git remote -v` and branch status
- THEN the remote SHALL point at `git@github.com:OnixResearch/cairn.git` and the worktree SHALL remain clean and synced.

#### Scenario: Remote note avoids credential overclaims
r[developer_experience.git_remote_relocation_cleanup.docs]
r[developer_experience.git_remote_relocation_cleanup.validation]
- GIVEN the operator note is committed
- WHEN future agents inspect repository docs
- THEN it SHALL describe the canonical URL and SHALL NOT expose credentials, SSH agent sockets, or imply credential validation.

### Requirement: CLI maintainability hotspot inventory
r[developer_experience.cli_maintainability_hotspot_inventory.scan] The system SHOULD maintain an operator-readable inventory of large Cairn source/test hotspots after release checkpoints.

#### Scenario: Inventory records future split seams
r[developer_experience.cli_maintainability_hotspot_inventory.docs]
- GIVEN a post-release maintainability review
- WHEN the operator scans source and test sizes
- THEN the inventory SHALL list the largest hotspots, candidate future splits, and focused verification commands.

#### Scenario: Inventory remains non-mutating advisory evidence
r[developer_experience.cli_maintainability_hotspot_inventory.boundary]
r[developer_experience.cli_maintainability_hotspot_inventory.validation]
- GIVEN the inventory is committed
- WHEN validation runs
- THEN it SHALL avoid behavior-change claims, generated artifacts, broad rewrites, and unverified refactor assertions.

### Requirement: Core policy parser boundary split
r[developer_experience.core_policy_parser_boundary_split.map] The system SHOULD keep the core policy parser review surface split into named modules for public DTOs, errors, generic field extraction, Steel profile decoding, parser assembly, and tests.

#### Scenario: Policy parser responsibilities are visible
r[developer_experience.core_policy_parser_boundary_split.modules]
- GIVEN a maintainer inspects `crates/cairn-core/src/verified/policy`
- WHEN they review the module layout
- THEN public policy DTOs, errors, field helpers, Steel profile decoding, parser assembly, and tests SHALL live in named files behind the same public facade.

#### Scenario: Policy parser behavior is preserved
r[developer_experience.core_policy_parser_boundary_split.behavior]
r[developer_experience.core_policy_parser_boundary_split.tests]
r[developer_experience.core_policy_parser_boundary_split.validation]
- GIVEN the parser split is implemented
- WHEN focused policy tests and repository validation run
- THEN generated policy parsing, unsupported-version rejection, Steel profile validation, public exports, issue-code material, and Tracey coverage SHALL pass without intentional policy or fixture drift.

### Requirement: Package licenses follow the core-shell boundary

r[cairn.license_boundary.package_split] `cairn-core` MUST remain available under `MIT OR Apache-2.0`, and `cairn-cli` MUST declare `AGPL-3.0-or-later`.

#### Scenario: A consumer selects one package

- GIVEN a consumer inspects package metadata
- WHEN it selects `cairn-core` or `cairn-cli`
- THEN the selected package MUST expose the license assigned to its architectural role.

### Requirement: Complete license artifacts are checked in

r[cairn.license_boundary.artifacts] Cairn MUST ship complete texts for every repository-owned package license and MUST provide a boundary map that identifies permissive copied-content surfaces.

#### Scenario: A source distribution is reviewed

- GIVEN a recipient receives the Cairn source tree
- WHEN license files are inspected
- THEN AGPL, MIT, and Apache terms MUST be present and the package/content mapping MUST be explicit.

### Requirement: Consumer content remains permissive

r[cairn.license_boundary.consumer_boundary] Cairn project templates, examples, schemas, and starter lifecycle content MUST remain available under `MIT OR Apache-2.0`, and documentation MUST NOT claim that processing user-authored content changes its license.

#### Scenario: Cairn initializes another repository

- GIVEN Cairn copies starter content into a consumer repository
- WHEN that repository chooses its project license
- THEN the copied Cairn starter content MUST remain permissively licensed and the Cairn CLI license MUST NOT be represented as automatically licensing unrelated user-authored content.

### Requirement: License declarations are validated

r[cairn.license_boundary.validation] Cairn MUST deterministically reject repository-owned manifest or documentation drift that reverses the core-shell split while ignoring third-party dependency license expressions.

#### Scenario: Core metadata becomes AGPL accidentally

- GIVEN `cairn-core` inherits or declares AGPL
- WHEN the license-boundary check runs
- THEN the check MUST fail with a package-specific mismatch.

#### Scenario: CLI metadata remains permissive accidentally

- GIVEN `cairn-cli` inherits the permissive workspace license
- WHEN the license-boundary check runs
- THEN the check MUST fail with a package-specific mismatch.

### Requirement: Focused verification proves the boundary

r[cairn.license_boundary.final_validation] The change MUST verify package metadata, complete license artifacts, current boundary documentation, and both expected mismatch classes.

#### Scenario: The final repository state is checked

- GIVEN the intended core-shell mapping and license files are present
- WHEN focused verification and Cairn gates run
- THEN the boundary MUST pass without requiring product runtime tests unrelated to packaging.
