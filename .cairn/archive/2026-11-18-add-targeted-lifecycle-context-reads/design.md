# Targeted lifecycle context design

## Context

Requirement markers and optional task-order markers already supply stable identities.
Existing MCP specification reads return a whole specification and reject content above the current limit.
Declared guidance already separates policy constraints, project facts, and advisory text.

## Decisions

### Decision: Use a strict selector union

Add `cairn context read` and an equivalent narrow MCP read tool.
The request selects an explicit root, artifact scope, and exactly one requirement ID, task ID, or section path.
Requirement reads select a specification and a defining requirement ID.
Task reads select a change's tasks artifact and an existing declared task ID.
Section reads select a declared artifact and a complete heading path. Duplicate heading paths are ambiguous.
Keep whole-artifact commands unchanged. A task without an ID uses the existing whole-artifact path, not a synthetic line identity.

### Decision: Reuse structural parsers and exact bytes

Reuse the existing requirement parser, task-order parser, and Markdown fence handling.
A definition differs from a reference or an example inside a code fence.
The core resolves selectors over loaded text and returns UTF-8 byte spans with structural boundaries.
The shell returns exact admitted bytes and source BLAKE3 identity. It does not rewrite prose or normalize whitespace.
A requirement block includes its scenarios. A section includes descendants until the next heading at the same or higher level.
Task records retain their original task text and declared links.

### Decision: Expose validity and prerequisites separately

Selection validity does not imply whole-artifact validity.
Return bounded document diagnostics, source authority labels, and typed prerequisite references alongside selected content.
Do not expand linked specifications, dependencies, or guidance transitively.
A reference beyond a named limit produces an explicit omitted count and exact follow-up route, not a completeness claim.
Required policy constraints remain visible or remain explicit required references before the consumer can treat the result as sufficient context.

### Decision: Keep reads bounded and explicit

Nickel owns limits for artifact bytes, selected bytes, selector length, heading depth, diagnostics, and references.
Use existing capability-relative no-follow readers for project and lifecycle roots.
Unknown fields, invalid selector combinations, unsafe paths, invalid UTF-8, oversized blocks, and stale expected identities fail before disclosure.
No cache, registry service, generated ID store, or new cross-repository dependency is necessary.
The incremental-observation package later composes these results into source-bound pages and differences.

## Alternatives rejected

- Raw line-number IDs change after unrelated edits and cannot serve as stable task identities.
- Semantic search introduces relevance guesses where an exact ID already exists.
- Selecting the first duplicate hides malformed source and ambiguity.
- Automatic dependency expansion recreates large context payloads.

## Tests and evidence

Baseline current full specification, context, and workflow instruction tests.
Add exact-span fixtures for nested headings, Unicode, CRLF, fenced markers, adjacent requirements, and task links.
Add duplicate-ID, unknown-ID, missing task identity, duplicate-heading, malformed-document, unsafe-path, and oversized-block controls.
Compare CLI/MCP selected bytes and identities with the original artifact under identical policy.
Run the common benchmark for task implementation and blocker repair, including follow-up reference reads.
A smaller selected block does not establish improvement when an oracle rejects missing context.

## Risks and non-claims

A narrow read can omit relevant prose outside declared links. Document this scope and preserve whole-artifact fallback.
Exact spans prove source selection, not truth, semantic sufficiency, or requirement satisfaction.
Lifecycle-core maintainers own selector semantics. Shell maintainers own bounded reads and transport parity.
