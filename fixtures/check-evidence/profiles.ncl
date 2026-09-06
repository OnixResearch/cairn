# Synthetic cohort for admission tests. These executable bytes are not installed.
let C = import "../../contracts/check-evidence-producer.ncl" in
[
  {
    schema = "cairn.check-profile.v1",
    check_id = "tests",
    kind = "cargo_test",
    executable = "/nix/store/reviewed-cargo/bin/cargo",
    executable_hash = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    tool_cohort_hash = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    target = "x86_64-unknown-linux-gnu",
    features = [],
    environment = {},
    working_root = ".",
    bounds = { id = "short", deadline_ms = 1000, teardown_ms = 100, stdout_bytes = 4096, stderr_bytes = 4096 },
  } | C.profile,
]
