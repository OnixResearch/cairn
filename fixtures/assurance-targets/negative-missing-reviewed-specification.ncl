# Rejected: the generated record must carry the plain digest of the contract's
# reviewed specification, keyed by its path.
let Contract = import "../../contracts/assurance-targets.ncl" in
let targets = (import "./octet-example/targets.json") | Contract.TargetsFile in
let generated = (import "./octet-example/generated.json") | Contract.GeneratedSources in
Contract.project (std.array.first targets.contracts)
  (std.record.update "specifications" [] generated)
