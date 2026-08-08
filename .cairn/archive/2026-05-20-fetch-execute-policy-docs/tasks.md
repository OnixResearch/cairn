## Phase 1: Policy and docs

- [x] [serial] r[cairn.supersede.fetch_execute_policy_docs.policy.schema] Add `evidence registry fetch-execute` to the Nickel-authored receipt schema catalog and regenerate/check policy JSON.
- [x] [serial] r[cairn.supersede.fetch_execute_policy_docs.policy.inventory] Add fetch-execute to the deterministic surface inventory with reviewed replay expectations and determinism claim.
- [x] [serial] r[cairn.supersede.fetch_execute_policy_docs.docs.flow] Update operator documentation and workflow docs with the local/offline fetch-execute command flow.
- [x] [serial] r[cairn.supersede.fetch_execute_policy_docs.docs.non_claims] Document non-claims around network transport, credentials, remote trust, and unsafe paths.

## Phase 2: Verification

- [x] [serial] r[cairn.supersede.fetch_execute_policy_docs.policy.schema] Add focused test coverage that the emitted fetch-execute receipt matches the policy schema catalog.
- [x] [serial] r[cairn.supersede.fetch_execute_policy_docs.policy.inventory] Run policy export/check, focused registry test, workspace verification, Cairn validate, Tracey coverage, and flake check.
- [x] [serial] r[cairn.supersede.fetch_execute_policy_docs.docs.flow] Sync, archive, commit, and push the completed Cairn change.
