let contracts = import "../../contracts/consumer-bootstrap.ncl" in
{
  schema = "cairn.bootstrap-project.v1",
  policy_locator = "../policy.json",
  policy = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  cohort = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
  complete_manifest = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
} | contracts.project_declaration
