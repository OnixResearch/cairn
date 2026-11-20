# A registry export whose schema identifier is not the admitted one fails closed
# before any row is selected.
let Contract = import "../../contracts/assurance-targets.ncl" in
let registry = import "./registry-derived/requirement-registry.json" in
let wrong = std.record.update
  "schema_version"
  "cairn.content-bound-requirement-registry.v0"
  registry
in
Contract.requirement_from_registry wrong "octet.aeneas_lean_proof_rail.theorem_adequacy"
