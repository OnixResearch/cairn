let C = import "../../../contracts/verification-impact.ncl" in
{
  schema_version = "cairn.verification-impact-evidence.v1",
  source_identity = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  scope = { repository_id = "onixresearch/cairn", target = "x86_64-linux", features = ["default"] },
} | C.structural_evidence
