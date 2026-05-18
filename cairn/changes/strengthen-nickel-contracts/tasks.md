# Tasks

- [x] [serial] Specify receipt schema contracts in Nickel for gate, lifecycle, migration, action entries, and BLAKE3 hash fields. r[cairn.supersede.nickel_contracts.receipt_schemas]
- [x] [serial] Strengthen Nickel policy invariants for supported version, hash algorithm/canonicalization, non-empty arrays, duplicate-free vocabularies, and known artifact dependencies. r[cairn.supersede.nickel_contracts.semantic_invariants]
- [x] [parallel] Add exported policy data for task marker vocabulary, lifecycle artifact dependencies, finding defaults, and receipt schema declarations. r[cairn.supersede.nickel_contracts.generated_vocabularies]
- [x] [parallel] Refactor Rust policy parsing/evaluation to consume generated vocabularies instead of hardcoded marker/artifact/receipt-key literals where practical. r[cairn.supersede.nickel_contracts.generated_vocabularies] r[cairn.supersede.nickel_contracts.boundary]
- [x] [parallel] Parse gate exemption markers and validate required keys/date shape against Nickel-owned exemption marker contracts without adding ambient time. r[cairn.supersede.nickel_contracts.exemption_markers]
- [x] [serial] Add invalid Nickel fixtures and export/check helper assertions for malformed receipts, malformed policy semantics, stale generated policy, duplicate vocabulary entries, and unknown artifact dependencies. r[cairn.supersede.nickel_contracts.contract_rails]
- [x] [serial] Add CLI/core regression tests that compare public receipt keys and action keys against generated policy schema declarations. r[cairn.supersede.nickel_contracts.receipt_schemas]
- [x] [serial] Update README and repo-maintained Hermes skill source to describe Nickel-owned schema/vocabulary responsibilities and runtime Nickel-free boundary. r[cairn.supersede.nickel_contracts.contract_rails]
- [x] [serial] Run focused verification: `python3 tools/export-policy.py --check`, `cargo test --workspace`, `nix run path:$PWD#cairn -- validate --root .`, and proposal/design/tasks gates for this change. r[cairn.supersede.nickel_contracts.contract_rails]
