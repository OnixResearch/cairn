# A requirement id that the registry export contains twice fails closed.
let Contract = import "../../contracts/assurance-targets.ncl" in
let registry = import "./registry-derived/requirement-registry.json" in
let rows = registry.rows @ [std.array.first registry.rows] in
Contract.requirement_from_registry
  (std.record.update "rows" rows registry)
  "octet.aeneas_lean_proof_rail.theorem_adequacy"
