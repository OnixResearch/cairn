# Selected-work snapshots

`cairn selected-work` returns one versioned, typed observation of one explicit
change (and optionally one authored task) for external coordinators such as
Kiln and Lattice. It composes existing native owners: readiness owns stage
decisions, scoped guidance owns declared context, and workflow obligations own
obligation meaning, guidance, the completion predicate and its evaluation.
The snapshot adds identities and transport. It decides nothing new.

Schema: `cairn.selected-work.v1`. Rust DTO and admission:
`cairn_core::verified::selected_work` (`SelectedWorkSnapshot`,
`ExpectedCohort`, `compose`, `admit_portable`). Nickel contract:
`contracts/selected-work.ncl`. Bounds: `cairn-policy/selected-work.ncl`
exported to `cairn-policy/generated/selected-work.json`.

## Invocation

```sh
cairn selected-work CHANGE (--root PATH | --lifecycle-store ID) [--task ID] \
  [--policy PATH] [--contract FILE --results FILE --subject-hash HASH] \
  [--cross-repo-evidence FILE]... \
  [--obligation-definition ID [--obligation-observations FILE]] \
  [--disclosure local|portable] [--expected-cohort JSON] [--detail]
cairn --machine selected-work ...
cairn mcp --call cairn_selected_work '{"change":"demo","root":"fixtures/basic","obligation_definition":"verify"}'
```

- Exactly one of `--root` or `--lifecycle-store` is required.
- `--task` selects an authored `[task:ID]`; list positions and line numbers are
  never task identity. Without it the whole change is selected.
- `--contract`, `--results`, `--subject-hash` and `--cross-repo-evidence` are
  passed to native readiness and bound into the source manifest.
- `--obligation-definition` selects an admitted workflow obligation definition
  from the checked bundled catalog (`verify`, `verify.bounded-exception`).
- `--obligation-observations FILE` supplies explicit observations for that
  definition. The path is relative to the selected root, read under the
  shared budget and bound into the source manifest. It requires
  `--obligation-definition`.
- `--disclosure` defaults to `portable`. `--detail` requires
  `--disclosure local` and `--expected-cohort`.
- `--expected-cohort` takes the `cohort` object of an earlier response. A
  different current cohort is refused.

MCP arguments use the same names in snake case (`lifecycle_store`,
`subject_hash`, `cross_repo_evidence` as an array, `obligation_definition`,
`obligation_observations`, `expected_cohort` as an object, `detail` as a
boolean). CLI and MCP return the same JSON; MCP wraps it in
`structuredContent`. `--machine` wraps it in `cairn.cli-machine.v1`.

The plain CLI exits `0` with the snapshot on stdout. Any refusal exits `1`
with `error: ...` on stderr and nothing on stdout. Admission refusals name a
`SelectedWorkError` variant: `InvalidPolicy`, `UnsupportedSchema`,
`InvalidIdentity`, `InvalidSelection`, `Duplicate`, `InvalidSource`,
`Incomplete`, `BudgetExceeded`, `CrossedSubject`, `StaleCohort`,
`InvalidReadiness`, `InvalidContext`, `UnsafePortable` or
`InvalidObligations`. Use the machine envelope's `exit_class` for decisions,
not message text.

## Wire record

Every struct denies unknown fields. All digests are lowercase 64-hex BLAKE3.

| field | meaning |
| --- | --- |
| `schema`, `producer` | `cairn.selected-work.v1`, `cairn` |
| `selection` | `{change, task}`; `task` is an authored ID or `null` |
| `cohort` | the identities below |
| `source` | projection revision and the sorted manifest of selected artifact bytes (`present`, `missing`, `unavailable`) |
| `policy_hash`, `profile`, `profile_hash` | native workflow policy and profile identities |
| `dependencies` | task and change prerequisite facts with `ready`, `blocked`, `unavailable` or `unsupported` state |
| `completion` | native completion stage state and contract/results/subject hashes |
| `context` | portable declared context entries (no raw text) |
| `readiness` | native readiness receipt identities, stage decision hashes, blockers and advisory actions |
| `units` | implementation units with typed `requirement` (`r[...]`) and `acceptance` (`a[...]`) references scoped to the change or a task |
| `remaining` | `implementation`, `completion`, `sync`, `archive` work; never omitted when unresolved |
| `enumeration` | complete flag, observed/total counts and the aggregate byte budget |
| `semantic_extensions` | `dependency_meanings: unsupported`; `obligation_explanations: supported` or `not_selected` |
| `obligations` | `{"state":"not_selected"}` or `{"state":"exported","export":{...}}` |
| `local` | `null` in portable output; local detail only |

### Cohort identities

Each value is the canonical-JSON BLAKE3 of a domain-tagged array:

| cohort field | changes when |
| --- | --- |
| `source` | any selected artifact byte, path or status changes |
| `intent` | source, policy, profile, dependencies, completion inputs, units or remaining work change (units, dependencies and remaining work hash in canonical order) |
| `context` | declared context selection or entries change |
| `verification_subject` | completion subject, contract, results or native readiness identities change |
| `dependencies` | the selection, selected unit task IDs, a prerequisite edge or a prerequisite's observed state changes; prose and input ordering do not affect it |
| `obligations` | the obligation section changes, including any evaluation change |
| `selection` | the selection or enumeration changes |
| `observation` | any identity above, portable readiness or semantic extensions change |

`verification_subject` and `context` never depend on the obligation section.

## Obligation section

When a definition is selected, `obligations.export` contains:

