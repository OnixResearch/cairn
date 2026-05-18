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
`layout`, `mutated`, `plan_hash`, `policy`, `policy_hash`, `reasons`, and
`receipt_hash`; sync receipts also include `delta_specs`. Lifecycle action
entries expose stable `kind`, `path`, and `description` fields. Sync/archive
execution first enforces the configured deterministic gate when the base plan is
otherwise unblocked. Executed sync merges delta
requirement sections into accepted specs; it preserves accepted spec title,
purpose, and `## Requirements` wrapper content instead of copying partial delta
documents over accepted specs:

```bash
nix run path:$PWD#cairn -- sync ready --root /path/to/repo --execute
nix run path:$PWD#cairn -- archive ready --root /path/to/repo --execute
```

## Agent-run receipts

`cairn-core` includes pure helpers for deterministic agent-run receipts:
`agent_run_receipt_hash`, `agent_run_receipt_issues`, and
`evaluate_agent_run_completion`. Agent wrappers can use these to produce a
canonical receipt with `schema_version`, `task_id`, `agent_identity`,
`input_hash`, `plan_hash`, `tool_manifest`, `mutation_manifest`,
`verification_manifest`, `nondeterminism_manifest`, `verdict`, and
`receipt_hash`. Successful receipts must carry verification evidence; side-effect
receipts must cite a pre-mutation plan hash; hidden or forbidden entropy fails
the completion gate. See `docs/agent-run-receipts.md` for wrapper guidance and
privacy rules. r[verify cairn.supersede.agent_run_receipts.schema]
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
`conflicts`, `blocked`, `dry_run`, `mutated`, `policy`, `policy_hash`,
`source_manifest_hash`, `destination_manifest_hash`, `manifest_hash`, and
`receipt_hash`. Migration action entries expose stable `source`, `destination`,
`source_hash`, and `destination_hash` fields.

## Hermes skill source

`docs/hermes/cairn-workflow-skill.md` is the repo-maintained Hermes skill
source for future `cairn` work. It captures the FCIS boundary,
Cairn-native lifecycle workflow, legacy OpenSpec migration bridge,
Slotcar/Nix verification rails, command contracts, and known pitfalls without
transient session state or secrets. Copy or patch it into a local Hermes skill
named `cairn-workflow` when updating agent workflow knowledge.
