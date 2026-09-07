# Cairn-owned profile authoring. This contract does not authorize execution.
let MaximumDeadlineMs = 3600000 in
let MaximumTeardownMs = 60000 in
let MaximumStreamBytes = 1048576 in
let Digest = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in
let Label = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[a-z][a-z0-9._-]{0,127}$" value
) in
let PositiveBound = fun maximum => std.contract.from_predicate (fun value =>
  std.is_number value && value == std.number.floor value && value > 0 && value <= maximum
) in
let Bounds = {
  id | Label,
  deadline_ms | PositiveBound MaximumDeadlineMs,
  teardown_ms | PositiveBound MaximumTeardownMs,
  stdout_bytes | PositiveBound MaximumStreamBytes,
  stderr_bytes | PositiveBound MaximumStreamBytes,
} in
let Profile = {
  schema | std.contract.from_predicate (fun value => value == "cairn.check-profile.v1"),
  check_id | Label,
  kind | std.contract.from_predicate (fun value => value == "cargo_test" || value == "nix_check"),
  executable | String,
  executable_hash | Digest,
  tool_cohort_hash | Digest,
  target | Label,
  features | Array Label,
  environment | { _ : String },
  working_root | std.contract.from_predicate (fun value => value == "."),
  bounds | Bounds,
} in
{
  profile = Profile,
  bounds = Bounds,
  # The pure Rust admission also checks exact argv, canonical sets, and byte bounds.
  # No default environment, zero-test waiver, cache waiver, or executable is supplied.
}