| field | meaning |
| --- | --- |
| `binding` | `ObligationBinding`: workflow ID/version, definition ID, intent, workflow policy and catalog identities, `binding_blake3` |
| `predicate` | `cairn.workflow-obligation-predicate.v1`, bound to `subject_blake3 = cohort.verification_subject`, `context_blake3 = cohort.context` and the selected alternatives; `predicate_blake3` is recomputable |
| `guidance` | structured, presentation-free guidance (`requirements_blake3`, sections) |
| `nodes` | typed facts, `cairn.workflow-obligation-node.v1`, below |
| `action_preconditions`, `expected_observations` | typed definition entries |
| `observations_supplied` | whether an observations file was supplied |
| `evaluation` | `cairn.workflow-obligation-report.v1` for exactly this predicate |

`cairn obligation export` with `subject_blake3` and `context_blake3` set to
the snapshot's `verification_subject` and `context` returns the same
predicate and obligation IDs.

### Obligation node facts

Nodes are a pre-order walk of the completion expression followed by exception
paths. Each node has:

- `id`, `kind` (`evidence`, `all_of`, `any_of`, `applicable`, `temporal`,
  `exception`), `parent`, `alternative`, `children`, `alternatives`;
- `selection`: the most restrictive state on its path, ordered
  `required < selected_alternative < conditional < selection_missing <
  unselected_alternative`. Only the first three can contribute to completion;
  `selection_missing` means the plan is incomplete;
- `conditions`: ancestor `applicable` node IDs;
- `evidence`: exact native predicate references (`predicate`, `kind`
  `evidence` or `applicability`, `requirement_id`, `owner`,
  `accepted_revision_blake3`, `reference_blake3`);
- `expected_observations` and `command_classes` naming the same predicates;
- `temporal_check` and `exception` (`replaces`, native `exception_id`,
  `request_revision`, `policy_identity`, `scope_rule_id`, `control`);
- `obligation_blake3`: the node's meaning under the binding (selection
  independent);
- `guidance_blake3`: that meaning under the structured guidance and selection;
- `condition_blake3`: the remaining discharge condition under the predicate
  (subject, context, selection and evidence revisions).

`verify_obligation_node_facts(predicate, guidance, nodes)` recomputes the
structure and every identity.

### Evaluation

`evaluation.outcome` is `proved`, `exception_accepted`, `unsatisfied`,
`blocked`, `abandoned`, `unknown` or `unsupported`. `proved` and
`successful_delivery` are separate booleans; `exception_accepted` delivers
without proving the original condition. `satisfied` and `remaining` list node
IDs (and unready action preconditions). Missing observations stay `unknown`;
activity counts, checked task boxes and success statements are recorded in
`ignored_claims` only.

Admission refuses an exported section whose predicate or evaluation names
another subject, context or workflow policy (`CrossedSubject`), and refuses
(`InvalidObligations`) stale facts, a successful outcome with a remaining node,
an unsuccessful non-terminal outcome without a remaining node, and a successful
outcome without supplied observations.

### Observations file

```json
{
  "selected_alternatives": {"verify.traceability": "receipt-chain"},
  "evidence": [{
    "predicate": "agent-receipt.freshness",
    "requirement_id": "cairn.supersede.agent_run_receipts.freshness",
    "revision_blake3": "<accepted revision from the node fact>",
    "subject_blake3": "<cohort.verification_subject>",
    "context_blake3": "<cohort.context>",
    "policy_blake3": "<policy_hash>",
    "observation_id": "check:freshness",
    "state": "satisfied"
  }],
  "progress": [], "agent_claims": null, "exceptions": [],
  "temporal": null, "terminal": null
}
```

All fields are optional. Evidence for another subject, context, policy or
revision stays out of scope and does not satisfy anything.

## Consumer use

1. Request a snapshot, retain its `cohort` as the expected cohort, and admit
   the portable record with `admit_portable(policy, snapshot, expected)` or an
   equivalent independent check.
2. Treat `obligations.state = not_selected`, `selection_missing` nodes,
   `unknown`/`unsupported` outcomes, `enumeration.complete = false` and
   unavailable stages as incomplete, never as an empty satisfied set.
3. Map node facts into consumer vocabulary without parsing guidance prose. For
   a Lattice workflow intent, each `required`, `selected_alternative` or
   `conditional` leaf supplies the obligation ID and its `obligation_blake3`,
   `guidance_blake3` and `condition_blake3`; its `evidence` references supply
   required observations. Cohort fields map directly: `selection_identity <-
   cohort.selection`, `obligations_identity <- cohort.obligations`,
   `context_identity <- cohort.context`, `source_identity <- cohort.source`,
   `dependency_identity <- cohort.dependencies`, `policy_identity <-
   policy_hash`, `profile_identity <- profile_hash`. The consumer chooses its
   own operation classes and `b3:` prefixing.
4. Re-request with `--expected-cohort` before a detail read or continuation. A
   changed source, intent, context, subject, policy or evaluation produces a
   new cohort and requires the consumer's own refresh or replan.

## Non-claims

The snapshot reports selected native observations and Cairn's obligation
evaluation over supplied evidence. It does not prove artifact truth, code
correctness, completion, archive readiness or release eligibility. Advisory
actions, guidance prose and successful transport grant no execution, approval,
sync, archive or release authority. Kiln owns work, admission and attempt
identities; Lattice executes admitted work. Dependency meanings (#7) are
`unsupported`. No live Kiln campaign or Lattice execution qualified this
contract; its conformance rests on the core, consumer and CLI/MCP fixtures.
