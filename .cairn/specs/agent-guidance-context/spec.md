# Agent Guidance Context Specification

## Purpose

Defines the `agent-guidance-context` capability.

## Requirements

### Requirement: Project guidance configuration is typed and checked

r[cairn.agent_guidance_context.project_config] Cairn MUST define context references, artifact guidance, and operation guidance through typed project Nickel source and a checked runtime projection. Generated policy MUST bound collections, selectors, paths, text, authority classes, and receipt fields.

#### Scenario: Valid guidance projection exports deterministically

r[cairn.agent_guidance_context.project_config.valid]
- GIVEN valid project context references and guidance satisfy the typed contracts
- WHEN project configuration export runs
- THEN equivalent source values MUST produce the same canonical JSON bytes and BLAKE3 source identity
- AND ordinary context and workflow commands MUST use the checked projection without evaluating Nickel.

#### Scenario: Stale or invalid projection fails

r[cairn.agent_guidance_context.project_config.invalid]
- GIVEN project guidance source or projection is stale, malformed, duplicated, out of bounds, or uses an unknown selector
- WHEN export, context, or workflow admission runs
- THEN Cairn MUST reject the project guidance with stable diagnostics
- AND it MUST NOT load or emit the rejected context text.

### Requirement: Context declarations fail closed before source loading

r[cairn.agent_guidance_context.admission] Cairn MUST admit normalized context and guidance declarations before reading selected files. Admission MUST reject duplicate IDs, conflicting duplicates, unknown selectors, unsupported authority classes, invalid priorities, and values outside policy bounds.

#### Scenario: Equivalent declarations normalize deterministically

r[cairn.agent_guidance_context.admission.valid]
- GIVEN declarations contain equivalent repeated selectors and otherwise valid bounded values
- WHEN pure declaration admission runs
- THEN Cairn MUST normalize selectors into one stable ordered set
- AND equivalent declaration values MUST produce the same admitted identity.

#### Scenario: Conflicting declaration blocks all source loading

r[cairn.agent_guidance_context.admission.invalid]
- GIVEN declarations contain duplicate IDs with different values, unknown selectors, unsupported authority, or values outside policy bounds
- WHEN declaration admission runs
- THEN Cairn MUST reject the declaration set with stable diagnostics
- AND the shell MUST not load any context source for the rejected set.

### Requirement: Context sources are local and bounded

r[cairn.agent_guidance_context.sources] Cairn MUST load only declared safe relative UTF-8 text files under the observed project root or selected lifecycle root. It MUST NOT expand globs, URLs, environment values, commands, absolute paths, parent traversal, symlinks, or unbounded binary data.

#### Scenario: Safe local context source loads

r[cairn.agent_guidance_context.sources.valid]
- GIVEN a declared context file is within its capability root, valid UTF-8, and within policy bounds
- WHEN the shell loads the selected source
- THEN Cairn MUST report its safe relative path, byte length, BLAKE3 identity, selectors, and required status
- AND it MAY include the text only in the explicit local read result.

#### Scenario: Unsafe or missing required source fails

r[cairn.agent_guidance_context.sources.invalid]
- GIVEN a selected source is absolute, traversing, symlinked, globbed, remote, command-derived, invalid UTF-8, oversized, or missing when required
- WHEN source admission runs
- THEN Cairn MUST emit a stable diagnostic and reject the selected result
- AND it MUST NOT read outside the declared capability root or use an optional ambient fallback.

#### Scenario: Missing optional source remains visible

r[cairn.agent_guidance_context.sources.optional]
- GIVEN a selected context source is declared optional and does not exist
- WHEN source admission runs
- THEN Cairn MUST report the source as missing with its declared identity facts
- AND it MUST continue only when policy permits optional absence.

### Requirement: Context selection is deterministic and focused

r[cairn.agent_guidance_context.selection] Cairn MUST select context by admitted workflow profile, artifact ID, and policy-known operation ID. It MUST normalize duplicates and order entries by authority class, declared priority, source root, and stable ID or path.

#### Scenario: Artifact selectors return focused context

r[cairn.agent_guidance_context.selection.artifact]
- GIVEN project declarations include global and artifact-specific context entries
- WHEN the operator requests workflow instructions for one artifact
- THEN Cairn MUST include global entries and entries that select that profile and artifact
- AND it MUST omit entries that select only unrelated artifacts or operations.

