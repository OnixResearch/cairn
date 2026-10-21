# Cairn-owned profile authoring. This contract does not authorize execution.
let MaximumDeadlineMs = 3600000 in
let MaximumFlakeCheckDeadlineMs = 10800000 in
let MaximumTeardownMs = 60000 in
let MaximumStreamBytes = 1048576 in
let MaximumDependencyFiles = 64 in
let MaximumDependencyFileBytes = 16777216 in
let MaximumDependencyTotalBytes = 67108864 in
let DependencyPath = fun value =>
  std.is_string value
  && std.string.is_match "^[^\\x00]+$" value
  && std.array.all
    (fun part => part != "" && part != "." && part != "..")
    (std.string.split "/" value)
in
let DependencyProjection = std.contract.from_predicate (fun value =>
  std.is_array value
  && std.array.length value <= MaximumDependencyFiles
  && std.array.all DependencyPath value
  && std.array.sort std.string.compare value == value
  && std.array.all (fun item =>
    std.array.fold_left
      (fun count candidate => if candidate == item then count + 1 else count)
      0 value == 1
  ) value
) in
let Digest = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in
let Label = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[a-z][a-z0-9._-]{0,127}$" value
) in
let PositiveBound = fun maximum => std.contract.from_predicate (fun value =>
  std.is_number value && value == std.number.floor value && value > 0 && value <= maximum
) in
let BoundsFor = fun maximum_deadline => {
  id | Label,
  deadline_ms | PositiveBound maximum_deadline,
  teardown_ms | PositiveBound MaximumTeardownMs,
  stdout_bytes | PositiveBound MaximumStreamBytes,
  stderr_bytes | PositiveBound MaximumStreamBytes,
} in
let Profile = {
  schema | std.contract.from_predicate (fun value => value == "cairn.check-profile.v1"),
  check_id | Label,
  kind | std.contract.from_predicate (fun value =>
    std.array.any (fun kind => value == kind) ["cargo_test", "nix_check", "cargo_workspace_test", "cargo_clippy", "cargo_fmt", "nix_flake_check"]),
  executable | String,
  executable_hash | Digest,
  tool_cohort_hash | Digest,
  target | Label,
  features | Array Label
    | std.contract.from_predicate (fun value =>
      kind == "cargo_test" || std.array.length value == 0),
  environment | { _ : String },
  working_root | std.contract.from_predicate (fun value => value == "."),
  bounds | BoundsFor
    (if kind == "nix_flake_check" then MaximumFlakeCheckDeadlineMs else MaximumDeadlineMs),
} in
{
  profile = Profile,
  bounds = BoundsFor MaximumDeadlineMs,
  dependency_projection = DependencyProjection,
  dependency_bounds = {
    maximum_files = MaximumDependencyFiles,
    maximum_file_bytes = MaximumDependencyFileBytes,
    maximum_total_bytes = MaximumDependencyTotalBytes,
  },
  # Projection members name actual no-follow regular files; byte bounds are checked at observation.
  # A projection is not dependency-closure review, package resolution, or remote attestation.
  # The pure Rust admission also checks exact argv, canonical sets, and byte bounds.
  # No default environment, zero-test waiver, cache waiver, or executable is supplied.
}
