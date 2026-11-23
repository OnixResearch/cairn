# Classify the readiness command and admit only permitted command routes

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
- Admission of `generated_agent_workflow_policy` rejects a descriptor whose
  text names a Cairn command that resolves to no catalog entry, or to an entry
  whose class the descriptor does not allow. The rule reads every rendered
  descriptor text field and applies to every workflow and catalog entry.
- Core and CLI tests cover permitted, unpermitted, uncatalogued and
  longest-prefix cases, and replay both 9082e1ae defects against the shipped
  policy.
- `docs/generated-agent-workflows.md` and `CHANGELOG.md` describe the rule.

## Impact

- **Files**: `cairn-policy/default.ncl`,
  `cairn-policy/generated/cairn-policy.json`,
  `crates/cairn-core/src/verified/generated_agent_workflow/admission.rs`,
  `crates/cairn-core/src/verified/generated_agent_workflow/tests.rs`,
  `crates/cairn-cli/tests/generated_agent_workflows.rs`, policy-hash fixtures
  and goldens, `docs/generated-agent-workflows.md`, `CHANGELOG.md`.
- **Compatibility**: the generated policy, its hash, the command catalog
  identity and the workflow catalog identity change, so installed entry guides
  report `stale-source`. Policy loading admits the generated workflow policy, so
  a Cairn binary with this change refuses a policy that still carries either
  defect, including an unmodified copy of the 9082e1ae default. Consumers must
  re-vendor the default policy with this binary.
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
