# Nickel contracts for cairn policy profiles.

let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in

let NonEmptyArray = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in

let SupportedPolicyVersion = std.contract.from_predicate (fun value =>
  std.is_number value && value == 1
) in

let Blake3Algorithm = std.contract.from_predicate (fun value =>
  std.is_string value && value == "BLAKE3"
) in

let Canonicalization = std.contract.from_predicate (fun value =>
  std.is_string value && value == "serde_json_compact_sorted_keys"
) in

let Blake3Hex = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-fA-F]{64}$" value
) in

let Stage = std.contract.from_predicate (fun value =>
  std.is_string value && (value == "proposal" || value == "design" || value == "tasks" || value == "implementation" || value == "archive")
) in

let Strictness = std.contract.from_predicate (fun value =>
  std.is_string value && (value == "strict" || value == "warn" || value == "off")
) in

let MarkerToken = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^\\[(parallel|serial|depends:[^\\]]+)\\]$" value
) in

let MarkerKind = std.contract.from_predicate (fun value =>
  std.is_string value && (value == "literal" || value == "depends-prefix")
) in

let Severity = std.contract.from_predicate (fun value =>
  std.is_string value && (value == "low" || value == "medium" || value == "high" || value == "critical")
) in

let FindingClass = std.contract.from_predicate (fun value =>
  std.is_string value && (value == "omission" || value == "systematic" || value == "incoherent" || value == "preference" || value == "env" || value == "pattern")
) in

let FindingScope = std.contract.from_predicate (fun value =>
  std.is_string value && (value == "proposal" || value == "spec" || value == "design" || value == "tasks" || value == "review" || value == "tooling")
) in

let FindingRoute = std.contract.from_predicate (fun value =>
  std.is_string value && (value == "auto-fix" || value == "prompt" || value == "deterministic-check" || value == "spec-rule" || value == "human" || value == "guidance")
) in

let Promoted = std.contract.from_predicate (fun value =>
  std.is_string value && (value == "yes" || value == "no")
) in

{
  Artifact = {
    id | NonEmptyString,
    generates | NonEmptyString,
    requires | Array NonEmptyString | default = [],
  },

  ArtifactSchema = {
    id | NonEmptyString,
    description | NonEmptyString,
    artifacts | Array Artifact,
  },

  Marker = {
    id | NonEmptyString,
    token | MarkerToken,
    kind | MarkerKind,
  },

  TaskMarkerPolicy = {
    enabled | Bool,
    stages | NonEmptyArray | Array Stage,
    strictness | Strictness,
    ignore_archives | Bool,
    markers | NonEmptyArray | Array Marker,
  },

  FindingPolicy = {
    severity | Severity,
    class | FindingClass,
    scope | FindingScope,
    route | FindingRoute,
    promoted | Promoted,
  },

  GatePolicy = {
    stages | NonEmptyArray | Array Stage,
    ignore_archives | Bool,
    finding | FindingPolicy,
  },

  ReportContract = {
    verdicts | NonEmptyArray | Array NonEmptyString,
    sections | NonEmptyArray | Array NonEmptyString,
    tagged_findings_required_fields | NonEmptyArray | Array NonEmptyString,
  },

  ReceiptSchema = {
    command | NonEmptyString,
    top_level_keys | NonEmptyArray | Array NonEmptyString,
    action_keys | Array NonEmptyString | default = [],
    issue_keys | Array NonEmptyString | default = [],
    case_keys | Array NonEmptyString | default = [],
    mismatch_keys | Array NonEmptyString | default = [],
    hash_fields | Array NonEmptyString | default = [],
  },

  ReplayCase = {
    id | NonEmptyString,
    command_kind | NonEmptyString,
    args | NonEmptyArray | Array NonEmptyString,
    fixture_root | NonEmptyString,
    expected_mutability | NonEmptyString,
    comparison_modes | NonEmptyArray | Array NonEmptyString,
    display_only_paths | Array NonEmptyString | default = [],
    hash_fields | Array NonEmptyString | default = [],
    normalization_rules | NonEmptyArray | Array NonEmptyString,
  },

  DeterminismReplayPolicy = {
    cases | NonEmptyArray | Array ReplayCase,
  },

  ExemptionMarkerPolicy = {
    required_fields | NonEmptyArray | Array NonEmptyString,
    date_fields | NonEmptyArray | Array NonEmptyString,
    optional_fields | Array NonEmptyString | default = [],
  },

  SyncArchivePolicy = {
    require_all_tasks_done | Bool,
    archive_date_prefix | Bool,
    default_archive_dir | NonEmptyString,
  },

  MutationGate = {
    require_passing_gate | Bool,
  },

  MigrationPolicy = {
    require_manifest | Bool,
  },

  MutationPolicy = {
    sync | MutationGate,
    archive | MutationGate,
    migrate | MigrationPolicy,
  },

  ReceiptHashPolicy = {
    required | Bool,
    algorithm | Blake3Algorithm,
    canonicalization | Canonicalization,
  },

  Exemption = {
    scope | NonEmptyString,
    owner | NonEmptyString,
    reason | NonEmptyString,
    expires | NonEmptyString,
  },

  Policy = {
    name | NonEmptyString,
    version | SupportedPolicyVersion,
    schemas | NonEmptyArray | Array ArtifactSchema,
    lifecycle_stages | NonEmptyArray | Array Stage,
    task_marker_policy | TaskMarkerPolicy,
    gate_policy | GatePolicy,
    finding_defaults | FindingPolicy,
    report_contract | ReportContract,
    receipt_schemas | NonEmptyArray | Array ReceiptSchema,
    determinism_replay_policy | DeterminismReplayPolicy,
    sync_archive_policy | SyncArchivePolicy,
    mutation_policy | MutationPolicy,
    receipt_hash_policy | ReceiptHashPolicy,
    exemption_marker_policy | ExemptionMarkerPolicy,
    exemptions | Array Exemption,
  },

  Blake3Hex = Blake3Hex,
}
