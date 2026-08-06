# Tasks: Artifact workflow contract

## Phase 1: Baseline and policy

- [x] [serial] B1 Record baseline policy freshness, status, gate, sync, archive, MCP, validation, and release-readiness results. r[cairn.artifact_workflow_contract.verification]
- [x] [serial] P1 Extend typed Nickel contracts with workflow artifact IDs, paths, dependencies, stages, templates, bounds, and non-claims. r[cairn.artifact_workflow_contract.policy]
- [x] [depends:cairn.artifact_workflow_contract.policy] P2 Export deterministic workflow policy projections and reject duplicate, cyclic, unknown, or unsafe graph values. r[cairn.artifact_workflow_contract.policy]

## Phase 2: Pure core

- [x] [depends:cairn.artifact_workflow_contract.policy] C1 Add pure workflow graph admission and stable topological ordering. r[cairn.artifact_workflow_contract.policy] r[cairn.artifact_workflow_contract.functional_boundary]
- [x] [depends:cairn.artifact_workflow_contract.status] C2 Add pure `done`, `ready`, `blocked`, and `skipped` state derivation with blockers and unlocks. r[cairn.artifact_workflow_contract.status]
- [x] [depends:cairn.artifact_workflow_contract.instructions] C3 Add pure next-action selection and instruction-envelope construction from policy and observed artifact facts. r[cairn.artifact_workflow_contract.instructions] r[cairn.artifact_workflow_contract.functional_boundary]

## Phase 3: CLI and MCP shells

- [x] [depends:cairn.artifact_workflow_contract.status] S1 Add read-only `workflow status` argument parsing, bounded artifact observation, hashing, and JSON output. r[cairn.artifact_workflow_contract.cli] r[cairn.artifact_workflow_contract.status]
- [x] [depends:cairn.artifact_workflow_contract.instructions] S2 Add read-only `workflow instructions` selection and JSON output without artifact mutation. r[cairn.artifact_workflow_contract.cli] r[cairn.artifact_workflow_contract.instructions]
- [x] [depends:cairn.artifact_workflow_contract.cli] S3 Add strict read-only MCP workflow status and instruction tools over the shared core results. r[cairn.artifact_workflow_contract.mcp]

## Phase 4: Verification and documentation

- [x] [parallel] V1 Add positive policy, core, CLI, MCP, ordering, state, instruction, and determinism fixtures. r[cairn.artifact_workflow_contract.verification]
- [x] [parallel] V2 Add negative cycle, duplicate, unknown, unsafe-path, malformed-artifact, unknown-selector, and unknown-field fixtures. r[cairn.artifact_workflow_contract.verification] r[cairn.artifact_workflow_contract.boundary]
- [x] [depends:cairn.artifact_workflow_contract.cli] V3 Add compatibility tests for existing status, gate, sync, archive, and MCP receipt fields. r[cairn.artifact_workflow_contract.compatibility]
- [x] [depends:cairn.artifact_workflow_contract.instructions] D1 Document workflow status, instructions, exact fields, state meanings, and non-claims. r[cairn.artifact_workflow_contract.docs]
- [x] [depends:cairn.artifact_workflow_contract.verification] V4 Run policy freshness, formatting, focused tests, Clippy, purity smoke, traceability, lifecycle gates, release readiness, and relevant Nix checks. r[cairn.artifact_workflow_contract.verification] r[cairn.artifact_workflow_contract.functional_boundary]

## Verification Coverage

- `Scenario: Valid workflow graph exports deterministically` -> P1, P2, C1, V1
- `Scenario: Invalid workflow graph fails admission` -> P2, C1, V2
- `Scenario: Dependencies derive deterministic states` -> C2, V1
- `Scenario: Invalid present artifact blocks dependents` -> C2, V2
- `Scenario: Ready artifact returns exact instructions` -> C3, S2, V1
- `Scenario: Unknown artifact returns a stable error` -> C3, S2, V2
- `Scenario: CLI and MCP return equivalent workflow facts` -> S1, S2, S3, V1
- `Scenario: Existing lifecycle receipts remain compatible` -> V3
- `Scenario: Workflow output does not promote lifecycle claims` -> V2, D1
- `Scenario: Workflow decisions remain pure` -> C1, C2, C3, V4
