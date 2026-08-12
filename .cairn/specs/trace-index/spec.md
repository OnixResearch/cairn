# Trace Index Specification

## Purpose

Defines the `trace-index` capability.

## Requirements

### Requirement: Canonical content-bound trace index

r[cairn.trace_index.canonical_index] Cairn MUST build a canonical trace index that binds each declared marker link to the selected profile, policy BLAKE3, content-bound requirement-registry BLAKE3, requirement-row BLAKE3, evidence role, safe source path, exact source-file BLAKE3, exact marker BLAKE3, and half-open marker byte span.

#### Scenario: Input order does not change the index

- GIVEN the same validated requirement registry, traceability profile, policy, and evidence files in different input orders
- WHEN the core builds each trace index
- THEN both indexes MUST have identical sorted records and the same index BLAKE3

#### Scenario: Line numbers remain display data

- GIVEN one exact marker occurrence with calculated byte and line bounds
- WHEN the core calculates its marker and link identities
- THEN the marker BLAKE3 MUST bind exact marker bytes
- AND the link BLAKE3 MUST bind the exact source-file content and byte span without using display line numbers as identity

### Requirement: Current-root freshness verification

r[cairn.trace_index.freshness] Cairn MUST verify a saved trace index against the current validated requirement registry, selected policy profile, policy content, and bounded evidence-file snapshot.

#### Scenario: Current content passes

- GIVEN a canonical trace index and unchanged current inputs
- WHEN the operator verifies the index
- THEN Cairn MUST report a deterministic passing verification receipt

#### Scenario: Source content is stale

- GIVEN a canonical trace index and a changed source file
- WHEN the operator verifies the index
- THEN Cairn MUST report the stale source identity
- AND verification MUST fail

#### Scenario: Marker location changes

- GIVEN a canonical trace index and a marker that moved or was copied
- WHEN the operator verifies the index
- THEN Cairn MUST report changed link content or location
- AND verification MUST fail

#### Scenario: Policy content changes

- GIVEN a canonical trace index and changed traceability policy content
- WHEN the operator verifies the index
- THEN Cairn MUST report policy drift
- AND verification MUST fail

### Requirement: Exact requirement query

r[cairn.trace_index.query] Cairn MUST return one exact requirement row and its sorted declared links from a valid saved trace index.

#### Scenario: Known requirement is shown

- GIVEN a valid trace index that contains a requirement ID
- WHEN the operator queries that exact ID
- THEN Cairn MUST return the content-bound requirement row and all sorted declared links

#### Scenario: Unknown requirement is rejected

- GIVEN a valid trace index that does not contain a requirement ID
- WHEN the operator queries that ID
- THEN Cairn MUST return a deterministic unknown-requirement issue

### Requirement: Trace-index CLI

r[cairn.trace_index.cli] Cairn MUST expose deterministic `traceability export`, `traceability verify`, and `traceability show` commands for saved content-bound trace indexes.

#### Scenario: Export writes a new artifact

- GIVEN a current content-bound requirement registry and valid traceability inputs
- WHEN the operator exports a trace index to a new output path
- THEN Cairn MUST write canonical JSON and report its index BLAKE3

#### Scenario: Malformed index fails closed

- GIVEN a saved JSON value outside the closed trace-index schema
- WHEN the operator verifies or queries the index
- THEN Cairn MUST reject the value without using partial index data

### Requirement: Policy-owned assurance and anchor

r[cairn.trace_index.policy] Cairn MUST define each traceability profile with a supported assurance level and anchor kind through typed Nickel policy.

#### Scenario: Declared marker profile is accepted

- GIVEN a profile with `assurance_level = "declared"` and `anchor_kind = "exact_marker_bytes"`
- WHEN Cairn loads the generated policy
- THEN the profile MUST be accepted for marker-level trace indexes

#### Scenario: Unsupported assurance is rejected

- GIVEN a profile with an unsupported assurance level or anchor kind
- WHEN Cairn loads the generated policy
- THEN policy loading MUST fail before a source scan

### Requirement: Validation coverage and claim boundary

