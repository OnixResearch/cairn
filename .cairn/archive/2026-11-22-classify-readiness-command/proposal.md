# Classify the readiness command and test the shipped command routes

## Why

The generated agent-workflow policy assigns each Cairn command invocation in
`command_catalog` to a command class, and each workflow descriptor lists the
classes it allows. At `9082e1ae` the catalog placed `cairn readiness <change>`
in the `gate` class. The `apply` workflow allows `readiness` but not `gate`,
yet its immediate action tells the agent to run `cairn readiness CHANGE`. An
agent that keeps to its allowed classes is told to run a command outside them,
or to skip the readiness observation the descriptor requires.

The catalog entry predates the `readiness` class: the compact entry guide
(`6a40f8d0`) added the catalog with `readiness` in `gate`, and the readiness
adoption (`163acb33`) added the `readiness` class and gave it to `apply`,
`verify`, `drain` and `archive` without moving the catalog entry. No catalog
entry used the `readiness` class.

The same defect affects `drain`: its immediate action names
`cairn drain-plan --view readiness`, which has no catalog entry and so no
class. Admission checked that descriptor classes exist, but nothing checked the
commands a descriptor names. Kiln found the `apply` case while reviewing its
Cairn re-vendor; every consumer that imports Cairn's default policy, Kiln and
Lattice included, inherits both.

## What Changes

- `cairn readiness <change> --root PATH` moves to the `readiness` class.
- A new catalog entry `drain-plan-readiness` places
  `cairn drain-plan --view readiness --root PATH` in the `readiness` class.
- A test over the shipped default policy checks that every Cairn command a
  workflow's text names resolves to a catalog entry in a class that workflow
  allows. It reads every rendered descriptor text field as Markdown, covers
  every workflow and catalog entry, and fails on the 9082e1ae catalog. The
  scanner is test code; policy admission does not read workflow text.
- Admission and lookup diagnostics truncate identifiers longer than 64 bytes.
- Core tests cover the shipped policy, the 9082e1ae catalog, permitted,
  unpermitted, uncatalogued, longest-prefix, fenced, formatted, linked,
  unpaired-marker and repeated-mention cases and long identifiers; a CLI test
  shows a policy with either defect still loads.
- `docs/generated-agent-workflows.md` and `CHANGELOG.md` describe the check.

## Impact

- **Files**: `cairn-policy/default.ncl`,
  `cairn-policy/generated/cairn-policy.json`,
  `crates/cairn-core/src/verified/generated_agent_workflow/admission.rs`,
  `crates/cairn-core/src/verified/generated_agent_workflow/command_routes.rs`,
  `crates/cairn-core/src/verified/generated_agent_workflow/tests.rs`,
  `crates/cairn-cli/tests/generated_agent_workflows.rs`, policy-hash fixtures
  and goldens, `docs/generated-agent-workflows.md`, `CHANGELOG.md`.
- **Compatibility**: the generated policy, its hash, the command catalog
  identity and the workflow catalog identity change, so installed entry guides
  report `stale-source`. Policy loading is unchanged: a policy that still
  carries either defect, including an unmodified copy of the 9082e1ae default,
  loads. Consumers re-vendor the default policy to pick up the fix.
- **Testing**: `policy export --check`, `cargo test --workspace`, Clippy,
  `cargo fmt --check`, `cairn validate`, the change gates and
  `nix flake check`.

## Out of Scope

- Matching command options. A named command resolves by its subcommand words;
  `cairn drain-plan` with and without `--view readiness` is one command.
- Checking entry-guide text, which renders catalog invocations directly and has
  no allowed classes.
- Granting authority. Command classes remain guidance; gates and policy stay
  authoritative.
