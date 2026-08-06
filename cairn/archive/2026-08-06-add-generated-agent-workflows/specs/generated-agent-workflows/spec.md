# Generated Agent Workflows Specification Delta

## ADDED Requirements

### Requirement: Agent workflow descriptors are typed policy values

r[cairn.generated_agent_workflows.descriptors] Cairn MUST define agent workflows through typed Nickel policy and checked runtime projections. Each descriptor MUST bind a goal, success evidence, allowed command classes, mutation class, progress rule, output shape, constraints, bounds, and non-claims.

#### Scenario: Valid workflow descriptors export deterministically

r[cairn.generated_agent_workflows.descriptors.valid]
- GIVEN workflow descriptors have unique IDs, known command classes, explicit success evidence, bounded text, and complete non-claims
- WHEN policy export and runtime admission run
- THEN equivalent descriptor values MUST produce the same canonical projection and BLAKE3 identities
- AND runtime rendering MUST use the checked projection without evaluating Nickel.

#### Scenario: Invalid descriptor fails admission

r[cairn.generated_agent_workflows.descriptors.invalid]
- GIVEN a descriptor has a duplicate ID, unknown command class, hidden mutation, missing success evidence, unbounded text, or missing non-claim
- WHEN policy export or runtime admission runs
- THEN Cairn MUST reject the descriptor with stable diagnostics
- AND no adapter may render or install that workflow.

### Requirement: The initial workflow catalog is bounded

r[cairn.generated_agent_workflows.catalog] Cairn MUST register `explore`, `propose`, `apply`, `verify`, `drain`, and `archive` as the initial workflow catalog. Rendered text MUST place the goal before success evidence, constraints, immediate action, and output shape.

#### Scenario: Catalog exposes stable workflow facts

r[cairn.generated_agent_workflows.catalog.list]
- GIVEN the generated workflow policy is current
- WHEN the operator lists or shows agent workflows
- THEN Cairn MUST report sorted IDs, goals, mutation classes, command classes, versions, and non-claims
- AND it MUST not render a tool-specific path without an explicit adapter selection.

#### Scenario: Rendered prompt uses product contracts

r[cairn.generated_agent_workflows.catalog.contracts]
- GIVEN an admitted workflow references artifact, machine, context, gate, or receipt commands
- WHEN an adapter renders its text
- THEN the text MUST direct the agent to inspect those current product results
- AND it MUST not duplicate their decision logic or embed raw command output.

### Requirement: Explore workflow is read-only by construction

r[cairn.generated_agent_workflows.explore] The `explore` descriptor MUST allow only policy-classified read commands. It MUST forbid change creation, artifact edits, mutation execution, and implementation-completion claims.

#### Scenario: Explore descriptor permits only read work

r[cairn.generated_agent_workflows.explore.valid]
- GIVEN the admitted explore descriptor and any admitted adapter
- WHEN descriptor admission and rendering run
- THEN every allowed command class MUST be read-only
- AND rendered constraints MUST state that the agent creates no change or lifecycle artifact.

#### Scenario: Explore mutation reference fails admission

r[cairn.generated_agent_workflows.explore.invalid]
- GIVEN explore references a mutation-capable command class or omits its no-write constraint
- WHEN policy export, descriptor admission, or adapter rendering runs
- THEN Cairn MUST reject the workflow with a stable diagnostic
- AND it MUST not produce an installable explore file.

### Requirement: Tool adapters are versioned and authority-preserving

r[cairn.generated_agent_workflows.adapters] Cairn MUST define versioned policy-known adapters for Agent Skills, Claude, and Codex formats. Each adapter MUST bind exact project-relative paths, front matter, naming, escaping, and file-count bounds.

#### Scenario: Adapter renders exact project-local files

r[cairn.generated_agent_workflows.adapters.valid]
- GIVEN an admitted descriptor set, adapter, and safe explicit project destination
- WHEN pure rendering and file planning run
- THEN Cairn MUST produce deterministic relative paths and exact UTF-8 bytes within adapter bounds
- AND equivalent inputs MUST produce the same BLAKE3 file identities.

