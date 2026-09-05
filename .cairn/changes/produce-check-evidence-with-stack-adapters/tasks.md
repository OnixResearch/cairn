# Check evidence producer tasks

## Contract and core

- [ ] [serial] [task:baseline] Record completion, developer-loop, and gate-interchange baselines with current and rejected evidence. r[cairn.check_evidence_producer.conformance]
- [ ] [serial] [task:contracts] [after:baseline] Define typed Nickel producer profiles, command grammars, required scope, safe diagnostics, and named execution bounds. r[cairn.check_evidence_producer.contract] r[cairn.check_evidence_producer.local_run]
- [ ] [serial] [task:core] [after:contracts] Implement pure plan admission, status normalization, applicability, and publication decisions with positive and negative unit tests. r[cairn.check_evidence_producer.contract] r[cairn.check_evidence_producer.applicability] r[cairn.check_evidence_producer.publication]

## Stack adapters and effects

- [ ] [serial] [task:reuse] [after:core] Admit exact stack contracts and producer fixtures with recorded revisions, license notices, and consumer-owned translation boundaries. r[cairn.check_evidence_producer.adapters] r[cairn.check_evidence_producer.conformance]
- [ ] [serial] [task:local] [after:reuse] Add reviewed Cargo and Nix execution profiles through Bounded Exec with explicit plan, authority, and executable rechecks. r[cairn.check_evidence_producer.local_run]
- [ ] [serial] [task:imports] [after:local] Add supported saved-observation adapters for Kiln, Lattice, Mantle, Octet, and Valence without producer-schema changes. r[cairn.check_evidence_producer.adapters]
- [ ] [serial] [task:publication] [after:imports] Add capability-relative artifact publication, attempt identity, source rechecks, safe diagnostics, and interrupted-state handling. r[cairn.check_evidence_producer.applicability] r[cairn.check_evidence_producer.publication]

## Consumer evidence and handoff

- [ ] [serial] [task:controls] [after:publication] Add denied, empty-test, failed-payload, skipped, timed-out, cancelled, flooded, stale, substituted, and teardown-failure controls. r[cairn.check_evidence_producer.local_run] r[cairn.check_evidence_producer.adapters] r[cairn.check_evidence_producer.conformance]
- [ ] [serial] [task:consumers] [after:controls] Replay fresh Cairn and pinned Kiln consumer fixtures through the original completion and loop validators without manual result edits. r[cairn.check_evidence_producer.conformance]
- [ ] [serial] [task:verification] [after:consumers] Publish checked recipes, run full checks, retain consumer receipts, and prepare reviewed sync and archive plans. r[cairn.check_evidence_producer.contract] r[cairn.check_evidence_producer.conformance]

All tasks describe future implementation. Planning gates do not complete these tasks.
Archive execution is not a prerequisite of any task. Use the normal lifecycle after implementation evidence passes.
