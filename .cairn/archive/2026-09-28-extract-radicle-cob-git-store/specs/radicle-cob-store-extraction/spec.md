# Product-neutral COB Git store extraction

## ADDED Requirements

### Requirement: Pinned and licensed in-repository source

r[cairn.radicle_cob_store_extraction.pinned_vendor] Cairn MUST consume product-neutral signed Radicle COB Git storage from a reviewed, commit-pinned in-repository Cargo vendor snapshot with an exact source archive BLAKE3 and preserved source licenses, without a sibling-path product dependency or invented remote.

#### Scenario: Repeated trusted import
- GIVEN the exact source Git object and an identical vendor snapshot
- WHEN an operator runs the repository-owned import script from any current directory
- THEN it MUST validate the immutable archive and leave the existing package unchanged

#### Scenario: Source or vendor drift
- GIVEN the source object has a mismatching archive digest or the vendor snapshot contains user changes
- WHEN the import runs
- THEN it MUST reject the import without replacing the vendor package

### Requirement: Direct generic storage cutover

r[cairn.radicle_cob_store_extraction.cutover] Cairn MUST use `GitCobStore` and `GitCobStoreError` directly in COB domain evaluators, CLI, and interoperability tests, remove its duplicate local storage implementation, and preserve existing signed change/ref format, explicit signer errors, domain authority, reducers, mutation locks, and receipt identities.

#### Scenario: Signed histories remain valid
- GIVEN a signed local COB history and explicit keystore
- WHEN Cairn creates, reads, updates, inspects, or rolls back a namespace ref
- THEN the existing object and ref identities and domain postconditions MUST remain valid

#### Scenario: Malformed signed ref
- GIVEN a symbolic or malformed ref, invalid signature, or mismatched signed namespace
- WHEN a consumer enumerates COBs
- THEN the generic store MUST fail closed before the domain accepts state

### Requirement: Bounded ownership and verification

r[cairn.radicle_cob_store_extraction.boundary] Cairn MUST retain policy, lifecycle, authorization, and local-only claims, document the consumer and maintenance owner, and run focused adapter, interoperability, lifecycle, traceability, Cargo, and relevant Nix checks before archive.

#### Scenario: Local storage is mistaken for transport
- GIVEN a successful signed local COB change
- WHEN a receipt or documentation describes the outcome
- THEN it MUST NOT claim node runtime, remote transport, distributed convergence, key trust, or release eligibility
