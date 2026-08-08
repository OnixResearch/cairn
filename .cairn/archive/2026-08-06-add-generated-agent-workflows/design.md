# Design: Generated agent workflows

## Context

Cairn maintains `docs/hermes/cairn-workflow-skill.md` and external tool integrations can copy it. Manual copies can become stale as command, receipt, profile, and evidence contracts change.

The artifact workflow contract provides exact authoring states and instructions. The machine contract provides stable direct CLI outcomes. Typed guidance adds bounded project context.

This change generates thin adapters over those product contracts. It does not duplicate lifecycle logic in prompts.

## Goals

- Generate consistent project-local agent workflow files from one typed source.
- Support a small initial adapter set with explicit versions.
- Make generated effects preview-first and content-addressed.
- Include a no-write exploration workflow.
- Detect stale or locally modified generated files without overwriting them.

## Non-Goals

- Detect agent tools or modify user-home configuration.
- Call a model, agent, network, or package manager.
- Guarantee prompt compliance.
- Reimplement Cairn gates in generated prose.
- Match every OpenSpec integration in the first slice.

## Decisions

### Decision: Define workflow descriptors in typed policy

**Choice:** Generated policy will define tool-neutral descriptors for `explore`, `propose`, `apply`, `verify`, `drain`, and `archive`.

Each descriptor will bind a goal, observable success criteria, required evidence, allowed Cairn command classes, mutation class, progress rule, output shape, constraints, and non-claims.

Policy export will reject duplicate IDs, unknown command classes, missing success criteria, unbounded text, hidden mutation, and unsupported non-claims.

**Rationale:** One typed source reduces drift and keeps prompt behavior reviewable.

### Decision: Keep prompts compact and contract-driven

**Choice:** Rendered workflow text will place the goal first, then success evidence, constraints, immediate action, and output shape.

Prompts will instruct agents to call artifact workflow, machine CLI, context, gates, and receipt commands rather than restating their logic. Generated files will not embed raw project context or command output.

**Rationale:** Small prompts age better and use stable product contracts as authority.

### Decision: Make explore read-only by construction

**Choice:** The `explore` descriptor will allow only policy-classified read commands. It will state that the agent must not create a change, edit lifecycle artifacts, execute mutation plans, or claim implementation completion.

Adapter rendering will fail if an explore descriptor references a mutation-capable command class or omits the no-write constraint.

This static property does not prove that an external agent will comply.

**Rationale:** Exploration should support investigation before lifecycle commitment without becoming an unrecorded mutation path.

### Decision: Use versioned policy-known adapters

**Choice:** The initial adapter registry will include policy-known formats for Agent Skills, Claude, and Codex. Each adapter will define exact project-relative output paths, file names, front matter, command naming, escaping, and maximum file count.

Adapters are deterministic renderers over one admitted descriptor set. They cannot add commands, authority, or claims that the descriptor lacks.

**Rationale:** A small reviewed set gives useful coverage without a broad maintenance matrix.

### Decision: Use explicit project-local planning and installation

**Choice:** Add these commands:

```text
cairn agent workflow list
cairn agent workflow show WORKFLOW
cairn agent workflow plan --adapter ADAPTER --destination PATH
cairn agent workflow install --adapter ADAPTER --destination PATH [--execute]
cairn agent workflow check --adapter ADAPTER --destination PATH
```

Destination paths must remain under an explicit project capability root. Cairn will not inspect installed tools or choose a destination implicitly.

Install is preview-first. Execution rechecks policy, descriptor, adapter, destination, and prior file identities before atomic file replacement.

**Rationale:** Explicit destinations and plans prevent ambient user-home effects.

### Decision: Bind generated files with a manifest

**Choice:** Every plan will include a canonical manifest with generator version, policy identity, descriptor identities, adapter ID and version, relative paths, expected prior identities, generated BLAKE3 identities, and required non-claims.

Freshness check will classify files as current, missing, stale-source, modified, unmanaged-conflict, or unsupported-version. Modified or unmanaged files block replacement until the operator resolves them explicitly.

Cairn will not delete obsolete generated files in the first slice. It will report exact removal candidates.

**Rationale:** Content identity makes updates reviewable and prevents silent loss of local edits.

### Decision: Keep generated workflows outside lifecycle authority

**Choice:** Generated files are local integration artifacts. Their presence, freshness, or use will not satisfy a gate, review, dependency, evidence, archive, or release requirement.

Generated text will name CLI and MCP receipts as the product facts. It will retain non-claims about agent compliance and semantic correctness.

**Rationale:** Prompts guide external tools. They are not lifecycle evidence.

### Decision: Preserve the functional-core boundary

**Choice:** `cairn-core` will own descriptor and adapter admission, prompt model construction, deterministic rendering, path planning, freshness classification, and manifest identity material.

`cairn-cli` will own policy loading, destination capability roots, prior-file reads, locks, atomic writes, JSON output, and exit status.

No runtime component will call a model, agent tool, network, package manager, or shell interpreter.

**Rationale:** Rendering and planning remain deterministic and testable without external systems.

## Rejected Alternatives

### Maintain one hand-written file per tool

Manual copies drift and require repeated review. Adapters render one canonical descriptor set.

### Auto-detect tools and install globally

Detection and user-home mutation introduce ambient state and surprising effects. Selection and destination remain explicit.

### Embed project context in generated files

This can disclose or stale project text. Generated workflows request current bounded context at use time.

### Treat prompt freshness as agent correctness

Fresh files do not prove execution behavior. The contract makes no compliance claim.

## Test Design

Positive tests cover every descriptor, initial adapters, deterministic renderings, path plans, empty destinations, managed updates, manifests, freshness states, explore read-only admission, and repeated byte-stable output.

Negative tests cover duplicate IDs, unknown command classes, hidden mutation, missing no-write constraints, unknown adapters, unsafe destinations, traversal, symlinks, stale plans, modified and unmanaged files, adapter-added authority, secret-like context embedding, network or model hooks, and partial replacement.

Compatibility tests confirm that lifecycle commands and the maintained Hermes source remain unchanged until an explicit later migration.

## Risks and Trade-offs

- Tool formats can change. Adapter versions and freshness diagnostics make drift explicit.
- Generated prose can still be misunderstood. Product commands and receipts remain authoritative.
- Managed-file protection can require manual cleanup. It prevents silent overwrite and deletion.

## Claim Boundary

A valid generation receipt proves only descriptor and adapter admission, deterministic rendered bytes, selected destination facts, and observed file transitions.

It does not prove tool installation, tool compatibility beyond fixtures, agent execution, prompt compliance, semantic correctness, mutation safety, evidence truth, or release eligibility.

## Source Adaptation

The generated cross-tool workflow model adapts OpenSpec supported-tool delivery and explore mode. Cairn keeps a small adapter set, explicit project-local effects, BLAKE3 manifests, and strict lifecycle authority boundaries.
