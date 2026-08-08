## Phase 1: Implementation

- [x] I1 [serial] Map the current semantic-review facade and test responsibilities. r[semantic_review.split_semantic_review_core.focused_modules.visible_boundaries]
- [x] I2 [serial] Move inline semantic-review tests/fixtures into a dedicated test submodule behind the same facade. r[semantic_review.split_semantic_review_core.focused_modules]
- [x] I3 [serial] Preserve narrow bridges and public re-exports for existing core and CLI consumers. r[semantic_review.split_semantic_review_core.focused_modules.narrow_bridges]

## Phase 2: Verification

- [x] V1 [serial] Run focused semantic-review core checks. r[semantic_review.split_semantic_review_core.behavior_preserved.focused_checks]
- [x] V2 [serial] Run repository validation, Tracey coverage, diff checks, and final workspace gate. r[semantic_review.split_semantic_review_core.behavior_preserved.repository_checks]
