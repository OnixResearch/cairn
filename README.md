# cairn

`cairn` is a Rust lifecycle engine intended to supersede the upstream
OpenSpec CLI rather than chase long-term interop. Native repositories use an
`cairn/` lifecycle layout; legacy `openspec/` trees remain supported as an
import/migration source. Cairn adds a Nickel-authored policy/config/schema
layer and a Slotcar-enforced functional-core / imperative-shell boundary.

Boundaries:

- Nickel owns structured policy: project config, artifact schemas, lifecycle
  stages, gates, reports, findings, task markers, sync/archive policy, and
  bounded exemptions.
- `cairn-core` owns pure deterministic transforms only. The strict core lives
  under `crates/cairn-core/src/verified/` so Slotcar's `verified_purity`
  checks continuously guard it.
- `cairn-cli` owns imperative shell behavior: argument parsing, filesystem
  discovery, policy file reads, stdout/stderr, process exits, and JSON
  presentation.
- Runtime commands consume checked-in generated policy JSON by default; Nickel
  evaluation is a development/export step.

## Layout

```text
cairn-policy/        Nickel contracts, defaults, fixtures, generated JSON
crates/cairn-core/   Pure Rust models, policy parsing, repo/status/read/gate/doctor/EARS-style validation
crates/cairn-cli/    CLI/MCP shell for init/change/spec/status/doctor/validate/policy/migrate/gate plus safe sync/archive planning/execution
fixtures/               Fixture legacy and native lifecycle repositories
docs/hermes/            Copyable Hermes skill source for cairn workflow
tools/                  Export/check helpers
```

Native project lifecycle content lives under:

```text
cairn/specs/         Accepted specs
cairn/changes/       Active changes
cairn/archive/       Archived completed changes
```

Commands use the native `cairn/` layout for normal runtime operations. Legacy
`openspec/` content is read only by the explicit `migrate openspec` import bridge;
there is no implicit runtime fallback. `cairn init` and `cairn change
create` create native layout artifacts.

## Local checks

```bash
python3 tools/export-policy.py --check
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo tigerstyle check --output-format json --artifact-dir artifacts/slotcar-run
nix run path:$PWD#cairn -- validate --root .
nix flake check path:$PWD -L
nix run path:$PWD#cairn -- doctor --root fixtures/basic
nix run path:$PWD#cairn -- status --root fixtures/basic
```

`nix flake check` includes policy freshness, Rust formatting, workspace tests,
Cairn validation, traceability coverage, Clippy, the Slotcar consumer check,
and a self-dogfood check that runs the built `cairn` CLI against this
repository plus the fixture repos. `cairn validate` checks change/task policy
plus EARS-style requirement strength and GIVEN/WHEN/THEN scenario structure for
accepted specs and active change delta specs. Slotcar artifacts should be written
under `artifacts/`, which is ignored by git.

## Traceability

Tracey configuration lives at `.config/tracey/config.styx` and scans accepted
specs plus Rust/Python implementation references:

```bash
tracey query status
tracey query uncovered
tracey query untested
python3 tools/check-tracey-coverage.py --root .
```

The Nix `tracey-coverage` check runs the deterministic helper so every accepted
`r[...]` requirement must have at least one `r[impl ...]` or `r[verify ...]`
reference in source-controlled code. Tracey remains optional tooling and is not
part of the `cairn` runtime.

## Running with Nix

```bash
nix run path:$PWD#cairn -- init --root /path/to/repo
nix run path:$PWD#cairn -- change create replace-openspec --root /path/to/repo
nix run path:$PWD#cairn -- change list --root fixtures/basic
nix run path:$PWD#cairn -- change show demo --root fixtures/basic
nix run path:$PWD#cairn -- spec list --root fixtures/basic
nix run path:$PWD#cairn -- spec show demo --root fixtures/basic
nix run path:$PWD#cairn -- status --root fixtures/basic
nix run path:$PWD#cairn -- doctor --root fixtures/basic
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- gate tasks demo --root fixtures/basic
nix run path:$PWD#cairn -- sync demo --root fixtures/basic
nix run path:$PWD#cairn -- migrate openspec --root fixtures/basic
```

`doctor [--root PATH] [--policy PATH]` is read-only and emits a compact JSON
health receipt for the selected layout, policy load status, discovered spec/change
counts, validation issue count, and diagnostic checks. Unlike `validate`, it
reports policy read/parse failures as JSON instead of aborting before a receipt.

