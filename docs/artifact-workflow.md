# Artifact workflow inspection

Cairn exposes artifact readiness through two read-only commands. Nickel policy owns the artifact graph, templates, bounds, and non-claims.

These commands do not replace lifecycle gates. They report structural facts for one active change.

<!--
r[impl cairn.artifact_workflow_contract.docs]
r[verify cairn.artifact_workflow_contract.docs.boundary]
r[verify cairn.artifact_workflow_contract.boundary.nonclaim]
-->

## Inspect artifact states

Run this command:

```bash
cairn workflow status CHANGE --root PATH
```

Select a named policy schema when the policy contains more than one schema:

```bash
cairn workflow status CHANGE --root PATH --schema spec-driven
```

The command reads bounded artifact files under `cairn/changes/CHANGE/`. It does not write repository or local state.

The receipt includes these identity fields:

- `policy_hash` identifies the complete generated policy.
- `input_hash` identifies the change, selected schema, and bounded observations.
- `receipt_hash` identifies the complete command result.
- Each file row contains its path and BLAKE3 hash.

The `artifacts` array uses policy order after stable topological admission. Each row includes dependencies, missing dependencies, blockers, unlocks, and files.

The `next_actions` array contains ready artifact IDs. Policy order resolves ties when several artifacts are ready.

## Understand artifact states

Cairn uses four states:

- `done`: The artifact exists and passes bounded structural admission.
- `ready`: The artifact is absent, and each required artifact is complete.
- `blocked`: A dependency is incomplete, or a present artifact is invalid.
- `skipped`: The selected policy omits the artifact.

A skipped artifact satisfies a dependency only when policy permits that transition.

A `done` state does not mean that the artifact is true or approved. A `ready` state does not grant mutation authority.

## Get artifact instructions

Run this command:

```bash
cairn workflow instructions ARTIFACT --change CHANGE --root PATH
```

Use `--schema ID` to select a non-default schema.

The command returns one instruction envelope. The envelope includes:

- the exact output path or output directory;
- the policy-owned instruction text;
- structural constraints;
- declared dependencies;
- apply prerequisites;
- upstream paths and BLAKE3 hashes;
- the current artifact state and blockers;
- required non-claims.

The command can return instructions for a blocked artifact. Read the `state` and `blockers` fields before you author the artifact.

## Use the MCP tools

Cairn exposes equivalent read-only MCP tools:

```text
cairn_workflow_status
cairn_workflow_instructions
```

`cairn_workflow_status` requires `change`. It accepts optional `root`, `policy`, and `schema` fields.

`cairn_workflow_instructions` requires `change` and `artifact`. It accepts optional `root`, `policy`, and `schema` fields.

Both schemas set `additionalProperties` to `false`. Cairn rejects missing fields, unknown fields, and unknown selectors before artifact observation.

The CLI and MCP tools use the same pure status and instruction model. MCP framing adds transport metadata only.

## Policy ownership

`cairn-policy/default.ncl` contains the authored workflow schema. The checked JSON projection contains the runtime value.

The schema binds:

- ordered artifact IDs;
- safe output paths;
- path and structural-admission kinds;
- dependency IDs;
- lifecycle stages;
- instruction templates;
- constraints and apply prerequisites;
- explicit bounds;
- skip behavior;
- required non-claims.

Runtime commands do not evaluate Nickel. Policy loading rejects duplicate IDs, unknown dependencies, cycles, unsafe paths, invalid templates, and unsupported values.

## Authority boundaries

Workflow output proves only the selected policy graph and observed structural artifact facts.

Workflow output does not prove:

- artifact truth or semantic quality;
- review approval;
- implementation correctness;
- task completion;
- gate passage;
- safe mutation;
- evidence validity;
- release eligibility.

Run the proposal, design, and tasks gates as separate authority checks. Review sync and archive plans before execution.
