# Release-run ledgers

`cairn release-run validate|status|explain --input <ledger.json>` reads a bounded local ledger and evaluates it against the current generated Cairn policy hash. The CLI owns file and policy loading plus JSON presentation; the core owns run identity, transition, linkage, blocker, resumability, and non-claim decisions. The commands never resume a step or mutate a remote service. r[impl cairn.release_evidence.durable_run_ledger.shell]

A ledger binds a lifecycle-root identity, current policy, command identity, profile, deterministic step transitions, run-scoped artifact references, diagnostics, blockers, resume hints, and canonical BLAKE3 identities. `status` summarizes terminal state and a local next action. `explain` lists blockers, stale inputs, missing evidence, resumable steps, and validation issues. r[impl cairn.release_evidence.durable_run_ledger.model] r[impl cairn.release_evidence.durable_run_ledger.status_explain]

Passing ledger validation proves ledger syntax, Cairn-owned identity linkage, transition validity, and non-claim preservation only. It does not publish artifacts, create tags or GitHub releases, upload registry packages, sign artifacts, prove release eligibility or evidence truth, or trust external services. r[verify cairn.release_evidence.durable_run_ledger.final_validation]
