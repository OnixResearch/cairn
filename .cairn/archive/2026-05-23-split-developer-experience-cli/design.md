# Design: developer-experience CLI module split

## Boundaries

- `developer_experience/mod.rs`: public façade for `main.rs`, preserving `run_template`, `run_scaffold`, and `run_discover_specs` entrypoints.
- `template.rs`: onboarding template list/apply command handling and template-specific existing-path logic.
- `scaffold.rs`: prompt-to-change `scaffold change` command handling and request/input file handling.
- `discovery.rs`: reverse-spec discovery CLI orchestration, directory include expansion, text-file filtering, draft rendering, and discovery receipt construction.
- `common.rs`: shared safe write and planned-file helpers used by template/scaffold/discovery shells.

## Behavior preservation

The split is mechanical: dispatch names, error strings, policy IDs, receipt keys, mutation manifests, and safe-path behavior remain unchanged. The façade keeps the module path expected by `main.rs`, so no external command call sites change.

## Tracey coverage

The accepted developer-experience split requirements are covered by implementation refs in the façade and existing command-family refs in the moved modules. Focused CLI tests continue to verify template, scaffold, and discovery receipts.

## Verification strategy

Run focused developer-experience CLI tests first, then formatting, workspace tests, clippy, Cairn validation, Tracey coverage, and final Nix flake check before commit.
