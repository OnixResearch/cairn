# Tasks

- [x] [serial] [task:select] Distinguish an absent gate list from a malformed one and report the refusal in the gate summary and the `repository_gate_policy_listed` check. r[release_evidence.consumer_gate_selection] r[release_evidence.consumer_gate_selection.malformed]
- [x] [serial] [task:tests] [after:select] Add negative CLI tests for each malformed form and a positive test for a policy object without the list, beside the consumer-selected fixture tests. r[release_evidence.consumer_gate_selection.selected] r[release_evidence.consumer_gate_selection.absent] r[release_evidence.consumer_gate_selection.malformed]
- [x] [serial] [task:docs] [after:tests] Document the selection contract in `docs/cairn-mcp.md` and the changelog. r[release_evidence.consumer_gate_selection]
- [x] [serial] [task:consumers] [after:docs] Check in-repository policies and the Lattice and Kiln consumers for lists the change would refuse. r[release_evidence.consumer_gate_selection.malformed]
- [x] [serial] [task:checks] [after:consumers] Run the policy export check, workspace tests, Clippy, `cairn validate`, the change gates and `nix flake check`. r[release_evidence.consumer_gate_selection]

## Validation and completion

- [x] [serial] [task:lifecycle] [after:checks] Sync the delta, record the evidence and archive only after every required check passes. r[release_evidence.consumer_gate_selection]
