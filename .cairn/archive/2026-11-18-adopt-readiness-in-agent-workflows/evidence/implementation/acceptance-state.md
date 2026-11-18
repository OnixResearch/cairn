# Adoption acceptance state

Status: **blocked on one regression introduced by the dependency merge.**

## What was repaired during integration

The branch had never been compiled. Merging the archived
`add-compact-agent-readiness` branch exposed real defects, which were fixed:

1. `readiness.rs` conflicted between adopt's `unavailable_readiness_snapshot`
   helper and compact's `gather`-based body. The compact body referenced `root`
   and `policy_path` that adopt's function does not take. Adopt's body was
   restored.
2. `fresh_readiness_report` wrapped `readiness_report_with_budget(...)` — which
   returns `ReadinessReport` — in `serde_json::to_value(...)?` while declaring a
   `Result<ReadinessReport, _>` return type. Now returns the report directly.
3. `collect_archived_change_ids_with_budget` was private while adopt's observer
   calls it; widened to `pub(crate)`.
4. `unavailable_readiness_snapshot(change, input, &error)` passed `&Box<dyn
   Error>` where `&(dyn Error + 'static)` was required; now `error.as_ref()`.
5. `flake.nix` gained compact's policy-fresh derivation alongside measurement's
   benchmark derivation, and `usage.rs` conflicts were resolved with adopt's
   `drain-plan` line preserved.

After these fixes `cargo build -p cairn-cli --bin cairn` succeeds,
`cairn policy export --check` reports fresh, and `cairn validate --root .` is
valid with 0 issues.

## Remaining failure

`cargo test -p cairn-cli --test readiness_completion` fails
`completion_and_stale_missing_and_symlink_controls`: the `archive` stage is
`blocked` where the fixture expects `ready` (decision hash
`bcd5889c736ddb447e6c1d3461579fba79443217a7ef19c261248ea2cec01af4`,
`valid: true`).

The other suites pass: `cargo test -p cairn-core --lib readiness` is 54/54 and
the `readiness_drain` and `readiness` CLI suites pass. The regression therefore
sits in the merged readiness path rather than in drain selection.

This must be diagnosed before the change is archived: either the merge changed
which observations the archive stage sees, or fix (2) exposed a path that had
never executed. No archive, sync or completion claim is made here.
