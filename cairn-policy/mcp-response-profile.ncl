# r[impl cairn.mcp_projection.profile]
# r[impl cairn.mcp_projection.status]
let Positive = std.contract.from_predicate (fun n => std.is_number n && n > 0 && std.number.floor n == n) in
let Profile = [| 'legacy, 'structured-summary |] in
let Contract = {
  schema | String,
  default_profile | Profile,
  admitted_profiles | Array Profile,
  max_summary_bytes | Positive,
  max_issue_count | Positive,
  fallback | String,
  readiness_states | Array String,
  implementation_outcomes | Array String,
  gate_verdicts | Array String,
} in
{
  schema = "cairn.mcp-response-profile.v1",
  default_profile = 'legacy,
  admitted_profiles = ['legacy, 'structured-summary],
  max_summary_bytes = 256,
  max_issue_count = 9999,
  fallback = "summary omitted (budget exceeded); complete detail in structuredContent",
  readiness_states = ["ready", "blocked", "unavailable", "not_applicable"],
  implementation_outcomes = ["ready", "blocked", "incomplete_limit", "completion_only", "empty"],
  gate_verdicts = ["PASS", "FAIL"],
} | Contract
