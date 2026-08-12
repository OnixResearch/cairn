## Phase 1: Lifecycle contract

- [x] [serial] Specify canonical index identity, exact marker locators, and claim boundaries for r[cairn.trace_index.canonical_index].
- [x] [serial] Specify export, verification, and direct query behavior for r[cairn.trace_index.cli].

## Phase 2: Functional core

- [x] [serial] Implement canonical trace-index construction and deterministic sorting for r[cairn.trace_index.canonical_index].
- [x] [serial] Implement closed-schema validation and current-root freshness checks for r[cairn.trace_index.freshness].
- [x] [serial] Implement exact requirement lookup for r[cairn.trace_index.query].
- [x] [serial] Add positive and negative core tests for r[cairn.trace_index.validation].

## Phase 3: Imperative shell and policy

- [x] [serial] Add typed policy fields for the declared assurance level and exact-marker anchor for r[cairn.trace_index.policy].
- [x] [serial] Add bounded export, verify, and show command adapters for r[cairn.trace_index.cli].
- [x] [serial] Add CLI tests for deterministic export and direct query for r[cairn.trace_index.query].
- [x] [serial] Add stale-source, moved-marker, copied-marker, malformed-index, and policy-drift tests for r[cairn.trace_index.freshness].

## Phase 4: Documentation and completion

- [x] [serial] Document commands, evidence boundaries, and the Ambient and Unison precedents for r[cairn.trace_index.docs].
- [x] [serial] Run focused and full repository checks for r[cairn.trace_index.validation].
- [x] [serial] Sync the accepted specification and archive the completed change for r[cairn.trace_index.docs].
