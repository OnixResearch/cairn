# Consumer-facing authoring contract and projection for Cairn assurance targets.
# Nickel owns the shapes, admitted values and the clause projection; Rust admission
# owns cross-field registry binding, bounds, stable issue codes and verification that
# a checked-in Octet contract equals this projection. Runtime commands read the
# checked-in JSON exports and never start the Nickel evaluator.
let Digest = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in
let NonEmpty = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let Natural = std.contract.from_predicate (fun value =>
  std.is_number value && value == std.number.floor value && value >= 0
) in
let OneOf = fun alternatives => std.contract.from_predicate (fun value =>
  std.is_string value && std.array.elem value alternatives
) in
let RecordValue = std.contract.from_predicate (fun value => std.is_record value) in

let SourceSpan = {
  start_byte | Natural,
  end_byte | Natural,
  start_line | Natural,
  end_line | Natural,
} in
# One target binds exactly one content-bound registry row. A target may copy the
# row from `cairn requirement-registry export` or derive it by requirement_id
# with `requirement_from_registry`.
# r[impl cairn.assurance_targets.row_binding]
let RowContract = {
  requirement_id | NonEmpty,
  specification_path | NonEmpty,
  specification_blake3 | Digest,
  source_span | SourceSpan,
  requirement_body_blake3 | Digest,
  row_blake3 | Digest,
} in
let Specification = {
  path | NonEmpty,
  blake3 | Digest,
} in
# Registry derivation. `registry` is the parsed `cairn requirement-registry
# export` (`import "requirement-registry.json"`). Both helpers select exactly
# one row by requirement_id and fail closed on anything else, so a target can
# never bind a row that the export does not contain exactly once.
let registry_schema = "cairn.content-bound-requirement-registry.v1" in
let registry_rows = fun registry requirement_id =>
  if !(std.is_record registry) then
    std.fail_with "the requirement registry export must be a record"
  else if !(std.record.has_field "schema_version" registry) then
    std.fail_with "the requirement registry export must declare schema_version"
  else if registry.schema_version != registry_schema then
    std.fail_with
      "the requirement registry export must declare schema_version %{registry_schema}"
  else if !(std.record.has_field "rows" registry) || !(std.is_array registry.rows) then
    std.fail_with "the requirement registry export must carry a rows array"
  else
    std.array.filter
      (fun row =>
        std.is_record row
        && std.record.has_field "requirement_id" row
        && row.requirement_id == requirement_id)
      registry.rows
in
let row_for = fun registry requirement_id =>
  let matching = registry_rows registry requirement_id in
  if std.array.length matching == 1 then
    std.array.first matching
  else
    std.fail_with
      "the requirement registry export must contain exactly one row for requirement_id %{requirement_id}; found %{std.string.from_number (std.array.length matching)}"
in
let registry_row_requirement = fun registry requirement_id =>
  let row = row_for registry requirement_id in
  ({
    requirement_id = row.requirement_id,
    specification_path = row.specification_path,
    specification_blake3 = row.specification_blake3,
    source_span = {
      start_byte = row.source_span.start_byte,
      end_byte = row.source_span.end_byte,
      start_line = row.source_span.start_line,
      end_line = row.source_span.end_line,
    },
    requirement_body_blake3 = row.requirement_body_blake3,
    row_blake3 = row.row_blake3,
  }) | RowContract
in
let registry_row_specification = fun registry requirement_id =>
  let row = row_for registry requirement_id in
  ({ path = row.specification_path, blake3 = row.specification_blake3 }) | Specification
in
# A projected target names its evidence job by id and by the path of the job's
# owner-authored definition. Source and tool identity never appear on a target.
# r[impl cairn.assurance_targets.evidence_job]
let EvidenceJob = {
  job_id | NonEmpty,
  definition | NonEmpty,
} in
# Source identities come only from the separately generated source record.
let SourceBinding = {
  locator | NonEmpty,
  blake3 | Digest,
} in

let TargetRecord = {
  requirement | RowContract,
  # `unassessed` is not a target and no default assurance exists.
  assurance | OneOf ["reviewed", "bounded_check", "checked_proof"],
  claim | NonEmpty,
  informal_remainder | NonEmpty,
  evidence_job | optional | EvidenceJob,
  checked_scope | optional | Array NonEmpty,
  evidence_ids | optional | Array NonEmpty,
  evidence_classes | optional | Array NonEmpty,
  provider | optional | NonEmpty,
  negative_controls | optional | Array NonEmpty,
} in
# Reviewed targets carry no projection burden. Every other target must name exactly
# one evidence job and the evidence that may discharge its claim.
let TargetContract = std.contract.custom (fun label value =>
  let target = std.contract.apply TargetRecord label value in
  if target.assurance == "reviewed" then
    'Ok target
  else if !(std.record.has_field "evidence_job" target) then
    'Error { message = "a projected target requires evidence_job" }
  else if !(std.record.has_field "checked_scope" target) then
    'Error { message = "a projected target requires checked_scope" }
  else if std.array.length target.checked_scope == 0 then
    'Error { message = "a projected target requires a non-empty checked_scope" }
  else if !(std.record.has_field "evidence_ids" target) then
    'Error { message = "a projected target requires evidence_ids" }
  else if std.array.length target.evidence_ids == 0 then
    'Error { message = "a projected target requires non-empty evidence_ids" }
  else if !(std.record.has_field "evidence_classes" target) then
    'Error { message = "a projected target requires evidence_classes" }
  else if std.array.length target.evidence_classes == 0 then
    'Error { message = "a projected target requires non-empty evidence_classes" }
  else if !(std.record.has_field "negative_controls" target) then
    'Error { message = "a projected target requires negative_controls" }
  else
    'Ok target
) in

