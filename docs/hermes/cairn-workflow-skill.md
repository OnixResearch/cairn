---
name: cairn-workflow
description: >
  Work on the Cairn Rust/Nix lifecycle engine in /home/brittonr/git/cairn:
  native Cairn specs and archives, MCP tools, deterministic receipts,
  release-readiness evidence, Tracey coverage, Slotcar FCIS checks, and Nix
  verification. Use for Cairn CLI/core/spec/test/flake/docs work.
---

# Cairn Workflow

<!-- r[verify cairn.supersede.hermes_skill] r[verify cairn.supersede.hermes_skill.discover] -->
<!-- r[verify cairn.supersede.hermes_skill.canonical_cairn] r[verify cairn.supersede.hermes_skill.canonical_cairn.path] -->
<!-- r[verify cairn.supersede.hermes_skill.canonical_cairn.live] r[verify cairn.supersede.hermes_skill.redirect] -->
<!-- r[verify cairn.supersede.hermes_skill.redirect.local] r[verify cairn.supersede.hermes_skill.contracts] r[verify cairn.supersede.hermes_skill.audit] -->

Use this Hermes skill for work in `/home/brittonr/git/cairn`. Cairn is the canonical product, repository, CLI, package, lifecycle layout, and skill name. Treat `ironspec` and upstream `openspec` references as legacy aliases, historical archives, or explicit migration inputs unless the task is specifically about compatibility.

## Project shape

- Rust workspace with Nix flake workflow.
- CLI shell: `crates/cairn-cli`.
- Pure deterministic core: `crates/cairn-core`.
- Native lifecycle content lives under `cairn/`.
- Legacy OpenSpec lifecycle content under `openspec/` is an explicit import/migration source, not a runtime fallback layout.
- Fixtures live under `fixtures/` and are part of the behavior contract.
- Slotcar input MUST stay `github:OnixResearch/slotcar`.
- Run the CLI through Nix as `nix run path:$PWD#cairn -- ...`.

## Operating mode

- For public CLI behavior, compatibility semantics, mutation, architecture, receipt schemas, or roadmap work, write a native `cairn/changes/<slug>/` package first.
- Small fixture/test/doc hardening can be direct when it does not alter public behavior.
- Prefer one narrow ROI slice at a time, then verify, sync/archive the change when applicable, commit, and push.
- Treat upstream OpenSpec as a legacy import/change-management compatibility surface, not the primary Cairn authority.

## Native change package workflow

Create changes with:

```bash
nix run path:$PWD#cairn -- change create <slug> --root .
```

A complete change normally includes:

- `proposal.md`
- `design.md`
- `tasks.md`
- `specs/<accepted-spec>/spec.md`

Validate the package before implementation or committing spec-only work:

```bash
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- gate proposal <slug> --root .
nix run path:$PWD#cairn -- gate design <slug> --root .
nix run path:$PWD#cairn -- gate tasks <slug> --root .
```

Task markers must reference requirement IDs present in that change delta or use explicit dependency markers. Cross-spec refs to already-accepted IDs can fail the tasks gate.

## FCIS boundary

- `cairn-core` is the functional core: pure deterministic transforms only.
- Keep pure code under `crates/cairn-core/src/verified/` so Slotcar checks cover it.
- Filesystem, environment, process execution, stdout/stderr, mutation, and JSON presentation belong in `crates/cairn-cli`.
- Do not add filesystem traversal, environment reads, process execution, clocks, async, `println!`, `eprintln!`, or JSON rendering helpers to core.

Core purity smoke check:

```bash
rg 'std::fs|fs::|std::env|std::process|println!|eprintln!|read_dir|read_to_string|to_json|std::time|tokio::|async' crates/cairn-core/src
```

Expected result: no matches.

## Current command contracts

- Native lifecycle/read commands:
  - `cairn init --root PATH`
  - `cairn change create <name> --root PATH`
  - `cairn change list --root PATH`
  - `cairn change show <name> --root PATH`
  - `cairn spec list --root PATH`
  - `cairn spec show <id> --root PATH`
  - native `cairn/` layout is used for normal runtime commands; legacy `openspec/` is migration-only and is not an implicit fallback.
- Diagnostics:
  - `cairn doctor --root PATH [--policy PATH]`
  - read-only receipt fields include `valid`, `layout`, spec/change counts, validation issue count, and named checks.
  - policy read/parse failures are reported as JSON checks rather than process failures.
- Validation:
  - `cairn validate --root PATH`
  - receipt fields include `issues`, `change_issues`, `spec_issues`, and `specs_validated`.
  - accepted specs and active delta specs are checked for EARS-style quality.
- Gate policy:
  - `cairn gate proposal <change> --root PATH`
  - `cairn gate design <change> --root PATH`
  - `cairn gate tasks <change> --root PATH`
  - stable public receipt fields are `change`, `stage`, `verdict`, `valid`, `issues`, `policy`, `layout`, `policy_hash`, `input_hash`, and `receipt_hash`.
  - AgentKit/OpenGate is a policy-shape reference only; Cairn owns the Nickel policy under `cairn-policy/` and does not depend on AgentKit at runtime.
  - Nickel owns generated receipt/action/hash schema declarations, marker vocabulary, exemption marker required/date fields, and semantic policy invariants. Runtime Rust consumes `cairn-policy/generated/cairn-policy.json`; do not add runtime Nickel execution.
  - Gates read proposal/design/tasks/delta specs in the CLI, pass a pure snapshot to core, and fail on unknown `r[...]` references, unlinked tasks at the tasks gate, or malformed `gate-exempt[...]` markers that do not match the generated exemption marker contract.
