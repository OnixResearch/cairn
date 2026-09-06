let C = import "../../../contracts/verification-impact.ncl" in
{
  identity = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  scope = { repository_id = "onixresearch/cairn", target = "x86_64-linux", features = ["default"] },
  members = [{ path = "../outside.rs", content_blake3 = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", generated = false }],
} | C.source_manifest
