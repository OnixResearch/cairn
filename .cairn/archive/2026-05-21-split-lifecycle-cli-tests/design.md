## Context

Recent Cairn maintainability slices split command families out of `main.rs` and moved determinism, content-addressed evidence, and evidence-registry tests into dedicated integration-test targets. The remaining `tests/cli.rs` file still contains native lifecycle coverage for init/change/read/validate/gate/sync/archive/migration plus unrelated evidence/MCP coverage.

## Approach

Create `crates/cairn-cli/tests/lifecycle.rs` and move the lifecycle cluster from `tests/cli.rs` into it. The moved target will keep local copies of only the helpers it needs: command construction, JSON command execution, fixture copy/temp-root setup, golden loading, receipt-schema helpers, and reproducibility helpers. The catch-all CLI test file should no longer own the lifecycle mutation/read/gate proof.

The split is behavior-preserving: public command parsing, JSON receipts, fixtures, goldens, and accepted specs must not change. If a moved helper becomes unused in either file, remove it from that file only.

## Risks

- **Helper drift**: duplicated test helpers may diverge. Mitigation: keep helpers minimal and local to the moved cluster.
- **Lost coverage**: moving tests can accidentally drop an assertion. Mitigation: move complete tests first, then run the focused lifecycle target and remaining CLI target.
- **Fixture/golden churn**: this slice should not regenerate fixtures or receipt goldens. Mitigation: treat fixture/golden diffs as blockers unless caused by accidental formatting-only movement outside test files.

## Verification

- `cargo test -p cairn-cli --test lifecycle`
- `cargo test -p cairn-cli --test cli`
- `cargo test --workspace`
- `cairn validate --root .`
- `cairn tracey coverage --root .`
- `git diff --check`