let ContractRecord = {
  contract_id | NonEmpty,
  specification | Specification,
  targets | Array TargetContract,
  # Octet accepted assumptions and excluded behaviors pass through unchanged.
  assumptions | Array RecordValue,
  exclusions | Array RecordValue,
  non_claims | Array String,
} in
# Every target binds the contract's specification, and a requirement appears once.
let duplicate_ids = fun ids id =>
  std.array.length (std.array.filter (fun other => other == id) ids) > 1
in
let ContractContract = std.contract.custom (fun label value =>
  let contract = std.contract.apply ContractRecord label value in
  let ids = std.array.map (fun target => target.requirement.requirement_id) contract.targets in
  let specification_matches = std.array.all
    (fun target =>
      target.requirement.specification_path == contract.specification.path
      && target.requirement.specification_blake3 == contract.specification.blake3)
    contract.targets
  in
  if !specification_matches then
    'Error { message = "every target requirement must bind the contract specification path and blake3" }
  else if std.array.any (fun id => duplicate_ids ids id) ids then
    'Error { message = "requirement_id must be unique within a contract" }
  else
    'Ok contract
) in

# r[impl cairn.assurance_targets.contract]
let TargetsFileContract = {
  schema_version | OneOf ["cairn.assurance-targets.v1"],
  contracts | Array ContractContract,
} in

# The plain BLAKE3 of a reviewed specification file at the revision, produced by
# a tool over the exact bytes. It is never the registry's domain-separated hash
# and never typed into a target: Octet verifies the plain digest itself.
let SpecificationDigest = {
  path | NonEmpty,
  blake3 | Digest,
} in
let GeneratedEntryRecord = {
  requirement_id | NonEmpty,
  sources | Array SourceBinding,
  source_ids | Array String | default = [],
  policy_ids | Array String | default = [],
  tool_identity_hashes | Array Digest | default = [],
  artifact_hashes | Array Digest | default = [],
} in
let GeneratedEntry = std.contract.custom (fun label value =>
  let entry = std.contract.apply GeneratedEntryRecord label value in
  if std.array.length entry.sources == 0 then
    'Error { message = "sources must be non-empty for every generated entry" }
  else
    'Ok entry
) in
let GeneratedSourcesContract = {
  schema_version | OneOf ["cairn.assurance-target-sources.v1"],
  specifications | Array SpecificationDigest,
  entries | Array GeneratedEntry,
} in

let tier = fun assurance =>
  if assurance == "checked_proof" then
    "formal_proof"
  else if assurance == "bounded_check" then
    "empirical_check"
  else
    std.fail_with "only bounded_check and checked_proof targets are projected"
in
let entry_for = fun generated requirement_id =>
  let matching = std.array.filter
    (fun entry => entry.requirement_id == requirement_id)
    generated.entries
  in
  if std.array.length matching == 1 then
    std.array.first matching
  else
    std.fail_with
      "every projected target requires exactly one generated source entry for its requirement_id"
in
let specification_for = fun generated path =>
  let matching = std.array.filter
    (fun specification => specification.path == path)
    generated.specifications
  in
  if std.array.length matching == 1 then
    std.array.first matching
  else
    std.fail_with
      "the generated source record requires exactly one plain specification digest for the contract specification path"
in
# The fixed clause mapping from assurance targets to an Octet requirement contract.
# `reviewed` targets produce no clause: Octet has no review disposition.
# r[impl cairn.assurance_targets.projection]
let project_fn = fun contract generated =>
  let clause = fun target =>
    let entry = entry_for generated target.requirement.requirement_id in
    let mapped_tier = tier target.assurance in
    let base = {
      clause_id = target.requirement.requirement_id,
      statement = target.claim,
      required = true,
      disposition = mapped_tier,
      expected_evidence_tier = mapped_tier,
      checked_scope = target.checked_scope,
      allowed_evidence_ids = target.evidence_ids,
      allowed_source_requirement_ids = [target.requirement.requirement_id],
      required_evidence_classes = target.evidence_classes,
      required_negative_controls = target.negative_controls,
      required_bindings = {
        source_ids = entry.source_ids,
        policy_ids = entry.policy_ids,
        tool_identity_hashes = entry.tool_identity_hashes,
        artifact_hashes = entry.artifact_hashes,
        checked_scope = [],
        evidence_tier = mapped_tier,
      },
      source_bindings = entry.sources,
    } in
    if std.record.has_field "provider" target then
      base & { required_provider_id = target.provider }
    else
      base
  in
  {
    schema_version = "octet-requirement-contract/v1",
    contract_id = contract.contract_id,
    reviewed_specification =
      specification_for generated contract.specification.path,
    clauses =
      contract.targets
      |> std.array.filter (fun target => target.assurance != "reviewed")
      |> std.array.map clause,
    assumptions = contract.assumptions,
    exclusions = contract.exclusions,
    non_claims = contract.non_claims,
  }
in

{
  Row = RowContract,
  Target = TargetContract,
  Contract = ContractContract,
  TargetsFile = TargetsFileContract,
  GeneratedSources = GeneratedSourcesContract,
  requirement_from_registry = registry_row_requirement,
  specification_from_registry = registry_row_specification,
  project = project_fn,
}
