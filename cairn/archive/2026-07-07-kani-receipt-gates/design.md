# Design: Kani receipt gates

## Approach

Cairn gates continue to verify saved receipts. A `cairn probe run` or external CI step may run Kani and store a receipt. The pure core validates the receipt view; the CLI shell reads files and emits deterministic gate diagnostics.

## Eligibility checks

A Kani receipt is eligible for a behavioral requirement only when it has:

- matching requirement id and source scope;
- current content hash and policy hash;
- verifier/toolchain identity;
- passing status for the named property;
- contract or unwind evidence appropriate to the claim;
- stub classification and verified-stub links when stubs are used;
- required non-claims.

## Gate behavior

Proposal/design/tasks gates may reference planned Kani evidence. Release-readiness and archive gates may require saved receipts. Gates do not invoke Kani by default and do not mutate lifecycle files.

## Fixture strategy

Positive fixtures cover a scoped passing bounded receipt and a contract-backed receipt. Negative fixtures cover stale content, failed verdict, missing tool identity, unbounded claim without unwind or contract support, unverified-stub promotion, and weakened non-claims.

## What is explicitly not provided

- No inline Kani execution inside default gates.
- No verifier-soundness claim.
- No whole-program claim from a scoped receipt.
