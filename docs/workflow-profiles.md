# Workflow profiles

Cairn policy defines the lifecycle workflow profiles. A change selects one profile during creation.

```text
cairn change create NAME --profile PROFILE --root PATH
```

The command writes the profile ID and version to `metadata.json`. Cairn does not read a profile from user-global files, environment variables, agent prompts, or remote packages.

An old change without profile metadata uses `spec-driven`. Cairn does not write fallback metadata during read-only inspection.

## Profiles

### `spec-driven`

This profile keeps the existing lifecycle:

1. Write `proposal.md`.
2. Write one or more `specs/<id>/spec.md` delta files.
3. Write `design.md`.
4. Write `tasks.md`. Link each task to an `r[...]` requirement.
5. Run the proposal, design, and tasks gates.
6. Sync the delta specs, then archive the change.

### `assurance`

This profile includes all `spec-driven` artifacts. It also requires:

- `risk.md`
- `verification.md`
- `reviews/assurance.json`

The review receipt uses `cairn.semantic-review.review-receipt.v1`. It must approve the current proposal, delta specs, design, risk, tasks, and verification files.

Update every `reviewed_inputs` BLAKE3 hash after an input changes. A missing or stale review blocks the tasks gate, sync, and archive.

### `outcome-machine`

<!-- r[impl cairn.outcome_machine_profile.docs] -->

This profile includes all `spec-driven` artifacts. It also requires `outcome-machine.json` before design work.

Select one route:

- `delivery` connects an immediate outcome and current consumer to a durable capability contribution.
- `foundation` connects a current consumer and target outcome to an adoption path.
- `bounded_exception` records urgent work, its reason, recurrence decision, revisit trigger, and capability non-claim.

Every route names a maintenance owner, tracked inputs, deterministic checks, AI-state facts, fresh-state disposition, completion state, and non-claims.

The scaffold starts with `planned` completion and fresh-state states. Replace every `CAIRN_PLACEHOLDER` before the design gate.

Planning gates can pass while completion remains planned. Sync, archive, validation, and drain readiness remain blocked until completion has BLAKE3-bound observed evidence.

Fresh-state reproduction must become `observed` with BLAKE3-bound evidence or `not_applicable` with a bounded rationale. It cannot depend on hidden session state.

Each evidence reference contains a supported kind, safe change-local path, and 64-character BLAKE3 identity:

```json
{
  "kind": "test",
  "path": "evidence/focused-test.json",
  "blake3": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
}
```

Supported kinds are `test`, `proof`, `receipt`, `replay`, `observation`, and `review`.

Cairn reads each referenced file through the change directory capability. A missing, unsafe, oversized, or changed file blocks completion.

At completion, set `completion.status` to `observed`. Then add at least one evidence reference.

Resolve the fresh-state disposition in the same update.

Completion authority must remain `repository`. Commit counts, task counts, archive counts, and generated-code volume cannot establish completion.

Cairn validates structure, bounds, route consistency, and evidence-reference presence. Repository checks and reviewers retain authority over value, truth, and release readiness.

### `no-spec-delta`

This profile creates no `specs/` directory. It requires:

- `proposal.md`
- `design.md`
- `acceptance.md`
- `tasks.md`
- `reviews/no-spec-classification.json`

Set these metadata fields:

```json
{
  "spec_effect": "none",
  "spec_effect_rationale": "Explain the bounded reason."
}
```

Define stable local criteria in `acceptance.md`:

```text
a[change.local-criterion] State the local acceptance criterion.
```

Every checkbox task must reference a known `a[...]` criterion. These IDs stay in the archived change. They are not accepted `r[...]` requirements.

The classification review must approve the current proposal, design, acceptance, and tasks files. A delta-spec file, missing rationale, malformed criterion, unlinked task, or stale review blocks progress.

A valid sync is an explicit no-op. Its receipt has `spec_effect = "none"`, no accepted-spec write action, the acceptance IDs, and the review receipt hash.

Archive reloads the profile, artifacts, tasks, and review. It does not move the change if any identity changed after preflight.

## Inspection

Use these read-only commands:

```text
cairn workflow status CHANGE --root PATH
cairn workflow instructions ARTIFACT --change CHANGE --root PATH
```

The output reports the selected profile, version, source, effect, artifact graph, acceptance IDs, review hashes, and non-claims. The CLI and MCP tools use the same checked policy projection.

The `--schema` workflow option is an assertion only. It cannot replace the profile in change metadata.

## Claim boundary

Profile conformance proves only that Cairn applied one admitted workflow contract to observed structural facts and review links.

A no-spec profile does not prove zero behavior effect. An assurance profile does not prove risk-analysis truth, test sufficiency, code correctness, or release eligibility.

An outcome-machine profile does not prove product value, evidence truth, semantic correctness, reuse quality, maintenance quality, or release eligibility.

Domain checks and reviewers keep their authority.
