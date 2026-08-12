# Lifecycle Namespaces Specification Delta

## ADDED Requirements

### Requirement: Namespace catalogs are typed and deterministic

r[cairn.lifecycle_namespaces.catalog] Cairn MUST author lifecycle-namespace catalogs in Nickel. Cairn MUST export one checked JSON runtime projection with a canonical BLAKE3 catalog identity.

#### Scenario: Valid catalog exports deterministically

r[cairn.lifecycle_namespaces.catalog.valid]
- GIVEN a namespace catalog satisfies the selected Nickel contract
- WHEN the operator exports the runtime projection
- THEN equivalent definitions MUST produce the same canonical JSON bytes and `NamespaceCatalogHash`
- AND ordinary lifecycle commands MUST read the checked projection without evaluating Nickel.

#### Scenario: Invalid or stale catalog fails closed

r[cairn.lifecycle_namespaces.catalog.invalid]
- GIVEN a catalog has malformed IDs, duplicate definitions, invalid constraints, missing non-claims, or a stale projection
- WHEN Cairn admits the catalog
- THEN Cairn MUST emit deterministic diagnostics before query, registry projection, completion planning, or mutation
- AND it MUST preserve the last valid projection and lifecycle state.

### Requirement: Namespace wire values preserve type and meaning

r[cairn.lifecycle_namespaces.values] Cairn MUST represent namespace values as a closed discriminated union. Definitions MUST declare value kind, subject applicability, cardinality, authority class, and policy-bounded constraints.

#### Scenario: Typed value matches its definition

r[cairn.lifecycle_namespaces.values.valid]
- GIVEN a definition permits the supplied value variant, subject kind, cardinality, and constraints
- WHEN assignment admission runs
- THEN Cairn MUST preserve the exact typed value and definition identity
- AND it MUST NOT flatten the value into an untyped string.

#### Scenario: Wrong value or subject fails

r[cairn.lifecycle_namespaces.values.invalid]
- GIVEN an assignment has the wrong value variant, subject kind, cardinality, bound, or nominal type
- WHEN assignment admission runs
- THEN Cairn MUST reject the assignment with a stable field-specific diagnostic
- AND Cairn MUST NOT admit that assignment into query, projection, or completion input.

### Requirement: Protocol identities use nominal domain types

r[cairn.lifecycle_namespaces.newtypes] Cairn MUST use distinct private-field domain types for namespace, assignment, subject, repository, target, catalog-hash, snapshot-hash, and receipt-hash identities.

#### Scenario: Valid identity enters the domain model

r[cairn.lifecycle_namespaces.newtypes.valid]
- GIVEN a wire identity satisfies its nominal parser and named bounds
- WHEN the pure core admits the value
- THEN Cairn MUST construct only the matching domain type
- AND canonical display and serialization MUST preserve its wire identity.

#### Scenario: Identity substitution fails

r[cairn.lifecycle_namespaces.newtypes.invalid]
- GIVEN a caller presents one valid wire identity where another nominal domain type is required
- WHEN typed API construction or deserialization admission runs
- THEN Cairn MUST reject the substitution before domain evaluation
- AND raw strings MUST NOT bypass the nominal constructors.

### Requirement: Admission order uses bounded typestate

r[cairn.lifecycle_namespaces.typestate] Cairn MUST distinguish unchecked, admitted, observed, and bound values at meaningful admission boundaries. Authoritative core decisions MUST accept only admitted catalogs, admitted assignments, and bound evidence.

#### Scenario: Valid admission reaches a usable state

r[cairn.lifecycle_namespaces.typestate.valid]
- GIVEN unchecked catalog or assignment data satisfies every selected invariant
- WHEN admission completes
- THEN Cairn MUST return the corresponding admitted type through a private transition
- AND query or projection functions MUST accept that admitted type.

#### Scenario: Invalid transition is unavailable

r[cairn.lifecycle_namespaces.typestate.invalid]
- GIVEN data remains unchecked or repository evidence remains only observed
- WHEN a caller attempts query, projection, or central completion
- THEN the typed API MUST prevent the invalid call path where compile-time enforcement applies
- AND runtime admission MUST fail closed at deserialization boundaries.

### Requirement: Namespace assignments classify without granting authority

r[cairn.lifecycle_namespaces.assignments] Cairn MUST model assignments as separately identified records that bind a subject, namespace, typed value, catalog identity, source provenance, authority class, and non-claims.

#### Scenario: Classification assignment is valid