#### Scenario: Unknown selector fails before file loading

r[cairn.agent_guidance_context.selection.unknown]
- GIVEN a declaration or request names an unknown profile, artifact, or operation selector
- WHEN selector admission runs
- THEN Cairn MUST reject the selector with a stable known-value diagnostic
- AND it MUST not load any file selected only by the rejected value.

### Requirement: Guidance authority remains explicit

r[cairn.agent_guidance_context.authority] Cairn MUST classify selected entries as `policy_constraint`, `project_fact`, or `advisory_guidance`. Project configuration MUST NOT create policy constraints or alter command, path, gate, profile, mutation, evidence, or release authority.

#### Scenario: Project guidance cannot impersonate policy

r[cairn.agent_guidance_context.authority.reject]
- GIVEN project configuration assigns policy authority or declares an unsupported control field
- WHEN declaration admission runs
- THEN Cairn MUST reject that entry with a stable authority diagnostic
- AND command execution MUST continue to use generated policy and typed inputs only.

#### Scenario: Advisory text remains non-enforcing

r[cairn.agent_guidance_context.authority.advisory]
- GIVEN admitted advisory guidance asks an agent to take an action
- WHEN a Cairn gate or mutation command runs
- THEN the command MUST ignore that prose for authorization and verdict decisions
- AND explicit policy, arguments, gates, and evidence MUST remain necessary.

### Requirement: Context composes with workflow instructions

r[cairn.agent_guidance_context.integration] Cairn MUST extend read-only context and artifact instruction results with selected entry metadata, bounded local text, diagnostics, authority classes, and identities. These read surfaces MUST perform no repository, local-state, process, or network mutation.

#### Scenario: Workflow instructions include bounded selected text

r[cairn.agent_guidance_context.integration.instructions]
- GIVEN admitted context and guidance select a ready workflow artifact
- WHEN the operator requests artifact instructions
- THEN Cairn MUST emit policy constraints first, project facts second, and advisory guidance last
- AND each included entry MUST bind its selectors, authority, source, bounds, and BLAKE3 identity.

#### Scenario: Context inspection does not satisfy a gate

r[cairn.agent_guidance_context.integration.non_authority]
- GIVEN context and guidance load without diagnostics
- WHEN validation, a gate, dependency evaluation, sync, archive, evidence, or release readiness runs
- THEN successful context loading MUST NOT satisfy or weaken that command
- AND existing required inputs and receipts MUST remain necessary.

### Requirement: MCP guidance results match CLI semantics

r[cairn.agent_guidance_context.mcp] Cairn MUST expose selected guidance through strict read-only MCP context and workflow tools. MCP and CLI results MUST derive from the same pure selected values.

#### Scenario: CLI and MCP context facts are equivalent

r[cairn.agent_guidance_context.mcp.parity]
- GIVEN the same policy, project projection, selected roots, local text observations, profile, artifact, and operation
- WHEN CLI and MCP inspection run
- THEN both surfaces MUST report equivalent selected entries, diagnostics, authority classes, and identities
- AND transport framing MAY remain different.

#### Scenario: MCP rejects unknown guidance arguments

r[cairn.agent_guidance_context.mcp.invalid]
- GIVEN an MCP context request contains an unknown field or selector
- WHEN strict argument validation runs
- THEN the tool MUST reject the request before context file loading
- AND no mutation-capable tool may be selected as a fallback.

### Requirement: Local and portable identities are separate

r[cairn.agent_guidance_context.receipts] Guidance results MUST bind selected raw text and local source facts in a full local hash. A separate portable identity MUST bind normalized selectors, authority classes, relative paths, content BLAKE3 identities, project identity, and policy without raw text or absolute paths.

#### Scenario: Portable evidence omits raw context

r[cairn.agent_guidance_context.receipts.portable]
- GIVEN a valid local guidance result includes selected context text
- WHEN Cairn derives a portable identity or exports an admitted portable role
- THEN the portable value MUST include content identities instead of raw context and absolute paths
- AND equivalent semantic inputs under different safe checkout paths MUST produce the same portable identity.

#### Scenario: Raw portable promotion fails

r[cairn.agent_guidance_context.receipts.reject_raw]
- GIVEN a caller attempts to assign raw guidance text to a portable evidence role without a later explicit policy
- WHEN evidence admission runs
- THEN Cairn MUST reject the promotion with a stable diagnostic
- AND it MUST not write the raw text into the portable bundle.

