# Advisory verification-impact contracts. These contracts do not execute checks
# and do not authorize evidence reuse or release-gate omission.
let MaximumSourceMembers = 2048 in
let MaximumChangedPaths = 512 in
let MaximumChecks = 128 in
let MaximumRequirementBindings = 4096 in
let MaximumEdges = 4096 in
let MaximumFeatures = 64 in
let MaximumPathBytes = 4096 in
let MaximumTextBytes = 512 in
let MaximumPriorEvidence = 128 in
let MaximumDependencies = 64 in
let Digest = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in
let Label = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[a-z][a-z0-9._-]{0,127}$" value
) in
let RequirementId = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && std.string.length value <= MaximumTextBytes
  && std.string.is_match "^[a-z0-9._-]+$" value
) in
let SafeRelativePath = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && std.string.length value <= MaximumPathBytes
  && std.string.is_match "^[A-Za-z0-9][A-Za-z0-9._/-]*$" value
  && !(std.string.is_match ".*//.*" value)
  && !(std.string.is_match ".*/\\.\\.?(/.*)?" value)
) in
let BoundedArray = fun maximum => std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value <= maximum
) in
let CanonicalStringArray = fun maximum => std.contract.from_predicate (fun value =>
  std.is_array value
  && std.array.length value <= maximum
  && std.array.sort std.string.compare value == value
  && std.array.all
    (fun item =>
      std.array.fold_left
        (fun count candidate => if candidate == item then count + 1 else count)
        0
        value
      == 1
    )
    value
) in
let Scope = {
  repository_id | String,
  target | Label,
  features | CanonicalStringArray MaximumFeatures,
} in
let SourceMember = {
  path | SafeRelativePath,
  content_blake3 | Digest,
  generated | Bool,
} in
let SourceManifest = {
  identity | Digest,
  scope | Scope,
  members | BoundedArray MaximumSourceMembers | Array SourceMember,
} in
let CheckDefinition = {
  check_id | Label,
  target | Label,
  features | CanonicalStringArray MaximumFeatures,
  mandatory | Bool,
} in
let RequirementCheck = {
  requirement_id | RequirementId,
  check_id | Label,
} in
let RequirementEdge = {
  from_requirement_id | RequirementId,
  to_requirement_id | RequirementId,
} in
let Applicability = {
  subject_hash | Digest,
  policy_hash | Digest,
  dependency_hashes | CanonicalStringArray MaximumDependencies,
  source_projection_hash | Digest,
} in
let PriorEvidence = {
  evidence_id | Label,
  check_ids | CanonicalStringArray MaximumChecks,
  applicability | Applicability,
} in
let CheckMap = {
  schema_version | std.contract.from_predicate (fun value => value == "cairn.verification-check-map.v1"),
  map_id | Label,
  coverage_complete | Bool,
  default_check_ids | CanonicalStringArray MaximumChecks,
  checks | BoundedArray MaximumChecks | Array CheckDefinition,
  requirement_checks | BoundedArray MaximumRequirementBindings | Array RequirementCheck,
  dependency_edges | BoundedArray MaximumEdges | Array RequirementEdge,
  dependency_hashes | CanonicalStringArray MaximumDependencies,
  macro_opaque_paths | CanonicalStringArray MaximumChangedPaths,
  generated_source_paths | CanonicalStringArray MaximumChangedPaths,
  incomplete_source_paths | CanonicalStringArray MaximumChangedPaths,
} in
let SourceComparison = {
  schema_version | std.contract.from_predicate (fun value => value == "cairn.verification-impact.v1"),
  before_manifest | SourceManifest,
  after_manifest | SourceManifest,
  asserted_changed_paths | CanonicalStringArray MaximumChangedPaths,
  # Trace indexes remain admitted by cairn-core's content-bound validator.
  before_trace_index,
  after_trace_index,
  check_map | CheckMap,
  prior_evidence | BoundedArray MaximumPriorEvidence | Array PriorEvidence,
} in
let Uncertainty = {
  code | Label,
  target | String,
  message | String,
} in
let Explanation = {
  source_path | String,
  requirement_id | String,
  check_id | String,
  reason | Label,
} in
{
  scope = Scope,
  source_member = SourceMember,
  source_manifest = SourceManifest,
  check_definition = CheckDefinition,
  requirement_check = RequirementCheck,
  requirement_edge = RequirementEdge,
  check_map = CheckMap,
  source_comparison = SourceComparison,
  applicability = Applicability,
  prior_evidence = PriorEvidence,
  uncertainty = Uncertainty,
  explanation = Explanation,
  bounds = {
    max_source_members = MaximumSourceMembers,
    max_changed_paths = MaximumChangedPaths,
    max_checks = MaximumChecks,
    max_requirement_bindings = MaximumRequirementBindings,
    max_edges = MaximumEdges,
    max_features = MaximumFeatures,
    max_path_bytes = MaximumPathBytes,
    max_text_bytes = MaximumTextBytes,
    max_prior_evidence = MaximumPriorEvidence,
    max_dependencies = MaximumDependencies,
  },
}
