let C = import "../../../contracts/verification-impact.ncl" in
let h = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" in
{
  schema_version = "cairn.verification-impact.v1",
  before_manifest = {
    identity = h,
    scope = { repository_id = "onixresearch/cairn", target = "x86_64-linux", features = ["default"] },
    members = [{ path = "crates/demo/src/lib.rs", content_blake3 = h, generated = false }],
  },
  after_manifest = {
    identity = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    scope = { repository_id = "onixresearch/cairn", target = "x86_64-linux", features = ["default"] },
    members = [{ path = "crates/demo/src/lib.rs", content_blake3 = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc", generated = false }],
  },
  asserted_changed_paths = ["crates/demo/src/lib.rs"],
  before_trace_index = {},
  after_trace_index = {},
  check_map = {
    schema_version = "cairn.verification-check-map.v1",
    map_id = "cairn-local",
    coverage_complete = true,
    default_check_ids = ["workspace-tests"],
    checks = [
      { check_id = "demo-tests", target = "x86_64-linux", features = ["default"], mandatory = false },
      { check_id = "policy-fresh", target = "x86_64-linux", features = ["default"], mandatory = true },
      { check_id = "workspace-tests", target = "x86_64-linux", features = ["default"], mandatory = false },
    ],
    requirement_checks = [{ requirement_id = "cairn.verification.demo", check_id = "demo-tests" }],
    dependency_edges = [],
    macro_opaque_paths = [],
    generated_source_paths = [],
    incomplete_source_paths = [],
  },
} | C.source_comparison
