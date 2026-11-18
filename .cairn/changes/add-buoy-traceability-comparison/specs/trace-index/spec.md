# Buoy comparison qualification

## ADDED Requirements

### Requirement: Pinned upstream comparison identity

r[cairn.trace_index.buoy_comparison.pinned_identity] Cairn MUST retain one reviewed upstream comparison identity that names the exact revision, recursive source content hash, Rust toolchain, BLAKE3 feature selection, read-only command surface, machine report schemas, and source evidence, and MUST NOT require network access or a private upstream acceptance setup at comparison-check runtime.

#### Scenario: Source identity changes invalidate the receipt

- GIVEN a retained comparison receipt
- WHEN the pinned revision, source content hash, toolchain, feature, command surface, mapping, policy, or fixture bytes change
- THEN the computed comparison identity MUST change

#### Scenario: Executed claim without an artifact is rejected

- GIVEN an upstream identity whose artifact is not realized
- WHEN a manifest declares an executed upstream observation
- THEN the manifest MUST be rejected without producing an accepting receipt

### Requirement: Versioned declared projection

r[cairn.trace_index.buoy_comparison.projection] Cairn MUST compare a small versioned semantic subset through an explicit projection that preserves original Cairn requirement and source identities and MUST NOT treat the projection as a second production authority.

#### Scenario: Producer schemas keep distinct identities

- GIVEN native Cairn observations and declared upstream observations that both use BLAKE3
- WHEN the comparison receipt is built
- THEN the producer report identities MUST use distinct domains and retain their schemas
- AND equal digest bytes across schemas MUST NOT be reported as agreement

#### Scenario: Unsupported dimensions stay explicit

- GIVEN a dimension outside the selected semantic subset
- WHEN a case is inventoried
- THEN the case MUST carry an explicit unsupported or expected-divergence disposition
- AND it MUST NOT become a passing executed result

### Requirement: Independent mutation qualification

r[cairn.trace_index.buoy_comparison.mutation] Cairn MUST specify native expected outcomes independently from declared upstream observations, execute positive and negative fixtures for missing marker, duplicate marker, moved span, changed requirement body, and schema/version drift, and reject a mutation that restates its base observation or base native expectation.

#### Scenario: Planted drift is detected

- GIVEN a clean executed base fixture and a mutation that removes, copies, or moves a marker, changes a requirement body, or changes the saved index schema
- WHEN the comparison executes the ordinary trace-index commands on a temporary fixture copy
- THEN the native classification MUST change from the base classification
- AND the comparison MUST report the change rather than a pass

#### Scenario: Originals are unchanged

- GIVEN the retained fixture set
- WHEN the comparison executes
- THEN only private temporary copies MUST be used for export and verification
- AND the retained fixture bytes MUST remain unchanged

### Requirement: Advisory comparison boundary

r[cairn.trace_index.buoy_comparison.boundary] Cairn MUST keep comparison receipts advisory and report-only, MUST retain raw native observations, command dispositions, runtime scope, unsupported cases, known disagreements, and the reuse decision, and MUST leave existing traceability gate meanings unchanged.

#### Scenario: Existing trace-index checks remain independent

- GIVEN the existing repository-scale trace-index live proof
- WHEN the comparison check runs
- THEN the live proof MUST continue to execute its own export, verification, query, and stale-source negative case
- AND no comparison result MUST substitute for those checks

#### Scenario: No behavioral or satisfaction claim

- GIVEN a valid comparison receipt
- WHEN an operator reads it
- THEN it MUST state that structural comparison does not prove behavior, requirement satisfaction, implementation correctness, test adequacy, proof soundness, release eligibility, or lifecycle readiness
