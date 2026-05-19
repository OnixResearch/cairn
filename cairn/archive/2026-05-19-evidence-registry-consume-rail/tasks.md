# Tasks: Evidence Registry Consume Rail

## Phase 1: Change package

- [x] [serial] r[cairn.supersede.evidence_registry_consume.select] Write proposal, design, tasks, and spec delta for deterministic registry selection.
- [x] [serial] r[cairn.supersede.evidence_registry_consume.fetch_plan] Define fetch-plan requirements and receipt expectations.

## Phase 2: Implementation

- [x] [serial] r[cairn.supersede.evidence_registry_consume.select] Add Rust CLI registry selection command, stable selectors, deterministic receipt, and issue codes.
- [x] [serial] r[cairn.supersede.evidence_registry_consume.fetch_plan] Add Rust CLI registry fetch-plan command and deterministic metadata-only receipt.
- [x] [serial] r[cairn.supersede.evidence_registry_consume.policy] Update Nickel policy receipt schemas and deterministic surface inventory.
- [x] [serial] r[cairn.supersede.evidence_registry_consume.docs] Update README and agent receipt docs for consumer-side registry commands.

## Phase 3: Verification and archive

- [x] [serial] r[cairn.supersede.evidence_registry_consume.verify] Add positive/negative tests and run targeted registry-consume checks.
- [x] [serial] r[cairn.supersede.evidence_registry_consume.verify] Run full Rust/Nickel gates, sync/archive the change, commit, and push.
