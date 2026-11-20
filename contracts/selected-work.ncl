# Selected-work snapshot input and portable wire contracts. No execution authority.
# The Rust DTOs in cairn_core::verified::selected_work deny unknown fields and
# are authoritative for decoding; these contracts give the same closed shape to
# Nickel-authored fixtures and consumer review.
let Digest = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in
let Nullable = fun Contract => std.contract.custom (fun label value =>
  if value == null then 'Ok value else std.contract.check Contract label value
) in
let Exactly = fun expected => std.contract.from_predicate (fun value => value == expected) in
let Selector = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[A-Za-z0-9._-]{1,256}$" value
) in
let ReferenceId = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0 && std.string.length value <= 256
  && std.string.is_match "^[A-Za-z0-9._-]+( [A-Za-z0-9._-]+)*$" value
) in
let Locator = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0 && std.string.length value <= 4096
  && std.string.is_match "^[A-Za-z0-9_.-]+(/[A-Za-z0-9_.-]+)*$" value
  && std.array.all (fun part => part != "." && part != "..") (std.string.split "/" value)
) in
let ObligationId = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[a-z0-9._-]{1,96}$" value
) in
let RevisionKind = [| 'exact, 'projection |] in
let SourceRoot = [| 'lifecycle, 'repository |] in
let SourceStatus = [| 'present, 'missing, 'unavailable |] in
let Availability = [| 'ready, 'blocked, 'unavailable, 'unsupported |] in
let WorkState = [| 'ready, 'blocked, 'complete, 'unavailable |] in
let StageState = [| 'ready, 'blocked, 'unavailable, 'not_applicable |] in
let SemanticAvailability = [| 'supported, 'not_selected, 'unsupported, 'unavailable |] in
let SourceArtifact = { root | SourceRoot, path | Locator, bytes | Number,
  blake3 | String, status | SourceStatus } in
let Selection = { change | Selector, task | Nullable Selector } in
let Reference = { kind | [| 'requirement, 'acceptance |], id | ReferenceId,
  scope | { kind | [| 'change, 'task |], task | optional | Selector } } in
let Unit = { task | Nullable Selector, state | WorkState,
  references | Array Reference, verification_contracts | Array Locator } in
let Dependency = { task | Selector, prerequisite | Selector, state | Availability,
  identity | Digest } in
let Completion = { state | Availability, contract_hash | Nullable Digest,
  results_hash | Nullable Digest, subject_hash | Nullable Digest } in
let Remaining = { kind | [| 'implementation, 'completion, 'sync, 'archive |],
  state | Availability, subject | Nullable String } in
let Enumeration = { complete | Bool, observed | Number, total | Nullable Number,
  budget_bytes | Number, consumed_bytes | Number } in
let PortableEntry = { id | Selector, authority | String,
  source_root | Nullable String, path | Nullable Locator,
  status | Availability, content_hash | Nullable Digest } in
let PortableContext = { selection_hash | Digest, portable_hash | Digest,
  entries | Array PortableEntry } in
let Stage = { stage | String, state | StageState, decision_hash | Digest } in
let Blocker = { id | String, stage | String, code | String, target | String } in
let Action = { id | String, kind | String, causes | Array String,
  target | String, prerequisites | Array String, authority | [| 'advisory |],
  expected_observation | String, original_argv_hash | Digest,
  portable_argv | Array String } in
let Cohort = { source | Digest, intent | Digest, context | Digest,
  verification_subject | Digest, dependencies | Digest, obligations | Digest, selection | Digest,
  observation | Digest } in
# Workflow obligations (#12) transported for one snapshot. The obligation
# module owns predicate, guidance and evaluation shapes; consumers depend on
# the typed node facts, binding, identities and evaluation outcome below.
let ObligationBinding = { workflow_id | String, workflow_version | Number,
  definition_id | String, intent_blake3 | Digest, workflow_policy_blake3 | Digest,
  catalog_identity | Digest, binding_blake3 | Digest } in
let EvidenceReference = { predicate | ObligationId,
  kind | [| 'evidence, 'applicability |], requirement_id | String, owner | String,
  accepted_revision_blake3 | Digest, reference_blake3 | Digest } in
let ExceptionReference = { replaces | ObligationId, exception_id | String,
  request_revision | Number, policy_identity | Digest, scope_rule_id | String,
  control | String } in
let ObligationNode = {
  schema | Exactly "cairn.workflow-obligation-node.v1",
  id | ObligationId,
  kind | [| 'evidence, 'all_of, 'any_of, 'applicable, 'temporal, 'exception |],
  parent | Nullable ObligationId,
  alternative | Nullable ObligationId,
  children | Array ObligationId,
  alternatives | Array ObligationId,
  selection | [| 'required, 'selected_alternative, 'conditional,
    'selection_missing, 'unselected_alternative |],
  conditions | Array ObligationId,
  evidence | Array EvidenceReference,
  expected_observations | Array ObligationId,
  command_classes | Array String,
  temporal_check | Nullable ObligationId,
  exception | Nullable ExceptionReference,
  obligation_blake3 | Digest,
  guidance_blake3 | Digest,
  condition_blake3 | Digest,
} in
let RemainingObligation = { id | ObligationId, kind | String, state | String,
  reason | String, evidence | Array String } in
let Evaluation = {
  schema | Exactly "cairn.workflow-obligation-report.v1",
  binding | ObligationBinding,
  subject_blake3 | Digest,
  context_blake3 | Digest,
  outcome | [| 'proved, 'exception_accepted, 'unsatisfied, 'blocked, 'abandoned,
    'unknown, 'unsupported |],
  proved | Bool,
  successful_delivery | Bool,
  satisfied | Array ObligationId,
  remaining | Array RemainingObligation,
  predicate_blake3 | Digest,
  input_blake3 | Digest,
  report_blake3 | Digest,
  ..
} in
let ObligationExport = {
  binding | ObligationBinding,
  predicate | { schema | Exactly "cairn.workflow-obligation-predicate.v1",
    binding | ObligationBinding, subject_blake3 | Digest, context_blake3 | Digest,
    predicate_blake3 | Digest, .. },
  guidance | { binding_blake3 | Digest, requirements_blake3 | Digest, .. },
  nodes | Array ObligationNode,
  action_preconditions | Array { id | ObligationId, action | String,
    required_predicates | Array ObligationId, authority | String },
  expected_observations | Array { id | ObligationId, command_class | String,
    predicate | ObligationId, description | String },
  observations_supplied | Bool,
  evaluation | Evaluation,
} in
let Obligations = std.contract.custom (fun label value =>
  if value == { state = "not_selected" } || value == { state = 'not_selected } then
    'Ok value
  else
    std.contract.check { state | [| 'exported |], export | ObligationExport } label value
) in
let Snapshot = {
  schema | Exactly "cairn.selected-work.v1",
  producer | Exactly "cairn",
  selection | Selection,
  cohort | Cohort,
  source | { revision | { kind | RevisionKind, identity | Digest },
    artifacts | Array SourceArtifact },
  policy_hash | Digest,
  profile | Selector,
  profile_hash | Digest,
  dependencies | Array Dependency,
  completion | Completion,
  context | PortableContext,
  readiness | { native_receipt_hash | Digest,
    native_scope | { change | Selector, policy_hash | Digest,
      source_hash | Digest, evidence_hash | Digest },
    native_input_hash | Digest, native_policy_hash | Digest,
    valid | Bool, classification | String,
    stages | Array Stage, blockers | Array Blocker, actions | Array Action },
  units | Array Unit,
  remaining | Array Remaining,
  enumeration | Enumeration,
  semantic_extensions | { dependency_meanings | SemanticAvailability,
    obligation_explanations | SemanticAvailability },
  obligations | Obligations,
  local | Exactly null,
} in
# Caller-supplied obligation observations (`--obligation-observations`). The
# producer binds subject, context and workflow policy from the snapshot.
let ObligationObservations = {
  selected_alternatives | { _ | ObligationId } | optional,
  evidence | Array { predicate | ObligationId, requirement_id | String,
    revision_blake3 | Digest, subject_blake3 | Digest, context_blake3 | Digest,
    policy_blake3 | Digest, observation_id | String,
    state | [| 'satisfied, 'unsatisfied, 'unknown, 'unsupported |] } | optional,
  progress | Array Dyn | optional,
  agent_claims | Dyn | optional,
  exceptions | Array Dyn | optional,
  temporal | Dyn | optional,
  terminal | Dyn | optional,
} in
{
  schema = "cairn.selected-work.v1",
  max_artifacts = 512,
  max_units = 512,
  max_references = 4096,
  max_context_entries = 512,
  max_observation_bytes = 67108864,
  contracts = { snapshot = Snapshot, cohort = Cohort, selection = Selection,
    source_artifact = SourceArtifact, reference = Reference, unit = Unit,
    dependency = Dependency, completion = Completion, remaining = Remaining,
    enumeration = Enumeration, portable_context = PortableContext,
    obligations = Obligations, obligation_node = ObligationNode,
    obligation_observations = ObligationObservations },
}
