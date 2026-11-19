# Readiness-backed drain projection declarations.
#
# The manifest is keyed by exact active change identifiers. Every selector is
# explicit: this file never declares discovery, inference, execution, or
# mutation authority. Rust admits these values and performs pure ordering.
# r[impl cairn.agent_workflow_readiness.compatibility]
# r[impl cairn.agent_workflow_readiness.order]

let Positive = std.contract.from_predicate (fun n =>
  std.is_number n && n > 0 && std.number.floor n == n
) in
let MaximumPathBytes = 4096 in
let RelativePath = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && std.string.length value <= MaximumPathBytes
  && !std.string.is_match "^/" value
  && !std.string.is_match "^~" value
  && std.array.all
    (fun part => part != "" && part != "." && part != "..")
    (std.string.split "/" value)
) in
let ChangeName = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && std.string.length value <= 128
  && std.array.all
    (fun part => part != "" && part != "." && part != "..")
    (std.string.split "/" value)
) in
let SubjectHash = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && std.string.length value <= 128
) in
let SelectorContract = {
  change | ChangeName,
  contract | optional | RelativePath,
  results | optional | RelativePath,
  subject_hash | optional | SubjectHash,
  cross_repo_evidence | optional | Array RelativePath,
} in
let has_select = fun name entry =>
  std.array.any (fun field => field == name) (std.record.fields entry)
in
let CompletionSelectors = fun entry =>
  has_select "contract" entry == has_select "results" entry
  && has_select "contract" entry == has_select "subject_hash" entry
in
let ManifestContract = {
  schema
    | std.contract.from_predicate (fun value =>
      value == "cairn.workflow-readiness-manifest.v1"
    ),
  changes
    | Array SelectorContract
    | std.contract.from_predicate (fun values =>
      std.array.length values > 0
      && std.array.length values <= 256
      && std.array.all CompletionSelectors values
      && std.array.all
        (fun entry =>
          std.array.length
            (std.array.filter (fun other => other.change == entry.change) values)
          == 1
        )
        values
    ),
} in
let ActionKindContract = std.contract.from_predicate (fun value =>
  std.is_string value
  && (value == "task"
    || value == "dependency_wait"
    || value == "artifact_repair"
    || value == "evidence_refresh"
    || value == "diagnostic_inspection"
    || value == "unresolved"
    || value == "sync_preview"
    || value == "archive_preview")
) in
{
  contracts = {
    Selector = SelectorContract,
    Manifest = ManifestContract,
    ActionKind = ActionKindContract,
  },
  policy = {
    schema = "cairn.workflow-readiness-policy.v1",
    manifest_schema = "cairn.workflow-readiness-manifest.v1",
    view_schema = "cairn.drain-readiness.v1",
    default_view = "legacy",
    readiness_view = "readiness",
    views = ["legacy", "readiness"],
    action_order = [
      "task",
      "dependency_wait",
      "artifact_repair",
      "evidence_refresh",
      "diagnostic_inspection",
      "unresolved",
      "sync_preview",
      "archive_preview",
    ],
    preview_kinds = ["sync_preview", "archive_preview"],
    task_prerequisite_stages = ["artifact", "proposal", "design", "tasks"],
    instruction = {
      tool = "cairn_workflow_instructions",
      cli = ["cairn", "workflow", "instructions"],
      task_artifact = "tasks",
      expected_observation = "task completed, marked in tasks.md, and backed by recorded checks",
    },
    bounds = {
      max_changes | Positive = 256,
      max_manifest_bytes | Positive = 65536,
      max_manifest_issues | Positive = 16,
      max_evidence_files | Positive = 64,
      max_actions | Positive = 128,
      max_actions_per_change | Positive = 8,
      max_path_bytes | Positive = 4096,
      max_subject_hash_bytes | Positive = 128,
    },
    non_claims = [
      "readiness-backed drain guidance is advisory and executes no action",
      "an unavailable observation is never promoted by guidance, transport success, or task checkboxes",
      "the drain view does not discover evidence, measure subjects, fetch remote repositories, or authorize mutation",
    ],
  },
}
