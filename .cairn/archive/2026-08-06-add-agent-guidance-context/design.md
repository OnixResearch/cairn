# Design: Typed agent guidance context

## Context

The lifecycle-store change defines `.cairn/project.ncl`, a checked runtime projection, direct store references, and read-only context inspection.

The artifact workflow contract defines policy-owned artifact instructions. It does not add project-specific facts or advisory rules.

This change composes those surfaces after both land. It does not modify the scope of either active prerequisite.

## Goals

- Let repositories declare bounded local context files and advisory guidance.
- Select only entries relevant to one artifact or operation.
- Bind every loaded input by BLAKE3 identity.
- Keep enforceable policy separate from project facts and advice.
- Prevent raw context from entering portable evidence by default.

## Non-Goals

- Fetch remote content or inspect transitive store references.
- Execute commands found in guidance.
- Let prose override policy or command authorization.
- Detect every secret, false statement, or semantic conflict.
- Generate tool-specific workflow files.

## Decisions

### Decision: Extend the checked project declaration

**Choice:** Extend `.cairn/project.ncl` and its checked JSON projection with these typed collections:

- `context_refs`: bounded UTF-8 file references with source root, safe relative path, required flag, and selectors.
- `artifact_guidance`: bounded advisory text selected by profile and artifact ID.
- `operation_guidance`: bounded advisory text selected by a policy-known operation ID.

The generated policy will define collection counts, text sizes, path depth, allowed source roots, known selectors, and required authority labels.

Normal commands will read only the checked projection and reject source-projection drift.

**Rationale:** Project declarations remain shared, reviewable, typed, and replayable.

### Decision: Allow only bounded local file references

**Choice:** A context reference can select a safe relative UTF-8 text file under the observed project root or selected lifecycle root.

References cannot contain absolute paths, parent traversal, globs, URLs, environment expansion, command text, symlink traversal, or unbounded binary data. The CLI will load each file through an explicit capability root and report missing optional or required sources differently.

**Rationale:** Context loading is local file admission, not a general data-fetch system.

### Decision: Select context through pure typed selectors

**Choice:** Each entry can select profile IDs, artifact IDs, or policy-known operation IDs. Empty selectors mean all admitted values within the declared class.

The pure core will reject unknown selectors, normalize duplicates, and sort selected entries by class, declared priority, source root, and path or guidance ID.

It will return policy constraints first, project facts second, and advisory guidance last.

**Rationale:** Stable selection prevents agents from loading all project text or choosing an ambient order.

### Decision: Keep authority classes explicit

**Choice:** Every output entry will carry one authority class:

- `policy_constraint`: generated policy that commands enforce.
- `project_fact`: explicitly declared local context supplied for authoring.
- `advisory_guidance`: project text that Cairn does not enforce.

Project configuration cannot create `policy_constraint` entries or alter command, path, gate, profile, mutation, or evidence authority.

Cairn will not interpret free-form text to decide whether it contradicts policy. Execution continues to use policy and typed inputs only.

**Rationale:** Labels and data flow prevent advice from becoming hidden authority without claiming semantic text analysis.

### Decision: Extend context and workflow instructions

**Choice:** Read-only context output will report declaration health and selected entry metadata. Workflow instructions will include the selected bounded text for the requested artifact, together with relative path, byte length, BLAKE3 identity, selectors, authority class, and diagnostics.

MCP will expose the same semantic values through strict read-only tools. Unknown fields or selectors fail before file loading.

No context or instruction command will edit files, run a process, contact a network, or satisfy a lifecycle gate.

**Rationale:** Agents receive needed facts at the existing read-only authoring boundary.

### Decision: Separate local and portable identities

**Choice:** The full local result hash will bind selected text and local source facts. A separate portable identity will bind normalized selectors, authority classes, relative paths, content BLAKE3 identities, policy, and project identity.

Portable evidence export will omit raw context text and absolute paths. Raw context can appear only in the explicit local read result.

**Rationale:** Agents need text locally, but evidence sharing normally needs identity rather than content.

### Decision: Bound privacy claims

**Choice:** Cairn will validate declared paths, bounds, encoding, and output classes. It will not claim that declared context is free of secrets or safe to disclose.

Documentation will require operators to declare only text they intend local agents to read. Evidence commands will fail if a caller attempts to promote raw context into a portable role without a later explicit policy.

**Rationale:** Deterministic loading cannot prove content sensitivity.

### Decision: Preserve the functional-core boundary

**Choice:** `cairn-core` will own declaration admission, selector matching, duplicate handling, ordering, authority classification, result construction, and portable identity material.

`cairn-cli` will own project and policy loading, capability-root file reads, UTF-8 admission, BLAKE3 hashing, JSON output, MCP transport, and evidence omission.

**Rationale:** Selection logic remains testable without files, environments, processes, clocks, networks, or output effects.

## Rejected Alternatives

### Inject all repository documentation

This is unbounded and irrelevant for many artifacts. Explicit references and selectors keep input small.

### Use free-form prompt files as policy

Prompt text cannot replace typed gates and mutation authorization. Guidance remains advisory.

### Load URLs or run context commands

This introduces network, credentials, process effects, freshness, and trust questions. The first slice is local and read-only.

### Store raw context in portable receipts

This can disclose private project text. Portable identity uses BLAKE3 content identities instead.

## Test Design

Positive tests cover checked projections, both source roots, required and optional files, selector matching, authority ordering, workflow instruction composition, local and portable identities, CLI and MCP parity, and repeated deterministic output.

Negative tests cover stale projections, traversal, absolute paths, symlinks, globs, URLs, environment text, commands, invalid UTF-8, oversize text, duplicate IDs, unknown selectors, missing required files, policy-authority impersonation, raw portable export, and secret-like fixture non-claims.

Compatibility tests confirm unchanged context and workflow results when no guidance fields exist.

## Risks and Trade-offs

- Project text can contain secrets. The contract limits exposure but cannot classify all content.
- Guidance can be false or stale. Source identities expose changes but do not prove truth.
- More context can distract agents. Selectors and policy bounds keep results focused.

## Claim Boundary

A valid guidance result proves only declaration admission, selected local bytes, authority labels, BLAKE3 identities, and deterministic ordering.

It does not prove context truth, freshness beyond observed bytes, secrecy, semantic consistency, agent compliance, policy satisfaction, implementation correctness, or release eligibility.

## Source Adaptation

The context and artifact guidance interaction adapts OpenSpec project context and per-artifact rules. Cairn adds typed declarations, authority classes, local capability bounds, content identities, and portable-evidence limits.
