# A representation-only pair: no end-to-end efficiency verdict is permitted.
let Contract = import "../../contracts/agent-benchmark.ncl" in
let Baseline = import "./positive-input.ncl" in
{
  schema = "cairn.agent-benchmark.comparison.v1",
  allowed_candidate_differences = ["product_revision"],
  baseline = Baseline,
  candidate = Baseline & { runs | force = std.array.map
    (fun run => run & { cohort = { product_revision | force = "0000000000000000000000000000000000000000000000000000000000000000" } })
    Baseline.runs },
} | Contract.Comparison
