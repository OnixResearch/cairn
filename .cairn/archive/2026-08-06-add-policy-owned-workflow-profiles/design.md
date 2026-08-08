# Design: Policy-owned workflow profiles

## Context

Cairn generated policy contains one `spec-driven` artifact schema. Gates and scaffolding assume proposal, delta specs, design, and tasks.

The artifact workflow contract will expose schema dependencies and states. It needs a safe source for more than one workflow shape.

Some changes need extra assurance artifacts. Other bounded internal changes can have no accepted-spec effect, but the current model encourages an artificial delta requirement.

## Goals

- Let reviewed Nickel policy define several lifecycle workflow profiles.
- Preserve existing changes through a compatibility default.
- Add stronger assurance obligations without custom shell logic.
- Represent no-spec work explicitly without treating it as no-review work.

## Non-Goals

- Add user-global or remotely downloaded profile precedence.
- Infer a change risk class from prose or source code.
- Let an agent select a weaker profile implicitly.
- Prove that no-spec work has no behavior effect.
- Change a profile after substantive authoring starts.

## Decisions

### Decision: Define complete profiles in typed Nickel policy

**Choice:** Each profile will bind an ID, version, ordered artifacts, dependency graph, stage mapping, apply prerequisites, task-link classes, review obligations, receipt fields, and non-claims.

Policy export and runtime admission will reject duplicate IDs, unknown artifacts, cycles, missing gate mappings, unsafe paths, unsupported task-link classes, and weaker apply prerequisites than the profile declares.

Runtime commands will use only the checked generated projection.

**Rationale:** A complete profile prevents hidden behavior from being spread across Rust branches and prompt files.

### Decision: Select a profile explicitly at change creation

**Choice:** Add this optional form:

```text
cairn change create NAME --profile PROFILE --root PATH
```

The selected profile ID and version will enter change metadata and scaffolding receipts. A missing profile field maps to `spec-driven` for existing changes.

Unknown profiles fail before file creation. Profile selection will be immutable after any substantive lifecycle artifact exists in the first slice.

**Rationale:** Explicit creation-time selection avoids ambient policy changes and mid-change reinterpretation.

### Decision: Register a compatibility, assurance, and no-spec profile

**Choice:** The initial registry will contain:

- `spec-driven`: the current proposal, delta-spec, design, and tasks behavior.
- `assurance`: the spec-driven artifacts plus `risk.md`, `verification.md`, and policy-selected review receipts.
- `no-spec-delta`: proposal, design, `acceptance.md`, tasks, and a current no-spec classification review, with no delta-spec files.

The first slice will not add a generic low-rigor profile. A later profile can omit artifacts only after policy defines its risk admission and archive obligations.

**Rationale:** The initial profiles preserve compatibility, add rigor, and solve the clear false-delta case without a broad bypass.

### Decision: Give no-spec work local acceptance identities

**Choice:** `acceptance.md` will contain stable change-local `a[...]` criteria. Tasks in `no-spec-delta` changes must reference those criteria.

Acceptance criteria remain local to the archived change. Sync never promotes them into accepted specifications, and traceability never treats them as accepted `r[...]` requirements.

A no-spec change must include `spec_effect = "none"`, a bounded rationale, and a current semantic review receipt under a policy-known no-spec classification profile. The review binds proposal, design, acceptance, tasks, and selected source observations.

**Rationale:** Local criteria provide task accountability without inventing durable product requirements.

### Decision: Fail closed on no-spec conflicts

**Choice:** A `no-spec-delta` change will fail if it contains any delta-spec file, omits acceptance criteria, has unlinked tasks, lacks a current classification review, or requests a spec-mutating sync action.

A valid sync will produce an explicit no-op plan with `spec_effect`, review identity, acceptance identities, and required non-claims. Archive will recheck that plan, gates, tasks, and review before mutation.

**Rationale:** A typed effect and explicit no-op are safer than a loose `skip_specs` flag.

### Decision: Make assurance obligations policy data

**Choice:** The `assurance` profile will require policy-defined risk and verification artifact templates, review profiles, and apply prerequisites.

The profile does not interpret risk prose or prove test sufficiency. It checks required artifacts, structural content, source identities, and review receipt linkage.

**Rationale:** Stronger workflow must add inspectable obligations without overstating their truth.

### Decision: Project policy can narrow profile availability

**Choice:** Repository policy can allow a subset of generated profiles and can require one profile for named effect classes. Callers cannot define or weaken profiles through user-global configuration or environment variables.

Receipts will report selected profile, source, version, and diagnostics.

**Rationale:** A repository owns its lifecycle rigor. Ambient user state does not.

### Decision: Preserve the functional-core boundary

**Choice:** `cairn-core` will own profile admission, selection, artifact obligations, task-link validation, no-spec conflict detection, and sync or archive plan semantics over supplied values.

`cairn-cli` will own policy loading, metadata and artifact reads, review receipt loading, file creation, hashing, JSON output, and mutation.

**Rationale:** Profile decisions remain deterministic and testable without external effects.

## Rejected Alternatives

### Add a boolean `skip_specs`

A boolean has no acceptance, review, task-link, sync, or archive contract. The change uses a complete profile and typed effect.

### Load profiles from user home directories

This creates ambient workflow authority and poor replay. Only generated policy and explicit repository policy can select profiles.

### Infer the profile from changed files

File classification is incomplete and can change during work. Selection remains explicit and reviewable.

### Add a universal `lite` profile now

Cairn cannot infer low risk from a label. A later policy can define one with explicit admission and non-claims.

## Test Design

Positive tests cover all initial profiles, existing metadata fallback, exact scaffolds, artifact workflow states, assurance prerequisites, no-spec acceptance links, no-op sync, archive, and repeated deterministic receipts.

Negative tests cover unknown profiles, cycles, missing stage mappings, ambient selection, profile drift, no-spec delta files, missing rationale, malformed acceptance IDs, unlinked tasks, stale review, spec mutation, and partial archive effects.

Compatibility tests replay existing changes and receipts under the `spec-driven` fallback.

## Risks and Trade-offs

- Profiles increase policy and fixture surface. A shared graph and gate model limits branching.
- No-spec declarations can be wrong. Required review records the decision but does not prove it.
- Immutable selection can require recreating an early change. It prevents silent obligation changes.

## Claim Boundary

A selected profile proves only that Cairn applied one admitted workflow contract to observed artifacts and evidence.

A valid no-spec profile proves the declaration, local acceptance linkage, review linkage, and absence of delta files. It does not prove zero behavior effect.

An assurance profile proves required artifact and receipt presence under policy. It does not prove risk analysis truth, test sufficiency, code correctness, or release eligibility.

## Source Adaptation

The profile registry and no-spec interaction adapt OpenSpec custom schemas and `skip_specs`. Cairn uses typed policy, explicit selection, stronger evidence, local acceptance identities, and fail-closed mutation.
