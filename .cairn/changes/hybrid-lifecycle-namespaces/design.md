# Design: Hybrid lifecycle namespaces

## Context

Lifecycle stores already let several repositories select one external Cairn root. Direct references remain read-only, and explicit receipts satisfy cross-repository dependencies.

The global registry already stores provenance-bearing tag assignments. Its `namespace`, `key`, and `value` fields are plain strings.

The registry is a derived discovery catalog. It cannot satisfy dependencies, grant authority, or change lifecycle readiness.

The requested model needs one central planning store with typed classification. Each code repository must retain its implementation and evidence authority.

## Success Contract

The change succeeds when these results are observable:

- A central lifecycle store exports one deterministic typed namespace projection from Nickel source.
- Invalid namespace definitions or assignments fail before query, mutation planning, or registry projection.
- Namespace values retain their variant, cardinality, subject kind, source, and catalog identity.
- Central changes bind explicit repository targets and matching repository-owned completion receipts.
- Missing, stale, ambiguous, or mismatched target evidence blocks central completion without mutation.
- Namespace queries emit deterministic snapshot-bound receipts.
- Legacy tag records remain readable through an explicit compatibility adapter.
- Namespace assignments cannot represent lifecycle state, graph relations, authorization, evidence satisfaction, or release readiness.

False completion includes a string-only rename, ambient sibling scanning, registry-only authority, or completion from namespace membership.

## Goals

- Give central planning stores a typed and versioned classification vocabulary.
- Preserve repository-owned validation, implementation evidence, and release authority.
- Prevent accidental interchange of IDs and hashes in the Rust domain model.
- Prevent unchecked values from reaching query, projection, or completion decisions.
- Keep registry integration explicit, derived, and snapshot-bound.

## Non-Goals

- Coordinate repository transport or VCS mutation.
- Commit several repositories through one transaction.
- Replace lifecycle state machines or dependency graphs with classifications.
- Generate static Rust types from dynamic operator definitions.
- Make central planning evidence prove local repository correctness.

## Decisions

### Decision: Use namespace as the domain noun

**Choice:** The new model uses `NamespaceDefinition`, `NamespaceValue`, and `NamespaceAssignment`. A single `NamespaceId` replaces the current namespace-and-key string pair.

A namespace ID can contain bounded hierarchical segments, such as `portfolio.project` or `delivery.platform`. Segment syntax and bounds come from Nickel policy.

**Rationale:** A namespace is a named classification domain. A tag suggests an untyped free-form label and duplicates the current key model.

Existing `tag` fields and CLI selectors remain import-only compatibility surfaces. New APIs, schemas, documentation, and receipts use namespace terminology.

### Decision: Use a hybrid authority model

**Choice:** The central store owns portfolio plans, namespace definitions, namespace assignments, repository targets, and coordination receipts.

Each target repository owns its implementation, validation, current revision, and release evidence. The central store consumes only explicit bounded receipts.

**Rationale:** Central-only authority can hide repository drift. Pure federation makes portfolio queries and shared planning difficult.

The hybrid model gives one planning view without transferring repository authority.

### Decision: Keep namespace identity separate from store identity

**Choice:** The central lifecycle root contains these files:

```text
.cairn/namespaces.ncl
.cairn/namespaces.json
```

The Nickel file is the authored source. The JSON file is the checked runtime projection.

`cairn config export --kind namespaces --root PATH [--check] [--execute]` owns projection generation and freshness checks.

The namespace catalog has its own `NamespaceCatalogHash`. It does not change the stable lifecycle-store identity or local locator mapping.

Every assignment, query, target plan, and registry projection binds the exact catalog hash.

**Rationale:** A taxonomy update must invalidate dependent receipts. It must not make the registered store appear to be a different store.

### Decision: Use nominal newtypes for protocol identities

**Choice:** The pure core uses private-field newtypes for these fixed protocol concepts:

- `NamespaceId`
- `NamespaceAssignmentId`
- `LifecycleSubjectId`
- `RepositoryId`
- `RepositoryTargetId`
- `NamespaceCatalogHash`
- `RepositoryReceiptHash`
- `NamespaceSnapshotHash`

Each newtype owns parsing, bounds, canonical display, and construction rules. Raw strings do not enter admitted domain records.

**Rationale:** These values have different meanings even when their wire forms are strings. Newtypes prevent accidental substitution.

