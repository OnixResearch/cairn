# Workflow obligation design

## Ownership and API

`cairn-policy/workflow-obligations.ncl` owns the admitted obligation vocabulary,
native predicate references, observation model, the `verify` pilot and the
bounded `verify.bounded-exception` example, plus bounds and non-claims. Its entire
standalone root exports to
`cairn-policy/generated/workflow-obligations.json`; the CLI embeds that checked
artifact and never evaluates Nickel at runtime. The optional policy is separate
from the legacy default policy, so no existing policy, field-set digest or
receipt hash material changes.

Rust owns the versioned wire model, admission and pure evaluation in
`cairn_core::workflow_obligations`, re-exported at the crate root. Public entry
points:

- `admit_workflow_obligation_policy(policy, catalog, workflow_policy_blake3)`
- `find_workflow_obligation(catalog, workflow_id)`
- `obligation_predicate(admitted, policy, selection, subject_blake3, context_blake3)`
- `obligation_guidance(admitted, policy, selection)`
- `render_guidance(guidance, presentation)`
- `presentation_for_adapter(format)`
- `export_workflow_obligation(admitted, policy, selection, subject, context)`
- `evaluate_workflow_obligations(admitted, policy, request)`
- `render_with_workflow_obligations(catalog, obligations, adapter_id)`

Errors are `Unsupported`, `ScopeMismatch`, `InvalidInput`, `BoundExhaustion` or
`MissingAlternative`, serialized as tagged explicit outcomes. Admission returns
stable `WorkflowObligationIssue` diagnostics. No error becomes an empty
satisfied obligation set, silent pass or default policy. Deserialization denies
unknown fields, so an unborn schema, node kind or predicate kind is rejected
rather than ignored.

The CLI exposes `cairn obligation list|explain|export` with bounded reads. It
composes the current generated workflow catalog and checked obligation policy;
`explain` and `export` take an explicit case. `list` needs no case. None of the
commands mutate lifecycle state, start clocks or execute actions.

## Binding

A definition binds `workflow_id`, `workflow_version`, a 64-character
`intent_blake3`, and the admitted catalog descriptor. Admission computes an
`ObligationBinding` over definition identity, intent identity, the current
generated workflow policy hash and the admitted catalog identity. The
evaluation request must present that exact binding; the predicate additionally
binds the supplied `subject_blake3`, `context_blake3`, the selected
alternatives and the exact native predicate revisions. A different subject,
context, policy, catalog or evidence revision therefore cannot reuse the same
predicate identity.

Native predicate references pin a requirement ID, owner and exact accepted
revision digest. Predicates are `evidence` or `applicability`; evidence nodes
may reference only evidence predicates, and conditional nodes only
applicability predicates. The five pilot predicates are the archived #5 native
rows:

| predicate | native requirement | revision source |
| --- | --- | --- |
| `agent-receipt.verification-contract` | `cairn.supersede.agent_run_receipts.verification_contract` | #5 accepted row |
| `agent-receipt.pass-only` | `cairn.supersede.agent_run_receipts.pass_only` | #5 accepted row |
| `agent-receipt.freshness` | `cairn.supersede.agent_run_receipts.freshness` | #5 accepted row |
| `agent-receipt.completion-authority` | `cairn.supersede.agent_run_receipts.completion_authority` | #5 accepted row |
| `verification-obligation.claim-revision` | `cairn.verification_obligation_cob.claim_revision` | #5 accepted row |

Two additional references are policy-scoped: `exception.sandbox-revalidation`
for the compensating control and `verification-scope.applicable` for
conditional applicability. Neither duplicates a native evaluator.

## Categories kept separate

- `action_preconditions` gate an individual command class and reference
  predicates; they are reported and never counted as completion.
- `expected_observations` name the command class and predicate the agent should
  collect.
- `progress_facts` record activity. `ProgressObservation` is echoed with
  `effect: none_on_completion`.
- `completion` is the only expression that decides delivery.

An agent success statement, checked task boxes or activity counts are recorded
in `ignored_claims` and never enter evaluation. A terminal observation that is
`blocked` or `abandoned` forces a non-successful outcome even when evidence is
complete.

## Finite semantics

`ObligationExpr` is exactly `evidence`, `all_of`, `any_of`, `applicable` or
`temporal`. `all_of` combines children with unsatisfied > unsupported > unknown
> exception-accepted > satisfied. `any_of` requires exactly one declared
selected alternative per node; only that branch is evaluated, so partial
unselected branches cannot combine. A missing or unknown selection is
`unknown`, never an empty satisfied set. `applicable` requires its declared
applicability observation: true evaluates the branch, false reports
`not_applicable`, unknown stays `unknown`.

Every expression node has a stable ID, validated for uniqueness and depth.
Explanations list nodes in deterministic pre-order with state, detail, evidence
observation identities, children and any exception record. `satisfied` and
`remaining` are derived from the same node states. The overall outcome is
`proved`, `exception_accepted`, `unsatisfied`, `blocked`, `abandoned`,
`unknown` or `unsupported`; `proved` and `successful_delivery` are distinct
booleans.

## Temporal extension

`TemporalExtension` declares two distinct named events, a trace ID, an observer,
a clock basis, a required completeness and a finite `max_span_epochs`. The
supplied `TemporalObservation` must match trace, observer, clock, completeness
and the exact span (`horizon - window_start`), and events must come from the
declared observer. Missing events in a declared complete interval are
`unsatisfied`; partial intervals, unknown observers/clocks, out-of-horizon
events and absent observations remain `unknown`. A finite ordering result is
explicitly not evidence of unbounded safety or liveness; the explanation and
non-claims state that boundary.

## Policy exceptions

`ExceptionPath` binds one completion node to a native exception ID, request
revision, policy identity, scope rule and one `CompensatingControl` with an
evidence predicate. Evaluation reuses the native `PolicyExceptionState` and
`classify_applicability`:

- identity, request revision, policy identity, scope rule and declared control
  must all match;
- native applicability must be current; revoked, expired, unapproved or
  missing observations invalidate the path;
- the supplied validity epoch must be before the latest finite validity bound;
- the control predicate observation must be satisfied and in scope.

Only an `unsatisfied` original node can be replaced. An `unknown` node is never
replaced. Acceptance produces `exception_accepted` with `proved: false`,
retains `original: {node, state: unsatisfied, basis}` and reports applicability
and control state. The original condition is never relabelled as proved.

## Presentation invariance and legacy compatibility

`ObligationGuidance` carries structured sections derived from the admitted
definition and the resolved requirements hash. `Presentation` holds only
format, heading level and bullet character; its validator rejects any other
shape. `render_guidance` emits every section item unchanged. The machine
predicate and the guidance are built from the same admitted definition, so a
presentation edit cannot remove evidence, change command classes or add
authority.

`render_with_workflow_obligations` renders the legacy catalog first. A workflow
without an admitted definition keeps its exact legacy bytes, BLAKE3 and
descriptor identity. A converted workflow appends guidance and rebinds the
descriptor identity to the obligation binding so a plan cannot mix text from a
different definition. Existing `agent workflow` commands and defaults are
unchanged.

## Export for #14 and Kiln

`WorkflowObligationExport` carries the binding, predicate, obligation summaries
(kind and detail), alternatives, non-claims and boundary. The CLI exports the
same IDs and meanings used by evaluation and guidance for the same admitted
snapshot. Missing or unsupported observations cannot turn into an empty
satisfied set; they stay `remaining`. No transport, composition or execution
authority is added here.
