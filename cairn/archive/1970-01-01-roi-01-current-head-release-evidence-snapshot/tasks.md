# Tasks

## 1. Evidence generation

- [x] I1 [serial] Generate the current-HEAD release evidence index under `target/release-evidence-current-head/`. r[release_evidence.current_head_snapshot.generate]
- [x] I2 [serial] Check the generated index in current-head mode and capture receipt facts. r[release_evidence.current_head_snapshot.check_index]

## 2. Verification

- [x] V1 [serial] Run release-readiness, Cairn validate, Tracey coverage, and diff checks. r[release_evidence.current_head_snapshot.validation]
- [x] V2 [serial] If committing a summary, verify wording avoids stale current-head claims and excludes generated `target/` artifacts. r[release_evidence.current_head_snapshot.docs_boundary]
