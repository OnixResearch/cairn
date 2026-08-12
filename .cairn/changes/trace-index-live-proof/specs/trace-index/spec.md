# Trace Index Delta

## ADDED Requirements

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
