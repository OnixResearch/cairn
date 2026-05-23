# Semantic Review Readiness Gate

## Why

Cairn now validates and aggregates externally produced semantic review receipts, but release readiness does not yet consume that evidence. Operators need a deterministic readiness surface that can bind a reviewed semantic ledger when a release or agent handoff requires semantic review evidence.

## What changes

- Extend `cairn release-readiness` with an optional semantic review ledger input.
- Fail readiness when the supplied ledger receipt is invalid or reports a failing semantic review verdict.
- Report a skipped semantic-review check when no ledger is supplied and no conventional checked-in ledger exists.
- Document the boundary: Cairn consumes a structural ledger receipt; it does not run semantic reviewers.

## Non-goals

- Running LLMs or manual reviewers.
- Claiming Cairn can judge semantic correctness.
- Requiring every local development checkout to have a semantic review ledger.
