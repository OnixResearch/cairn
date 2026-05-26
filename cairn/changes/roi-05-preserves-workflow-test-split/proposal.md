# ROI 05: Preserves workflow test split

## Why

Preserves workflow documentation tests currently live beside corpus update integration tests. This is acceptable but couples docs/runbook coverage to corpus mutation behavior. A small test split would make future docs workflow changes cheaper and clearer.

## What changes

- Move Preserves operator/runbook documentation assertions into a focused test module or test target.
- Keep corpus update behavior tests focused on update/fail-closed semantics.
- Preserve all existing assertions and Tracey refs.

## Non-goals

- Do not change CLI behavior.
- Do not change corpus hashes or docs content except test references if needed.
