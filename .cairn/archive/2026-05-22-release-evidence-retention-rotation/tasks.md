# Tasks

- [x] [serial] Define the public local retention/rotation CLI contract, safe publication-root inventory, and label inputs for deterministic planning. r[release_evidence.retention_rotation.cli_contract]
- [x] [depends:1] Implement read-only retention planning over materialized publication trees by composing existing release-evidence publication validators. r[release_evidence.retention_rotation.composes_publication_validators]
- [x] [depends:2] Emit deterministic retention plan and receipt artifacts that bind publication manifest hashes, receipt hashes, labels, actions, reason codes, and verdict. r[release_evidence.retention_rotation.plan_receipt]
- [x] [depends:3] Add explicit rotation execution semantics with plan-hash verification, mutation manifests, safe relative paths, and dry-run default behavior. r[release_evidence.retention_rotation.rotation_execution]
- [x] [depends:2,3,4] Add positive and negative tests for keep/prune/supersede decisions, malformed labels, missing manifests, hash drift, duplicate current releases, unsafe paths, and execute-without-valid-plan failures. r[release_evidence.retention_rotation.fail_closed]
- [x] [depends:3,4,5] Update docs, policy schemas, generated policy JSON, and Tracey references for local retention non-claims before implementation verification/archive. r[release_evidence.retention_rotation.operator_docs]
