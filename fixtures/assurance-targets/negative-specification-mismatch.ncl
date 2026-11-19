# Rejected: the contract specification must equal its targets' registry rows.
let Contract = import "../../contracts/assurance-targets.ncl" in
let targets = import "./octet-example/targets.json" in
let contract = std.array.first targets.contracts in
{
  schema_version = targets.schema_version,
  contracts = [{
    contract_id = contract.contract_id,
    specification = {
      path = contract.specification.path,
      blake3 = "0000000000000000000000000000000000000000000000000000000000000000",
    },
    targets = contract.targets,
    assumptions = contract.assumptions,
    exclusions = contract.exclusions,
    non_claims = contract.non_claims,
  }],
} | Contract.TargetsFile
