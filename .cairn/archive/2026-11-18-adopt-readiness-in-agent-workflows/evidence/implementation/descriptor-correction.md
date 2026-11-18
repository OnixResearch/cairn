# Correction: descriptor readiness routes

Recorded after archive, during integration of the dependent change.

`cairn.agent_workflow_readiness.guidance` requires the apply, verify, drain and
archive descriptors to request readiness and exact artifact instruction routes.
When this change was archived, the descriptors in
`cairn-policy/default.ncl` had not been updated, so the
`readiness_guidance_routes_match_the_current_cli_and_mcp_schemas` test failed once
it was actually executed.

Two fixes were required and are now implemented in the dependent
`keep-agent-guidance-current` change, which owns the same generated workflow
policy and merges this branch:

1. A `readiness` read-only command class was added to
   `generated_agent_workflow_policy.command_classes`. Without it, admission
   rejects any descriptor that references readiness
   (`generated_workflow.descriptor.invalid: unknown command class: readiness`).
2. The apply, verify, drain and archive descriptors gained the `readiness`
   command class, a readiness entry in `required_evidence`, the exact readiness
   and workflow-instruction routes in `immediate_action`, and a shared non-claim
   that a descriptor requests readiness as an input route without promoting an
   unavailable observation to authority.

This note corrects the archive record: the guidance requirement is satisfied by
that follow-on change, not by the descriptor state present at archive time. It is
not a claim that this change's archived tree passes that test on its own.
