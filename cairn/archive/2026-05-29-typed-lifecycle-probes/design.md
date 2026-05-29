# Design: Typed Lifecycle Probes

## Context

Cairn already separates policy authoring from runtime evaluation:

- Nickel owns policy contracts, vocabulary, and generated policy JSON.
- Rust owns pure deterministic validation over loaded data.
- Receipts bind inputs, policy hashes, results, and receipt hashes.

Lifecycle artifacts have not caught up with that model. Requirements, scenarios, tasks, and verification links are still parsed from Markdown. That keeps human readability high, but it makes lifecycle validation depend on prose shape and leaves probe evidence as an external convention rather than typed data.

## Architecture

### Source, runtime, and view layers

Use three explicit layers:

```text
Nickel lifecycle source
  -> generated canonical lifecycle bundle
  -> Rust pure validation and receipts
  -> Markdown/doc projections when requested
```

Nickel lifecycle source is the authoring source of truth for new structured artifacts. Runtime commands do not evaluate Nickel. Export/check commands produce a canonical lifecycle bundle under source control or a declared artifact path, and `cairn-core` evaluates that bundle plus repository snapshots.

Markdown remains useful as a view. `cairn render markdown` can render structured lifecycle content for review, release notes, or GitHub browsing. `cairn migrate markdown` can import existing lifecycle content into structured source with explicit warnings for lossy or ambiguous constructs.

### Typed lifecycle model

Add pure core structs for the lifecycle IR:

- `LifecycleBundle`
- `SpecDocument`
- `ChangeDocument`
- `Requirement`
- `Scenario`
- `Task`
- `Probe`
- `EvidenceDeclaration`
- `FormatMetadata`

The CLI owns filesystem discovery, Nickel export/check invocation, bundle reads, Markdown import/render writes, and JSON printing. Core owns parsing already-loaded bundle data, semantic validation, hash material construction, and deterministic issue generation.

### Nickel contracts

Nickel contracts should validate:

- stable IDs and duplicate-free IDs within a bundle;
- normative requirement strength;
- scenario `given`, `when`, and `then` arrays;
- task coverage markers and dependency markers;
- probe command shape, fixture refs, assertion operators, and evidence roles;
- migration metadata describing original Markdown paths and lossy conversions;
- bounded format version and feature flags.

Contracts catch authoring errors before Rust accepts generated data. Rust still revalidates the generated bundle because checked-in JSON or Preserves data can be edited by hand.

### Probe model

A probe is a typed requirement-bound evidence plan. Initial probe assertions should be data predicates, not embedded code:

- JSON path equals expected scalar;
- JSON path contains expected scalar or issue code;
- receipt verdict equals expected verdict;
- file content hash unchanged or changed as declared;
- command exits with expected status class;
- artifact exists with expected BLAKE3 hash.

Each behavioral requirement should be able to cite at least one probe or stronger proof artifact. A probe must declare whether it is positive, negative, malformed-input, boundary, or regression coverage. Behavioral gates can then distinguish docs-only verification from executable evidence.

### Preserves boundary

Preserves is a good future candidate for canonical lifecycle and receipt material because it has a stable data model and can reduce JSON canonicalization drift. It should not be the first authoring format. The first implementation should keep generated JSON because existing receipts, tests, and policy data already use it.

A later change can add Preserves as an alternate or replacement canonical bundle encoding if it proves:

- byte-stable hash material;
- equivalent positive and negative fixture behavior;
- readable diagnostic mapping back to source IDs and paths;
- compatibility with existing receipt verification flows.

### Steel boundary

Steel with higher-order contracts should not become the base lifecycle source. It is executable code and would make lifecycle authoring harder to audit, sandbox, and replay.

Steel can be added later as an optional probe assertion DSL only when data predicates are insufficient. That profile must be disabled by default, capability-bounded, deterministic-budgeted, and Rust-authorized. Steel must never receive ambient filesystem, process, network, mutation, or receipt-hashing authority.

### Migration path

Implementation should proceed in compatibility stages:

1. Add typed IR and Nickel contracts while current Markdown remains valid.
2. Add export/check for structured lifecycle bundles and fixture validation.
3. Add Markdown import/render commands with lossy-conversion diagnostics.
4. Add probe validation and receipt assertions for at least one Cairn behavior.
5. Add gates that warn when behavioral requirements have docs-only evidence.
6. Later, require structured source for new changes once migration evidence is strong.

### Receipts

New commands should emit deterministic receipts with:

- command identity;
- lifecycle source format;
- generated bundle hash;
- policy hash when policy participates;
- migrated/rendered file manifests when files are written;
- probe result summaries;
- deterministic issue codes;
- receipt hash.

## Non-goals

- Do not remove Markdown support in the first slice.
- Do not execute Nickel in normal runtime validation commands.
- Do not make Steel a trusted lifecycle implementation.
- Do not require Preserves before JSON bundle equivalence is proven.
- Do not claim probes prove full correctness; they prove bounded facts tied to declared inputs and oracles.

## Verification

Focused checks:

```bash
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- gate proposal typed-lifecycle-probes --root .
nix run path:$PWD#cairn -- gate design typed-lifecycle-probes --root .
nix run path:$PWD#cairn -- gate tasks typed-lifecycle-probes --root .
```

Implementation checks should add positive and negative fixtures for typed lifecycle export, stale generated bundles, Markdown import/render round trips, malformed probe assertions, and passing/failing probe receipts.
