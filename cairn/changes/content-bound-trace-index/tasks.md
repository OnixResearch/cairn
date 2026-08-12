## Phase 1: Lifecycle contract

- [ ] [serial] Specify canonical index identity, exact marker locators, and claim boundaries for r[cairn.trace_index.canonical_index].
- [ ] [serial] Specify export, verification, and direct query behavior for r[cairn.trace_index.cli].

## Phase 2: Functional core

- [ ] [serial] Implement canonical trace-index construction and deterministic sorting for r[cairn.trace_index.canonical_index].
- [ ] [serial] Implement closed-schema validation and current-root freshness checks for r[cairn.trace_index.freshness].
- [ ] [serial] Implement exact requirement lookup for r[cairn.trace_index.query].
- [ ] [serial] Add positive and negative core tests for r[cairn.trace_index.validation].

## Phase 3: Imperative shell and policy

- [ ] [serial] Add typed policy fields for the declared assurance level and exact-marker anchor for r[cairn.trace_index.policy].
- [ ] [serial] Add bounded export, verify, and show command adapters for r[cairn.trace_index.cli].
- [ ] [serial] Add CLI tests for deterministic export and direct query for r[cairn.trace_index.query].
- [ ] [serial] Add stale-source, moved-marker, copied-marker, malformed-index, and policy-drift tests for r[cairn.trace_index.freshness].

## Phase 4: Documentation and completion

- [ ] [serial] Document commands, evidence boundaries, and the Ambient and Unison precedents for r[cairn.trace_index.docs].
- [ ] [serial] Run focused and full repository checks for r[cairn.trace_index.validation].
- [ ] [serial] Sync the accepted specification and archive the completed change for r[cairn.trace_index.docs].
