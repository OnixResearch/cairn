# Tasks: determinism entropy inventory

## Phase 1: Policy and schema

- [x] [serial] Add policy-owned replay entropy inventory contracts, default entries, and generated policy coverage. r[cairn.supersede.determinism_entropy_inventory.policy]
- [x] [serial] Add stable replay receipt schema fields for entropy inventory, assumptions, and policy hash. r[cairn.supersede.determinism_entropy_inventory.schema]

## Phase 2: CLI behavior

- [x] [depends:cairn.supersede.determinism_entropy_inventory.policy] Emit stable replay entropy inventory/assumption/hash fields. r[cairn.supersede.determinism_entropy_inventory.receipt]
- [x] [depends:cairn.supersede.determinism_entropy_inventory.policy] Drive rejected environment entropy failures from policy instead of hidden Rust literals. r[cairn.supersede.determinism_entropy_inventory.failures]

## Phase 3: Documentation and tests

- [x] [depends:cairn.supersede.determinism_entropy_inventory.receipt] Update README/docs with entropy inventory semantics and secret-safe disclosure rules. r[cairn.supersede.determinism_entropy_inventory.docs]
- [x] [depends:cairn.supersede.determinism_entropy_inventory.failures] Add focused positive/negative replay tests and schema assertions. r[cairn.supersede.determinism_entropy_inventory.tests]

## Phase 4: Verification and archive

- [x] [depends:cairn.supersede.determinism_entropy_inventory.tests] Run policy export, formatting, focused/full tests, Cairn validate/gates, Tracey, release-readiness, and replay smokes. r[cairn.supersede.determinism_entropy_inventory.verification]
- [x] [depends:cairn.supersede.determinism_entropy_inventory.verification] Sync accepted specs, archive the change, commit, and push. r[cairn.supersede.determinism_entropy_inventory.archive]
