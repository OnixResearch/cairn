# The generated source record for the pilot target, applying Cairn's contract.
# Source and tool identities are produced from the frozen graph, never typed on a target.
let Contract = import "../../../contracts/assurance-targets.ncl" in
(import "./generated.json") | Contract.GeneratedSources
