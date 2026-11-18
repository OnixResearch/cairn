# Entry guidance acceptance state

Status: **guidance behaviour verified; two inherited drain assertions outstanding.**

## Executed suites

| Suite | Result |
|---|---|
| `generated_agent_workflows` | 16/16 pass |
| `agent_guidance` | 7/7 pass |
| `readiness_completion` | 1/1 pass |
| `readiness` | 6/6 pass |
| `readiness_drain` | 4/6 pass |

`cairn policy export --check` is fresh, `cairn validate --root .` reports 0
issues, and `cairn traceability coverage --root .` is 2382/2382.

## Work required to make this change pass

Merging the archived readiness-adoption branch brought
`readiness_guidance_routes_match_the_current_cli_and_mcp_schemas`, which had never
been executed. Satisfying it required real implementation, not assertion changes:

1. A `readiness` read-only command class had to exist in
   `generated_agent_workflow_policy.command_classes`; without it, admission
   rejects the descriptor with `unknown command class: readiness`.
2. The apply, verify, drain and archive descriptors gained the `readiness` class,
   a readiness entry in `required_evidence`, the exact readiness and
   workflow-instruction routes in `immediate_action`, and a shared non-claim that
   a descriptor requests readiness as an input route without promoting an
   unavailable observation to authority.
3. `verify` also declares `Declared completion inputs and subject identity` in
   `required_evidence`, as its scenario requires.
4. Because the main policy changed, `fixtures/agent-completion/{contract,results}.json`
   were refreshed from the previous `policy_hash` to the new identity. Without
   that, authoritative completion reports `verification_contract.policy_drift`
   and the `completion` stage blocks.

A merged `@both` resolution initially produced an unclosed test function; the
displaced function now closes with `Ok(())` before the following `#[test]`.

## Outstanding

`readiness_drain` fails `readiness_view_orders_dependencies_and_withholds_blocked_work`
and `readiness_view_selects_the_declared_ready_task_route` at
`readiness_drain.rs:195` (`task_ordering.valid` is `false`, expected `true`).

A direct probe reproduces it outside the test harness. With the same fixture
(`z-prereq`, `a-dependent`, valid `[serial] [task:build]`/`[after:build]`
markers, both changes present) the projection returns:

```json
{"active":false,"available":false,"order":[],"ready_tasks":[],"valid":false}
```

`available: false` means the per-change `tasks.md` read was refused before the
markers were parsed, so this is not a marker or requirement-reference problem. The
ordering observer runs inside the shared multi-change observation budget, and it
is starved once earlier reads for the same request have consumed it.

That is a real defect in the drain composition: per-change ordering availability
must not depend on unrelated earlier reads in the same aggregate request. It must
be fixed in `crates/cairn-cli/src/drain_plan.rs` (budget provisioning or read
ordering) before this change is archived. The two tests are inherited from the
archived readiness-adoption change rather than authored here.
