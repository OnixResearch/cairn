# Design: Artifact workflow contract

## Context

`cairn-policy/default.ncl` declares the current lifecycle schema and artifact dependencies. The runtime consumes its checked JSON projection.

`cairn status` and change context report proposal, design, tasks, delta-spec presence, and task counts. They do not expose artifact readiness, dependency blockers, unlocks, or policy-owned instructions.

Agents can reconstruct this information from documentation, but that reconstruction is not a stable product contract.

## Goals

- Expose the selected policy workflow as deterministic artifact states.
- Give operators and agents exact next actions without changing lifecycle authority.
- Reuse one pure model for CLI and MCP output.
- Preserve existing status and receipt compatibility.

## Non-Goals

- Generate artifact prose or call a model.
- Infer semantic correctness from file presence.
- Let workflow readiness satisfy a gate.
- Add mutation through workflow commands.
- Add project context or tool-specific workflow installation.

## Decisions

### Decision: Treat generated policy as the workflow authority

**Choice:** The generated policy projection will define each workflow schema, ordered artifact, output path pattern, dependency IDs, stage relationship, instruction template, and apply prerequisites.

The pure core will reject duplicate artifact IDs, unknown dependencies, cycles, unsafe output paths, invalid templates, and unsupported state values. Runtime commands will not evaluate Nickel.

**Rationale:** Nickel remains the authored authority while Rust receives one bounded checked value.

### Decision: Derive four artifact states from supplied facts

**Choice:** The pure core will derive these states:

- `done`: the artifact exists and passes its bounded structural admission.
- `ready`: the artifact is absent and all dependencies are `done` or `skipped`.
- `blocked`: a dependency is not complete, the present artifact is invalid, or workflow admission failed.
- `skipped`: the selected policy profile explicitly omits the artifact.

A present invalid artifact remains `blocked` and carries stable diagnostic codes. It does not unlock dependent artifacts.

The result will include sorted `requires`, `missing`, `unlocks`, and `next_actions` fields. Policy order breaks ties between several ready artifacts.

**Rationale:** File presence alone is too weak. The model remains structural and does not judge prose meaning.

### Decision: Add read-only status and instruction commands

**Choice:** Add these commands:

```text
cairn workflow status CHANGE --root PATH
cairn workflow instructions ARTIFACT --change CHANGE --root PATH
```

Status will emit the selected schema identity, artifact rows, blockers, next actions, policy identity, input identity, and receipt identity.

Instructions will emit one selected artifact ID, exact output path, policy template, dependencies, upstream paths and BLAKE3 identities, structural constraints, and required non-claims.

Unknown changes, schemas, or artifacts fail with stable diagnostics. Both commands remain read-only.

**Rationale:** Separate status and instruction surfaces keep broad discovery distinct from one artifact authoring contract.

### Decision: Give MCP the same semantic results

**Choice:** Add strict read-only `cairn_workflow_status` and `cairn_workflow_instructions` tools. Their argument schemas will reject unknown fields.

CLI and MCP adapters will serialize one core result. MCP protocol envelopes remain unchanged.

**Rationale:** Agents receive the same lifecycle facts through either supported transport.

### Decision: Keep gates and mutation contracts separate

**Choice:** Workflow state will never satisfy or replace a gate. A `ready` artifact means only that its declared predecessors are complete enough for authoring.

Sync and archive will continue to run their existing gate and mutation preflight logic. Workflow instructions will not emit an approval, correctness, or release claim.

**Rationale:** Artifact sequencing is guidance, not lifecycle authority.

### Decision: Preserve the functional-core boundary

**Choice:** `cairn-core` will own graph admission, state derivation, next-action selection, and instruction construction over in-memory values.

`cairn-cli` will own policy loading, file observation, BLAKE3 file hashing, root selection, JSON output, MCP transport, and exit status.

**Rationale:** All workflow decisions remain testable without files, environments, processes, clocks, networks, or output effects.

## Rejected Alternatives

### Replace `cairn status`

This would change an existing compatibility surface. The change adds a separate workflow command family.

### Mark every present artifact as done

This would let malformed artifacts unlock later work. The selected structural admission must pass first.

### Put agent prose in Rust

This would split workflow authority between code and policy. Nickel owns templates and constraints.

### Let workflow commands edit artifacts

This would mix inspection with mutation. Artifact edits remain explicit external actions.

## Test Design

Positive tests cover valid graph admission, every state, several ready artifacts, deterministic ordering, exact instruction selection, CLI and MCP parity, and repeated byte-stable output.

Negative tests cover duplicate IDs, cycles, unknown dependencies, unsafe paths, invalid templates, unknown changes and artifacts, malformed present artifacts, unknown MCP fields, and attempted mutation assumptions.

Compatibility tests confirm that existing status, gate, sync, archive, and MCP receipts do not change.

## Risks and Trade-offs

- Structural admission can look like semantic approval. Every result states its bounded meaning.
- Policy templates become a compatibility surface. They need explicit schema versions and golden coverage.
- Future profiles can create several ready artifacts. Stable policy ordering prevents agent-dependent choices.

## Claim Boundary

A valid workflow result proves only the selected policy graph, observed structural artifact facts, and deterministic state derivation.

It does not prove artifact truth, review quality, implementation correctness, task completion, gate passage, safe mutation, or release eligibility.

## Source Adaptation

The artifact graph and instruction interaction adapt the OpenSpec artifact workflow. Cairn retains native layout, Nickel authority, pure decisions, strict receipts, and explicit lifecycle gates.
