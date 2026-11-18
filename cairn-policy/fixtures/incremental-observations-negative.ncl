let target = import "../incremental-observations.ncl" in
{ schema = "cairn.incremental-observations-policy.v1",
  ordering_version = "lexicographic-identity-byte-order.v1",
  bounds = {
    max_entry_bytes = 524288,
    default_page_entries = 0,
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
  required_non_claims = [],
} | target.contracts.Policy
