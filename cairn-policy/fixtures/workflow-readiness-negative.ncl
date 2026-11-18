# r[impl cairn.agent_workflow_readiness.compatibility]
let r = import "../workflow-readiness.ncl" in
{
  manifest | r.contracts.Manifest = {
    schema = "cairn.workflow-readiness-manifest.v1",
    changes = [
      {
        change = "demo",
        contract = "contract.json",
        results = "results.json",
        subject_hash = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
      },
      { change = "demo" },
    ],
  },
}
