## Phase 1: Package

- [x] P1 [serial] Define behavior-preserving split requirements for the metrics regression rail. r[cairn.semantic_review.metrics_regression_split.boundaries]

## Phase 2: Implementation

- [x] I1 [serial] Move public entrypoints and report hashing into a narrow module root/report seam. r[cairn.semantic_review.metrics_regression_split.public_api]
- [x] I2 [serial] Extract registry, fixture-pair, guidance-sync, and evidence-boundary helpers into focused private modules. r[cairn.semantic_review.metrics_regression_split.focused_helpers]
- [x] I3 [serial] Add/adjust tests proving the refactor preserves existing issue codes and receipt behavior. r[cairn.semantic_review.metrics_regression_split.behavior_preserved]

## Phase 3: Verification

- [x] V1 [serial] Run focused semantic-review checks. r[cairn.semantic_review.metrics_regression_split.focused_verification]
- [x] V2 [serial] Run Cairn validate, Tracey coverage, archive, final gate, commit, and push. r[cairn.semantic_review.metrics_regression_split.repository_verification]