- Sync/archive:
  - `sync <change>` and `archive <change>` are dry-run by default.
  - `--execute` is required for mutation.
  - executed sync merges delta requirement sections into accepted specs and preserves accepted title/purpose/`## Requirements` wrapper content; do not expect partial delta docs to replace accepted spec files.
  - stable public plan receipt fields are `actions`, `blocked`, `change`, `dry_run`, `input_hash`, `layout`, `mutated`, `plan_hash`, `policy`, `policy_hash`, `reasons`, and `receipt_hash`; sync receipts also include `delta_specs`.
  - sync/archive action entries expose stable `kind`, `path`, and `description` fields.
  - `--execute` refuses mutation when the configured gate adds blockers to an otherwise unblocked plan.
  - blocked plans must not mutate, even with `--execute`.
  - For real archives, set `CAIRN_ARCHIVE_DATE=$(date +%F)` before `cairn archive <change> --execute`.
- Migration:
  - `cairn migrate openspec --root PATH` emits a dry-run copy plan from legacy `openspec/` to native `cairn/`.
  - `--execute` copies accepted specs, active changes, and archives; it never deletes legacy content.
  - existing native destinations are blockers and must not be overwritten.
  - stable public migration receipt fields are `command`, `root`, `source_layout`, `destination_layout`, `actions`, `conflicts`, `blocked`, `dry_run`, `mutated`, `policy`, `policy_hash`, `source_manifest_hash`, `destination_manifest_hash`, `manifest_hash`, and `receipt_hash`.
  - migration action entries expose stable `source`, `destination`, `source_hash`, and `destination_hash` fields.
- MCP and agent receipts:
  - `cairn mcp --describe`
  - `cairn mcp --call <tool> '<json>'`
  - `cairn mcp --allow-mutations` is required for mutation-capable tools.
  - MCP tool schemas are strict object schemas with `additionalProperties=false`.
  - Default mutation denial is checked before argument validation for mutation-capable tools.
  - `cairn agent-receipt validate <path>` validates deterministic agent-run receipt fixtures.
  - `cairn agent-receipt verify-bundle <bundle.json> --root .` validates a persisted agent/CI evidence chain across an agent receipt, saved replay receipt, saved replay verification receipt, and optional release-readiness receipt.
  - `cairn agent-receipt smoke-mcp --root .` proves the MCP catalog, receipt validation path, strict invalid-argument behavior, and default mutation denial compose.
- Release readiness:
  - `cairn release-readiness --root .` emits a deterministic local operator bundle over Cairn validation, Tracey coverage, MCP smoke, repository gate summary, and receipt hashes.
  - Release-readiness receipts should be byte-for-byte reproducible for the same root.

## EARS-style validation contract

- Requirements need normative strength words such as `MUST`, `SHALL`, `SHOULD`, or `MAY`.
- Every `#### Scenario:` block needs `GIVEN`, `WHEN`, and `THEN` clauses.
- Keep validation decisions in pure core over in-memory `SpecSource` markdown; CLI owns file reads and JSON receipts.

## Verification rails

Run the full gate after substantive changes:

```bash
nix develop -c bash -c 'python3 tools/export-policy.py --check && cargo fmt --check && cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings'
nix run path:$PWD#cairn -- validate --root .
python3 tools/check-tracey-coverage.py --root .
nix flake check "path:$PWD" -L
```

Useful focused checks:

```bash
nix run "path:$PWD#cairn" -- validate --root .
nix run "path:$PWD#cairn" -- release-readiness --root .
nix build "path:$PWD#checks.x86_64-linux.self-dogfood" --no-link -L
nix build "path:$PWD#checks.x86_64-linux.tracey-coverage" --no-link -L
nix build "path:$PWD#checks.x86_64-linux.slotcar" --no-link -L
```

Remote builder warnings for `ssh-ng://root@10.10.10.1` are non-blocking if Nix continues locally and the requested checks pass.

## Tracey coverage rail

- Tracey config lives at `.config/tracey/config.styx`.
- Requirement markers in accepted specs use `r[...]`; implementation and test comments should use `r[impl ...]` and `r[verify ...]`.
- Run `tracey query status`, `tracey query uncovered`, and `tracey query untested` from the repo root when curating coverage.
- `tools/check-tracey-coverage.py --root .` is the deterministic bootstrap rail used by `nix flake check`; it ensures every accepted requirement has at least one source-controlled implementation or verification reference without making Tracey a runtime dependency.

## Known pitfalls

- Direct host `cargo` may be unavailable; use `nix develop -c ...`.
- Nix flakes snapshot tracked/staged files. Stage new source/doc files before `nix flake check` if the check needs them.
- `cairn sync <change> --execute` can report success without promoting a direct/full spec-shaped change body when the change spec lacks explicit delta headings. Before archive commit, search accepted specs for new `r[...]` IDs; if absent, manually append/merge the archived requirement text, rerun Cairn validation and Tracey coverage, then commit the repair with the archive.
- Historical archives and compatibility code may mention `openspec` or `ironspec`; do not churn those unless the task is explicitly a migration/rename cleanup.
- Slotcar Nix checks need a C compiler/linker; keep `pkgs.stdenv.cc` in check inputs.
- The canonical repo at `/home/brittonr/git/cairn` has origin `OnixResearch/cairn`; after verified slices, commit and push `main`.

## Skill maintenance

- Update this source when Cairn changes workflow contracts, verification rails, or command semantics.
- Keep it class-level and reusable; do not add transient session progress or credentials.
- Use this skill for Cairn work instead of `ironspec-workflow` or generic OpenSpec skills, except when the task is specifically about legacy OpenSpec compatibility.
