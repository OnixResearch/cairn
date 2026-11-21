# A requirement id that the registry export does not contain fails closed.
let Contract = import "../../contracts/assurance-targets.ncl" in
let registry = import "./registry-derived/requirement-registry.json" in
Contract.requirement_from_registry registry "octet.requirements.example.absent"
