# Unified readiness design

## Context

`change next` uses the dependency graph. `workflow status` reports structural artifacts.
`mcp_drain_plan` combines stage gates, task state, dependencies, and mutation plans in the MCP shell.
Completion, saved probes, and machine transport expose additional distinctions.
The new report composes existing decisions instead of inventing another readiness policy.

## Goals and non-goals

The goal is one inspectable causal report for Cairn operators and agents.
Cairn lifecycle maintainers own the model. CLI maintainers own observation and transport adapters.
There is no task runner, scheduler, auto-repair path, hidden registry input, or new storage service.

## Decisions

### Semantic model

A `ReadinessSnapshot` contains the selected change, policy identity, loaded lifecycle facts, explicit evidence, and observation scope.
A pure verified module invokes existing validators over those supplied values.
It returns a `ReadinessReport` with separate artifact, gate, completion, sync, and archive states.
States distinguish ready, blocked, unavailable, and not-applicable observations according to a checked contract.
A structurally valid report can describe a blocked lifecycle.

Each stage references its original validator decision and exact input identities.
The model preserves every applicable blocker. It does not recompute evidence truth from display strings.
Malformed or contradictory observations produce diagnostics, not optimistic defaults.
Reports expose which checks were evaluated and which remained unavailable.

### Causal actions

A typed action identifies its cause, safe target, prerequisites, expected observation, authority class, and exact argument values where supported.
The initial action families are artifact repair, evidence refresh, dependency wait, diagnostic inspection, sync preview, and archive preview.
Remediation rules live in checked Nickel policy and deterministic core code.
Unknown causes produce an unresolved diagnostic, not invented shell text.

A stale review action identifies the changed artifact and review input binding.
A missing execution result names the required check and producer contract.
A dependency action identifies the upstream change or evidence owner rather than a generic retry instruction.
Multiple causes can share one action without losing their individual blocker identities.
Stable policy order and canonical keys determine presentation order.

### Shell and transport

Existing capability-relative loaders gather bounded facts. They do not run probes or producer commands.
The composition root supplies concrete loaders and translates their infrastructure errors into bounded observations.
External capability ports remain narrow. Internal validators stay ordinary functions.

The proposed `cairn readiness CHANGE` command and matching MCP tool share the same semantic core.
JSON is the canonical automation view. A concise human projection uses the same report.
The new surface uses explicit lifecycle classification alongside report validity.
`--machine` keeps its existing transport contract. Legacy command fields and receipt hashes do not change.

Every emitted action is advisory. Mutation still requires a separate existing command, explicit permission, and fresh preflight.
An old readiness report cannot authorize a later write.

### Stack reuse

Reuse Cairn workflow, dependency, gate, probe, completion, trace, and mutation-plan validators at the program base revision.
Preserve existing Valence roles and evidence links instead of inventing another evidence graph.
Kiln remains the CI owner. Lattice remains the workflow executor.
No new Choregraph or Campaign dependency is required for a projection over the current Cairn dependency model.
Future Site or Subwayrat views consume the same report and receive no decision authority.

## Risks and trade-offs

The existing drain planner contains shell-local composition. Move only deterministic domain meaning into the core.
A shared report can hide distinctions behind one green indicator. Separate stage states and required non-claims prevent that collapse.
Reading several inputs creates drift risk. Bind one bounded observation set and reject mismatched identities.
The report does not promise protection from arbitrary external filesystem mutation. Actual mutation commands must recheck their own inputs.

## Validation strategy

Establish workflow, gate, drain, machine, and completion baselines before core changes.
Use direct-value core tests for exact state composition and action derivation.
Compare original validators with the report for successful and blocked fixtures.
Include failed probes, incomplete tasks, invalid order, stale reviews, missing completion rows, dependency conflicts, and unavailable evidence.
Require exact CLI/MCP semantic parity and zero product effects during inspection.
Negative transport fixtures retain blocked lifecycle state under a valid machine envelope.
Run focused tests, Nickel fixtures, strict Clippy, Slotcar, existing traceability, Cairn gates, and full Nix checks.

## Migration and completion

Ship the new report as an additive command and MCP tool.
Adopt the common model in drain presentation only after compatibility fixtures pass.
Document how consumers interpret transport, report validity, and stage readiness separately.
Retain a bounded comparison of diagnostic calls needed for known repairs. Do not claim a speedup without observations.
Completion requires implementation evidence, not the presence of this planning package.
