# Readiness-backed workflow design

## Context

The accepted unified-readiness contract keeps original validators authoritative.
The drain adapter currently repeats gate and plan composition in `mcp/read_tools.rs`.
The accepted drain specification also preserves a public default receipt. Migration therefore needs an additive versioned view.

## Decisions

### Decision: Preserve legacy drain by default

Add `--view readiness` to a new direct CLI drain-plan route and a strict `view` selector to `cairn_drain_plan`.
The existing MCP request without a view retains its receipt fields and hashes.
The readiness view has a new schema with shared per-change reports, declared dependency order, and selected next actions.
Keep legacy and new consumer fixtures until an explicit future compatibility change authorizes a default switch.

### Decision: Bind per-change evidence explicitly

The new view accepts a bounded input manifest keyed by exact active change IDs.
Each entry can select contract, results, subject identity, and repository-relative cross-repository summaries under existing readiness rules.
Nickel contracts reject duplicate or unknown changes, unsafe paths, partial completion inputs, and unknown fields.
Missing observations remain unavailable. The adapter does not discover evidence or infer subjects from task checkboxes.
A shared request budget bounds the complete multi-change observation, not only each individual readiness call.

### Decision: Reuse readiness and existing ordering

The shell gathers current per-change readiness through the existing adapter.
The core composes those observations with the existing dependency graph and declared ready-task order.
It does not recalculate gate policy or reinterpret completion validity.
Action selection respects dependency order, then existing task order, then stable readiness action order.
Unresolved causes remain unresolved. Suggested sync and archive actions remain previews without execute arguments.
A blocked dependency cannot become satisfied because another report has successful transport.

### Decision: Update consumers, not authority

Generated apply, verify, drain, and archive descriptors use readiness for diagnosis and exact artifact instruction routes for authoring.
They retain required tests, completion inputs, review evidence, and mutation preflight.
Guidance cannot run actions or make an unavailable observation authoritative.
The compact-entry-guide change later controls how much of this guidance a host loads.
This package owns both the mcp-drain delta and the generated-agent-workflows delta.

## Architecture and reuse

Reuse `derive_readiness`, existing observation budgets, dependency graphs, task-order admission, and generated workflow policy.
Keep filesystem reads and manifest decoding in the CLI shell.
Keep multi-change ordering and selection in pure functions over admitted facts.
No scheduler, new execution port, or cross-repository runtime dependency is necessary.

### Implementation decisions

- Nickel source `cairn-policy/workflow-readiness.ncl` exports the `policy` field to `cairn-policy/generated/workflow-readiness.json`. It declares the view names, the action-kind order, the preview kinds, the task prerequisite stages, the task instruction route, the collection bounds, and the non-claims.
- The readiness view schema is `cairn.drain-readiness.v1`. The manifest schema is `cairn.workflow-readiness-manifest.v1`. The manifest is a `changes` array of entries keyed by unique `change`; each entry may carry `contract`, `results`, `subject_hash`, and `cross_repo_evidence`.
- The direct route is `cairn drain-plan` with `--view legacy|readiness`, `--evidence-manifest FILE`, and the existing `--root`, `--policy`, and repeated `--cross-repo-evidence` options. The MCP tool accepts `view` and `evidence_manifest` with the same strictness.
- The MCP default (`view` absent) keeps its existing receipt fields and hashes. The direct CLI uses `command: "cairn drain-plan"`; the MCP receipt keeps `command: "mcp cairn_drain_plan"`.
- Manifest admission returns stable `workflow_readiness.manifest.*` diagnostics for unavailable, invalid, schema, bound, unknown-change, duplicate-change, partial-completion, unsafe-path, invalid-subject, and duplicate-evidence cases.
- The core selects a task action first for an eligible change, then readiness actions in the declared kind order. Ordering-blocked changes withhold task and preview selection.
- Generated workflow descriptors add the read `readiness` command class, the exact `cairn readiness CHANGE`, `cairn drain-plan --view readiness`, and `cairn workflow instructions ARTIFACT --change CHANGE` routes, and a shared advisory non-claim.

## Alternatives rejected

- Replacing legacy receipts in place breaks saved consumers and evidence hashes.
- Duplicating readiness decisions in drain perpetuates the original inconsistency.
- Automatically selecting missing evidence hides caller authority and scope.
- A generic agent-next execution command merges guidance with effects.

## Tests and evidence

Capture legacy drain, direct readiness, and original validator baselines.
Compare explicit spec-driven, assurance, outcome-machine, no-spec-delta, and compatibility-profile fixtures where admitted.
Cover valid completion, missing rows, stale subjects, failed probes, malformed declarations, dependency cycles, and unavailable observations.
Retain stage-local successes when other observations fail. Test shared observation limits across many changes.
Compare generated guidance routes with actual strict CLI/MCP schemas.
Measure complete selected-change loops with the common benchmark, including evidence selection and extra detail reads.

## Risks and non-claims

Legacy and new drain can disagree because the new view observes explicit completion that legacy drain does not admit.
Document that scoped difference instead of concealing it as transport noise.
Original execution validators remain authoritative. Guidance and drain reports do not prove task implementation or future mutation permission.
