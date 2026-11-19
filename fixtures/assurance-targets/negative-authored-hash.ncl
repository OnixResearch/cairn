# Rejected: a target never authors its own tool identity hash.
let Contract = import "../../contracts/assurance-targets.ncl" in
let targets = import "./octet-example/targets.json" in
let contract = std.array.first targets.contracts in
let projected = std.array.first contract.targets in
{
  schema_version = targets.schema_version,
  contracts = [{
    contract_id = contract.contract_id,
    specification = contract.specification,
    targets = [projected & {
      tool_identity_hashes = ["4ff9ff39f3ec4e1015d318f317230c2a568a3aded1b49c07a9ab814af2cdbefa"],
    }],
    assumptions = contract.assumptions,
    exclusions = contract.exclusions,
    non_claims = contract.non_claims,
  }],
} | Contract.TargetsFile
