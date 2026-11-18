let target = import "../targeted-context.ncl" in
{ schema = "cairn.targeted-context-policy.v1",
  bounds = {
    max_artifact_bytes = 1048576,
    max_selected_bytes = 0,
    max_selector_bytes = 512,
    max_heading_depth = 6,
    max_diagnostics = 64,
    max_references = 64,
  },
  required_non_claims = [],
} | target.contracts.Policy
