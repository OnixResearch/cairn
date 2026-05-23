# cairn

`cairn` is a Rust lifecycle engine for native repository lifecycle work.
Repositories use a `cairn/` lifecycle layout; legacy `openspec/` trees remain
supported only as an explicit import/migration source. Cairn adds a
Nickel-authored policy/config/schema layer and a Slotcar-enforced
functional-core / imperative-shell boundary.

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
cairn policy export --check
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
repository plus the fixture repos. Maintained gates use Rust CLI surfaces and
Nickel policy evaluation; Cairn does not keep checked-in Python or shell helper
scripts for these checks. `cairn validate` checks change/task policy
plus EARS-style requirement strength and GIVEN/WHEN/THEN scenario structure for
accepted specs and active change delta specs. Slotcar artifacts should be written
under `artifacts/`, which is ignored by git.

## Traceability

Tracey configuration lives at `.config/tracey/config.styx` and scans accepted
specs plus Rust/Nickel implementation references:

```bash
tracey query status
tracey query uncovered
tracey query untested
cairn tracey coverage --root .
```

The Nix `tracey-coverage` check runs the deterministic helper so every accepted
`r[...]` requirement must have at least one `r[impl ...]` or `r[verify ...]`
reference in source-controlled code. Tracey remains optional tooling and is not
part of the `cairn` runtime.

## Running with Nix

