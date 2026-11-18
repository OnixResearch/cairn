# Standalone policy for source-bound narrow lifecycle reads. Export with:
# nickel export --format json --field policy cairn-policy/targeted-context.ncl > cairn-policy/generated/targeted-context.json
# The checked-in JSON is a generated runtime projection, not an authority source.
let Positive = std.contract.from_predicate (fun n => std.is_number n && n > 0) in
let Bounds = {
  max_artifact_bytes | Positive,
  max_selected_bytes | Positive,
  max_selector_bytes | Positive,
  max_heading_depth | Positive,
  max_diagnostics | Positive,
  max_references | Positive,
} in
let Policy = {
  schema | std.contract.from_predicate (fun s => s == "cairn.targeted-context-policy.v1"),
  bounds | Bounds,
  required_non_claims | Array String,
} in
{
  contracts = { Policy = Policy },
  policy = {
    schema = "cairn.targeted-context-policy.v1",
    bounds = {
      max_artifact_bytes = 1048576,
      max_selected_bytes = 65536,
      max_selector_bytes = 512,
      max_heading_depth = 6,
      max_diagnostics = 64,
      max_references = 64,
    },
    required_non_claims = [
      "a selected block does not establish source truth, semantic sufficiency, requirement satisfaction, or lifecycle readiness",
      "references and policy constraints are prerequisites, not automatically expanded content or implementation evidence",
      "selected content and document validity are independent; omissions require explicit follow-up",
    ],
  } | Policy,
}
