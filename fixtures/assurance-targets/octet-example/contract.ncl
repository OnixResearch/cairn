# Octet requirement contract projected from the pilot targets and generated sources.
let Contract = import "../../../contracts/assurance-targets.ncl" in
let targets = (import "./targets.json") | Contract.TargetsFile in
let generated = (import "./generated.json") | Contract.GeneratedSources in
Contract.project (std.array.first targets.contracts) generated
