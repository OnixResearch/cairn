## Tasks

- [x] [serial] r[cairn.release_evidence.durable_run_ledger.model] Define the pure release-run ledger model and BLAKE3 identity profile.
- [x] [depends:release-ledger-model] r[cairn.release_evidence.durable_run_ledger.validation] Validate run identity, policy hash, command identity, step rows, artifact refs, diagnostics, blockers, resume hints, and non-claims.
- [x] [depends:release-ledger-validation] r[cairn.release_evidence.durable_run_ledger.status_explain] Add pure status and explain projections for blockers, stale inputs, missing evidence, resumability, and terminal verdicts.
- [x] [depends:release-ledger-validation] r[cairn.release_evidence.durable_run_ledger.fixtures.positive] Add positive fixtures for complete, blocked, and resumable release runs.
- [x] [depends:release-ledger-validation] r[cairn.release_evidence.durable_run_ledger.fixtures.negative] Add negative fixtures for stale policy, missing command identity, invalid transition, weakened non-claim, artifact substitution, and publish overclaim.
- [x] [depends:release-ledger-cli] r[cairn.release_evidence.durable_run_ledger.shell] Expose read-only CLI status/explain over existing local ledger artifacts while keeping mutation out of scope.
- [x] [depends:release-ledger-fixtures] r[cairn.release_evidence.durable_run_ledger.final_validation] Run focused ledger tests, CLI smoke tests, core purity smoke, Cairn gates, and release-readiness validation.