r[cairn.lifecycle_namespaces.assignments.valid]
- GIVEN an admitted namespace allows the supplied subject and typed value
- WHEN Cairn admits the assignment
- THEN it MUST create one deterministic assignment identity without changing the subject identity
- AND queries MUST use the assignment only as declared classification metadata.

#### Scenario: Authority promotion is rejected

r[cairn.lifecycle_namespaces.assignments.overclaim]
- GIVEN a definition or assignment attempts to encode lifecycle state, a graph relation, authorization, evidence satisfaction, verification role, or release readiness
- WHEN namespace admission runs
- THEN Cairn MUST reject the reserved semantic use with a stable diagnostic
- AND existing lifecycle, graph, authority, evidence, and release validators MUST retain their authority.

### Requirement: Central changes use explicit repository targets

r[cairn.lifecycle_namespaces.hybrid_targets] Cairn MUST model target repositories as explicit coordination records, not namespace assignments. Each target MUST bind repository, change, revision, receipt, policy, subject, and non-claim constraints.

#### Scenario: Matching repository evidence satisfies one target

r[cairn.lifecycle_namespaces.hybrid_targets.valid]
- GIVEN a central change declares a repository target and receives one current matching repository-owned completion receipt
- WHEN target-evidence validation runs
- THEN Cairn MUST bind the evidence to that target and report the target as satisfied
- AND the result MUST preserve the repository as the owner of implementation and validation facts.

#### Scenario: Missing or mismatched evidence blocks completion

r[cairn.lifecycle_namespaces.hybrid_targets.blocked]
- GIVEN required evidence is missing, stale, duplicated, ambiguous, malformed, unsupported, or mismatched
- WHEN central validation, sync, archive, or release readiness evaluates the target set
- THEN Cairn MUST keep the affected targets blocked with deterministic diagnostics
- AND an execution request MUST perform no lifecycle mutation.

### Requirement: Repository evidence input is explicit and bounded

r[cairn.lifecycle_namespaces.repository_evidence] Cairn MUST load repository evidence only from explicit files or bounded local directories. Cairn MUST NOT scan sibling checkouts, fetch remotes, or infer remote freshness.

#### Scenario: Explicit bundle is loaded safely

r[cairn.lifecycle_namespaces.repository_evidence.valid]
- GIVEN an operator supplies safe repository-evidence paths within configured bounds
- WHEN Cairn loads and validates the evidence
- THEN it MUST reject symlinks and unsafe paths and bind every accepted file identity
- AND the receipt MUST list safe source references without private absolute paths.

#### Scenario: Ambient repository state is ignored

r[cairn.lifecycle_namespaces.repository_evidence.ambient]
- GIVEN matching repositories, registry records, or receipts exist outside the explicit evidence input
- WHEN target-evidence validation runs
- THEN Cairn MUST ignore those ambient facts
- AND registry membership or sibling presence MUST NOT satisfy a repository target.

### Requirement: Namespace queries are deterministic and snapshot-bound

r[cairn.lifecycle_namespaces.query] Cairn MUST provide read-only namespace list, show, validate, and typed query operations for explicit roots or lifecycle-store IDs. Query receipts MUST bind all selectors and result identities.

#### Scenario: Equivalent selectors return one result

r[cairn.lifecycle_namespaces.query.valid]
- GIVEN one admitted namespace snapshot and equivalent typed selectors in different input orders
- WHEN Cairn evaluates the query
- THEN it MUST return the same canonical subject order, result hash, and receipt hash
- AND it MUST report the selected root, catalog hash, namespace ID, value, and subject kind.

#### Scenario: Invalid selector fails closed

r[cairn.lifecycle_namespaces.query.invalid]
- GIVEN a query has an unknown namespace, wrong value variant, unsupported subject kind, stale catalog, or ambiguous selector
- WHEN Cairn evaluates the query
- THEN it MUST emit deterministic diagnostics
- AND it MUST NOT broaden the selector or choose a different lifecycle root.

### Requirement: Global-registry projection remains derived

r[cairn.lifecycle_namespaces.registry_projection] Cairn MUST project admitted namespace records through an explicit policy-allowlisted ingest adapter. Registry storage and queries MUST NOT become namespace or lifecycle authority.

#### Scenario: Namespace projection preserves source facts

r[cairn.lifecycle_namespaces.registry_projection.valid]
- GIVEN an admitted namespace snapshot and a selected compatible registry adapter
- WHEN Cairn projects definitions and assignments
- THEN the registry records MUST preserve typed values, subjects, source identity, catalog hash, authority class, and non-claims
- AND the projection MUST bind the exact source snapshot identity.

