# Checked standalone policy for the bounded selected-work portable contract.
let selected = import "../contracts/selected-work.ncl" in
{
  schema | std.contract.from_predicate (fun value => value == "cairn.selected-work.v1") = selected.schema,
  max_artifacts | std.contract.from_predicate (fun value => value > 0 && value <= 512) = selected.max_artifacts,
  max_units | std.contract.from_predicate (fun value => value > 0 && value <= 512) = selected.max_units,
  max_references | std.contract.from_predicate (fun value => value > 0 && value <= 4096) = selected.max_references,
  max_context_entries | std.contract.from_predicate (fun value => value > 0 && value <= 512) = selected.max_context_entries,
  max_observation_bytes | std.contract.from_predicate (fun value => value > 0 && value <= 67108864) = selected.max_observation_bytes,
}
