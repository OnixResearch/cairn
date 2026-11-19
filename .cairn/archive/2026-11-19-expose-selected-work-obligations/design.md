# Selected-work snapshot and obligation export design

## Context

`implementation-ready` drains all active work and hashes projected facts rather
than admitted source bytes. Readiness owns stage-local validator decisions;
scoped guidance owns declared context; `workflow_obligations` owns obligation
meaning, predicate, guidance and evaluation. Kiln and Lattice must not infer
execution or completion authority from any of these projections.

## Decisions

### A checked Cairn cohort, not a scheduler

`cairn_core::verified::selected_work` defines the `cairn.selected-work.v1`
DTO (every struct denies unknown fields), pure `compose` and independent
`admit_portable`. The snapshot binds distinct identities:

| cohort field | preimage |
| --- | --- |
| `source` | selected source-byte manifest and its projection revision |
| `intent` | source, policy, profile, dependencies, completion inputs, units, remaining work |
| `context` | portable declared context selection and entries |
| `verification_subject` | completion subject, contract and results hashes plus native readiness identities |
| `dependencies` | selection, sorted selected unit task IDs and sorted `(task, prerequisite, state)` edges; fact identities (which embed prose) are excluded |
| `obligations` | the whole obligation section (`not_selected` or `exported`) |
| `selection` | change/task selection and enumeration |
| `observation` | schema, producer, every identity above, portable readiness and semantic extensions |

Each identity is the canonical-JSON BLAKE3 of a domain-tagged array.
Admission recomputes the cohort from content and refuses a crossed
verification subject before a stale expected cohort.

### Explicit bounded native observation

Exactly one `--root` or `--lifecycle-store` is required. The shell reads the
selected change and only its declared prerequisite closure under one aggregate
no-follow budget, observes twice and refuses drift between passes. Authored task
IDs are the only task identity; unknown or duplicate selectors refuse.

### Obligation export over the existing owner

The obligation module gains `obligation_node_facts` and
`verify_obligation_node_facts`. Facts are a pre-order walk of the admitted
completion expression followed by exception paths. Every fact carries:

- `obligation_blake3`: domain `cairn.workflow-obligation-node.v1` over the
  binding and the node meaning (kind, parent, alternative, children,
  conditions, native evidence references, expected observations, command
  classes, temporal check and exception reference). Selection is excluded.
- `guidance_blake3`: domain `cairn.workflow-obligation-node-guidance.v1` over
  the structured guidance binding, requirements identity, selection and the
  obligation identity.
- `condition_blake3`: domain `cairn.workflow-obligation-node-condition.v1`
  over the predicate identity, the obligation identity and selection state.

Selection state is the most restrictive state on the node path:
`required < selected_alternative < conditional < selection_missing <
unselected_alternative`. Only the first three can contribute to completion.

`selected_work::compose` takes an optional `ObligationSource` (admitted
catalog, definition ID, optional observations). It computes the snapshot's
verification subject and context first, then derives the predicate, guidance,
facts and evaluation from the same admitted definition with exactly those
identities. The catalog must be admitted against the snapshot's workflow
policy hash. Admission re-verifies the facts, the predicate identity, the
subject/context/policy binding and the evaluation's consistency: a successful
outcome cannot leave a node remaining, an unsuccessful non-terminal outcome
must name a remaining node, and an unobserved section cannot be successful.

### Two disclosure surfaces, one semantic composition

CLI and MCP share one shell producer. Portable output strips the local root,
raw context text and host paths; local detail requires an expected cohort.

## Risks / Trade-offs

- Full selected snapshots require two coherent passes and refuse under active
  source churn.
- Dependency meanings (#7) remain `unsupported`.
- The Nickel contract describes Nickel-authored fixtures; the Rust DTO is the
  decoding authority for JSON wire values.
- Lattice and Kiln must map typed facts into their own vocabularies; Cairn does
  not emit Lattice operation classes or Kiln work identities.
