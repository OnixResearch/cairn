# Tasks

## Selected-work contract

- [x] [serial] [task:port] Port the unintegrated selected-work DTO, admission, shell, CLI/MCP routes and tests onto the current tree; route readiness through the selected prerequisite closure. r[cairn.selected_work.contract] r[cairn.selected_work.observation] r[cairn.selected_work.transport]
- [x] [serial] [task:contract] [after:port] Extend the Nickel wire contract, generated bounds and positive/negative fixtures, and add a flake check. r[cairn.selected_work.contract] r[cairn.selected_work.obligations]
- [x] [serial] [task:discovery] [after:port] Add `cairn_selected_work` to the inspect, verify and full MCP discovery profiles and regenerate the discovery projection. r[cairn.selected_work.transport]

## Obligation export

- [x] [serial] [task:facts] Derive typed per-obligation facts with recomputable identities and a consumer verifier in the obligation module. r[cairn.workflow_obligations.consumer_facts]
- [x] [serial] [task:compose] [after:facts] [after:port] Bind an opt-in obligation definition and observations into the selected snapshot and its cohort; re-admit the section in portable admission. r[cairn.selected_work.obligations] r[cairn.selected_work.boundary]
- [x] [serial] [task:fixtures] [after:compose] Add core, consumer and real CLI/MCP fixtures for parity, incomplete alternatives, evidence changes, policy revision, crossed subjects and forged evaluations. r[cairn.selected_work.obligations] r[cairn.selected_work.conformance] r[cairn.workflow_obligations.consumer_facts]

## Verification and completion

- [x] [serial] [task:docs] [after:fixtures] Document the selected-work consumer contract for Kiln and Lattice, and the changelog entry. r[cairn.selected_work.boundary]
- [x] [serial] [task:verify] [after:docs] Run policy freshness, workspace tests, Clippy, formatting, lifecycle validation and the flake checks; retain the results. r[cairn.selected_work.conformance]
