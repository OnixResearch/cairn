## Phase 1: Policy and receipt contracts

- [x] [serial] r[cairn.supersede.nickel_contracts] Extend Nickel contracts/defaults for gate schema, task markers, bounded exemptions, lifecycle mutation policy, migration policy, and receipt hash fields.
- [x] [depends:cairn.supersede.nickel_contracts] r[cairn.supersede.nickel_contracts.export] Add or update policy export freshness verification so generated runtime JSON cannot drift from Nickel sources.

## Phase 2: Canonical BLAKE3 evidence

- [x] [serial] r[cairn.supersede.blake3_receipts] Add pure canonicalization and BLAKE3 evidence models for policy data, gate input snapshots, lifecycle plans, migration manifests, and receipt content.
- [x] [depends:cairn.supersede.blake3_receipts] r[cairn.supersede.blake3_receipts.gate] Add gate receipt hash fields and golden tests proving unchanged inputs reproduce stable hashes.
- [x] [depends:cairn.supersede.blake3_receipts] r[cairn.supersede.blake3_receipts.lifecycle_plans] Add sync/archive plan hash fields and fixture tests proving action changes alter plan hashes.
- [x] [depends:cairn.supersede.blake3_receipts] r[cairn.supersede.blake3_receipts.migration] Add migration source/destination manifest hashes and conflict/no-overwrite tests.

## Phase 3: Mandatory deterministic gates

- [x] [serial] r[cairn.supersede.deterministic_gates] Define required deterministic gates for proposal, design, tasks, pre-sync, pre-archive, migration, policy freshness, and traceability.
- [x] [depends:cairn.supersede.deterministic_gates] r[cairn.supersede.deterministic_gates.pre_sync] Make sync `--execute` refuse mutation when validation or required gate receipts fail.
- [x] [depends:cairn.supersede.deterministic_gates] r[cairn.supersede.deterministic_gates.pre_archive] Make archive `--execute` refuse mutation when tasks, sync status, or traceability gates fail.
- [x] [depends:cairn.supersede.deterministic_gates] r[cairn.supersede.deterministic_gates.policy_freshness] Wire policy freshness and deterministic receipt stability into the maintained Nix verification rail.

## Phase 4: Verification and documentation

- [x] [depends:cairn.supersede.nickel_contracts,cairn.supersede.blake3_receipts,cairn.supersede.deterministic_gates] r[cairn.supersede.deterministic_gates.stage] Update CLI goldens, fixtures, README, and `docs/hermes/cairn-workflow-skill.md` to document deterministic policy receipts.
- [x] [depends:cairn.supersede.deterministic_gates.traceability] r[cairn.supersede.deterministic_gates.traceability] Run focused tests plus `nix flake check path:$PWD -L`, then sync and archive this change when implementation is complete.
