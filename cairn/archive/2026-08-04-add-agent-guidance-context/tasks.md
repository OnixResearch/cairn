# Tasks: Typed agent guidance context

## Phase 1: Baseline and policy

- [x] [serial] B1 Record baseline project projection, context, workflow instructions, MCP, evidence, validation, and release-readiness results. r[cairn.agent_guidance_context.verification]
- [x] [serial] P1 Add typed Nickel contracts for context references, source roots, selectors, guidance, authority classes, bounds, receipt fields, and non-claims. r[cairn.agent_guidance_context.project_config] r[cairn.agent_guidance_context.authority]
- [x] [depends:cairn.agent_guidance_context.project_config] P2 Export deterministic checked projections and reject stale, duplicate, unknown, unsafe, or unauthorized values. r[cairn.agent_guidance_context.project_config] r[cairn.agent_guidance_context.admission]

## Phase 2: Pure core

- [x] [depends:cairn.agent_guidance_context.admission] C1 Add pure context declaration admission, selector normalization, duplicate handling, and stable diagnostics. r[cairn.agent_guidance_context.admission] r[cairn.agent_guidance_context.functional_boundary]
- [x] [depends:cairn.agent_guidance_context.selection] C2 Add pure profile, artifact, and operation selection with policy, fact, and advisory ordering. r[cairn.agent_guidance_context.selection] r[cairn.agent_guidance_context.authority]
- [x] [depends:cairn.agent_guidance_context.receipts] C3 Add pure local result and portable identity construction over loaded text observations. r[cairn.agent_guidance_context.receipts] r[cairn.agent_guidance_context.functional_boundary]

## Phase 3: CLI and MCP shells

- [x] [depends:cairn.agent_guidance_context.admission] S1 Extend project configuration export and freshness checks for context and guidance fields. r[cairn.agent_guidance_context.project_config]
- [x] [depends:cairn.agent_guidance_context.selection] S2 Add capability-root bounded UTF-8 reads, BLAKE3 identities, and required or optional source diagnostics. r[cairn.agent_guidance_context.sources] r[cairn.agent_guidance_context.functional_boundary]
- [x] [depends:cairn.agent_guidance_context.selection] S3 Extend read-only context and workflow instruction CLI results with selected entries and authority labels. r[cairn.agent_guidance_context.integration]
- [x] [depends:cairn.agent_guidance_context.integration] S4 Extend strict read-only MCP results with the same semantic values and unknown-field rejection. r[cairn.agent_guidance_context.mcp]
- [x] [depends:cairn.agent_guidance_context.receipts] S5 Omit raw context and absolute paths from portable evidence roles while preserving local result hashes. r[cairn.agent_guidance_context.receipts] r[cairn.agent_guidance_context.privacy]

## Phase 4: Verification and documentation

- [x] [parallel] V1 Add positive projection, source-root, selector, ordering, integration, identity, CLI, MCP, and repeatability fixtures. r[cairn.agent_guidance_context.verification]
- [x] [parallel] V2 Add negative stale, traversal, symlink, glob, URL, environment, command, encoding, size, duplicate, unknown, authority, and export fixtures. r[cairn.agent_guidance_context.verification] r[cairn.agent_guidance_context.boundary]
- [x] [depends:cairn.agent_guidance_context.integration] V3 Add compatibility tests for projects without context and guidance fields. r[cairn.agent_guidance_context.compatibility]
- [x] [depends:cairn.agent_guidance_context.integration] D1 Document declarations, selectors, authority classes, local output, evidence omission, privacy duties, and non-claims. r[cairn.agent_guidance_context.docs] r[cairn.agent_guidance_context.privacy]
- [x] [depends:cairn.agent_guidance_context.verification] V4 Run policy freshness, formatting, focused tests, Clippy, purity smoke, traceability, lifecycle gates, release readiness, and relevant Nix checks. r[cairn.agent_guidance_context.verification] r[cairn.agent_guidance_context.functional_boundary] ✅ Rust, policy, lifecycle, traceability, Slotcar, and Nix checks passed on 2026-08-04.

## Verification Coverage

- `Scenario: Valid guidance projection exports deterministically` -> P1, P2, S1, V1
- `Scenario: Stale or invalid projection fails` -> P2, S1, V2
- `Scenario: Safe local context source loads` -> S2, V1
- `Scenario: Unsafe or missing required source fails` -> C1, S2, V2
- `Scenario: Artifact selectors return focused context` -> C2, S3, V1
- `Scenario: Project guidance cannot impersonate policy` -> C1, C2, V2
- `Scenario: Workflow instructions include bounded selected text` -> C2, S3, V1
- `Scenario: CLI and MCP context facts are equivalent` -> S3, S4, V1
- `Scenario: Portable evidence omits raw context` -> C3, S5, V1, V2
- `Scenario: Projects without guidance retain compatibility` -> V3
- `Scenario: Context selection remains pure` -> C1, C2, C3, V4
- `Scenario: Documentation keeps privacy and authority bounded` -> D1, V2
