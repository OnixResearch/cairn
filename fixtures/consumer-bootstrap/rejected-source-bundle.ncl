let contracts = import "../../contracts/consumer-bootstrap.ncl" in
{
  schema = "cairn.bootstrap-source-bundle.v1",
  cohort = import "./cohort.ncl",
  export_request = "export-request.json",
  export_manifest = "export-manifest.json",
  workflow_source = "workflow.json",
  policy_locator = ".cairn/bootstrap/policy.json",
  artifacts = [
    { source = "policy.json", destination = policy_locator },
    { source = "other.json", destination = policy_locator },
  ],
} | contracts.source_bundle
