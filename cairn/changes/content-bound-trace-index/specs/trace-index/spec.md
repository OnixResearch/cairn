# Content-bound Trace Index Specification Delta

## ADDED Requirements

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
