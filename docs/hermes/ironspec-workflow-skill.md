---
name: cairn-workflow
description: >
  Work on the cairn Rust/Nix lifecycle engine with Slotcar-enforced
  FCIS boundaries, golden CLI contracts, fixture-backed behavior, and flake
  verification. Use when modifying cairn CLI/core/spec/test/flake behavior,
  implementing cairn changes, or maintaining native lifecycle/migration rails.
---

# Cairn Workflow

Use this Hermes skill for work in the `cairn` repository.

## Project shape

- Rust workspace with a Nix flake workflow.
- CLI shell: `crates/cairn-cli`.
- Pure deterministic core: `crates/cairn-core`.
- Native lifecycle content lives under `cairn/`.
- Legacy OpenSpec lifecycle content under `openspec/` is an explicit import/migration source, not a runtime fallback layout.
- Fixtures live under `fixtures/` and are part of the behavior contract.
- Slotcar input MUST stay `github:OnixResearch/slotcar`.
- Run the CLI through Nix as `nix run path:$PWD#cairn -- ...`.

## Operating mode

- For public CLI behavior, compatibility semantics, mutation, or roadmap work, write an `cairn`/OpenSpec change first.
- Small fixture/test/doc hardening can be direct when it does not alter public behavior.
- Prefer one narrow ROI slice at a time, then verify, archive the change, and commit.
- Treat upstream OpenSpec as a legacy import format, not the authority; `cairn` is intended to supersede it.

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
  - executed sync merges delta requirement sections into accepted specs and
    preserves accepted title/purpose/`## Requirements` wrapper content; do not
    expect partial delta docs to replace accepted spec files.
  - stable public plan receipt fields are `actions`, `blocked`, `change`, `dry_run`, `input_hash`, `layout`, `mutated`, `plan_hash`, `policy`, `policy_hash`, `reasons`, and `receipt_hash`; sync receipts also include `delta_specs`.
  - sync/archive action entries expose stable `kind`, `path`, and `description` fields.
  - `--execute` refuses mutation when the configured gate adds blockers to an otherwise unblocked plan.
  - blocked plans must not mutate, even with `--execute`.
- Migration:
  - `cairn migrate openspec --root PATH` emits a dry-run copy plan from legacy `openspec/` to native `cairn/`.
  - `--execute` copies accepted specs, active changes, and archives; it never deletes legacy content.
  - existing native destinations are blockers and must not be overwritten.
  - stable public migration receipt fields are `command`, `root`, `source_layout`, `destination_layout`, `actions`, `conflicts`, `blocked`, `dry_run`, `mutated`, `policy`, `policy_hash`, `source_manifest_hash`, `destination_manifest_hash`, `manifest_hash`, and `receipt_hash`.
  - migration action entries expose stable `source`, `destination`, `source_hash`, and `destination_hash` fields.

## EARS-style validation contract

- Requirements need normative strength words such as `MUST`, `SHALL`, `SHOULD`, or `MAY`.
- Every `#### Scenario:` block needs `GIVEN`, `WHEN`, and `THEN` clauses.
- Keep validation decisions in pure core over in-memory `SpecSource` markdown; CLI owns file reads and JSON receipts.

## Verification rails

Run the full gate after substantive changes:

```bash
nix develop -c bash -c 'python3 tools/export-policy.py --check && cargo fmt --check && cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo tigerstyle check --output-format json --artifact-dir artifacts/slotcar-run'
openspec validate --all --strict
nix flake check "path:$PWD" -L
```

Useful focused checks:

```bash
nix run "path:$PWD#cairn" -- validate --root .
nix build "path:$PWD#checks.x86_64-linux.self-dogfood" --no-link -L
nix build "path:$PWD#checks.x86_64-linux.openspec-validate" --no-link -L
nix build "path:$PWD#checks.x86_64-linux.tracey-coverage" --no-link -L
```

## Tracey coverage rail

- Tracey config lives at `.config/tracey/config.styx`.
- Requirement markers in accepted specs use `r[...]`; implementation and test
  comments should use `r[impl ...]` and `r[verify ...]`.
- Run `tracey query status`, `tracey query uncovered`, and
  `tracey query untested` from the repo root when curating coverage.
- `tools/check-tracey-coverage.py --root .` is the deterministic bootstrap rail
  used by `nix flake check`; it ensures every accepted requirement has at least
  one source-controlled implementation or verification reference without making
  Tracey a runtime dependency.

## Known pitfalls

- Direct host `cargo` may be unavailable; use `nix develop -c ...`.
- `pkgs.openspec` timed out inside a Nix flake check; the repo's OpenSpec check intentionally uses built `cairn validate --root .` plus JSON assertions.
- Slotcar Nix checks need a C compiler/linker; keep `pkgs.stdenv.cc` in the check inputs.
- Remote builder warnings can appear; treat them as non-blocking when local checks pass.
- The repo may have no git remote; commit locally but do not claim a push.

## Skill maintenance

- Update this source when `cairn` changes workflow contracts, verification rails, or command semantics.
- Keep it class-level and reusable; do not add transient session progress or credentials.
- To install/update locally, copy this content into a Hermes skill named `cairn-workflow` or patch the existing Hermes skill with the same stable guidance.
