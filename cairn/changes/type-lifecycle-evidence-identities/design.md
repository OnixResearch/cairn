# Design: Type lifecycle evidence identities

## Context

Cairn uses many hash roles in one lifecycle decision. A readiness input can include source, binary, sidecar, Valence receipt, Mantle binding, policy, graph, input, and final receipt hashes. Archive and collaboration cores add plan, state, content-root, chain, ledger, and operation identities.

These roles often share BLAKE3 text and `String`. Runtime validators reject many mismatches, but pure API signatures remain exchangeable.

## Decisions

### Decision: Start with a reviewed identity inventory

**Choice:** Inventory raw values in selected core structs, function parameters, results, enum payloads, and map keys. Classify each as domain ID, domain ref, BLAKE3 role, imported interoperability digest, safe path, wire compatibility, display text, or deferred.

**Rationale:** A broad mechanical wrapper change includes many values that do not carry identity semantics.

### Decision: Add two shared nominal primitives

**Choice:** Add private `DomainId<Role>` and `Blake3Digest<Role>` families with explicit role aliases. Constructors check bounds, grammar, canonical spelling, and required prefixes.

Imported non-BLAKE3 algorithms keep dedicated interoperability types. Refined values do not expose public fields, `Deref`, or unchecked primitive conversions.

**Rationale:** IDs and digests have different invariants. Marker roles prevent category exchange while sharing implementation.

### Decision: Nickel owns the domain catalog

**Choice:** Author the supported domain inventory and migration scopes in typed Nickel. Export deterministic runtime and Octet checker input.

**Rationale:** Human review must decide which roles are stable and which wire exceptions remain valid.

### Decision: Separate wire DTOs from admitted cores

**Choice:** Keep stable receipt, policy, fixture, and CLI JSON models in their current serialized shapes. Convert them through checked admission into typed decision models.

Known-role functions accept exact aliases. Heterogeneous receipt collections use closed enums or explicit tagged records.

**Rationale:** Cairn has public receipt compatibility obligations. Internal nominal types must not alter them.

### Decision: Migrate in risk-ordered cohorts

**Choice:** Use these cohorts:

1. stack provenance, evidence-chain, formal-proof, and function-address readiness.
2. agent receipts, replay, release readiness, registries, and evidence publication chains.
3. sync, archive, compaction, mutation manifests, collaboration, and lifecycle plans.

A cohort closes only after canonical bytes, receipt hashes, and negative fixtures pass.

**Rationale:** Readiness and receipt linkage have the highest cross-role density. Cohorts keep failures local.

### Decision: Preserve functional core and shell boundaries

**Choice:** Parsing, typed conversion, canonical material, hash calculation, validation, planning, and replay remain pure over in-memory values.

Repository walking, file reads, capability roots, process execution, clocks, stdout, stderr, and JSON presentation remain in `cairn-cli`.

**Rationale:** The migration must strengthen types without weakening the existing FCIS contract.

### Decision: Compile-test role separation

**Choice:** Add compile-fail fixtures for policy/input, plan/receipt, receipt/chain, source/binary, Valence/Mantle, state/content-root, archive/pack, change/workspace, and operation/actor substitutions.

Add compile-pass fixtures for same-role owned and borrowed flows.

**Rationale:** These fixtures prove the compiler guarantee directly.

### Decision: Preserve stable evidence identity

**Choice:** Capture current JSON, canonical material, receipt hashes, replay receipts, archive roots, and collaboration state hashes for each cohort. Require stable values unless a separate schema change approves a difference.

**Rationale:** Internal Rust safety must not rewrite lifecycle history.

## Test design

Positive tests cover constructors, exact-role APIs, wire admission, same-role compilation, readiness, replay, archive, collaboration, and unchanged stable receipts.

Negative tests cover malformed input, unsupported prefixes, wrong roles, stale hashes, duplicate tags, tampering, cross-domain compilation, weakened non-claims, and effect leakage into core.

## Risks and trade-offs

- The migration is large. Cohorts and explicit inventory prevent unbounded churn.
- Public Rust APIs can change. Compatibility adapters and exact-revision guidance reduce disruption.
- Generic aliases can produce verbose errors. Domain aliases keep diagnostics clear.
- Typed evidence can appear stronger than it is. Existing non-claims remain mandatory.

## Claim boundary

The change proves configured local role separation, checked identity syntax, and preserved compatibility evidence. It does not prove policy correctness, evidence truth, source correctness, verifier soundness, or release eligibility.
