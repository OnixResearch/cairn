# Add targeted lifecycle context reads

## Why

Cairn already has stable requirement markers, optional task identities, bounded specification reads, and declared guidance selectors.
At `cb0be66e0e992c0f4070956bb9d451d8a8fe7573`, `cairn_read_spec` returns a whole accepted specification.
An agent that needs one requirement or task can therefore receive much more content than the selected action requires.

## What Changes

- Add exact read selectors for requirement IDs, task IDs, and artifact sections.
- Bind every result to an explicit lifecycle scope, artifact identity, selector, and exact source span.
- Return bounded prerequisite references instead of expanding all linked content automatically.
- Preserve policy constraints, project facts, and advisory guidance as separate authority classes.
- Retain whole-artifact reads and reject ambiguous or unsupported narrow selectors.

## Impact

Current consumers: Cairn agents that implement or repair one identified task or requirement.
Immediate outcome: the agent receives the exact relevant source block and the references needed for its next read.
Durable contribution: a reusable deterministic selector model with CLI/MCP parity.
Maintenance owner: Cairn lifecycle-core and bounded-read adapter maintainers.

Reuse requirement parsing, task-order identity admission, artifact workflow metadata, and existing no-follow readers.
Expected changes include core selector contracts, shell adapters, MCP schemas, and source-span fixtures.
A task without a declared identity remains readable through its artifact. Cairn must not invent a persistent ID from its line number.

## Repeatability evidence

Compare selected text with exact bytes from the admitted artifact.
Cover valid requirement and task reads, duplicate IDs, duplicate headings, unknown sections, malformed source, and unsafe paths.
Retain invalid-document diagnostics rather than presenting a valid fragment as a valid document.
Measure complete narrow-read tasks against whole-artifact reads through the shared benchmark.

## Out of Scope

- Embedding search, semantic ranking, automatic transitive traversal, or repository-wide prompt discovery
- Editing tasks, rewriting specifications, or creating IDs during a read
- Treating selected text or declared links as implementation evidence

Dependency: `measure-agent-lifecycle-token-cost`. Implementation remains open.
