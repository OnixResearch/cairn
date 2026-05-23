# Tasks

- [x] [serial] Map the existing semantic-review core responsibilities to focused modules. r[semantic_review.split_semantic_review_core.focused_modules.visible_boundaries]
- [x] [serial] Move review receipt, revision handoff, analytics, completeness, profile, and common helper code behind the existing facade. r[semantic_review.split_semantic_review_core.focused_modules]
- [x] [serial] Keep bridge visibility narrow and preserve public re-exports used by CLI consumers. r[semantic_review.split_semantic_review_core.focused_modules.narrow_bridges]
- [x] [serial] Run focused semantic-review/core checks and repair import or behavior drift. r[semantic_review.split_semantic_review_core.behavior_preserved.focused_checks]
- [x] [serial] Run repository validation, formatting, linting, Tracey coverage, and diff checks before archive. r[semantic_review.split_semantic_review_core.behavior_preserved.repository_checks]
