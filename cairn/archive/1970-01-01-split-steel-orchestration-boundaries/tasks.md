# Tasks

- [x] S1 [serial] Split the Steel orchestration CLI source into dispatch, profile, evaluation, and receipt modules. r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.dispatch]
- [x] S2 [serial] Keep profile selection/fallback and script evaluation behavior deterministic and Rust-owned. r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.evaluation]
- [x] S3 [serial] Keep receipt/default-gate JSON shape and receipt hashing stable through pure builders. r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.receipts]
- [x] V1 [serial] Add/retain behavior-preserving tests for disabled, planned, denied, fallback, redaction, and default-gate cases. r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.tests]
- [x] V2 [serial] Run focused Steel tests, Cairn validation, Tracey coverage, and final repository gate before archive. r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.tests]
