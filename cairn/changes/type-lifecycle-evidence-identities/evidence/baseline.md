# Baseline evidence

Date: 2026-07-29

## Lifecycle package

These commands passed before the core migration:

```text
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- gate proposal type-lifecycle-evidence-identities --root .
nix run path:$PWD#cairn -- gate design type-lifecycle-evidence-identities --root .
nix run path:$PWD#cairn -- gate tasks type-lifecycle-evidence-identities --root .
```

## Core behavior

This command passed before the migration:

```text
nix develop -c cargo test -p cairn-core
```

Result: 398 tests passed.

The baseline includes these stable checks:

- Agent Preserves fixture hash: `5b561ff30ee566ca86c190b5bff2ba8c2ff90f771edc654e9e835f7ff5d92982`.
- Agent receipt canonical bytes and replay fields.
- Formal-proof readiness JSON and count projection.
- Evidence-chain readiness chain hash.
- Stack-provenance readiness JSON and boundary.
- Archive manifest roots, pack roots, receipts, and ledger history.
- Collaboration state hashes, content roots, portable envelopes, update plans, and resolution plans.
- Lifecycle receipt adapter family shapes and hash profiles.
- Global registry snapshots and portable snapshot verification.

These checks provide the comparison surface. They do not prove all compatibility or behavior.

<!--
r[verify cairn.nominal_evidence_identities.compatibility]
r[verify cairn.nominal_evidence_identities.compatibility.read]
-->