r[cairn.trace_index.validation] Cairn MUST test deterministic success and rejection paths for canonical index construction, closed-schema validation, stale inputs, unsafe paths, duplicate source paths, dangling markers, and unknown queries.

#### Scenario: Unsafe source path is rejected

- GIVEN an evidence file with an absolute path or parent traversal
- WHEN the core builds a trace index
- THEN index construction MUST fail with a safe-path issue

#### Scenario: Dangling marker is rejected

- GIVEN a marker that has no row in the selected content-bound requirement registry
- WHEN the core builds a trace index
- THEN index construction MUST fail with a dangling-requirement issue

### Requirement: Trace-index documentation

r[cairn.trace_index.docs] Cairn MUST document the trace-index commands, content identities, exact locator fields, verification workflow, and declared-linkage non-claims.

#### Scenario: Operator reads the claim boundary

- GIVEN the trace-index operator documentation
- WHEN an operator reads the command contract
- THEN the documentation MUST state that the index does not prove requirement satisfaction, implementation correctness, test adequacy, proof soundness, or release eligibility

### Requirement: Declaration marker owns requirement identity

r[cairn.trace_index.live_proof.declaration_marker] Cairn MUST derive a content-bound requirement row identity from exactly one bare marker on the first marker-bearing line of the requirement block and MUST treat later marker-like prose as hashed content without identity authority.

#### Scenario: Marker example in prose is not a second identity

- GIVEN a requirement has one declaration marker and later prose contains another bare marker example
- WHEN Cairn exports the content-bound requirement registry
- THEN Cairn MUST use only the declaration marker as the row identity
- AND the later prose MUST remain inside the requirement-body content hash

#### Scenario: Conflicting declaration markers fail

- GIVEN the first marker-bearing line declares distinct bare markers
- WHEN Cairn exports the content-bound requirement registry
- THEN the export MUST fail with a deterministic duplicate-declaration issue

#### Scenario: Missing declaration marker fails

- GIVEN a requirement block has no bare marker
- WHEN Cairn exports the content-bound requirement registry
- THEN the export MUST fail without selecting an identity from later prose

### Requirement: Repository-scale trace-index check

r[cairn.trace_index.live_proof.repository_check] Cairn MUST maintain a Nix check that runs requirement-registry export, trace-index export, trace-index verification, and exact requirement query against Cairn's accepted specifications and configured evidence roots.

#### Scenario: Current repository command chain passes

- GIVEN the packaged Cairn binary and one Nix-bound Cairn source snapshot
- WHEN the repository-scale trace-index check runs
- THEN every command MUST return a passing receipt
- AND the queried requirement MUST contain at least one declared link

#### Scenario: Stale index fails

- GIVEN the check exported a valid trace index
- WHEN a copied evidence source adds a new declared marker after export
- THEN trace-index verification MUST fail with a stale-link or unexpected-source issue

#### Scenario: Registry selection excludes unrelated markers

- GIVEN an evidence source contains a valid marker for an accepted ID that is not a row in the selected content-bound registry
- WHEN Cairn builds the selected trace index
- THEN Cairn MUST omit that marker from the selected index
- AND the authoritative full-corpus traceability coverage rail MUST remain responsible for dangling-reference rejection

### Requirement: Inspectable live proof receipts

r[cairn.trace_index.live_proof.receipts] The repository-scale trace-index check MUST retain the requirement registry, trace index, export receipt, verification receipt, query receipt, and negative stale-verification receipt in its Nix output.

#### Scenario: Operator inspects proof output

- GIVEN the repository-scale check passed
- WHEN an operator builds the check output
- THEN the output MUST contain canonical JSON artifacts for each required proof step
- AND each positive receipt MUST name its command, content identity, and passing verdict

### Requirement: Live proof validation

r[cairn.trace_index.live_proof.validation] Cairn MUST test accepted and rejected declaration-marker behavior and MUST run the maintained repository validation rails after the live-proof change.

#### Scenario: Positive and negative tests pass

