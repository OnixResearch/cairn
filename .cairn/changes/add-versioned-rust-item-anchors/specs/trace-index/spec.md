# Versioned Rust item anchors

## ADDED Requirements

### Requirement: Opt-in structural profile

r[cairn.trace_index.structural_profile] Cairn MUST accept a typed `rust_item_v1` traceability profile only with a safe explicit Rust configuration path, emit a self-bound v2 structural index, and preserve historical exact-marker v1 bytes and defaults.

#### Scenario: Legacy and structural profiles coexist

- GIVEN an existing exact-marker profile and a configured Rust-item profile
- WHEN either profile exports and verifies
- THEN exact-marker v1 material MUST retain its prior schema and hash inputs
- AND the structural profile MUST carry a distinct schema and interpretation identity

### Requirement: Saved-byte Rust adapter

r[cairn.trace_index.structural_adapter] Cairn MUST interpret bounded saved UTF-8 Rust bytes with an explicit target, features and cfg subset, and MUST reject unsupported expansion, unknown predicates, malformed syntax, unsafe paths and exhausted budgets without accepting a partial item.

#### Scenario: Unicode and structural cfg

- GIVEN CRLF source with `/* é */ fn x() {}`, Unicode identifiers, and `#[cfg(not(any()))]`
- WHEN the parser resolves an attached marker
- THEN item spans MUST use UTF-8 byte offsets without slicing panic
- AND `not(any())` MUST be evaluated structurally rather than by substring

### Requirement: Immutable requirement/item bindings

r[cairn.trace_index.structural_history] Cairn MUST keep requirement-row revision, exact source occurrence and parsed scoped item interpretation distinct, retain immutable prior bindings, and allow only the selected current attachment to satisfy freshness.

#### Scenario: Revised source and unchanged normalized tokens

- GIVEN a prior binding and a comment/format edit that preserves normalized item tokens
- WHEN the root is verified or a revised index is exported
- THEN old exact-source receipts MUST be stale
- AND the revised index MAY retain the prior binding as historical, not current

#### Scenario: Callee change does not imply proof freshness

- GIVEN a marked caller and a helper in the declared source snapshot
- WHEN helper bytes change while caller tokens do not
- THEN current-root freshness MUST fail for the old snapshot
- AND linkage MUST NOT claim behavior, tests, formal proof or release readiness

### Requirement: Structural consumer agreement

r[cairn.trace_index.structural_consumers] Export, verify, show and portable evidence verification MUST agree on selected requirement/item bindings and distinguish current, stale, historical, absent and unobserved states.

#### Scenario: History is not a current attachment

- GIVEN a valid v2 index containing a former item binding in history but no current attachment
- WHEN a consumer queries or verifies it
- THEN the former item MUST remain historical and MUST NOT pass as a fresh current result

### Requirement: Positive and negative structural coverage

r[cairn.trace_index.structural_validation] Cairn MUST exercise successful and rejected Rust item resolution, including nested duplicate names, moved/copied declarations, Unicode byte boundaries, structural cfg, parser/configuration changes, stale requirement/item revisions and budget exhaustion in pinned Rust/Nix validation.

#### Scenario: Unsupported inputs fail closed

- GIVEN malformed Rust, ambiguous selection, unresolved macro or unknown target predicate
- WHEN a structural index is exported
- THEN the operation MUST fail explicitly without emitting an accepting index
