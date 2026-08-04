# Tasks: Versioned CLI machine contract

## Phase 1: Baseline and policy

- [x] [serial] B1 Record baseline success bytes, stderr failures, exit statuses, receipt hashes, MCP framing, validation, and release readiness. r[cairn.cli_machine_contract.verification]
- [x] [serial] P1 Add typed Nickel policy for the machine schema version, exit classes, diagnostic fields, ordering, bounds, and non-claims. r[cairn.cli_machine_contract.policy]
- [x] [depends:cairn.cli_machine_contract.policy] P2 Export deterministic machine-contract policy and reject unknown classes, duplicate codes, unsafe targets, or invalid bounds. r[cairn.cli_machine_contract.policy]

## Phase 2: Pure core

- [x] [depends:cairn.cli_machine_contract.envelope] C1 Add pure typed outcome and machine-envelope models with stable success and failure construction. r[cairn.cli_machine_contract.envelope] r[cairn.cli_machine_contract.functional_boundary]
- [x] [depends:cairn.cli_machine_contract.diagnostics] C2 Add pure error classification, diagnostic normalization, privacy admission, and deterministic ordering. r[cairn.cli_machine_contract.diagnostics] r[cairn.cli_machine_contract.privacy]

## Phase 3: CLI shell

- [x] [depends:cairn.cli_machine_contract.envelope] S1 Add top-level `--machine` parsing and explicit incompatibility checks without changing default mode. r[cairn.cli_machine_contract.mode] r[cairn.cli_machine_contract.compatibility]
- [x] [depends:cairn.cli_machine_contract.diagnostics] S2 Route parser, dispatcher, policy, root, admission, planning, and preparation errors through typed machine outcomes. r[cairn.cli_machine_contract.diagnostics]
- [x] [depends:cairn.cli_machine_contract.envelope] S3 Add one stdout serialization path, empty controlled stderr, and preserved nonzero failure statuses. r[cairn.cli_machine_contract.envelope] r[cairn.cli_machine_contract.exit_status]
- [x] [depends:cairn.cli_machine_contract.compatibility] S4 Preserve nested command result bytes and receipt hashes inside successful envelopes. r[cairn.cli_machine_contract.compatibility]

## Phase 4: Verification and documentation

- [x] [parallel] V1 Add positive success, blocked-result, nested-hash, ordering, and repeatability fixtures. r[cairn.cli_machine_contract.verification]
- [x] [parallel] V2 Add negative parser, lookup, policy, unsafe-path, incompatible-option, privacy, and nonzero-status fixtures. r[cairn.cli_machine_contract.verification] r[cairn.cli_machine_contract.privacy]
- [x] [depends:cairn.cli_machine_contract.compatibility] V3 Add default CLI and MCP compatibility goldens. r[cairn.cli_machine_contract.compatibility]
- [x] [depends:cairn.cli_machine_contract.envelope] D1 Document invocation, envelope fields, exit classes, nested verdicts, controlled failures, privacy, and host-failure boundaries. r[cairn.cli_machine_contract.docs] r[cairn.cli_machine_contract.boundary]
- [x] [depends:cairn.cli_machine_contract.verification] V4 Run policy freshness, formatting, focused tests, Clippy, purity smoke, traceability, lifecycle gates, release readiness, and relevant Nix checks. r[cairn.cli_machine_contract.verification] r[cairn.cli_machine_contract.functional_boundary]

## Verification Coverage

- `Scenario: Machine success emits one envelope` -> C1, S1, S3, V1
- `Scenario: Controlled failure emits one envelope` -> C1, C2, S2, S3, V2
- `Scenario: Nested receipt identity is preserved` -> S4, V1
- `Scenario: Lifecycle failure remains distinct from transport failure` -> C1, S4, V1
- `Scenario: Stable diagnostics replace message matching` -> C2, S2, V2
- `Scenario: Machine mode preserves nonzero failure status` -> S3, V2
- `Scenario: Default CLI and MCP surfaces remain unchanged` -> S1, V3
- `Scenario: Sensitive values are not added to diagnostics` -> C2, V2
- `Scenario: Host-level failures remain outside the claim` -> D1, V2
- `Scenario: Outcome classification remains pure` -> C1, C2, V4