### Requirement: Guidance privacy claims remain bounded

r[cairn.agent_guidance_context.privacy] Cairn MUST state that path, encoding, size, selection, and evidence-omission checks do not prove that declared text is free of secrets or safe to disclose.

#### Scenario: Secret-like text remains an operator responsibility

r[cairn.agent_guidance_context.privacy.secret]
- GIVEN an operator explicitly declares a bounded local file that contains sensitive text
- WHEN local context inspection includes that file
- THEN Cairn MUST not label the text safe, public, or secret-free
- AND portable evidence omission MUST remain active by default.

#### Scenario: Undeclared file content is not loaded

r[cairn.agent_guidance_context.privacy.undeclared]
- GIVEN a project contains files that no admitted context reference selects
- WHEN context or workflow instruction loading runs
- THEN Cairn MUST not read or include those files
- AND directory discovery MUST not expand the declared context set.

### Requirement: Context decisions preserve the functional-core boundary

r[cairn.agent_guidance_context.functional_boundary] Cairn MUST keep declaration admission, selector matching, duplicate handling, ordering, authority classification, result construction, and portable identity material pure. The CLI MUST own project, policy, capability-root, file, encoding, hash, output, transport, and evidence effects.

#### Scenario: Context selection remains pure

r[cairn.agent_guidance_context.functional_boundary.pure]
- GIVEN admitted declarations and explicit loaded text observations
- WHEN Cairn selects and constructs a guidance result
- THEN the result MUST be deterministic without filesystems, environments, processes, clocks, networks, mutation, or output effects.

#### Scenario: Shell reads only requested bounded sources

r[cairn.agent_guidance_context.functional_boundary.shell]
- GIVEN the pure selector returns an ordered set of required source observations
- WHEN the CLI loads context
- THEN it MUST read only those safe capability-relative paths within policy bounds
- AND it MUST pass explicit text and identity observations back to the pure core.

### Requirement: Empty guidance preserves compatibility

r[cairn.agent_guidance_context.compatibility] Projects without context and guidance fields MUST retain existing project, context, workflow, gate, sync, archive, MCP, evidence, and release-readiness behavior.

#### Scenario: Projects without guidance retain compatibility

r[cairn.agent_guidance_context.compatibility.empty]
- GIVEN a valid existing project projection has no guidance fields
- WHEN profile-aware context and workflow inspection run
- THEN Cairn MUST derive an empty project-guidance set
- AND documented existing fields and receipt meanings MUST remain unchanged.

### Requirement: Guidance claims and verification are bounded

r[cairn.agent_guidance_context.boundary] A valid guidance result MUST claim only declaration admission, selected observed bytes, authority labels, identities, and deterministic ordering. It MUST NOT claim truth, secrecy, semantic consistency, agent compliance, policy satisfaction, code correctness, or release eligibility.

#### Scenario: Guidance output rejects claim promotion

r[cairn.agent_guidance_context.boundary.overclaim]
- GIVEN a result labels project facts or advice as verified policy, satisfied evidence, correct implementation, or release approval
- WHEN guidance result admission runs
- THEN Cairn MUST reject the unsupported authority label
- AND existing domain checks MUST retain their authority.

#### Scenario: Positive and negative suites close the context contract

r[cairn.agent_guidance_context.verification]
- GIVEN valid context fixtures and stale, unsafe, malformed, oversized, unknown, unauthorized, private, and overclaim fixtures
- WHEN focused tests and repository lifecycle checks run
- THEN valid cases MUST produce deterministic bounded results
- AND invalid cases MUST fail with stable diagnostics and no mutation.

### Requirement: Guidance documentation is complete

r[cairn.agent_guidance_context.docs] Cairn MUST document project declarations, source bounds, selectors, authority classes, instruction composition, local output, portable omission, privacy duties, compatibility, and non-claims.

#### Scenario: Operator can distinguish facts, policy, and advice

r[cairn.agent_guidance_context.docs.authority]
- GIVEN an operator reads the guidance guide
- WHEN the guide shows one composed instruction result
- THEN it MUST identify each policy constraint, project fact, and advisory entry
- AND it MUST state which Cairn commands ignore advisory prose for authority decisions.
