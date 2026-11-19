# The pilot targets file, applying Cairn's authoring contract. The projection-only
# registry rows are synthetic; see SOURCE.md.
let Contract = import "../../../contracts/assurance-targets.ncl" in
(import "./targets.json") | Contract.TargetsFile
