# Consumer bootstrap tasks

## Contract and core

- [x] [serial] [task:baseline] Record current init, policy, guidance, and migration baselines with the missing-policy and explicit-policy controls. r[cairn.consumer_bootstrap.conformance]
- [x] [serial] [task:contracts] [after:baseline] Define typed Nickel cohort, policy-selection, upgrade, ownership, and named-bound contracts with accepted and rejected fixtures. r[cairn.consumer_bootstrap.policy] r[cairn.consumer_bootstrap.compatibility]
- [x] [serial] [task:core] [after:contracts] Implement pure bootstrap admission, compatibility, publication plans, and recovery transitions with positive and negative unit tests. r[cairn.consumer_bootstrap.policy] r[cairn.consumer_bootstrap.compatibility] r[cairn.consumer_bootstrap.publication]

## Shell and integration

- [x] [serial] [task:reuse] [after:core] Admit pinned Nickel Export, Cap Root, and Durable File Publication contracts through consumer conformance tests and recorded notices. r[cairn.consumer_bootstrap.boundary] r[cairn.consumer_bootstrap.conformance]
- [x] [serial] [task:shell] [after:reuse] Implement preview-first setup, locked apply, exact rechecks, capability-relative publication, and explicit interrupted-state recovery. r[cairn.consumer_bootstrap.publication] r[cairn.consumer_bootstrap.boundary]
- [x] [serial] [task:guidance] [after:shell] Compose project policy and generated-workflow freshness findings into doctor without global guidance discovery. r[cairn.consumer_bootstrap.guidance] r[cairn.consumer_bootstrap.policy]
- [x] [serial] [task:negative-shell] [after:guidance] Add stale-plan, modified-file, symlink, permission, interruption, durability-unknown, and failed-recovery tests. r[cairn.consumer_bootstrap.publication] r[cairn.consumer_bootstrap.guidance] r[cairn.consumer_bootstrap.boundary]

## Consumer evidence and handoff

- [x] [serial] [task:consumer] [after:negative-shell] Reproduce clean setup and upgrades from fresh roots without ambient policy or user-global state. r[cairn.consumer_bootstrap.conformance]
- [x] [serial] [task:docs] [after:consumer] Publish the checked setup example, policy precedence, migration limits, compatibility diagnostics, and current project-local guidance. r[cairn.consumer_bootstrap.policy] r[cairn.consumer_bootstrap.guidance] r[cairn.consumer_bootstrap.boundary]
- [x] [serial] [task:verification] [after:docs] Run focused and full checks, retain exact consumer receipts, and prepare reviewed sync and archive plans. r[cairn.consumer_bootstrap.conformance]

Baseline evidence is in `evidence/implementation-first-pass.md`.
Contract and pure-core evidence is in `evidence/contracts-core-second-pass.md`.
Dependency-admission evidence is in `evidence/dependency-admission.md`.
Publication, guidance, and fresh-consumer progress is recorded in `evidence/finish-2746-bootstrap.md`.
The second-pass acceptance review is in `evidence/finish2-2746-bootstrap.md`.
Manual recovery satisfies the explicit-state scenario. Issued-token drift and controlled post-lock substitutions now have passing controls.
The full workspace/all-target/all-feature battery and final Nix check pass. Nix retains warning-level Octet findings, not strict Octet acceptance.
Only coordinator-owned active-change sync/archive plan review remains in the verification task.
Planning gates do not complete implementation tasks.
Archive execution is not a prerequisite of any task. Use the normal lifecycle after implementation evidence passes.