```bash
nix run path:$PWD#cairn -- init --root /path/to/repo
nix run path:$PWD#cairn -- change create native-lifecycle --root /path/to/repo
nix run path:$PWD#cairn -- template list --root /path/to/repo
nix run path:$PWD#cairn -- template apply agent-workflow --root /path/to/repo --execute
nix run path:$PWD#cairn -- scaffold change first-change --root /path/to/repo --request "Add the first reviewed capability" --execute
nix run path:$PWD#cairn -- discover-specs --root /path/to/repo --include src --include docs --out target/reverse-spec-draft.md
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

`template list` and `template apply <id>` provide a reviewed first-use catalog for
`agent-workflow`, `generic-cli`, `rust-nix`, `docs-lifecycle`, and
`release-evidence` onboarding profiles. Template application is preview-first
unless `--execute` is passed; executed receipts list created files and BLAKE3-bound
mutation manifests. Templates create quickstart artifacts only and do not write
accepted specs, contact network services, or claim remote publishing/signing.
r[impl cairn.developer_experience.onboarding_templates.quickstart]
r[impl cairn.developer_experience.onboarding_template_catalog_expansion.docs]

`scaffold change <name> --request <text>` turns explicit local request material
into a gate-ready draft change package with proposal, design, tasks, and a delta
spec. The command records an input hash, supports preview or `--execute`, and
uses deterministic templates rather than built-in LLM/network inference. r[impl
cairn.developer_experience.prompt_to_change_scaffold.docs]

`discover-specs --include <path>` is a read-only reverse-spec discovery rail for
existing repositories. Includes may name explicit files or safe repository-relative
directories; directory includes expand deterministically over text-like source,
docs, tests, and config files while skipping hidden/build/vendor artifacts. The
command inventories explicit evidence paths, emits review-needed candidate
requirements with evidence paths and input hashes, and only writes draft artifacts
when `--out` is provided. Candidates are not accepted specs or implementation proof
until reviewed through the normal Cairn lifecycle.
r[impl cairn.developer_experience.reverse_spec_discovery.docs]
r[impl cairn.developer_experience.reverse_spec_directory_inventory.docs]

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
Saved replay receipts can be rechecked with
`cairn determinism verify-receipt <receipt.json> --root .`; verification is
read-only and recomputes the replay receipt hash, compares current generated
policy and attestation hashes, validates selected cases/groups, and emits
nonzero deterministic failure receipts for malformed, stale, or tampered replay
evidence. Stable public replay verification fields are `command`, `root`,
`path`, `valid`, `verdict`, `issues`, `current_replay_policy_hash`,
`replay_policy_hash`, `replay_receipt_hash`, `stated_replay_receipt_hash`,
`selected_cases`, `selected_groups`, `environment_attestation_hash`, and
`receipt_hash`. r[verify cairn.supersede.replay_receipt_verification.docs]
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
`max_content_bytes`, and `receipt_hash`. Wrappers that publish multiple artifacts
can verify the whole chain with
`cairn agent-receipt verify-bundle <bundle.json> --root .`; the bundle references
an agent receipt, saved replay receipt, saved replay verification receipt,
optional release-readiness receipt, and optional coverage-audit receipt. Stable public agent evidence bundle verification fields
are `command`, `root`, `valid`, `verdict`, `issues`,
`current_replay_policy_hash`, `required_replay_group`, `agent_receipt_hash`,
`replay_receipt_hash`, `replay_verification_receipt_hash`,
`environment_attestation_hash`, `release_readiness_receipt_hash`,
`coverage_audit_receipt_hash`, and
`receipt_hash`. The first-class publication path is
`cairn evidence export --root . --out target/cairn-evidence/`, which materializes
coverage audit, replay, replay verification, release-readiness, agent bundle,
bundle verification, evidence manifest, and manifest verification artifacts in a
portable directory. Stable public evidence export fields are `command`, `root`,
`output`, `valid`, `verdict`, `issues`, `artifacts`, `manifest_hash`,
`manifest_verification_receipt_hash`, and `receipt_hash`. Consumers can verify a
received export with `cairn evidence verify --root . --bundle target/cairn-evidence/`,
which rehashes exported artifacts, composes the nested verifiers, and emits
stable fields `command`, `root`, `bundle`, `valid`, `verdict`, `issues`,
`artifacts`, `export_receipt_hash`, `manifest_hash`,
`manifest_verification_receipt_hash`, and `receipt_hash`. Consumers can then
materialize a bounded local copy with
`cairn evidence import --root . --bundle target/cairn-evidence/ --into target/cairn-imported-evidence/`,
which verifies first, copies only declared export artifacts, rejects undeclared
bundle files, and emits stable fields `command`, `root`, `bundle`, `into`,
`valid`, `verdict`, `issues`, `artifacts`, `verification_receipt_hash`, and
`receipt_hash`. Imported evidence can be republished with
`cairn evidence republish --root . --imported target/cairn-imported-evidence/ --out target/cairn-republished-evidence/`,
which verifies the import receipt, rehashes declared imported artifacts, rejects
extra or unsafe imported files, and emits stable fields `command`, `root`,
`imported`, `out`, `valid`, `verdict`, `issues`, `artifacts`,
`import_receipt_hash`, and `receipt_hash`. Published bundles can be cataloged
with `cairn evidence index --root . --bundle target/cairn-republished-evidence/ --out target/cairn-evidence-index.json`
and checked with `cairn evidence index verify --root . --index target/cairn-evidence-index.json`.
The index rehashes declared bundle artifacts, records the bundle kind, canonical
bundle receipt, receipt chain, artifact roles/paths/hashes, and emits stable
fields `command`, `root`, `bundle`, `out`, `valid`, `verdict`, `issues`,
`bundle_kind`, `bundle_receipt_hash`, `receipt_chain`, `policy_hash`,
`artifacts`, and `receipt_hash` without raw host paths. Multiple published
indexes can be aggregated with
`cairn evidence registry build --root . --index target/cairn-evidence-index.json --out target/cairn-evidence-registry.json --channel stable --release 2026-05-19 --platform x86_64-linux --compatibility-class current`
and checked with
`cairn evidence registry verify --root . --registry target/cairn-evidence-registry.json`.
Mirrored or offline copies can be checked before consumption with
`cairn evidence registry mirror-verify --root . --registry target/cairn-evidence-registry.json --mirror target`.
The mirror verifier reuses the registry's safe `index_relative` entries, rehashes
mirrored indexes, preserves labels (`channel`, `release`, `platform`,
`compatibility_class`), index hashes, bundle kinds, bundle receipt hashes,
receipt chains, artifact hashes, and policy hashes, and emits stable fields `command`, `root`, `out` or `registry`, `valid`,
`verdict`, `issues`, `entry_count`, `entries`, `registry_hash`, and
`receipt_hash` without raw host paths. Downstream agents can consume a registry
with
`cairn evidence registry select --root . --registry target/cairn-evidence-registry.json --channel stable --release 2026-05-19 --platform x86_64-linux --bundle-kind republished --out target/selected-cairn-evidence-index.json`
or produce a metadata-only plan with
`cairn evidence registry fetch-plan --root . --registry target/cairn-evidence-registry.json --out target/cairn-evidence-fetch-plan.json`.
Selection verifies the registry first, requires exactly one matching entry, and
binds selectors, registry receipt hash, registry verification receipt hash, and
selected entry metadata into a portable receipt. Fetch plans verify the registry
first, sort plan entries deterministically, and list safe index refs, label
metadata, receipt chains, artifact roles/paths/hashes, and policy hashes without
fetching network
content, credentials, raw host paths, or environment values. A verified plan can
then be locally materialized from an offline mirror/source directory with
`cairn evidence registry fetch-execute --root . --plan target/cairn-evidence-fetch-plan.json --source target/cairn-republished-evidence/ --out target/cairn-fetch-cache/`.
Fetch execution rejects unverified plans and unsafe relative paths, verifies each
artifact's BLAKE3 hash before and after copying, emits deterministic failure
receipts for missing or tampered artifacts, and records stable fields such as
`fetch_plan_receipt_hash`, `registry_receipt_hash`,
`registry_verification_receipt_hash`, `materialized_count`, `materialized`, and
`receipt_hash`; it still performs no network transport, credential handling, or
remote trust validation. r[cairn.supersede.fetch_execute_policy_docs.docs.flow]
r[cairn.supersede.fetch_execute_policy_docs.docs.non_claims] The outer publication/index layer can also be generated with
`cairn evidence-manifest generate --bundle bundle.json --agent-receipt agent.json --replay replay.json --replay-verification replay-verify.json --registry registry.json --registry-verification registry-verify.json --registry-selection registry-selection.json --registry-fetch-plan registry-fetch-plan.json --out manifest.json --root .`
and verified with `cairn evidence-manifest verify <manifest.json> --root .`; manifest artifacts
list stable roles such as `agent_evidence_bundle`, `agent_receipt`,
`replay_receipt`, `replay_verification`, optional `release_readiness`, and
optional `coverage_audit`, optional `evidence_registry`, optional
`evidence_registry_verification`, optional `evidence_registry_selection`, and
optional `evidence_registry_fetch_plan` with
expected hashes, schema labels, and optional public signer/anchor metadata.
Generation produces canonical role ordering and artifact hashes so wrappers do
not need custom JSON or hashing glue. Verification is read-only, composes
existing validators, and does not claim cryptographic signature or remote anchor
validity. Stable public evidence manifest verification fields are `command`,
`root`, `valid`, `verdict`, `issues`, `manifest_hash`, `artifacts`,
`coverage_audit_receipt_hash`, `signer_metadata_hash`, `anchor_metadata_hash`, and `receipt_hash`. See
`docs/agent-run-receipts.md` for wrapper guidance,
validation commands, and privacy rules. The checked-in release evidence index for verified payload commit `b253c9c` lives as an immutable audited snapshot at `docs/release-evidence/payload-b253c9c.md`; checked-in files must not claim to be the mutable current HEAD. Operators can generate the current clean HEAD evidence index into an artifact path with `cairn release-evidence index --root . --out target/release-evidence-current-head/index.md --artifact-dir target/release-evidence-current-head`; the command composes the existing release-readiness, evidence export, verify, import, republish, index, and index-verify rails, keeps bulky generated receipts in the artifact directory, fails closed when the checkout is dirty, and records only the evaluated payload HEAD rather than any later documentation commit. Operators can check generated or audited indexes with `cairn release-evidence check-index --root . --index target/release-evidence-current-head/index.md --mode current-head` or `--mode audited-payload`; release readiness also checks the conventional checked-in `docs/release-evidence-current-head.md` path if it exists so stale current-head claims fail closed. Producers can materialize a local publication tree from verified generated evidence with `cairn release-evidence publish --root . --index target/release-evidence-current-head/index.md --artifact-dir target/release-evidence-current-head --publish-dir target/release-evidence-publish --channel stable --release YYYY-MM-DD --retention local`; the command verifies the source evidence index, copies the Markdown index plus required artifacts under safe relative paths, emits `manifest.json` and a receipt with source index, source receipt, verification receipt, materialized artifact, label, and manifest hashes, and rejects unsafe paths, hash drift, malformed labels, and invalid source evidence. Operators can plan local retention over materialized publication trees with `cairn release-evidence retention-plan --root . --publish-root target/release-evidence-publish-root --out target/release-evidence-retention-plan.json --channel stable --keep-latest 1` and dry-run or execute local rotation with `cairn release-evidence rotate --root . --plan target/release-evidence-retention-plan.json [--execute]`; retention planning verifies each candidate `manifest.json`/`receipt.json`, emits deterministic keep/supersede/prune-candidate decisions, and rotation verifies the saved plan before writing a mutation manifest. The generated index, materialized publication tree, and retention/rotation plan are local/trusted evidence only: they do not create or move tags, validate credentials or network transport, prove remote mirror trust, validate signatures, upload to public release pages, delete remote evidence, or claim public unattended production readiness. r[impl release_evidence.automated_current_head_index.operator_docs] r[verify release_evidence.automated_current_head_index.operator_docs] r[verify release_evidence.automated_current_head_index.operator_docs.non_claims] r[impl release_evidence.publish_materialization.operator_docs] r[verify release_evidence.publish_materialization.operator_docs] r[verify release_evidence.publish_materialization.operator_docs.non_claims] r[impl release_evidence.retention_rotation.operator_docs] r[verify release_evidence.retention_rotation.operator_docs] r[verify release_evidence.retention_rotation.operator_docs.non_claims] r[verify release_evidence.current_head_staleness_guard.operator_docs] r[verify release_evidence.current_head_staleness_guard.archive_ready] r[verify release_evidence.current_head_evidence_index.behavior_preserved.focused_checks] r[verify
cairn.supersede.agent_run_receipts.schema]
r[verify cairn.supersede.agent_receipt_replay_evidence.docs]
r[verify cairn.supersede.agent_receipt_replay_group_coverage.docs]
r[verify cairn.supersede.agent_evidence_bundle_verification.docs]
r[verify cairn.supersede.signed_evidence_manifest.docs]
r[verify cairn.supersede.evidence_manifest_generate.docs]
r[verify cairn.supersede.determinism_coverage_release_evidence.release]
r[verify cairn.supersede.determinism_coverage_release_evidence.bundle_manifest]
r[verify cairn.supersede.determinism_coverage_release_evidence.docs]
r[verify cairn.supersede.determinism_coverage_release_evidence.archive]
r[verify cairn.supersede.agent_run_receipts.completion_gate]

r[impl release_evidence.current_head_staleness_guard.operator_docs]
r[verify release_evidence.current_head_staleness_guard.operator_docs]
r[verify release_evidence.current_head_staleness_guard.archive_ready]

## MCP server

`cairn mcp` exposes Cairn lifecycle receipts through a bounded MCP-style
JSON-RPC tool surface. The default server lists read-only inspection tools and
dry-run planning tools only; mutation tools are omitted unless the server starts
with `--allow-mutations`. Tool responses include short text plus
`structuredContent` receipts with Cairn hashes and an `mcp` read/destructive
annotation object. See `docs/cairn-mcp.md`. r[verify
cairn.supersede.mcp_server.verification.docs]

## Import legacy lifecycle content

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
Cairn-native lifecycle workflow, explicit legacy migration bridge,
Slotcar/Nix verification rails, command contracts, and known pitfalls without
transient session state or secrets. Copy or patch it into a local Hermes skill
named `cairn-workflow` when updating agent workflow knowledge.