Operator-defined namespace value types remain data-driven. Cairn does not generate Rust source from a namespace catalog.

### Decision: Use typestate for admission order only

**Choice:** The core uses these meaningful transitions:

```text
NamespaceCatalog<Unchecked> -> NamespaceCatalog<Admitted>
NamespaceAssignment<Unchecked> -> NamespaceAssignment<Admitted>
RepositoryEvidence<Observed> -> RepositoryEvidence<Bound>
```

Only admitted catalogs and assignments can enter query or projection functions. Only bound evidence can enter central completion decisions.

Constructors for admitted and bound states remain private. Validation returns the next state or typed diagnostics.

Persisted lifecycle status remains an explicit enum. It does not become a generic typestate parameter.

**Rationale:** Typestate removes invalid call order from internal APIs. It does not replace validation after deserialization.

Wider typestate use can create generic combinations without safety value. The change limits typestate to irreversible admission boundaries.

### Decision: Use a closed typed value sum

**Choice:** `NamespaceValue` is a discriminated union with policy-bounded variants:

- closed symbol
- nominal identifier
- bounded text
- boolean
- bounded integer
- date
- subject reference

A `NamespaceDefinition` declares one value kind, allowed subject kinds, cardinality, authority class, and constraints.

Cardinality is a closed enum. It distinguishes optional single, required single, optional multiple, and required multiple assignments.

Subject kinds are a closed enum. The initial set covers stores, changes, specs, requirements, evidence records, and repository targets.

**Rationale:** A typed sum preserves value meaning on the wire. Definition checks reject wrong variants before a value reaches a query.

### Decision: Keep classifications separate from state and relations

**Choice:** Namespace assignments classify subjects. They cannot encode these facts:

- lifecycle status
- dependency or supersession relations
- authorization or trust
- evidence satisfaction
- verification role
- release readiness

Policy reserves these semantic domains. Admission rejects definitions or assignments that attempt claim promotion.

Relations remain typed graph edges. Lifecycle status remains owned state. Evidence remains explicit receipt input.

**Rationale:** A general namespace system can become an untyped policy database. This boundary prevents namespace membership from becoming authority.

### Decision: Model repository targets as explicit records

**Choice:** A central change can declare repository targets separately from namespace assignments.

Each target binds a `RepositoryId`, target change identity, expected revision profile, required receipt kind, policy hash, subject hash, and non-claims.

Central validation, sync, archive, and release readiness accept explicit repository evidence files or bounded directories. They never inspect sibling checkouts or fetch remote state.

A completion decision requires one current matching receipt for each required target. Extra, duplicate, stale, or ambiguous receipts fail closed.

**Rationale:** A repository target is a coordination relationship, not a classification. Explicit receipts preserve local authority and currentness.

Central completion proves only that the presented target receipts match the declared coordination contract.

### Decision: Add read-only namespace queries first

**Choice:** Add these operator surfaces:

```text
cairn namespace list [--root PATH | --lifecycle-store ID]
cairn namespace show ID [--root PATH | --lifecycle-store ID]
cairn namespace query ID VALUE [--subject-kind KIND] [--root PATH | --lifecycle-store ID]
cairn namespace validate [--root PATH | --lifecycle-store ID]
```

The default MCP catalog adds equivalent read-only tools. Mutation remains authored through Nickel and the preview-first configuration export.

Each query binds the selected root, catalog hash, namespace ID, typed value selector, subject kind, result IDs, and result hash.

**Rationale:** Read-only queries prove the model before Cairn adds a Nickel source editor. This slice avoids an unsafe text-rewrite shell.

### Decision: Project namespaces into the global registry explicitly

**Choice:** Add a policy-allowlisted ingest adapter for admitted namespace projections.

The adapter preserves namespace IDs, typed values, subjects, source identity, catalog hash, authority class, and non-claims. It cannot reinterpret assignments as readiness or trust.

New registry snapshots use namespace-assignment records and namespace selectors. Version-one tag records remain accepted only through a compatibility adapter.

The compatibility adapter converts one legacy namespace-and-key pair into one canonical compatibility `NamespaceId`. It preserves the original record identity and source facts.

**Rationale:** The registry provides useful cross-repository discovery. It must remain a derived projection rather than the taxonomy owner.

### Decision: Preserve the functional-core boundary

