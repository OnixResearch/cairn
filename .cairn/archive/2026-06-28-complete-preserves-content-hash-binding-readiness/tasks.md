# Tasks

## 1. Content-hash binding

- [x] I1 [serial] Add common CLI helpers for inserting `preserves_content_hash` before final `receipt_hash` computation. r[cairn.determinism.preserves_content_hash_binding_readiness.binding]
- [x] I2 [serial] Bind `preserves_content_hash` onto replay, evidence-bundle, agent-bundle, and release-evidence receipt outputs that claim content identity. r[cairn.determinism.preserves_content_hash_binding_readiness.binding]
- [x] I3 [serial] Update Nickel/generated receipt schema policy so target commands list `preserves_content_hash` as a stable hash field. r[cairn.determinism.preserves_content_hash_binding_readiness.policy]

## 2. Release-readiness repair

- [x] I4 [serial] Refresh positive agent receipt replay policy/hash fixture values from current generated policy. r[cairn.determinism.preserves_content_hash_binding_readiness.mcp_smoke]
- [x] I5 [serial] Repair Tracey markers for Preserves validation requirements and remove dangling markers. r[cairn.determinism.preserves_content_hash_binding_readiness.tracey]

## 3. Verification

- [x] V1 [serial] Run focused validation: `validate`, proposal/design/tasks gates, cargo check/test, `tracey coverage`, `agent-receipt smoke-mcp`, and `release-readiness`. r[cairn.determinism.preserves_content_hash_binding_readiness.validation]
