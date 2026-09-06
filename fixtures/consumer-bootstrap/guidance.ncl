let contracts = import "../../contracts/consumer-bootstrap.ncl" in
{
  schema = "cairn.bootstrap-guidance.v1",
  adapter = "agent-skills",
  destination = ".agents/skills",
} | contracts.guidance_declaration
