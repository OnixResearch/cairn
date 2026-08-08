# Nominal Evidence Identities Specification

## Purpose

Defines the `nominal-evidence-identities` capability.

## Requirements

### Requirement: Cairn identity domains have reviewed policy

r[cairn.nominal_evidence_identities.policy] Cairn MUST define a typed Nickel catalog for enforced lifecycle IDs, references, BLAKE3 roles, imported digest roles, core scopes, and wire compatibility scopes.

#### Scenario: Domain catalog is deterministic

r[cairn.nominal_evidence_identities.policy.deterministic]
- GIVEN a valid domain catalog
- WHEN Cairn exports its runtime and Octet projections
- THEN both projections MUST be deterministic and bound to the reviewed policy identity.

### Requirement: Lifecycle and evidence identities are nominal

r[cairn.nominal_evidence_identities.types] Cairn MUST represent statically known lifecycle IDs and BLAKE3 roles with private Rust types after boundary admission.

#### Scenario: Same-role identity is accepted

r[cairn.nominal_evidence_identities.types.same_role]
- GIVEN a valid typed policy digest
- WHEN a pure core API requires a policy digest
- THEN the API MUST accept it without another expected-role string.

#### Scenario: Cross-role identity does not compile

r[cairn.nominal_evidence_identities.compile_time]
- GIVEN policy and receipt digests use distinct marker roles
- WHEN source passes a receipt digest to an API that requires a policy digest
- THEN the source MUST fail compilation with a type mismatch.

#### Scenario: Malformed identity fails before core use

r[cairn.nominal_evidence_identities.validation]
- GIVEN untrusted identity text is empty, oversized, malformed, has an unsupported prefix, or uses the wrong algorithm
- WHEN admission constructs the typed value
- THEN construction MUST fail with deterministic diagnostics
- AND the invalid value MUST NOT enter the pure core.

### Requirement: Migration follows a semantic inventory

r[cairn.nominal_evidence_identities.inventory] Cairn MUST classify selected raw values as domain ID, domain ref, BLAKE3 role, imported digest, safe path, wire compatibility, display text, or deferred before migration.

#### Scenario: Inventory excludes display and shell values

r[cairn.nominal_evidence_identities.inventory.exclusions]
- GIVEN selected models contain paths, command names, schema labels, diagnostics, and rendered text
- WHEN the inventory runs
- THEN display and shell-owned values MUST remain outside nominal identity enforcement unless a separate domain is approved.

### Requirement: Stable wire models admit typed core values

r[cairn.nominal_evidence_identities.wire_boundary] Cairn MUST preserve stable policy, receipt, fixture, and CLI JSON shapes in explicit wire models and MUST convert them into typed decision values before pure evaluation.

#### Scenario: Wire record admits typed core values

r[cairn.nominal_evidence_identities.wire_boundary.valid]
- GIVEN a supported wire record contains valid role-tagged IDs and digests
- WHEN boundary admission runs
- THEN Cairn MUST construct exact typed core values without changing the wire representation.

#### Scenario: Wrong wire role fails closed

r[cairn.nominal_evidence_identities.wire_boundary.invalid]
- GIVEN a valid digest appears under the wrong policy, receipt, chain, artifact, graph, state, or operation role
- WHEN boundary admission runs
- THEN Cairn MUST fail with a deterministic role diagnostic.

### Requirement: Readiness identities remain distinct

r[cairn.nominal_evidence_identities.readiness] Stack provenance, evidence-chain, formal-proof, and function-address readiness MUST retain exact source, binary, sidecar, Valence, Mantle, policy, graph, chain, and receipt identity roles.

#### Scenario: Readiness roles remain distinct

r[cairn.nominal_evidence_identities.readiness.validation]
- GIVEN two readiness fields contain equal BLAKE3 bytes under different roles
- WHEN readiness evaluation runs
- THEN Cairn MUST preserve both roles and MUST NOT let either field satisfy the other.

### Requirement: Receipt-chain identities remain distinct

r[cairn.nominal_evidence_identities.receipt_chains] Agent, replay, release, registry, manifest, publication, import, republish, index, and selection cores MUST retain exact policy, input, parent, receipt, chain, artifact, registry, and result identity roles.

#### Scenario: Wrong parent receipt cannot satisfy a chain

r[cairn.nominal_evidence_identities.receipt_chains.validation]
- GIVEN a syntactically valid receipt digest belongs to another parent role or evidence chain
- WHEN chain validation runs
- THEN Cairn MUST reject the chain with a deterministic parent-role diagnostic.

### Requirement: Lifecycle mutation identities remain distinct

r[cairn.nominal_evidence_identities.lifecycle] Sync, archive, compaction, mutation, collaboration, and lifecycle cores MUST retain exact change, workspace, actor, operation, state, content-root, archive, pack, plan, receipt, chain, and ledger identity roles.

#### Scenario: Stale state or wrong operation cannot authorize mutation

r[cairn.nominal_evidence_identities.lifecycle.validation]
- GIVEN a plan carries a valid state or operation identity from another change, workspace, archive, or collaboration object
- WHEN mutation planning or execution recheck runs
- THEN Cairn MUST reject the mismatch before filesystem mutation.

### Requirement: Stable evidence identity is preserved

r[cairn.nominal_evidence_identities.compatibility] The nominal-type migration MUST preserve stable documented CLI JSON fields, canonical material, receipt hashes, replay receipts, archive roots, collaboration state hashes, and accepted fixture bytes.

#### Scenario: Existing evidence remains readable

r[cairn.nominal_evidence_identities.compatibility.read]
- GIVEN a supported receipt, evidence bundle, archive pointer, or collaboration envelope from before migration
- WHEN the new wire adapter loads it
- THEN Cairn MUST reconstruct equivalent typed state without changing the compatibility contract.

### Requirement: Octet guards migrated Cairn scopes

r[cairn.nominal_evidence_identities.octet] After the policy becomes available, Cairn MUST enable reviewed Octet nominal-domain checks for migrated verified-core scopes.

#### Scenario: Raw identity regression is rejected

r[cairn.nominal_evidence_identities.octet.guard]
- GIVEN a migrated core field or signature reintroduces a declared identity as `String`
- WHEN Octet checks the scope
- THEN the selected nominal-domain check MUST fail.

### Requirement: Identity migration preserves functional core boundaries

r[cairn.nominal_evidence_identities.docs] Identity parsing, conversion, validation, planning, hashing, and replay MUST remain pure, while repository, filesystem, process, clock, network, and presentation effects remain in shells.

#### Scenario: Core remains effect-free

r[cairn.nominal_evidence_identities.docs.pure]
- GIVEN a migrated identity cohort is complete
- WHEN core purity checks run
- THEN no new ambient filesystem, environment, process, clock, network, or output effect MUST appear in `cairn-core`.

### Requirement: Typed evidence claims remain bounded

r[cairn.nominal_evidence_identities.final_checks] Cairn documentation and receipts MUST state that nominal identities prove configured local role separation, checked syntax, and compatibility evidence only.

#### Scenario: Boundary remains visible

r[cairn.nominal_evidence_identities.final_checks.boundary]
- GIVEN typed lifecycle and evidence identities pass all selected checks
- WHEN Cairn states the supported claim
- THEN it MUST NOT claim policy correctness, evidence truth, source correctness, verifier soundness, mutation safety outside checked plans, or release eligibility.
