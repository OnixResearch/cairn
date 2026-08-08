# Tasks

- [x] [serial] Define the public producer-side CLI contract, local publish layout, and optional label inputs for deterministic release evidence materialization. r[release_evidence.publish_materialization.cli_contract]
- [x] [depends:1] Implement the thin materialization shell over existing release-evidence and evidence-index validators without duplicating release validators. r[release_evidence.publish_materialization.composes_existing_validators]
- [x] [depends:2] Emit deterministic local manifest and receipt artifacts that bind source index, payload commit, materialized artifact roles, relative paths, hashes, labels, and verdict. r[release_evidence.publish_materialization.manifest_receipt]
- [x] [depends:2,3] Add positive and negative tests for safe materialization, path rejection, missing artifacts, hash drift, invalid source evidence, and malformed label metadata. r[release_evidence.publish_materialization.fail_closed]
- [x] [depends:3,4] Update docs, policy schemas, generated policy JSON, and Tracey references for local publication non-claims and retention boundaries before verification/archive. r[release_evidence.publish_materialization.operator_docs]
