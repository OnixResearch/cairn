# Tasks: Add an outcome-machine workflow profile

## Phase 1: Policy and contracts

- [x] [serial] Add the outcome-machine workflow schema, artifact admission, profile registry entry, and non-claims for r[cairn.outcome_machine_profile.selection].
- [x] [parallel] Add valid policy export coverage and invalid profile fixtures for r[cairn.outcome_machine_profile.verification].

## Phase 2: Functional core

- [x] [serial] Add bounded outcome-machine review types and pure route admission for r[cairn.outcome_machine_profile.routes].
- [x] [serial] Separate design-stage findings from completion blockers for r[cairn.outcome_machine_profile.completion].
- [x] [parallel] Add positive route, observed completion, and fresh-state tests for r[cairn.outcome_machine_profile.fresh_state].
- [x] [parallel] Add malformed, empty, placeholder, hidden-state, authority, and boundary tests for r[cairn.outcome_machine_profile.boundary].

## Phase 3: Imperative shell and lifecycle

- [x] [serial] Scaffold and load bounded `outcome-machine.json` artifacts for r[cairn.outcome_machine_profile.scaffold].
- [x] [parallel] Add change-create and existing-profile compatibility tests for r[cairn.outcome_machine_profile.selection].
- [x] [parallel] Add planning-gate and completion-blocker tests for r[cairn.outcome_machine_profile.completion].
- [x] [parallel] Add sync and archive denial and acceptance tests for r[cairn.outcome_machine_profile.verification].

## Phase 4: Documentation and validation

- [x] [serial] Document profile selection, routes, completion evidence, fresh-state disposition, and non-claims for r[cairn.outcome_machine_profile.docs].
- [x] [serial] Refresh generated policy and bound fixture identities for r[cairn.outcome_machine_profile.verification].
- [x] [serial] Run focused policy, core, CLI, gate, sync, archive, and compatibility checks for r[cairn.outcome_machine_profile.verification].
- [ ] [serial] Run Cairn validation, traceability, strict Clippy, Slotcar, and full Nix checks for r[cairn.outcome_machine_profile.verification].
- [ ] [serial] Sync the accepted specification and archive the completed change for r[cairn.outcome_machine_profile.docs].
