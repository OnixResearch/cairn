## Phase 1: Contract and fixtures

- [x] [serial] Define the `cross_repo_dependencies` metadata contract in policy-owned schema material. r[cairn.cross_repo_change_dependencies.contract]
- [x] [serial] Add positive fixtures for satisfied upstream proposal/design/tasks gate, validation, sync, and archive receipts. r[cairn.cross_repo_change_dependencies.fixtures.positive]
- [x] [serial] Add negative fixtures for every non-pass status, missing, stale-policy, stale-subject, wrong-repo, wrong-change, wrong-kind, unsupported-kind, unsafe-diagnostic, and hash-mismatch dependencies. r[cairn.cross_repo_change_dependencies.fixtures.negative]

## Phase 2: Validation core and shell

- [x] [serial] Implement pure dependency validation over normalized dependency rows and normalized upstream receipt summaries. r[cairn.cross_repo_change_dependencies.validation]
- [x] [serial] Emit deterministic validation receipts with row statuses, issue codes, input hash, policy hash, and receipt hash. r[cairn.cross_repo_change_dependencies.validation.receipt]
- [x] [serial] Add explicit CLI/MCP evidence loading without sibling-repo or remote auto-fetch. r[cairn.cross_repo_change_dependencies.shell]
- [x] [serial] Surface blocked cross-repo dependencies in validation, drain planning, and release-readiness summaries. r[cairn.cross_repo_change_dependencies.blockers]
- [x] [serial] Enforce explicit current-pass-only satisfaction across every supported receipt kind and preserve all other statuses as deterministic blockers. r[cairn.cross_repo_change_dependencies.status_feedback]
- [x] [serial] Emit safe actionable dependency diagnostics with stable IDs, expected/actual hashes, remediation, evidence-input or reproduction guidance, artifact refs, severity, and non-claims. r[cairn.cross_repo_change_dependencies.status_feedback]

## Phase 3: Documentation and verification

- [x] [parallel] Document metadata examples, supported receipt kinds, explicit evidence inputs, pass-only status semantics, actionable diagnostics, and non-claim boundaries. r[cairn.cross_repo_change_dependencies.docs]
- [x] [serial] Add core and CLI positive tests proving matching receipts satisfy dependencies. r[cairn.cross_repo_change_dependencies.final_validation.positive]
- [x] [serial] Add core and CLI negative tests proving bad or absent receipts fail closed without mutation. r[cairn.cross_repo_change_dependencies.final_validation.negative]
- [x] [serial] Run focused tests, Cairn validation, gates, and release-readiness evidence before sync/archive. r[cairn.cross_repo_change_dependencies.final_validation]
