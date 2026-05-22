# Tasks

- [x] [serial] Add a fail-closed release-evidence index parser for the generated Markdown payload block. r[release_evidence.current_head_staleness_guard.parser]
- [x] [serial] Add `release-evidence check-index` with audited-payload and current-head modes. r[release_evidence.current_head_staleness_guard.command]
- [x] [serial] Verify payload commit existence, ancestor relationship, and current-head equality where required. r[release_evidence.current_head_staleness_guard.git_checks]
- [x] [serial] Add positive audited-payload and current-head tests plus stale-current-head negative coverage. r[release_evidence.current_head_staleness_guard.tests]
- [x] [serial] Wire the conventional checked-in release evidence page into release-readiness so stale claims fail closed. r[release_evidence.current_head_staleness_guard.release_readiness]
- [x] [serial] Update operator docs to distinguish CI/local current-head artifacts from committed audited payload snapshots. r[release_evidence.current_head_staleness_guard.operator_docs]
- [x] [serial] Run focused and full verification, sync/archive the package, commit, and push. r[release_evidence.current_head_staleness_guard.archive_ready]