#### Scenario: Registry membership grants no authority

r[cairn.lifecycle_namespaces.registry_projection.boundary]
- GIVEN a registry contains a namespace assignment or a matching target-like record
- WHEN a lifecycle gate or completion validator runs without explicit domain evidence
- THEN registry membership MUST NOT satisfy the gate or target
- AND the consuming domain validator MUST remain necessary.

### Requirement: Legacy tag records are import-only compatibility inputs

r[cairn.lifecycle_namespaces.compatibility] Cairn MUST use namespace terminology for new domain APIs and schemas. An explicit compatibility adapter MUST keep legacy global-registry tag records readable.

#### Scenario: Legacy tag imports without type erasure

r[cairn.lifecycle_namespaces.compatibility.valid]
- GIVEN a valid legacy tag record has source provenance, namespace, key, value, and non-claims
- WHEN the compatibility adapter imports it
- THEN Cairn MUST create one canonical compatibility namespace ID and preserve the original source record identity
- AND it MUST identify the converted value as legacy text rather than an inferred stronger type.

#### Scenario: New source does not create legacy tag records

r[cairn.lifecycle_namespaces.compatibility.no_new_tags]
- GIVEN an admitted lifecycle-namespace source enters registry projection
- WHEN Cairn builds new registry records
- THEN it MUST create namespace-assignment records only
- AND it MUST NOT create a legacy tag record or use tag membership as authority.

### Requirement: Namespace decisions preserve the functional-core boundary

r[cairn.lifecycle_namespaces.fcis] Cairn MUST keep identity admission, typestate transitions, assignment validation, target matching, compatibility conversion, query evaluation, and hash material pure.

#### Scenario: Pure core evaluates admitted values

r[cairn.lifecycle_namespaces.fcis.core]
- GIVEN in-memory policy, namespace, assignment, target, and evidence values
- WHEN the core validates or evaluates them
- THEN the result MUST be deterministic without filesystems, environments, clocks, processes, networks, databases, mutation, or output effects.

#### Scenario: Shell owns and rechecks effects

r[cairn.lifecycle_namespaces.fcis.shell]
- GIVEN a reviewed namespace export or lifecycle mutation plan
- WHEN the operator requests execution
- THEN the shell MUST reopen and recheck exact policy, catalog, target, evidence, and destination identities before its first write
- AND any mismatch MUST block every planned effect.

### Requirement: Namespace receipts separate local and portable identity

r[cairn.lifecycle_namespaces.receipts] Cairn MUST emit full receipt hashes that bind local display facts. It MUST also emit portable hashes that exclude absolute paths and bind semantic identities.

#### Scenario: Equivalent roots share portable identity

r[cairn.lifecycle_namespaces.receipts.portable]
- GIVEN equivalent namespace and target state exists under different safe local paths
- WHEN Cairn emits query or completion receipts
- THEN full receipt hashes can differ because local display facts differ
- AND portable hashes MUST match when all semantic identities and results match.

#### Scenario: Changed input invalidates execution

r[cairn.lifecycle_namespaces.receipts.stale]
- GIVEN a catalog, assignment, target, policy, evidence file, or destination changes after preview
- WHEN the operator requests execution
- THEN Cairn MUST reject the stale plan with expected and observed safe identities
- AND it MUST preserve the previous lifecycle and projection state.

### Requirement: Lifecycle namespaces have complete verification and documentation

r[cairn.lifecycle_namespaces.verification] The change MUST include positive and negative policy, core, CLI, MCP, registry, compatibility, receipt, and mutation-preflight tests. It MUST document all authority boundaries.

#### Scenario: Positive suite proves supported behavior

r[cairn.lifecycle_namespaces.verification.positive]
- GIVEN valid catalogs, values, assignments, target receipts, queries, projections, and compatibility inputs
- WHEN focused and repository gates run
- THEN supported cases MUST produce deterministic passing results
- AND compile-time tests MUST cover protected typestate transitions where practical.

#### Scenario: Negative suite fails without partial effects

r[cairn.lifecycle_namespaces.verification.negative]
- GIVEN malformed, stale, conflicting, ambiguous, ambient, overclaiming, or type-invalid inputs
- WHEN focused and repository gates run
- THEN each case MUST fail with the expected stable diagnostic
- AND Cairn MUST NOT commit an invalid snapshot, assignment, target result, registry state, or lifecycle mutation.
