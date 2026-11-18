# r[impl cairn.agent_workflow_readiness.compatibility]
let r = import "../workflow-readiness.ncl" in
{
  complete | r.contracts.Selector = {
    change = "demo",
    contract = "contract.json",
    results = "results.json",
    subject_hash = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
    cross_repo_evidence = ["upstream.json"],
  },
  absent | r.contracts.Selector = { change = "other" },
  manifest | r.contracts.Manifest = {
    schema = "cairn.workflow-readiness-manifest.v1",
    changes = [
      {
        change = "demo",
        contract = "contract.json",
        results = "results.json",
        subject_hash = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
      },
      { change = "other", cross_repo_evidence = ["upstream.json"] },
    ],
  },
  kind | r.contracts.ActionKind = "dependency_wait",
}
