# Standalone policy for stateless incremental observation. Export with:
# nickel export --format json --field policy cairn-policy/incremental-observations.ncl > cairn-policy/generated/incremental-observations.json
# The checked-in JSON is a generated runtime projection, not an authority source.
let Positive = std.contract.from_predicate (fun n => std.is_number n && n > 0 && std.number.floor n == n) in
let Bounds = {
  max_entry_bytes | Positive,
  default_page_entries | Positive,
  max_page_entries | Positive,
  default_page_bytes | Positive,
  max_page_bytes | Positive,
  max_observation_bytes | Positive,
  max_prior_bytes | Positive,
  max_cursor_bytes | Positive,
  max_context_selectors | Positive,
  max_entries | Positive,
  max_selector_bytes | Positive,
} in
let PolicyContract = {
  schema | std.contract.from_predicate (fun s => s == "cairn.incremental-observations-policy.v1"),
  ordering_version | std.contract.from_predicate (fun s => s == "lexicographic-identity-byte-order.v1"),
  bounds | Bounds,
  required_non_claims | Array String,
} in
{
  contracts = { Policy = PolicyContract },
  policy = {
    schema = "cairn.incremental-observations-policy.v1",
    ordering_version = "lexicographic-identity-byte-order.v1",
    bounds = {
      max_entry_bytes = 524288,
      default_page_entries = 32,
      max_page_entries = 512,
      default_page_bytes = 65536,
      max_page_bytes = 4194304,
      max_observation_bytes = 16777216,
      max_prior_bytes = 16777216,
      max_cursor_bytes = 4096,
      max_context_selectors = 64,
      max_entries = 65536,
      max_selector_bytes = 4096,
    },
    required_non_claims = [
      "an observation identity binds admitted inputs; it does not prove source truth, secrecy, authenticity, or future mutation authority",
      "repeated bounded reads do not establish an atomic filesystem snapshot",
      "a cached or saved observation does not replace fresh mutation preflight, evidence checks, or lifecycle gates",
      "page cursors are deterministic continuation data, not authentication tokens",
    ],
  } | PolicyContract,
}