#### Scenario: Adapter cannot add authority

r[cairn.generated_agent_workflows.adapters.authority]
- GIVEN an adapter template adds a command, mutation permission, evidence claim, or authority absent from the source descriptor
- WHEN adapter admission or rendering runs
- THEN Cairn MUST reject the adapter with a stable diagnostic
- AND it MUST not emit or install the promoted text.

#### Scenario: Unknown adapter fails explicitly

r[cairn.generated_agent_workflows.adapters.unknown]
- GIVEN an adapter ID or version is absent from generated policy
- WHEN plan, install, or check runs
- THEN Cairn MUST reject the selector with a stable known-adapter list
- AND it MUST not detect tools or choose an ambient adapter.

### Requirement: Workflow commands use explicit destinations

r[cairn.generated_agent_workflows.commands] Cairn MUST expose read-only list, show, plan, and freshness-check commands plus preview-first installation. Plan, install, and check MUST require an explicit adapter and project-local destination.

#### Scenario: Read commands have no file effects

r[cairn.generated_agent_workflows.commands.read]
- GIVEN a valid workflow catalog and adapter
- WHEN list, show, plan, or check runs
- THEN Cairn MUST emit deterministic results without writing files or invoking tools
- AND plan MUST list every proposed managed-file effect.

#### Scenario: Implicit or unsafe destination fails

r[cairn.generated_agent_workflows.commands.destination]
- GIVEN a destination is absent, absolute outside the project root, traversing, symlinked, user-home derived, or selected by tool detection
- WHEN plan, install, or check admission runs
- THEN Cairn MUST reject the destination before rendering or mutation
- AND it MUST not inspect or change the unsafe location.

### Requirement: Generated manifests bind source and destination facts

r[cairn.generated_agent_workflows.manifest] Every workflow file plan MUST include a canonical manifest that binds generator version, policy identity, descriptor identities, adapter ID and version, relative paths, expected prior identities, generated BLAKE3 identities, and non-claims.

#### Scenario: Plan binds generated and prior identities

r[cairn.generated_agent_workflows.manifest.plan]
- GIVEN a valid adapter and observed safe destination state
- WHEN Cairn builds an installation plan
- THEN the manifest MUST bind every generated file and expected prior state
- AND plan identity MUST change when policy, descriptor, adapter, output bytes, or prior state changes.

#### Scenario: Freshness check reports every managed state

r[cairn.generated_agent_workflows.manifest.freshness]
- GIVEN a prior generated manifest and observed destination files
- WHEN freshness check runs
- THEN each managed file MUST be classified as current, missing, stale-source, modified, unmanaged-conflict, or unsupported-version
- AND the result MUST not change any file.

### Requirement: Installation is preview-first and conflict-safe

r[cairn.generated_agent_workflows.install] Workflow installation MUST require explicit `--execute` after a deterministic preview. Execution MUST recheck policy, descriptors, adapter, destination, plan, manifest, and expected prior file identities before atomic managed-file replacement.

#### Scenario: Fresh installation writes exact managed files

r[cairn.generated_agent_workflows.install.fresh]
- GIVEN a reviewed current plan targets a safe destination with matching expected prior state
- WHEN the operator executes installation
- THEN Cairn MUST atomically write exactly the planned files and manifest
- AND post-write verification MUST match every planned BLAKE3 identity.

#### Scenario: Changed destination blocks all replacement

r[cairn.generated_agent_workflows.install.conflict]
- GIVEN a managed file changed, an unmanaged file occupies a target, a symlink appears, the plan is stale, or an adapter version changed
- WHEN installation execution rechecks destination state
- THEN Cairn MUST block every planned replacement with stable diagnostics
- AND it MUST preserve all prior destination files.

#### Scenario: First slice reports obsolete files without deletion

r[cairn.generated_agent_workflows.install.obsolete]
- GIVEN a current adapter no longer generates a previously managed path
- WHEN plan or freshness check runs
- THEN Cairn MUST report the exact obsolete managed path as a removal candidate
- AND it MUST not delete that path in the first implementation slice.

