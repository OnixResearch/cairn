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
      command = "agent-receipt validate",
      top_level_keys = ["agent_receipt_hash", "command", "current_replay_policy_hash", "input_hash", "input_kind", "issues", "max_content_bytes", "path", "receipt_hash", "replay_evidence", "required_replay_cases", "required_replay_group", "stated_agent_receipt_hash", "valid", "verdict"],
      issue_keys = ["code", "message"],
      replay_evidence_keys = ["replay_artifact_path", "replay_environment_attestation_hash", "replay_policy_hash", "replay_receipt_hash", "replay_selected_cases", "replay_selected_groups", "replay_verdict"],
      hash_fields = ["agent_receipt_hash", "current_replay_policy_hash", "input_hash", "receipt_hash", "stated_agent_receipt_hash"],
    },
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
    {
      command = "determinism replay",
      top_level_keys = ["available_cases", "available_groups", "cases", "command", "entropy_assumptions", "entropy_inventory", "entropy_policy_hash", "environment_attestation", "mismatches", "mutability_observations", "nondeterminism_failures", "normalization_rules", "receipt_hash", "root", "selected_cases", "selected_groups", "skipped", "summary", "valid", "verdict"],
      case_keys = ["args", "command_kind", "comparison_modes", "display_only_paths", "expected_mutability", "fixture_root", "hash_fields", "id", "normalization_rules", "portable_evidence_hash", "runs"],
      mismatch_keys = ["actual", "case", "class", "comparison", "expected", "field_path", "reproduce"],
      hash_fields = ["receipt_hash"],
    },
    {
      command = "determinism verify-receipt",
      top_level_keys = ["command", "current_replay_policy_hash", "environment_attestation_hash", "issues", "path", "receipt_hash", "replay_policy_hash", "replay_receipt_hash", "root", "selected_cases", "selected_groups", "stated_replay_receipt_hash", "valid", "verdict"],
      issue_keys = ["actual", "code", "expected", "field_path"],
      hash_fields = ["current_replay_policy_hash", "environment_attestation_hash", "receipt_hash", "replay_policy_hash", "replay_receipt_hash", "stated_replay_receipt_hash"],
    },
    {
      command = "agent-receipt verify-bundle",
      top_level_keys = ["agent_receipt_hash", "command", "current_replay_policy_hash", "environment_attestation_hash", "issues", "receipt_hash", "release_readiness_receipt_hash", "replay_receipt_hash", "replay_verification_receipt_hash", "required_replay_group", "root", "valid", "verdict"],
      issue_keys = ["actual", "code", "expected", "field_path"],
      hash_fields = ["agent_receipt_hash", "current_replay_policy_hash", "environment_attestation_hash", "receipt_hash", "release_readiness_receipt_hash", "replay_receipt_hash", "replay_verification_receipt_hash"],
    },
    {
      command = "evidence-manifest verify",
      top_level_keys = ["anchor_metadata_hash", "artifacts", "command", "issues", "manifest_hash", "receipt_hash", "root", "signer_metadata_hash", "valid", "verdict"],
      artifact_keys = ["hash", "role", "schema", "valid"],
      issue_keys = ["actual", "code", "expected", "field_path"],
      hash_fields = ["anchor_metadata_hash", "manifest_hash", "receipt_hash", "signer_metadata_hash"],
    },
  ],

  determinism_replay_policy = {
    cases = [
      { id = "agent_receipt_validate", command_kind = "agent_receipt_validate", args = ["agent-receipt", "validate", "fixtures/agent-receipts/positive.json"], fixture_root = "<root>", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated"], display_only_paths = ["path"], hash_fields = ["agent_receipt_hash"], normalization_rules = ["portable_root_paths", "canonical_json"], },
      { id = "archive_dry_run", command_kind = "archive_dry_run", args = ["archive", "ready", "--root", "<root>"], fixture_root = "fixtures/archive-ready", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated", "cross_root"], display_only_paths = ["actions.path", "delta_specs"], hash_fields = ["input_hash", "plan_hash", "policy_hash", "receipt_hash"], normalization_rules = ["portable_root_paths", "canonical_json"], },
      { id = "gate_design", command_kind = "gate", args = ["gate", "design", "demo", "--root", "<root>"], fixture_root = "fixtures/basic", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated", "cross_root"], display_only_paths = [], hash_fields = ["input_hash", "policy_hash", "receipt_hash"], normalization_rules = ["portable_root_paths", "canonical_json"], },
      { id = "gate_proposal", command_kind = "gate", args = ["gate", "proposal", "demo", "--root", "<root>"], fixture_root = "fixtures/basic", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated", "cross_root"], display_only_paths = [], hash_fields = ["input_hash", "policy_hash", "receipt_hash"], normalization_rules = ["portable_root_paths", "canonical_json"], },
      { id = "gate_tasks", command_kind = "gate", args = ["gate", "tasks", "demo", "--root", "<root>"], fixture_root = "fixtures/basic", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated", "cross_root"], display_only_paths = [], hash_fields = ["input_hash", "policy_hash", "receipt_hash"], normalization_rules = ["portable_root_paths", "canonical_json"], },
      { id = "mcp_describe", command_kind = "mcp_describe", args = ["mcp", "--describe"], fixture_root = "<root>", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated", "cross_root"], display_only_paths = [], hash_fields = [], normalization_rules = ["portable_root_paths", "canonical_json"], },
      { id = "mcp_status", command_kind = "mcp_call", args = ["mcp", "--call", "cairn_status", "{root:<root>}"], fixture_root = "<root>", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated", "cross_root"], display_only_paths = ["content.root"], hash_fields = [], normalization_rules = ["portable_root_paths", "canonical_json"], },
      { id = "migrate_dry_run", command_kind = "migrate_dry_run", args = ["migrate", "openspec", "--root", "<root>"], fixture_root = "fixtures/basic", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated", "cross_root"], display_only_paths = ["actions.source", "actions.destination", "root"], hash_fields = ["source_manifest_hash", "destination_manifest_hash", "manifest_hash", "policy_hash", "receipt_hash"], normalization_rules = ["portable_root_paths", "canonical_json"], },
      { id = "release_readiness", command_kind = "release_readiness", args = ["release-readiness", "--root", "<root>"], fixture_root = ".", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated"], display_only_paths = ["root"], hash_fields = ["receipt_hash", "cairn_validation_hash", "tracey_coverage_hash", "mcp_smoke_receipt_hash"], normalization_rules = ["portable_root_paths", "canonical_json"], },
      { id = "status", command_kind = "status", args = ["status", "--root", "<root>"], fixture_root = "<root>", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated", "cross_root"], display_only_paths = ["root", "specs", "changes"], hash_fields = [], normalization_rules = ["portable_root_paths", "canonical_json"], },
      { id = "sync_dry_run", command_kind = "sync_dry_run", args = ["sync", "demo", "--root", "<root>"], fixture_root = "fixtures/basic", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated", "cross_root"], display_only_paths = ["actions.path", "delta_specs"], hash_fields = ["input_hash", "plan_hash", "policy_hash", "receipt_hash"], normalization_rules = ["portable_root_paths", "canonical_json"], },
      { id = "validate", command_kind = "validate", args = ["validate", "--root", "<root>"], fixture_root = "<root>", expected_mutability = "read_only_or_dry_run", comparison_modes = ["repeated", "cross_root"], display_only_paths = ["root"], hash_fields = ["receipt_hash"], normalization_rules = ["portable_root_paths", "canonical_json"], },
    ],
    groups = [
      { id = "agent", description = "Agent receipt replay evidence preflight", cases = ["agent_receipt_validate", "mcp_describe", "status", "validate"], },
      { id = "full", description = "All policy-owned replay cases", cases = ["agent_receipt_validate", "archive_dry_run", "gate_design", "gate_proposal", "gate_tasks", "mcp_describe", "mcp_status", "migrate_dry_run", "release_readiness", "status", "sync_dry_run", "validate"], },
      { id = "release", description = "Release-readiness replay preflight", cases = ["agent_receipt_validate", "mcp_describe", "release_readiness", "status", "validate"], },
      { id = "smoke", description = "Cheap status and validation replay preflight", cases = ["status", "validate"], },
    ],
    entropy_inventory = [
      { id = "archive_date_env", class = "environment", source = "CAIRN_ARCHIVE_DATE,IRONSPEC_ARCHIVE_DATE", handling = "reject", receipt_field = "nondeterminism_failures", description = "Archive date environment overrides are replay-relevant clock/date entropy and are rejected during replay when present; values are never disclosed." },
      { id = "clock_date_default", class = "clock_date", source = "archive default date", handling = "normalize", receipt_field = "entropy_assumptions", description = "Archive dry-run replay assumes the deterministic default archive date unless an explicit rejected override is present." },
      { id = "locale_timezone", class = "locale_timezone", source = "process locale and timezone", handling = "normalize", receipt_field = "entropy_assumptions", description = "Replay receipts use canonical JSON, sorted keys, and policy-owned string data rather than locale-sensitive formatting." },
      { id = "cwd_root_paths", class = "filesystem_path", source = "current working directory and replay root", handling = "normalize", receipt_field = "root,normalization_rules", description = "Root-specific checkout and temporary prefixes are normalized before portable hash computation; display paths remain operator-facing." },
      { id = "filesystem_order", class = "filesystem_metadata", source = "directory iteration order and file metadata", handling = "normalize", receipt_field = "mutability_observations", description = "Replay manifests sort root-relative paths and hash file contents; volatile host metadata is not part of portable evidence." },
      { id = "external_command_versions", class = "toolchain", source = "cairn binary, Nix/devshell, and external checker versions", handling = "disclose", receipt_field = "entropy_assumptions", description = "Replay discloses that toolchain identity is represented by policy and receipt hashes rather than raw host paths." },
      { id = "nix_build_context", class = "build_context", source = "Nix flake/devshell context", handling = "disclose", receipt_field = "entropy_assumptions", description = "Replay assumes a reviewed Nix-built Cairn surface and does not capture machine-specific store paths in portable evidence." },
    ],
    environment_attestation = [
      { id = "cairn_package_version", class = "binary_identity", source = "CARGO_PKG_VERSION", handling = "disclose", receipt_field = "environment_attestation.cairn_version", description = "Replay discloses the Cairn package name and version compiled into the binary, not the host binary path." },
      { id = "generated_policy_hash", class = "policy_identity", source = "cairn-policy/generated/cairn-policy.json", handling = "hash", receipt_field = "environment_attestation.generated_policy_hash", description = "Replay binds the generated policy content by BLAKE3 hash." },
      { id = "entropy_policy_hash", class = "entropy_policy_identity", source = "determinism_replay_policy.entropy_inventory", handling = "hash", receipt_field = "environment_attestation.entropy_policy_hash", description = "Replay binds the reviewed entropy inventory by BLAKE3 hash." },
      { id = "replay_selection", class = "selection_identity", source = "--case/--group/default replay selection", handling = "disclose", receipt_field = "environment_attestation.selected_cases,environment_attestation.selected_groups", description = "Replay discloses the selected case and group ids used to produce evidence." },
      { id = "attestation_policy", class = "attestation_policy_identity", source = "determinism_replay_policy.environment_attestation", handling = "hash", receipt_field = "environment_attestation.attestation_policy_hash", description = "Replay binds this reviewed environment attestation policy by BLAKE3 hash." },
      { id = "toolchain_build_context", class = "build_context", source = "Nix/devshell/toolchain context", handling = "disclose", receipt_field = "environment_attestation.policy_facts", description = "Replay discloses reviewed build/toolchain context as policy facts and avoids machine-specific store paths." },
    ],
  },

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
