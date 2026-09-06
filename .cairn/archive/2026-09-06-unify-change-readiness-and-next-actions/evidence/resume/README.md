# Readiness baseline

The baseline task is complete. The readiness implementation remains open.
Cairn maintainers own these fixtures and their later comparison with the new readiness report.

The saved receipts cover the existing workflow, gate, drain, machine, and completion surfaces.
The valid completion fixture reports `valid: true` and `verdict: PASS`.
The changed-subject fixture reports `valid: false`, `verdict: FAIL`, and `verification_contract.subject_stale`.
Both fixtures report `checks_executed: false`. They validate supplied evidence and do not prove fresh execution of tests or Clippy.

The rejected fixture still identifies its receipt class as `authoritative`.
The new readiness report must inspect validity, verdict, and subject binding separately from that class label.
The existing workflow and drain receipts do not authorize mutation or establish completion.

The pre-implementation Nix Rust-test baseline passed in this worktree.
Its retained log is in the primary checkout at `.pi/cairn-drain-resume-20260905/readiness-baseline.log`.
The packaged CLI for these baseline receipts is the retained `.pi/cairn-octet-fix-cli/bin/cairn` binary in the primary checkout.
These paths locate local evidence. They are not product dependencies.

The canonical Octet worktree now contains the gate-contract source repair at commit `a7f7e1b`.
Its focused deny-all gate passes, but the full workspace gate still rejects `cairn-core`.
That separate repair does not implement readiness or complete this change.
The proposal, design, and tasks receipts check lifecycle conformance only.
