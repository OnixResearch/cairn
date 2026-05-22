# Tasks

- [ ] [serial] Define the release-evidence module boundary and keep the dispatcher facade small. r[release_evidence.cli_source_split.focused_modules]
- [ ] [parallel] Move current-HEAD index generation into a focused module while preserving CLI behavior and receipts. r[release_evidence.cli_source_split.index_module]
- [ ] [parallel] Move publish materialization into a focused module while preserving manifest/receipt behavior. r[release_evidence.cli_source_split.publish_module]
- [ ] [parallel] Move retention planning into a focused module while preserving deterministic plan behavior. r[release_evidence.cli_source_split.retention_module]
- [ ] [parallel] Move rotation execution into a focused module while preserving dry-run/execute receipt behavior. r[release_evidence.cli_source_split.rotate_module]
- [ ] [serial] Extract only narrow shared helpers needed by multiple modules and avoid broad crate-root shims. r[release_evidence.cli_source_split.narrow_bridges]
- [ ] [serial] Run focused release-evidence tests and confirm no intentional receipt, issue-code, fixture, or policy drift. r[release_evidence.cli_source_split.behavior_preserved]
- [ ] [serial] Run repository verification: formatting, workspace tests, clippy, Cairn validate, Tracey coverage, and diff check. r[release_evidence.cli_source_split.repository_checks]
- [ ] [serial] Sync this delta into accepted specs, archive the package, commit, and push the verified slice. r[release_evidence.cli_source_split.archive_ready]
