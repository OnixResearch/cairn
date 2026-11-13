# r[impl cairn.prose_workflow.configuration]
let AtMostSixteen = std.contract.from_predicate (fun xs => std.array.length xs <= 16 && std.array.length xs > 0) in
let AtMostSixtyFour = std.contract.from_predicate (fun xs => std.array.length xs <= 64) in
let Profile = {
  name | String,
  include_rules | Array String | AtMostSixtyFour,
  exclude_rules | Array String | AtMostSixtyFour,
} in
{
  schema_version = "cairn.prose-profiles.v1",
  profiles | Array Profile | AtMostSixteen = [
    { name = "default", include_rules = [], exclude_rules = [] },
    { name = "agreement", include_rules = ["E0001"], exclude_rules = [] },
  ],
}
