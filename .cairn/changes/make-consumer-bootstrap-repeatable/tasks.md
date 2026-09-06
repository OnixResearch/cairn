# Consumer bootstrap tasks

## Contract and core

- [x] [serial] [task:baseline] Record current init, policy, guidance, and migration baselines with the missing-policy and explicit-policy controls. r[cairn.consumer_bootstrap.conformance]
- [ ] [serial] [task:contracts] [after:baseline] Define typed Nickel cohort, policy-selection, upgrade, ownership, and named-bound contracts with accepted and rejected fixtures. r[cairn.consumer_bootstrap.policy] r[cairn.consumer_bootstrap.compatibility]
- [ ] [serial] [task:core] [after:contracts] Implement pure bootstrap admission, compatibility, publication plans, and recovery transitions with positive and negative unit tests. r[cairn.consumer_bootstrap.policy] r[cairn.consumer_bootstrap.compatibility] r[cairn.consumer_bootstrap.publication]

## Shell and integration

- [ ] [serial] [task:reuse] [after:core] Admit pinned Nickel Export, Cap Root, and Durable File Publication contracts through consumer conformance tests and recorded notices. r[cairn.consumer_bootstrap.boundary] r[cairn.consumer_bootstrap.conformance]
- [ ] [serial] [task:shell] [after:reuse] Implement preview-first setup, locked apply, exact rechecks, capability-relative publication, and explicit interrupted-state recovery. r[cairn.consumer_bootstrap.publication] r[cairn.consumer_bootstrap.boundary]
- [ ] [serial] [task:guidance] [after:shell] Compose project policy and generated-workflow freshness findings into doctor without global guidance discovery. r[cairn.consumer_bootstrap.guidance] r[cairn.consumer_bootstrap.policy]
- [ ] [serial] [task:negative-shell] [after:guidance] Add stale-plan, modified-file, symlink, permission, interruption, durability-unknown, and failed-recovery tests. r[cairn.consumer_bootstrap.publication] r[cairn.consumer_bootstrap.guidance] r[cairn.consumer_bootstrap.boundary]

## Consumer evidence and handoff

- [ ] [serial] [task:consumer] [after:negative-shell] Reproduce clean setup and upgrades from fresh roots without ambient policy or user-global state. r[cairn.consumer_bootstrap.conformance]
- [ ] [serial] [task:docs] [after:consumer] Publish the checked setup example, policy precedence, migration limits, compatibility diagnostics, and current project-local guidance. r[cairn.consumer_bootstrap.policy] r[cairn.consumer_bootstrap.guidance] r[cairn.consumer_bootstrap.boundary]
- [ ] [serial] [task:verification] [after:docs] Run focused and full checks, retain exact consumer receipts, and prepare reviewed sync and archive plans. r[cairn.consumer_bootstrap.conformance]

Only the baseline task has runtime evidence in `evidence/implementation-first-pass.md`.
All remaining tasks require further implementation. Planning gates do not complete these tasks.
Archive execution is not a prerequisite of any task. Use the normal lifecycle after implementation evidence passes.