`gate proposal|design|tasks <change>` evaluates AgentKit/OpenGate-compatible
stage semantics from Cairn-owned Nickel policy data and returns a JSON
receipt. Stable public gate fields are `change`, `stage`, `verdict`, `valid`,
`issues`, `policy`, `layout`, `policy_hash`, `input_hash`, and `receipt_hash`.
Those receipt/action/hash field declarations are owned by
`cairn-policy/default.ncl`, exported to `cairn-policy/generated/cairn-policy.json`,
and regression-tested against CLI JSON output. Runtime commands consume the
generated JSON policy and do not import, execute, or shell out to Nickel. The
gate also reads active change artifacts into a pure core snapshot: unknown
`r[...]` references in proposal/design/tasks fail, tasks-stage checkbox tasks
must reference a known delta requirement or carry an explicit `[depends:...]`
marker from the generated marker vocabulary, and `gate-exempt[...]` markers must
match the generated exemption marker contract (`reason=`, `owner=`, and ISO-like
`until=YYYY-MM-DD`).

`sync` and `archive` are dry-run by default. Use `--execute` only when you want
local filesystem mutation after reviewing the plan. Stable public lifecycle
receipt fields are `actions`, `blocked`, `change`, `dry_run`, `input_hash`,
`layout`, `mutated`, `mutation_manifest`, `plan_hash`, `policy`, `policy_hash`,
`reasons`, and `receipt_hash`; sync receipts also include `delta_specs`.
Lifecycle action entries expose stable `kind`, `path`, and `description` fields.
`input_hash`, `plan_hash`, and `receipt_hash` are computed from root-portable
JSON: checkout/temp-directory prefixes are normalized before hashing while
operator-facing `path` and `root` fields remain display paths. Executed
`sync`, `archive`, and `migrate` receipts include a `mutation_manifest` with
before/after BLAKE3 content hashes and a manifest hash; dry-run receipts set it
to `null`. Sync/archive
execution first enforces the configured deterministic gate when the base plan is
otherwise unblocked. Executed sync merges delta
requirement sections into accepted specs; it preserves accepted spec title,
purpose, and `## Requirements` wrapper content instead of copying partial delta
documents over accepted specs:

```bash
nix run path:$PWD#cairn -- sync ready --root /path/to/repo --execute
nix run path:$PWD#cairn -- archive ready --root /path/to/repo --execute
```

## Agent/CI determinism replay

`cairn determinism replay --root <fixture-or-repo>` is a read-only workspace
preflight for agents and CI. Its default case list is exported from
`cairn-policy/generated/cairn-policy.json` under `determinism_replay_policy`, so
case ids, command kinds, fixture roots, comparison modes, display-only fields,
hash fields, and normalization rules are reviewed as policy instead of hidden CLI
literals. Cases currently cover status, validate, proposal/design/tasks gates,
sync/archive/migration dry-runs, agent-receipt validation, release-readiness, MCP
describe, and selected MCP read-only calls. Replay copies fixture roots when
cross-root comparison is requested, runs cases repeatedly, and emits a single
deterministic receipt.

For CI triage, select one case with `--case <id>` or one policy group with
`--group <id>`, render compact logs with `--format summary` or `--format jsonl`,
and write the full artifact with `--out <path>`:

```bash
cairn determinism replay --root . --group smoke --format summary --out target/replay.json
```

Replay groups are policy-owned named suites (`smoke`, `agent`, `release`, and
`full`) whose membership lives in generated policy. Replay exits nonzero when
evidence is invalid, including unknown case IDs, unknown group IDs, selector
conflicts, hidden entropy, mutation-denied preflights, or mismatches. Stable public determinism replay fields are `command`, `root`, `valid`, `verdict`,
`available_cases`, `available_groups`, `selected_cases`, `selected_groups`,
`entropy_inventory`, `entropy_assumptions`, `entropy_policy_hash`,
`environment_attestation`, `summary`, `cases`, `skipped`, `mismatches`,
`nondeterminism_failures`, `mutability_observations`, `normalization_rules`,
and `receipt_hash`; the group
selector fields are `available_groups`, `selected_groups`. Case entries expose stable `id`,
`command_kind`, `args`, `fixture_root`, `expected_mutability`,
`comparison_modes`, `normalization_rules`, `display_only_paths`, `hash_fields`,
`runs`, and `portable_evidence_hash` fields. `entropy_inventory` is policy-owned
and declares each source `id`, class, source name, handling mode, receipt field,
and description; `entropy_assumptions` is the compact non-rejected subset and
`entropy_policy_hash` is a BLAKE3 hash of the reviewed inventory. Current
inventory coverage includes clock/date defaults, locale/timezone formatting,
cwd/root path normalization, rejected archive-date env vars, filesystem
order/metadata, external command versions, and Nix/build context. Rejected
entropy failures disclose source names and handling but never environment values.
`environment_attestation` is policy-owned and binds the replay to stable Cairn
package/version identity, generated policy hash, entropy policy hash, selected
cases/groups, replay scope, reviewed environment facts, and
`attestation_hash`. It intentionally omits raw environment values, usernames,
absolute checkout roots, and machine-specific temporary or store paths.
Mismatch entries expose stable `case`, `comparison`, `field_path`, `class`,
`expected`, `actual`, and `reproduce` fields. The default oracle rejects mutation flags and records
mutation observations as deterministic failures.