### Requirement: Generated workflows remain non-authoritative

r[cairn.generated_agent_workflows.boundary] Generated workflow files, manifests, freshness results, and installation receipts MUST NOT satisfy lifecycle gates, reviews, dependencies, evidence requirements, archive readiness, or release readiness.

#### Scenario: Generated workflows do not satisfy lifecycle authority

r[cairn.generated_agent_workflows.boundary.nonclaim]
- GIVEN all generated files are current and an external agent reports successful use
- WHEN Cairn evaluates validation, gates, sync, archive, evidence, or release readiness
- THEN generated workflow state MUST not satisfy or weaken those checks
- AND current product receipts and explicit evidence MUST remain necessary.

#### Scenario: Generator performs no external execution

r[cairn.generated_agent_workflows.boundary.effects]
- GIVEN any valid or invalid workflow generation request
- WHEN Cairn lists, shows, plans, installs, or checks generated workflows
- THEN it MUST not call a model, agent tool, network, package manager, or shell interpreter
- AND it MUST not claim that a supported tool is installed or compatible beyond checked fixtures.

### Requirement: Rendering preserves the functional-core boundary

r[cairn.generated_agent_workflows.functional_boundary] Cairn MUST keep descriptor and adapter admission, prompt model construction, rendering, path planning, freshness classification, and manifest identity material pure. The CLI MUST own policy, destination, file, lock, atomic write, output, and exit effects.

#### Scenario: Rendering and planning remain pure

r[cairn.generated_agent_workflows.functional_boundary.pure]
- GIVEN admitted descriptors, adapter, destination observations, and policy
- WHEN Cairn renders and plans generated files
- THEN the result MUST be deterministic without filesystems, environments, processes, clocks, networks, mutation, or output effects.

#### Scenario: Shell rechecks every managed-file effect

r[cairn.generated_agent_workflows.functional_boundary.shell]
- GIVEN a reviewed generation plan and manifest
- WHEN installation executes
- THEN the shell MUST reload and recheck exact source and destination facts under the applicable lock
- AND a mismatch MUST block all writes.

### Requirement: Existing workflow surfaces remain compatible

r[cairn.generated_agent_workflows.compatibility] Adding generated workflows MUST NOT change existing lifecycle commands, MCP tools, artifact workflow results, machine envelopes, project guidance results, or the maintained Hermes source without an explicit later migration.

#### Scenario: Existing lifecycle and Hermes surfaces remain compatible

r[cairn.generated_agent_workflows.compatibility.existing]
- GIVEN unchanged compatibility fixtures and the maintained Hermes source
- WHEN generated workflow support is added
- THEN documented existing bytes, fields, and meanings MUST remain unchanged
- AND generation-specific files MUST appear only after explicit installation execution.

### Requirement: Generated workflow verification and documentation are complete

r[cairn.generated_agent_workflows.verification] The change MUST include positive and negative policy, core, adapter, render, path, manifest, install, freshness, permission, compatibility, and claim-boundary tests.

#### Scenario: Positive and negative suites close generation

r[cairn.generated_agent_workflows.verification.suites]
- GIVEN valid descriptors and malformed, hidden-mutation, unknown, unsafe, stale, modified, unmanaged, promoted, and partial-write fixtures
- WHEN focused tests and repository lifecycle checks run
- THEN valid cases MUST produce deterministic plans and exact managed files
- AND invalid cases MUST fail with stable diagnostics and no partial mutation.

### Requirement: Generated workflow documentation is complete

r[cairn.generated_agent_workflows.docs] Cairn MUST document workflow goals, adapters, explicit destinations, preview, installation, freshness, conflicts, obsolete files, explore boundaries, tool limitations, and non-claims.

#### Scenario: Operator can install without ambient tool state

r[cairn.generated_agent_workflows.docs.install]
- GIVEN an operator reads the generated-workflow guide
- WHEN the operator selects an adapter and destination
- THEN the guide MUST provide exact plan, install, and check commands
- AND it MUST state that Cairn does not detect tools, write user-home configuration, or guarantee agent compliance.
