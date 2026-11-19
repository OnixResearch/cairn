# Rejected: only `cairn.assurance-targets.v1` is admitted.
let Contract = import "../../contracts/assurance-targets.ncl" in
let targets = import "./octet-example/targets.json" in
{
  schema_version = "cairn.assurance-targets.v2",
  contracts = targets.contracts,
} | Contract.TargetsFile
