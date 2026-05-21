## Phase 1: Test split

- [x] [serial] Inspect the existing evidence/registry mega test and helper dependencies. r[cairn.supersede.split_evidence_registry_cli_tests.focused_target]
- [x] [serial] Move the portable evidence registry chain coverage into a dedicated integration-test file. r[cairn.supersede.split_evidence_registry_cli_tests.behavior_preserving]
- [x] [serial] Preserve positive and negative assertions for registry verify, mirror verify, select, fetch-plan, fetch-execute, safe paths, missing artifacts, and hash mismatches. r[cairn.supersede.split_evidence_registry_cli_tests.behavior_preserving]
- [x] [parallel] Confirm the catch-all CLI test file no longer owns the full evidence registry chain proof. r[cairn.supersede.split_evidence_registry_cli_tests.cli_shrinks]

## Phase 2: Verification

- [x] [serial] Run focused evidence-registry and remaining CLI integration tests. r[cairn.supersede.split_evidence_registry_cli_tests.focused_target]
- [x] [serial] Run full workspace verification, Cairn validate, Tracey coverage, and diff checks. r[cairn.supersede.split_evidence_registry_cli_tests.behavior_preserving]
- [x] [serial] Archive the completed Cairn change, commit, and push. r[cairn.supersede.split_evidence_registry_cli_tests.cli_shrinks]
