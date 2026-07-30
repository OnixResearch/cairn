# Tasks: Lifecycle stores, references, context, and worksets

## Phase 1: Baseline and policy

- [x] [serial] B1 Record baseline policy freshness, root-local CLI tests, global-registry tests, validation, release readiness, and lifecycle gates. r[cairn.lifecycle_stores.verification]
- [x] [serial] P1 Add typed Nickel contracts for store identity, project declarations, references, opener profiles, named bounds, receipts, and non-claims. r[cairn.lifecycle_stores.identity] r[cairn.lifecycle_stores.project_config] r[cairn.lifecycle_stores.worksets]
- [x] [depends:cairn.lifecycle_stores.identity] P2 Add deterministic generated projections and stale-projection checks for store and project configuration. r[cairn.lifecycle_stores.identity] r[cairn.lifecycle_stores.project_config]
- [x] [depends:cairn.lifecycle_stores.identity] P3 Add policy-owned eligible root commands, opener profiles, local-state schemas, issue codes, and portable hash fields. r[cairn.lifecycle_stores.root_resolution] r[cairn.lifecycle_stores.receipts]

## Phase 2: Pure core

- [x] [depends:cairn.lifecycle_stores.identity] C1 Add pure store identity admission and local locator transition planning with expected prior state identities. r[cairn.lifecycle_stores.identity] r[cairn.lifecycle_stores.locator]
- [x] [depends:cairn.lifecycle_stores.root_resolution] C2 Add pure root resolution over explicit selectors, bounded native roots, project pointers, locator snapshots, and fallback facts. r[cairn.lifecycle_stores.root_resolution]
- [x] [depends:cairn.lifecycle_stores.project_config] C3 Add pure direct-reference normalization, accepted-spec indexing, context results, and doctor findings. r[cairn.lifecycle_stores.references] r[cairn.lifecycle_stores.context]
- [x] [depends:cairn.lifecycle_stores.worksets] C4 Add pure workset transition, opener argv, action, local receipt, and portable identity planning. r[cairn.lifecycle_stores.worksets] r[cairn.lifecycle_stores.receipts]

## Phase 3: Imperative shell and commands

- [x] [depends:cairn.lifecycle_stores.locator] S1 Add capability-root local-state access, exclusive locks, stale-plan checks, and atomic replacement for locator and workset state. r[cairn.lifecycle_stores.locator] r[cairn.lifecycle_stores.functional_boundary]
- [x] [depends:cairn.lifecycle_stores.identity] S2 Add preview-first `store setup`, `register`, `unregister`, `list`, `show`, and `doctor` commands. r[cairn.lifecycle_stores.commands] r[cairn.lifecycle_stores.transport_boundary]
- [x] [depends:cairn.lifecycle_stores.root_resolution] S3 Add `--lifecycle-store` to policy-listed root-local commands and preserve existing `--store` meanings. r[cairn.lifecycle_stores.root_cli] r[cairn.lifecycle_stores.root_resolution]
- [x] [depends:cairn.lifecycle_stores.context] S4 Add read-only `context` output and direct-reference health to `doctor`, with exact store-qualified spec commands. r[cairn.lifecycle_stores.context] r[cairn.lifecycle_stores.references]
- [x] [depends:cairn.lifecycle_stores.worksets] S5 Add preview-first workset create, remove, and open effects plus read-only list and show commands. r[cairn.lifecycle_stores.worksets] r[cairn.lifecycle_stores.transport_boundary]

## Phase 4: Verification and documentation

- [x] [parallel] V1 Add positive identity, locator, root-resolution, pointer, reference, context, doctor, workset, opener, and portable-hash fixtures. r[cairn.lifecycle_stores.verification]
- [x] [parallel] V2 Add negative selector, duplicate, stale, malformed, symlink, unsafe-path, transport, ambient-state, overclaim, and partial-mutation fixtures. r[cairn.lifecycle_stores.verification] r[cairn.lifecycle_stores.boundary]
- [x] [depends:cairn.lifecycle_stores.commands] D1 Write a Cairn lifecycle-store user guide with setup, registration, pointers, references, context, doctor, worksets, precedence, file locations, and non-claims. r[cairn.lifecycle_stores.docs]
- [ ] [depends:cairn.lifecycle_stores.verification] V3 Run policy freshness, formatting, focused core and CLI tests, Clippy, purity smoke, traceability, lifecycle gates, release readiness, and relevant Nix checks. r[cairn.lifecycle_stores.verification] r[cairn.lifecycle_stores.functional_boundary]

## Verification Coverage

- `Scenario: Valid store identity exports deterministically` -> P1, P2, C1, V1
- `Scenario: Stale or malformed store identity fails` -> P2, V2
- `Scenario: Registration commits one checked mapping` -> C1, S1, S2, V1
- `Scenario: Duplicate or stale registration fails without replacement` -> C1, S1, V2
- `Scenario: Explicit store selection resolves deterministically` -> C2, S3, V1
- `Scenario: Conflicting selectors fail closed` -> C2, S3, V2
- `Scenario: Native root wins over a project pointer` -> C2, V1, V2
- `Scenario: Direct references produce a bounded accepted-spec index` -> C3, S4, V1
- `Scenario: References do not affect lifecycle authority` -> C3, V2
- `Scenario: Store setup never runs VCS transport` -> S2, V2
- `Scenario: Workset open requires explicit execution` -> C4, S5, V1, V2
- `Scenario: Arbitrary opener commands are rejected` -> C4, V2
- `Scenario: Local display paths do not change portable identity` -> C4, V1
- `Scenario: Pure decisions remain in the functional core` -> C1, C2, C3, C4, V3
- `Scenario: Documentation preserves the source and claim boundaries` -> D1, V3
