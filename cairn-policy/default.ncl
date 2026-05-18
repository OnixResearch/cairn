let Contracts = import "contracts.ncl" in
{
  name = "cairn-default",
  version = 1,

  schemas = [{
    id = "spec-driven",
    description = "OpenSpec proposal/spec/design/tasks lifecycle",
    artifacts = [
      { id = "proposal", generates = "proposal.md", requires = [] },
      { id = "specs", generates = "specs/**/*.md", requires = ["proposal"] },
      { id = "design", generates = "design.md", requires = ["proposal"] },
      { id = "tasks", generates = "tasks.md", requires = ["specs", "design"] },
    ],
  }],

  lifecycle_stages = ["proposal", "design", "tasks", "implementation", "archive"],

  task_marker_policy = {
    enabled = true,
    stages = ["tasks"],
    strictness = "strict",
    ignore_archives = true,
    markers = [
      { id = "parallel", token = "[parallel]", kind = "literal" },
      { id = "serial", token = "[serial]", kind = "literal" },
      { id = "depends", token = "[depends:<task-or-id>]", kind = "depends-prefix" },
    ],
  },

  finding_defaults = {
    severity = "high",
    class = "omission",
    scope = "tasks",
    route = "auto-fix",
    promoted = "no",
  },

  gate_policy = {
    stages = ["proposal", "design", "tasks"],
    ignore_archives = true,
    finding = {
      severity = "high",
      class = "omission",
      scope = "tasks",
      route = "auto-fix",
      promoted = "no",
    },
  },

  report_contract = {
    verdicts = ["PASS", "WARN", "FAIL"],
    sections = ["Findings", "Required Fixes", "OpenSpec Check", "Gaps", "Next Steps"],
    tagged_findings_required_fields = ["class", "scope", "route", "promoted"],
  },

  receipt_schemas = [
    {
      command = "gate",
      top_level_keys = ["change", "input_hash", "issues", "layout", "policy", "policy_hash", "receipt_hash", "stage", "valid", "verdict"],
      issue_keys = ["message", "promoted", "scope"],
      hash_fields = ["input_hash", "policy_hash", "receipt_hash"],
    },
    {
      command = "sync",
      top_level_keys = ["actions", "blocked", "change", "delta_specs", "dry_run", "input_hash", "layout", "mutated", "mutation_manifest", "plan_hash", "policy", "policy_hash", "reasons", "receipt_hash"],
      action_keys = ["description", "kind", "path"],
      hash_fields = ["input_hash", "plan_hash", "policy_hash", "receipt_hash"],
    },
    {
      command = "archive",
      top_level_keys = ["actions", "blocked", "change", "dry_run", "input_hash", "layout", "mutated", "mutation_manifest", "plan_hash", "policy", "policy_hash", "reasons", "receipt_hash"],
      action_keys = ["description", "kind", "path"],
      hash_fields = ["input_hash", "plan_hash", "policy_hash", "receipt_hash"],
    },
    {
      command = "migrate",
      top_level_keys = ["actions", "blocked", "command", "conflicts", "destination_layout", "destination_manifest_hash", "dry_run", "manifest_hash", "mutated", "mutation_manifest", "policy", "policy_hash", "receipt_hash", "root", "source_layout", "source_manifest_hash"],
      action_keys = ["destination", "destination_hash", "source", "source_hash"],
      hash_fields = ["destination_manifest_hash", "manifest_hash", "policy_hash", "receipt_hash", "source_manifest_hash"],
    },
  ],

  exemption_marker_policy = {
    required_fields = ["reason", "owner", "until"],
    date_fields = ["until"],
    optional_fields = ["scope"],
  },

  sync_archive_policy = {
    require_all_tasks_done = true,
    archive_date_prefix = true,
    default_archive_dir = "cairn/archive",
  },

  mutation_policy = {
    sync = { require_passing_gate = true },
    archive = { require_passing_gate = true },
    migrate = { require_manifest = true },
  },

  receipt_hash_policy = {
    required = true,
    algorithm = "BLAKE3",
    canonicalization = "serde_json_compact_sorted_keys",
  },

  exemptions = [],
} | Contracts.Policy
