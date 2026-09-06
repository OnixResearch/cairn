let contracts = import "../../contracts/consumer-bootstrap.ncl" in
{
  schema = "cairn.bootstrap-guidance.v1",
  adapter = "agent-skills",
  destination = "../global",
} | contracts.guidance_declaration
