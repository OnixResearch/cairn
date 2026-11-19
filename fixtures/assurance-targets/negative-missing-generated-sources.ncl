# Rejected: a projected target with no generated source entry cannot be projected.
let Contract = import "../../contracts/assurance-targets.ncl" in
let targets = (import "./octet-example/targets.json") | Contract.TargetsFile in
let generated = {
  schema_version = "cairn.assurance-target-sources.v1",
  entries = [],
} | Contract.GeneratedSources in
Contract.project (std.array.first targets.contracts) generated
