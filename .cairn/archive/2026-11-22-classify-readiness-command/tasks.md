# Tasks

- [x] [serial] [task:classify] Move `cairn readiness` to the `readiness` class, catalog `cairn drain-plan --view readiness` as `readiness`, and regenerate the policy export. r[cairn.generated_agent_workflows.command_routes] r[cairn.generated_agent_workflows.command_routes.permitted]
- [x] [serial] [task:admit] [after:classify] Reject descriptors that name a Cairn command outside the catalog or outside their allowed classes, over every rendered descriptor field. r[cairn.generated_agent_workflows.command_routes] r[cairn.generated_agent_workflows.command_routes.refused]
- [x] [serial] [task:tests] [after:admit] Add core tests for permitted, longest-prefix, unpermitted, uncatalogued and every-field cases, and a CLI test that admits the shipped policy and refuses each 9082e1ae defect. r[cairn.generated_agent_workflows.command_routes.permitted] r[cairn.generated_agent_workflows.command_routes.refused]
- [x] [serial] [task:fixtures] [after:tests] Refresh policy-hash fixtures and goldens for the new generated policy. r[cairn.generated_agent_workflows.command_routes]
- [x] [serial] [task:docs] [after:fixtures] Document the rule in `docs/generated-agent-workflows.md` and the changelog. r[cairn.generated_agent_workflows.command_routes]
- [x] [serial] [task:checks] [after:docs] Run the policy export check, workspace tests, Clippy, `cargo fmt --check`, `cairn validate`, the change gates and `nix flake check`. r[cairn.generated_agent_workflows.command_routes]

## Validation and completion

- [x] [serial] [task:lifecycle] [after:checks] Sync the delta, record the evidence and archive only after every required check passes. r[cairn.generated_agent_workflows.command_routes]
