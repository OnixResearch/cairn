let Contracts = import "../contracts.ncl" in
(import "../default.ncl") & {
  schemas = [{
    id = "bad-schema",
    description = "unknown artifact dependency fixture",
    artifacts = [
      { id = "tasks", generates = "tasks.md", requires = ["missing"] },
    ],
  }],
} | Contracts.Policy