- GIVEN marker examples, conflicting declarations, missing declarations, current content, and stale content fixtures
- WHEN the focused and repository checks run
- THEN accepted fixtures MUST pass
- AND rejected fixtures MUST fail with deterministic diagnostics

### Requirement: Live proof documentation

r[cairn.trace_index.live_proof.docs] Cairn MUST document how to build and inspect the repository-scale trace-index proof and MUST state its bounded claim.

#### Scenario: Proof is discoverable without session logs

- GIVEN an operator reads the trace-index documentation
- WHEN the operator needs evidence that the command chain works on Cairn
- THEN the documentation MUST name the maintained Nix check and its retained receipt files
- AND it MUST state that the proof does not establish implementation correctness or requirement satisfaction

### Requirement: Evidence bundle trace export

r[cairn.trace_index.evidence_bundle.export] Cairn MUST export the content-bound requirement registry, trace index, and passing current-root trace-index verification receipt as required artifacts in its normal evidence bundle.

#### Scenario: Evidence export creates trace artifacts

- GIVEN a valid repository with accepted requirements and declared source links
- WHEN the operator runs `cairn evidence export`
- THEN the bundle MUST contain the canonical requirement registry and trace index
- AND the bundle MUST contain a passing trace-index verification receipt

### Requirement: Trace evidence portability

r[cairn.trace_index.evidence_bundle.portability] Cairn MUST preserve declared trace artifacts through evidence import, republish, indexing, registry selection, mirror verification, and local fetch workflows that operate on generic bundle artifacts.

#### Scenario: Generic evidence transport retains trace artifacts

- GIVEN a valid evidence bundle with declared trace artifacts
- WHEN Cairn imports, republishes, indexes, or locally materializes that bundle
- THEN each trace artifact MUST retain its role, safe relative path, and exact BLAKE3

### Requirement: Typed trace artifact verification

r[cairn.trace_index.evidence_bundle.verification] Cairn MUST validate trace-index and trace-index-verification manifest roles with closed schemas, self-bound identities, and exact requirement-registry binding.

#### Scenario: Matching trace artifacts pass

- GIVEN a manifest with a valid registry, trace index, and passing verification receipt
- WHEN Cairn verifies the manifest
- THEN each trace artifact role MUST pass
- AND the index's requirement-registry BLAKE3 MUST match the declared registry artifact

#### Scenario: Tampered trace artifact fails

- GIVEN a manifest whose trace index, verification receipt, or registry binding changed
- WHEN Cairn verifies the manifest
- THEN verification MUST fail with a deterministic artifact issue

### Requirement: Bundle trace freshness

r[cairn.trace_index.evidence_bundle.freshness] Cairn MUST verify the bundled trace index against the current repository root during evidence bundle verification.

#### Scenario: Current source passes

- GIVEN an unchanged repository and its exported evidence bundle
- WHEN the operator runs `cairn evidence verify`
- THEN current-root trace-index verification MUST pass

#### Scenario: Changed source fails

- GIVEN a repository source changed after evidence export
- WHEN the operator runs `cairn evidence verify`
- THEN verification MUST fail with a stale trace-index issue

### Requirement: Bundle integration validation

r[cairn.trace_index.evidence_bundle.validation] Cairn MUST test successful trace artifact export and verification plus rejected hash tampering, registry mismatch, malformed schemas, and stale source content.

#### Scenario: Positive and negative bundle tests pass

- GIVEN current, tampered, malformed, mismatched, and stale bundle fixtures
- WHEN the focused evidence tests run
- THEN current artifacts MUST pass
- AND invalid artifacts MUST fail with deterministic diagnostics

### Requirement: Trace evidence documentation

r[cairn.trace_index.evidence_bundle.docs] Cairn MUST document the trace artifacts in normal evidence bundles and their declared-linkage claim boundary.

#### Scenario: Operator understands the bundle proof

- GIVEN the evidence and trace-index documentation
- WHEN an operator reads the bundle workflow
- THEN the documentation MUST name the trace artifacts and verification commands
- AND it MUST state that the artifacts do not prove requirement satisfaction or implementation correctness
