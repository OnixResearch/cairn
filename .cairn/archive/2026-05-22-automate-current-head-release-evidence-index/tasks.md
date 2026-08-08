# Tasks

- [x] [serial] Define the public CLI shape and deterministic output contract for the generated current-HEAD release evidence index. r[release_evidence.automated_current_head_index.cli_contract]
- [x] [depends:1] Implement the thin CLI orchestration over existing release-readiness and evidence publication rails without duplicating validators. r[release_evidence.automated_current_head_index.composes_existing_rails]
- [x] [depends:2] Add positive tests that generated index output binds the payload HEAD, lists required artifacts/receipts, and verifies the generated evidence chain. r[release_evidence.automated_current_head_index.generated_index]
- [x] [depends:2] Add negative tests for dirty/mismatched/incomplete evidence states with deterministic failure receipts or errors. r[release_evidence.automated_current_head_index.fail_closed]
- [x] [depends:3,4] Update docs/policy/Tracey references for the new operator workflow and run focused plus repository verification before sync/archive. r[release_evidence.automated_current_head_index.operator_docs]
