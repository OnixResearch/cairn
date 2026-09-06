# Cross-repository adoption tasks

## Contract and core

- [x] [serial] [task:baseline] Record dependency-receipt, registry, and unified-readiness baselines with current, incomplete, and rejected inputs. r[cairn.adoption_report.conformance]
- [x] [serial] [task:contracts] [after:baseline] Define typed snapshot, stage, coverage, provenance, and query contracts with explicit producer cohorts and named bounds. r[cairn.adoption_report.inputs] r[cairn.adoption_report.stages]
- [x] [serial] [task:core] [after:contracts] Implement pure relationship joins, independent adoption stages, unknown-state preservation, and deterministic report identities. r[cairn.adoption_report.stages] r[cairn.adoption_report.unknowns]

## Stack adapters and queries

- [x] [serial] [task:producers] [after:core] Admit saved stack-deps and stack-doctor contracts through exact producer validation evidence without copying discovery or reconciliation code. r[cairn.adoption_report.inputs] r[cairn.adoption_report.boundary]
- [ ] [serial] [task:evidence] [after:producers] Compose existing Cairn lifecycle, completion, registry, and applicable Valence admission without promoting upstream or catalog authority. r[cairn.adoption_report.stages] r[cairn.adoption_report.boundary]
- [ ] [serial] [task:interfaces] [after:evidence] Add equivalent bounded CLI and MCP queries with explicit coverage and unified readiness actions. r[cairn.adoption_report.queries]
- [ ] [serial] [task:controls] [after:interfaces] Add old-pin, missing-check, stale-policy, conflicting-revision, unsupported-schema, cycle, and partial-scope controls. r[cairn.adoption_report.unknowns] r[cairn.adoption_report.conformance]

## Consumer evidence and handoff

- [ ] [serial] [task:consumers] [after:controls] Replay saved Cairn-to-Octet and Octet-to-Lattice cohorts from a fresh directory without sibling checkouts or services. r[cairn.adoption_report.boundary] r[cairn.adoption_report.conformance]
- [ ] [serial] [task:docs] [after:consumers] Publish the exact producer recipe, authority map, supported versions, diagnostic actions, and bounded-stage interpretation. r[cairn.adoption_report.inputs] r[cairn.adoption_report.queries] r[cairn.adoption_report.boundary]
- [ ] [serial] [task:verification] [after:docs] Run focused and full checks, retain current input receipts, and prepare reviewed sync and archive plans. r[cairn.adoption_report.conformance]

All tasks describe future implementation. Planning gates do not complete these tasks.
Archive execution is not a prerequisite of any task. Use the normal lifecycle after implementation evidence passes.
