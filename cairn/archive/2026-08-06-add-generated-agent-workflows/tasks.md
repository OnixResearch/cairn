# Tasks: Generated agent workflows

## Phase 1: Baseline and policy

- [x] [serial] B1 Record baseline Hermes source, CLI and MCP catalogs, workflow contracts, policy freshness, validation, and release readiness. r[cairn.generated_agent_workflows.verification]
- [x] [serial] P1 Add typed Nickel contracts for workflow descriptors, goals, success evidence, command classes, mutation classes, progress, output, bounds, and non-claims. r[cairn.generated_agent_workflows.descriptors]
- [x] [depends:cairn.generated_agent_workflows.descriptors] P2 Register explore, propose, apply, verify, drain, and archive descriptors with deterministic identities. r[cairn.generated_agent_workflows.catalog]
- [x] [depends:cairn.generated_agent_workflows.adapters] P3 Register versioned Agent Skills, Claude, and Codex adapters with safe project-relative output rules. r[cairn.generated_agent_workflows.adapters]

## Phase 2: Pure core

- [x] [depends:cairn.generated_agent_workflows.descriptors] C1 Add pure descriptor admission, command permission checks, goal-first prompt models, and non-claim enforcement. r[cairn.generated_agent_workflows.descriptors] r[cairn.generated_agent_workflows.explore]
- [x] [depends:cairn.generated_agent_workflows.adapters] C2 Add pure adapter admission, deterministic escaping, rendering, path checks, and adapter authority limits. r[cairn.generated_agent_workflows.adapters] r[cairn.generated_agent_workflows.functional_boundary]
- [x] [depends:cairn.generated_agent_workflows.manifest] C3 Add pure file-plan, expected-prior, freshness-state, and BLAKE3 manifest construction. r[cairn.generated_agent_workflows.manifest] r[cairn.generated_agent_workflows.install]

## Phase 3: CLI shell

- [x] [depends:cairn.generated_agent_workflows.catalog] S1 Add read-only `agent workflow list` and `show` commands. r[cairn.generated_agent_workflows.commands]
- [x] [depends:cairn.generated_agent_workflows.manifest] S2 Add deterministic project-local plan and check commands with explicit adapter and destination. r[cairn.generated_agent_workflows.commands] r[cairn.generated_agent_workflows.manifest]
- [x] [depends:cairn.generated_agent_workflows.install] S3 Add preview-first install with capability roots, locks, stale-plan recheck, and atomic managed-file replacement. r[cairn.generated_agent_workflows.install] r[cairn.generated_agent_workflows.functional_boundary]
- [x] [depends:cairn.generated_agent_workflows.install] S4 Block modified, unmanaged, symlinked, unsafe, and unsupported destination states without partial writes or deletion. r[cairn.generated_agent_workflows.install] r[cairn.generated_agent_workflows.boundary]

## Phase 4: Verification and documentation

- [x] [parallel] V1 Add positive descriptor, adapter, render, manifest, fresh install, managed update, check, and repeatability fixtures. r[cairn.generated_agent_workflows.verification]
- [x] [parallel] V2 Add negative hidden-mutation, missing-constraint, unknown-adapter, unsafe-path, stale-plan, modified-file, unmanaged-file, authority, network, model, and partial-write fixtures. r[cairn.generated_agent_workflows.verification] r[cairn.generated_agent_workflows.boundary]
- [x] [depends:cairn.generated_agent_workflows.explore] V3 Add static positive and negative tests for the no-write explore descriptor across every adapter. r[cairn.generated_agent_workflows.explore]
- [x] [depends:cairn.generated_agent_workflows.compatibility] V4 Add compatibility checks for lifecycle commands and the maintained Hermes source. r[cairn.generated_agent_workflows.compatibility]
- [x] [depends:cairn.generated_agent_workflows.commands] D1 Document adapter selection, explicit destinations, preview, install, freshness, managed conflicts, explore boundaries, and non-claims. r[cairn.generated_agent_workflows.docs]
- [x] [depends:cairn.generated_agent_workflows.verification] V5 Run policy freshness, formatting, focused tests, Clippy, purity smoke, traceability, lifecycle gates, release readiness, and relevant Nix checks. r[cairn.generated_agent_workflows.verification] r[cairn.generated_agent_workflows.functional_boundary]

## Verification Coverage

- `Scenario: Valid workflow descriptors export deterministically` -> P1, P2, C1, V1
- `Scenario: Invalid descriptor fails admission` -> P1, C1, V2
- `Scenario: Explore descriptor permits only read work` -> C1, V3
- `Scenario: Adapter renders exact project-local files` -> P3, C2, V1
- `Scenario: Adapter cannot add authority` -> C2, V2
- `Scenario: Plan binds generated and prior identities` -> C3, S2, V1
- `Scenario: Fresh installation writes exact managed files` -> C3, S3, V1
- `Scenario: Changed destination blocks all replacement` -> C3, S3, S4, V2
- `Scenario: Freshness check reports every managed state` -> C3, S2, V1, V2
- `Scenario: Generated workflows do not satisfy lifecycle authority` -> V2, D1
- `Scenario: Existing lifecycle and Hermes surfaces remain compatible` -> V4
- `Scenario: Rendering and planning remain pure` -> C1, C2, C3, V5
