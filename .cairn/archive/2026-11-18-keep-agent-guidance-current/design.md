# Current compact guidance design

## Context

Cairn already supports generated workflows, adapter manifests, preview-first installation, and freshness checks.
The source audit found stale user-installed guidance. Cairn must not repair user-global files without separate authority.
The readiness-adoption prerequisite supplies current diagnostic routes before this package changes their presentation.

## Decisions

### Decision: Extend the existing generator

Add a compact entry descriptor to the current generated workflow policy.
It states the goal, current native layout, exact discovery and readiness routes, authority limits, and how to request one artifact's instructions.
Keep command and profile names in a checked catalog projection rather than duplicate prose constants.
Add command-contract identity to the existing source, policy, generator, and adapter bindings.
Do not introduce another renderer or installation system.

### Decision: Keep stage details separate

The entry guide references existing stage-specific workflow descriptors and artifact instruction commands.
It does not concatenate all workflow descriptions, receipt schemas, and examples.
Nickel owns entry and stage text bounds and the mandatory boundary set.
A budget that cannot contain required authority and evidence constraints fails generation.
The host chooses when to load a stage guide. Generated text cannot force host compliance.

### Decision: Preserve safe installation

Use existing `agent workflow list`, `show`, `install`, and `check` surfaces with an additive entry selection.
Installation remains project-local, preview-first, and plan-hash bound.
Keep manifest freshness states and no-overwrite rules for modified, missing-managed, unmanaged, unsupported, and linked paths.
Keep deliberate recovery for interrupted installation.
Do not inspect user-home instructions or ambient AGENTS.md files.
An operator can explicitly submit a copied guide and declared manifest identity for report-only freshness inspection.
Arbitrary prose without an admitted identity remains unclassified rather than semantically certified.

### Decision: Check maintained examples against contracts

Extract maintained example argv through a repository-owned fixture table with named expected outcomes.
Validate those argv and layout/profile names against current command contracts in bounded fixtures.
Do not run arbitrary commands from Markdown or execute destructive examples.
Keep exact examples for supported and legacy-only layouts, unsupported profiles, and stale command identities.
Update the repository-maintained workflow source and public guide links together.

## Architecture and reuse

Pure generation and freshness decisions consume explicit catalog and file observations.
Shell adapters own reads, writes, locks, and manifest publication through existing code.
Reuse existing adapters for Agent Skills, Claude, and Codex. Do not add host-specific authority to the core.
No new shared component or agent-runtime dependency is necessary.

## Alternatives rejected

- Another handwritten global skill repeats the source-of-truth drift.
- Removing safety text to meet a token target weakens the operating contract.
- Automatic user-home updates exceed project-local installation authority.
- Parsing arbitrary prose as a command contract invites false freshness claims.

## Tests and evidence

Baseline current generator, install, freshness, and positive/negative adapter fixtures.
Compare semantic entry routes across all supported adapters.
Test stale layouts, command-catalog drift, unknown profiles, insufficient budgets, malformed manifests, destination races, and interrupted installation.
Prove denied updates preserve existing bytes. Check maintained examples without executing their product effects.
Measure actual host-loaded entry and stage text through the shared benchmark, including duplicate old-guide exposure.

## Risks and non-claims

Generated files can be current while a host still loads stale text elsewhere.
Report only observed selected files and declared host exposure. Do not claim global installation or agent compliance.
Generated-workflow maintainers own source contracts and fixtures. Host owners retain context-loading behavior.