## Agent-run receipts

`cairn-core` includes pure helpers for deterministic agent-run receipts:
`agent_run_receipt_hash`, `agent_run_receipt_issues`, and
`evaluate_agent_run_completion`. Agent wrappers can use these to produce a
canonical receipt with `schema_version`, `task_id`, `agent_identity`,
`input_hash`, `plan_hash`, `tool_manifest`, `mutation_manifest`,
`verification_manifest`, `nondeterminism_manifest`, `replay_evidence`,
`verdict`, and `receipt_hash`. The `replay_evidence` object binds the agent
run to deterministic replay preflight evidence with stable
`replay_receipt_hash`, `replay_policy_hash`, `replay_selected_cases`,
`replay_selected_groups`, `replay_verdict`, and optional
`replay_environment_attestation_hash` fields, plus optional
display-only `replay_artifact_path`. Agent wrappers should run
`cairn determinism replay --group agent` and record the resulting `agent` group
and expanded case list. `agent-receipt validate <path>` checks that replay
evidence cites a passing replay verdict, the current generated policy hash, the
policy-owned `agent` replay group, and every case expanded from that group.
Missing, stale, failed, or insufficient replay evidence emits deterministic issue
codes such as `replay_evidence.missing`, `replay_evidence.policy_hash_stale`,
`replay_evidence.missing_group_coverage`, and
`replay_evidence.insufficient_coverage`. The CLI can replay a stored receipt
with `agent-receipt validate <path>` and emits deterministic validation JSON for
positive and negative fixtures. Stable public agent receipt validation fields
are `command`, `path`, `input_kind`, `valid`, `verdict`, `issues`,
`agent_receipt_hash`, `input_hash`, `stated_agent_receipt_hash`,
`current_replay_policy_hash`, `required_replay_group`, `required_replay_cases`, `replay_evidence`,
`max_content_bytes`, and `receipt_hash`. See `docs/agent-run-receipts.md` for
wrapper guidance, validation commands, and privacy rules. r[verify
cairn.supersede.agent_run_receipts.schema]
r[verify cairn.supersede.agent_receipt_replay_evidence.docs]
r[verify cairn.supersede.agent_receipt_replay_group_coverage.docs]
r[verify cairn.supersede.agent_run_receipts.completion_gate]

## MCP server

`cairn mcp` exposes Cairn lifecycle receipts through a bounded MCP-style
JSON-RPC tool surface. The default server lists read-only inspection tools and
dry-run planning tools only; mutation tools are omitted unless the server starts
with `--allow-mutations`. Tool responses include short text plus
`structuredContent` receipts with Cairn hashes and an `mcp` read/destructive
annotation object. See `docs/cairn-mcp.md`. r[verify
cairn.supersede.mcp_server.verification.docs]

## Migrate legacy OpenSpec content

`migrate openspec` is also dry-run by default. It copies accepted specs, active
changes, and archived changes from legacy `openspec/` into native `cairn/`
layout only when `--execute` is passed. Existing native destination files are
reported as blockers and are never overwritten. Stable public migration receipt
fields are `command`, `root`, `source_layout`, `destination_layout`, `actions`,
`conflicts`, `blocked`, `dry_run`, `mutated`, `mutation_manifest`, `policy`,
`policy_hash`, `source_manifest_hash`, `destination_manifest_hash`,
`manifest_hash`, and `receipt_hash`. Migration action entries expose stable
`source`, `destination`, `source_hash`, and `destination_hash` fields.

## Hermes skill source

`docs/hermes/cairn-workflow-skill.md` is the repo-maintained Hermes skill
source for future `cairn` work. It captures the FCIS boundary,
Cairn-native lifecycle workflow, legacy OpenSpec migration bridge,
Slotcar/Nix verification rails, command contracts, and known pitfalls without
transient session state or secrets. Copy or patch it into a local Hermes skill
named `cairn-workflow` when updating agent workflow knowledge.
