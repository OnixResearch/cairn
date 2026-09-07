# Consumer-owned bootstrap input contracts. These contracts grant no path or I/O authority.
let MaximumArtifacts = 64 in
let MaximumArtifactBytes = 16777216 in
let MaximumTotalBytes = 67108864 in
let MaximumLabelBytes = 256 in
let MaximumUpgradeSources = 64 in
let Identity = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in
let Label = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
  && std.string.length value <= MaximumLabelBytes
  && std.string.is_match "^[a-zA-Z0-9-]+$" value
) in
let NaturalBound = fun maximum => std.contract.from_predicate (fun value =>
  std.is_number value && value == std.number.floor value && value > 0 && value <= maximum
) in
let UniqueIdentities = fun maximum => std.contract.from_predicate (fun values =>
    std.is_array values && std.array.length values <= maximum
    && std.array.all (fun value => std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value) values
    && std.array.all (fun value =>
      std.array.fold_left (fun count other => if other == value then count + 1 else count) 0 values == 1
    ) values
  )
in
let Bounds = {
  max_artifacts | NaturalBound MaximumArtifacts,
  max_artifact_bytes | NaturalBound MaximumArtifactBytes,
  max_total_bytes | NaturalBound MaximumTotalBytes
    | std.contract.from_predicate (fun value => value >= max_artifact_bytes),
}
in
let Cohort = {
  schema | std.contract.from_predicate (fun value => value == "cairn.consumer-bootstrap.v1"),
  tool | Identity,
  policy | Identity,
  export | Identity,
  evaluator | Identity,
  source | Identity,
  workflow | Identity,
  profile | Label,
  managed_locators | UniqueIdentities MaximumArtifacts
    | std.contract.from_predicate (fun value =>
      std.array.length value > 0 && std.array.length value <= bounds.max_artifacts
    ),
  upgrade_from | UniqueIdentities MaximumUpgradeSources,
  bounds | Bounds,
}
in
let PolicySelection = {
  source | std.contract.from_predicate (fun value =>
    value == "explicit" || value == "project" || value == "legacy_default"
  ),
  identity | Identity,
} in
let RelativeLocator = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
  && std.string.length value <= MaximumLabelBytes
  && std.string.is_match "^[a-zA-Z0-9_.-]+(/[a-zA-Z0-9_.-]+)*$" value
  && std.array.all (fun part => part != "." && part != "..") (std.string.split "/" value)
) in
let ProjectDeclaration = {
  schema | std.contract.from_predicate (fun value => value == "cairn.bootstrap-project.v1"),
  policy_locator | RelativeLocator,
  policy | Identity,
  cohort | Identity,
  complete_manifest | Identity,
} in
let InstalledArtifact = {
  destination | RelativeLocator,
  identity | Identity,
  bytes | NaturalBound MaximumArtifactBytes,
} in
let GuidanceDeclaration = {
  schema | std.contract.from_predicate (fun value => value == "cairn.bootstrap-guidance.v1"),
  adapter | Label,
  destination | RelativeLocator,
} in
let CompleteSelection = {
  artifacts | optional | Array InstalledArtifact
    | std.contract.from_predicate (fun values => std.array.length values <= MaximumArtifacts),
  schema | std.contract.from_predicate (fun value => value == "cairn.bootstrap-complete-selection.v1"),
  cohort | Identity,
  policy | Identity,
  policy_locator | RelativeLocator,
} in
let SourceArtifact = {
  source | RelativeLocator,
  destination | RelativeLocator
    | std.contract.from_predicate (fun value =>
      value != ".cairn/bootstrap/prepared.json"
      && value != ".cairn/bootstrap/complete.json"
      && value != ".cairn/bootstrap/selection.json"
      && value != ".cairn/bootstrap/guidance.json"
      && value != ".cairn-bootstrap.lock"
      && std.array.all (fun part => part != ".cairn-bootstrap-stage") (std.string.split "/" value)
    ),
} in
let SourceBundle = {
  schema | std.contract.from_predicate (fun value => value == "cairn.bootstrap-source-bundle.v1"),
  cohort | Cohort,
  export_request | RelativeLocator,
  export_manifest | RelativeLocator,
  workflow_source | RelativeLocator,
  policy_locator | RelativeLocator,
  artifacts | Array SourceArtifact
    | std.contract.from_predicate (fun values =>
      std.array.length values > 0 && std.array.length values <= cohort.bounds.max_artifacts
      && std.array.any (fun value => value.destination == policy_locator) values
      && std.array.all (fun value =>
        std.array.length (std.array.filter (fun other => other.destination == value.destination) values) == 1
      ) values
    ),
} in
let Ownership = {
  locator | Identity,
  identity | Identity,
} in
let Upgrade = {
  from | Identity,
  to | Identity,
} in
let Recovery = {
  prepared_plan | Identity,
  decision | std.contract.from_predicate (fun value =>
    value == "resume_unchanged" || value == "publish_complete_manifest"
    || value == "require_synchronization" || value == "manual_repair"
  ),
} in
{
  identity = Identity,
  bounds = Bounds,
  cohort = Cohort,
  policy_selection = PolicySelection,
  project_declaration = ProjectDeclaration,
  complete_selection = CompleteSelection,
  guidance_declaration = GuidanceDeclaration,
  source_bundle = SourceBundle,
  ownership = Ownership,
  upgrade = Upgrade,
  recovery = Recovery,
}
