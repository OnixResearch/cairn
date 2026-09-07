let contracts = import "../../contracts/consumer-bootstrap.ncl" in
{
  schema = "cairn.bootstrap-complete-selection.v1",
  cohort = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
  policy = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  policy_locator = "../policy.json",
} | contracts.complete_selection
