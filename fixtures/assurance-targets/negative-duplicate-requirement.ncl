# Rejected: a requirement_id appears at most once within a contract.
let Contract = import "../../contracts/assurance-targets.ncl" in
let targets = import "./octet-example/targets.json" in
let contract = std.array.first targets.contracts in
let projected = std.array.first contract.targets in
let reviewed = std.array.at 1 contract.targets in
{
  schema_version = targets.schema_version,
  contracts = [{
    contract_id = contract.contract_id,
    specification = contract.specification,
    targets = [
      projected,
      {
        requirement = projected.requirement,
        assurance = reviewed.assurance,
        claim = reviewed.claim,
        informal_remainder = reviewed.informal_remainder,
      },
    ],
    assumptions = contract.assumptions,
    exclusions = contract.exclusions,
    non_claims = contract.non_claims,
  }],
} | Contract.TargetsFile
