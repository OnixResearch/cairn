# Tasks: Review metrics regression rail

## Phase 1: Define reusable contract

- [x] [serial] S1: Create a native Cairn package for the generic review metrics regression rail [r[cairn.semantic_review.review_metrics_regression_rail.registry_binds_surfaces]]
- [x] [serial] S2: Capture the downstream split: Cairn owns the invariant, projects own category bodies and fixtures [r[cairn.semantic_review.review_metrics_regression_rail.downstream_category_ownership]]

## Phase 2: Implement registry and receipt validation

- [x] [serial] I1: Add pure core DTOs and validation for category registry entries, including duplicate/missing binding diagnostics [r[cairn.semantic_review.review_metrics_regression_rail.registry_binds_surfaces]]
- [ ] [serial] I2: Add fixture-pair validation that checks negative diagnostics and positive passing coverage [r[cairn.semantic_review.review_metrics_regression_rail.fixture_pair_enforcement]]
- [ ] [parallel] I3: Add guidance/task/spec synchronization checks for diagnostic codes and requirement ids [r[cairn.semantic_review.review_metrics_regression_rail.guidance_task_sync]]
- [ ] [parallel] I4: Add secret-free evidence boundary checks for metrics snapshot references [r[cairn.semantic_review.review_metrics_regression_rail.secret_free_evidence]]
- [ ] [serial] I5: Expose a deterministic CLI receipt command for downstream projects to run against their registry and fixture roots [r[cairn.semantic_review.review_metrics_regression_rail.downstream_category_ownership]]

## Phase 3: Verify and archive

- [ ] [serial] V1: Add positive and negative Cairn fixtures for registry drift, fixture-pair drift, guidance/task drift, and unsafe evidence [r[cairn.semantic_review.review_metrics_regression_rail.registry_binds_surfaces]] [r[cairn.semantic_review.review_metrics_regression_rail.fixture_pair_enforcement]] [r[cairn.semantic_review.review_metrics_regression_rail.guidance_task_sync]] [r[cairn.semantic_review.review_metrics_regression_rail.secret_free_evidence]]
- [ ] [serial] V2: Run focused semantic-review tests plus `nix run path:$PWD#cairn -- validate --root .`, proposal/design/tasks gates, and `git diff --check` [r[cairn.semantic_review.review_metrics_regression_rail.downstream_category_ownership]]
