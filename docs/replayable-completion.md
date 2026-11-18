# Replayable completion acceptance report

> Status: the first result requested by issue #4 is the explanation itself. It
> reads existing receipts and adds no engine, gate or scheduler. Codifying the
> argument as a checked requirement needs a companion command that projects
> these facts (for example `cairn explain completion <change>`); until that
> exists, this report is documentation, not a new admission rule.

## The decision explained here

On 2026-11-18 the change `compile-workflow-obligations` was accepted and
archived in its own worktree after its gates passed. That acceptance is the
completion decision this report explains. It is a real decision made by this
repository, not a fixture.

## What follows, and from what

The decision was: **the change's declared work is complete under the repository
default profile, and its specification delta may be merged into the accepted
specifications.**

It rested on these facts, in the order the gates consumed them:

| element | recorded fact |
|---|---|
| requirement revisions | the change's spec delta `specs/workflow-obligations/spec.md`, declaring `cairn.workflow_obligations.{policy, evaluation, temporal, exception, guidance, adapter_invariance, legacy_compatibility, export}` |
| admitted evidence | `cargo test -p cairn-core --lib workflow_obligations` 15 passed; `cargo test -p cairn-core --lib refinement_typed_dependencies` 19 passed; `cargo test -p cairn-cli --test workflow_obligations` 6 passed; `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` clean |
| lifecycle evidence | `cairn validate --root .`: valid, 0 issues; `cairn traceability coverage --root .`: 2400/2400 referenced; `cairn policy export --check`: fresh |
| audit note | `evidence/implementation/verification.md`, which also records the five defects the review corrected before those runs |

### The inference rules applied

1. **Task substance and linkage** — every checkbox task names a known
   requirement reference, and the archive gate refuses a preview while any task
   is open.
2. **Required artifacts** — proposal, design, tasks, metadata and a spec delta
   exist and parse under the change's workflow profile.
3. **Traceability coverage** — every `r[...]` reference resolves to a
   requirement declared in accepted specifications, with no dangling reference.
4. **Policy freshness** — the checked-in generated policy equals the policy the
   evaluators used.

The conclusion is the conjunction of those gates, not any single test result.

## Assumptions

- The generated policy in the worktree is the policy the gates read
  (`policy export --check` compares them).
- The repository root was the worktree being archived.
- Evaluation is local, deterministic and bounded; no external solver, network
  or model was consulted.
- The archived date was supplied explicitly (`CAIRN_ARCHIVE_DATE=2026-11-18`).

## Replay

A reader replays the decision from the same inputs:

```console
cairn policy export --check
cairn validate --root .
cairn traceability coverage --root .
cairn archive compile-workflow-obligations --root .        # preview
```

The first three reproduce the recorded verdicts. The archive preview reproduces
"ready to archive" only before execution; after the archive it reports the
change as absent from the active set, which is the expected post-archive
observation and not drift.

On the composed branch where the change now lives, the same three evaluators
were executed after integration: `cairn validate` 0 issues, `cairn traceability
coverage` complete (2460/2460 at that merge, 2470/2470 after the concurrent
footprint change), and the policy fresh. The composed core suite reported 831
passing tests after the final merge.

## What remains unestablished

- **Semantic correspondence.** No gate proved that the typed contracts and
  obligation definitions capture the full meaning of the prose requirements.
  Correspondence review remains a separate, unperformed step.
- **Behavioural correctness.** The tests exercise the evaluators; they do not
  prove that the workflow obligations are correct for the repository's real
  workflows beyond the shipped pilot.
- **Exhaustive composition.** The composed branch was verified with core and
  binary suites; the full set of CLI integration targets was not re-run to
  completion in this session.
- **Consistency, revision, footprint and refinement analyses** were run and
  archived as advisory capabilities. Their models and witnesses did not
  contribute to this decision and must not.

## Adoption decision

Adopt this reading as the required form of a completion explanation, and keep
its authority bounded exactly as recorded: the decision follows from policy-owned
gates over admitted evidence, and the advisory analyses remain explanations. No
new gate, solver or scheduler is introduced, and no advisory result is promoted
to gate authority.

Historical decisions stay explainable from the artifacts they retained; where an
artifact is missing, the report says so instead of reconstructing it.
