# Rejected: a projected target must name exactly one evidence job.
let Contract = import "../../contracts/assurance-targets.ncl" in
let targets = import "./octet-example/targets.json" in
let contract = std.array.first targets.contracts in
let projected = std.array.first contract.targets in
{
  schema_version = targets.schema_version,
  contracts = [{
    contract_id = contract.contract_id,
    specification = contract.specification,
    targets = [std.record.remove "evidence_job" projected],
    assumptions = contract.assumptions,
    exclusions = contract.exclusions,
    non_claims = contract.non_claims,
  }],
} | Contract.TargetsFile
