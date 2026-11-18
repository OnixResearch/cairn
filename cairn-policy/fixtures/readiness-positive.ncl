let r = import "../readiness.ncl" in
{ snapshot | r.contracts.Snapshot = {
  scope = { change = "demo", policy_hash = "policy", source_hash = "source", evidence_hash = "evidence" },
  stages = [{ stage = 'completion, state = 'blocked, decision_hash = "decision" }],
}, action | r.contracts.Action = {
  id = "action", kind = 'evidence_refresh, causes = ["missing-result"], target = "check-id",
  prerequisites = ["current-subject"], authority = 'advisory,
  expected_observation = "current passing execution receipt", argv = [],
}, agent_view | r.contracts.View = {
  schema = "cairn.readiness-agent.v1",
  scope = { change = "demo", policy_hash = "policy", source_hash = "source", evidence_hash = "evidence" },
  valid = true, classification = 'blocked,
  stages = [{ stage = 'completion, state = 'blocked, decision_hash = "decision" }],
  mutation_authorized = false, readiness_policy_hash = "readiness-policy", view_policy_hash = "view-policy", input_hash = "input",
  full_receipt_hash = "full", receipt_hash = "projection", diagnostics = [],
  blockers = [], actions = [], blocker_count = 0, action_count = 0,
  omitted_blockers = 0, omitted_actions = 0, omitted_stage_decisions = 1, complete = false,
  detail_selectors = [{ kind = 'stage, id = "completion" }],
}, detail_request | r.contracts.DetailRequest = {
  kind = 'stage, id = "completion", expected_receipt = "full",
} }
