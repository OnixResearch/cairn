let C = import "../../../contracts/verification-impact.ncl" in
{
  repository_id = "onixresearch/cairn",
  target = "x86_64-linux",
  features = ["serde", "default"],
} | C.scope
