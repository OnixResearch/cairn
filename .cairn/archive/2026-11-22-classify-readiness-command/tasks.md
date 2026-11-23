# Tasks

- [x] [serial] [task:classify] Move `cairn readiness` to the `readiness` class, catalog `cairn drain-plan --view readiness` as `readiness`, and regenerate the policy export. r[cairn.generated_agent_workflows.command_routes] r[cairn.generated_agent_workflows.command_routes.permitted]
- [x] [serial] [task:check] [after:classify] Add a test-only scanner that reads every rendered descriptor field as Markdown and reports a named Cairn command outside the catalog or outside the descriptor's allowed classes; run it over the shipped default policy. Policy admission does not read workflow text. r[cairn.generated_agent_workflows.command_routes] r[cairn.generated_agent_workflows.command_routes.refused]
- [x] [serial] [task:tests] [after:check] Add core tests for the shipped policy, the 9082e1ae catalog, permitted, longest-prefix, unpermitted, uncatalogued, every-field, fenced, formatted, linked, unpaired-marker and repeated-mention cases, and a CLI test that a policy with either 9082e1ae defect still loads. r[cairn.generated_agent_workflows.command_routes.permitted] r[cairn.generated_agent_workflows.command_routes.refused]
- [x] [serial] [task:review] [after:tests] Bound identifiers in admission and lookup diagnostics to 64 bytes with a truncation marker. r[cairn.generated_agent_workflows.command_routes]
- [x] [serial] [task:fixtures] [after:review] Refresh policy-hash fixtures and goldens for the new generated policy. r[cairn.generated_agent_workflows.command_routes]
- [x] [serial] [task:docs] [after:fixtures] Document the rule in `docs/generated-agent-workflows.md` and the changelog. r[cairn.generated_agent_workflows.command_routes]
- [x] [serial] [task:checks] [after:docs] Run the policy export check, workspace tests, Clippy, `cargo fmt --check`, `cairn validate`, the change gates and `nix flake check`. r[cairn.generated_agent_workflows.command_routes]

## Validation and completion

- [x] [serial] [task:lifecycle] [after:checks] Sync the delta, record the evidence and archive only after every required check passes. r[cairn.generated_agent_workflows.command_routes]
