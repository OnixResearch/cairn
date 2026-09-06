# r[impl cairn.adoption_report.inputs]
# r[impl cairn.adoption_report.stages]
# r[impl cairn.adoption_report.queries]
# r[impl cairn.adoption_report.boundary]
let Positive = std.contract.from_predicate (fun n => std.is_number n && n > 0 && std.number.floor n == n) in
let Count = std.contract.from_predicate (fun n => std.is_number n && n >= 0 && std.number.floor n == n) in
let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let Blake3Hex = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-fA-F]{64}$" value
) in
let OptionalNonEmptyString = std.contract.from_predicate (fun value =>
  value == null || (std.is_string value && std.string.length value > 0)
) in
let Stage = [| 'upstream_lifecycle, 'consumer_pin, 'consumer_compatibility, 'integration_evidence |] in
let StageState = [| 'supported, 'gap, 'missing, 'stale, 'unavailable, 'blocked, 'diverged, 'conflict, 'partial, 'unknown |] in
let ActionKind = [| 'evidence_refresh, 'diagnostic_inspection, 'unresolved |] in
let ProducerKind = [| 'stack_deps_audit, 'stack_deps_reconciliation, 'stack_doctor_health, 'cairn_lifecycle, 'cairn_registry, 'consumer_evidence, 'valence_evidence |] in
let relationship_key_contract = {
  relationship_id | NonEmptyString,
  upstream_repository | NonEmptyString,
  upstream_change | NonEmptyString,
  consumer_repository | NonEmptyString,
  required_revision | NonEmptyString,
  subject_id | NonEmptyString,
  policy_id | NonEmptyString,
} in
let query_bounds_contract = {
  max_relationships | Positive,
  max_observations | Positive,
  max_provenance | Positive,
} in
let query_contract = {
  scope_id | NonEmptyString,
  relationships | Array relationship_key_contract,
  bounds | query_bounds_contract,
} in
let producer_cohort_contract = {
  producer_id | NonEmptyString,
  kind | ProducerKind,
  supported_schemas | Array NonEmptyString,
  source_revision | NonEmptyString,
  validation_evidence | Blake3Hex,
  required_non_claims | Array NonEmptyString,
} in
let provenance_contract = {
  producer_id | NonEmptyString,
  schema | NonEmptyString,
  source_revision | NonEmptyString,
  tool_identity | NonEmptyString,
  policy_identity | NonEmptyString,
  observation_scope | NonEmptyString,
  receipt_hash | Blake3Hex,
} in
let stage_observation_contract = {
  relationship | relationship_key_contract,
  stage | Stage,
  state | StageState,
  producer_id | NonEmptyString,
  producer_status | NonEmptyString,
  evidence_hashes | Array Blake3Hex,
  observed_revision | OptionalNonEmptyString,
  message | NonEmptyString,
} in
let coverage_contract = {
  partial | Bool,
  selected_relationships | Count,
  observed_relationships | Count,
  omitted_relationships | Array NonEmptyString,
  bound_exhausted | Array NonEmptyString,
} in
let snapshot_contract = {
  schema | NonEmptyString,
  query | query_contract,
  provenance | Array provenance_contract,
  observations | Array stage_observation_contract,
  coverage | coverage_contract,
} in
let stage_report_contract = {
  stage | Stage,
  state | StageState,
  producer_statuses | Array NonEmptyString,
  evidence_hashes | Array Blake3Hex,
  observed_revisions | Array NonEmptyString,
  messages | Array NonEmptyString,
} in
let relationship_report_contract = {
  relationship | relationship_key_contract,
  complete | Bool,
  stages | Array stage_report_contract,
} in
let action_contract = {
  id | Blake3Hex,
  kind | ActionKind,
  authority | [| 'advisory |],
  owner_repository | NonEmptyString,
  relationship_id | NonEmptyString,
  stage | Stage,
  prerequisites | Array NonEmptyString,
  expected_observation | NonEmptyString,
  effect_authorized | Bool,
} in
let report_contract = {
  schema | NonEmptyString,
  valid | Bool,
  complete | Bool,
  scope_id | NonEmptyString,
  input_hash | Blake3Hex,
  report_hash | Blake3Hex,
  coverage | coverage_contract,
  provenance | Array provenance_contract,
  relationships | Array relationship_report_contract,
  actions | Array action_contract,
  diagnostics | Array NonEmptyString,
} in
{
  contracts = {
    RelationshipKey = relationship_key_contract,
    QueryBounds = query_bounds_contract,
    Query = query_contract,
    ProducerCohort = producer_cohort_contract,
    Provenance = provenance_contract,
    StageObservation = stage_observation_contract,
    Coverage = coverage_contract,
    Snapshot = snapshot_contract,
    StageReport = stage_report_contract,
    RelationshipReport = relationship_report_contract,
    Action = action_contract,
    Report = report_contract,
  },
  policy = {
    schema = "cairn.adoption.policy.v1",
    snapshot_schema = "cairn.adoption.snapshot.v1",
    report_schema = "cairn.adoption.report.v1",
    named_bounds = {
      max_relationships | Positive = 512,
      max_observations | Positive = 4096,
      max_provenance | Positive = 128,
      max_source_bytes | Positive = 16777216,
      max_query_scope_bytes | Positive = 4096,
    },
    supported_producer_cohorts = [
      {
        producer_id = "workspace-stack-deps-v2",
        kind = 'stack_deps_audit,
        supported_schemas = ["onixresearch.stack-dependency-catalog.v2", "onixresearch.stack-dependency-receipt.v1", "onixresearch.stack-dependency-reconciliation.v2"],
        source_revision = "22515fb7bc01d378312053d389d4bbdd55637a50",
        validation_evidence = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        required_non_claims = [
          "does-not-prove-upstream-correctness",
          "does-not-prove-semantic-compatibility",
          "does-not-prove-release-readiness",
          "does-not-prove-whole-stack-correctness",
        ],
      },
      {
        producer_id = "workspace-stack-doctor-v1",
        kind = 'stack_doctor_health,
        supported_schemas = ["onixresearch.stack-health-receipt.v1"],
        source_revision = "22515fb7bc01d378312053d389d4bbdd55637a50",
        validation_evidence = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        required_non_claims = [
          "Stack health does not prove child repository correctness.",
          "Revision equality does not prove semantic compatibility.",
          "A complete cohort does not create a shared release unit.",
          "A clean report does not prove release readiness, deployment success, or whole-stack correctness.",
        ],
      },
    ] | Array producer_cohort_contract,
    stage_order = ['upstream_lifecycle, 'consumer_pin, 'consumer_compatibility, 'integration_evidence],
    action_kinds = ['evidence_refresh, 'diagnostic_inspection, 'unresolved],
    required_non_claims = [
      "adoption reports do not scan sibling repositories, fetch remotes, update pins, execute checks, or mutate releases",
      "registry membership cannot satisfy missing producer evidence",
      "upstream lifecycle completion does not prove downstream adoption",
      "matching pins do not prove compatibility, integration, release readiness, or current remote state",
    ],
  },
}
