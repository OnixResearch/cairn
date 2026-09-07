let contracts = import "../../contracts/consumer-bootstrap.ncl" in
let prior = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" in
let next = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb" in
{
  ownership = { locator = prior, identity = next } | contracts.ownership,
  upgrade = { from = prior, to = next } | contracts.upgrade,
  selection = { source = "project", identity = next } | contracts.policy_selection,
  recovery = { prepared_plan = prior, decision = "require_synchronization" } | contracts.recovery,
}