**Choice:** `cairn-core` owns these pure decisions:

- newtype parsing and admission
- catalog and assignment validation
- typestate transitions
- cardinality and applicability checks
- target-evidence matching
- namespace query evaluation
- compatibility conversion
- canonical BLAKE3 identity material
- effect-plan and receipt-value construction

`cairn-cli` owns files, paths, bounded reads, Nickel export, locks, atomic writes, CLI and MCP transport, JSON output, and process exits.

Storage and global-registry adapters receive admitted domain values. They cannot define namespace meaning.

**Rationale:** Domain meaning remains deterministic and directly testable. External effects remain visible at the shell boundary.

### Decision: Use preview-first repository evidence binding

**Choice:** Namespace projection writes and central lifecycle mutations remain preview-first.

The shell reopens namespace, target, policy, and repository-evidence inputs under the applicable lock before its first write. Any identity change blocks all effects.

Receipts contain full local hashes and portable hashes. Portable hashes exclude absolute host paths and retain semantic identities.

**Rationale:** Central coordination creates more stale-input risks. Rechecks prevent a reviewed plan from using changed taxonomy or evidence.

## Approach Registry

| Family | Mechanism | Result | Reason |
|---|---|---|---|
| central-only | One store owns planning and repository completion truth | Rejected | It transfers repository authority and hides local drift. |
| registry-only | Use global-registry tags as the taxonomy | Rejected | Registry tags are strings, and registry state is derived. |
| federated-only | Keep all namespaces inside each code repository | Rejected | Shared queries and portfolio planning need repeated reconciliation. |
| hybrid | Central typed namespaces plus repository-owned evidence | Selected | It centralizes classification without centralizing implementation authority. |

These passes were serial and correlated. The adversarial audit focused on authority promotion, stale evidence, type erasure, and hidden transport.

## Compatibility and Migration

The first compatible release reads current `cairn.global-registry-snapshot.v1` tag records. It does not create new tag records from namespace-aware sources.

A namespace-aware registry schema stores typed values without flattening them to strings. Export and verification preserve the source schema and conversion receipt.

The CLI keeps legacy tag selection only when the selected snapshot uses the legacy schema. New snapshots require namespace selectors.

Documentation marks `tag` as a compatibility term. It does not rename Git tags, Radicle namespaces, or unrelated release labels.

## Test Design

Positive tests cover:

- deterministic Nickel export and freshness checks
- each supported namespace value variant
- valid cardinality and subject applicability
- newtype round trips and canonical display
- each admitted typestate transition
- deterministic queries under reordered source input
- complete multi-repository target evidence
- portable receipt identity across different local paths
- registry projection and legacy compatibility import

Negative tests cover:

- malformed namespace IDs and duplicate definitions
- wrong value variants and invalid constraints
- wrong subject kinds and cardinality conflicts
- direct construction attempts for admitted types
- unchecked values presented to query or completion APIs
- reserved semantic-domain and claim-promotion attempts
- missing, duplicate, stale, ambiguous, or mismatched target evidence
- changed catalog, policy, target, or evidence after preview
- implicit sibling scanning or global-registry lookup
- legacy conversion collisions and type erasure
- mutation after any failed recheck

Property tests compare canonical identities under input reordering. Compile-fail tests cover forbidden typestate transitions where practical.

## Risks and Trade-offs

- Namespace catalogs can grow into broad policy stores. Closed value kinds and reserved domains limit this risk.
- Dynamic catalogs cannot provide one Rust type per operator definition. Fixed protocol newtypes still protect core identities.
- Typestate can increase generic complexity. This design limits it to three admission pipelines.
- Central evidence can become stale between planning and execution. The shell reopens and rebinds all inputs before mutation.
- Legacy tag conversion can lose meaning if flattened. The compatibility adapter preserves source records and rejects collisions.
- Central planning can look like central release authority. Receipts and documentation keep that claim out of scope.

## Claim Boundary

A valid namespace catalog proves only that definitions satisfy the selected contract and produce one deterministic identity.

A valid assignment proves only that one typed value applies to one declared subject under that catalog.

A complete target set proves only that presented repository receipts match declared identities and currentness constraints.

A namespace query proves only deterministic selection from one observed namespace snapshot.

None of these results proves implementation correctness, remote freshness, social trust, evidence truth, lifecycle readiness, or release eligibility.
