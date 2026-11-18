# Five-requirement pilot

## Inputs

- Native registry: exported from this worktree with
  `cairn requirement-registry export --root . --repository-id cairn
  --revision-kind git --revision <HEAD> --out pilot-registry.json
  --policy cairn-policy/generated/cairn-policy.json`.
  Receipt: schema `cairn.requirement-registry.export-receipt.v1`,
  75 specifications, 894 rows, `valid: true`, `verdict: pass`.
- Contracts: the five typed contracts already authored in
  `cairn-policy/generated/typed-requirement-contracts.json`, each rebound to the
  matching native row from that registry:
  `cairn.supersede.agent_run_receipts.verification_contract`,
  `.pass_only`, `.freshness`, `.completion_authority`, and
  `cairn.verification_obligation_cob.claim_revision`.
- Case: `pilot-case.json`, profile `cairn.requirement-semantics.finite.v1`.

## Executed

```
cairn requirement-analysis inspect --case pilot-case.json \
  --registry pilot-registry.json --root .
```

`pilot-report.json` records `outcome: analyzed` with:

| query | status |
|---|---|
| shared-assumption satisfiability | consistent |
| activation reachable (each of five) | consistent |
| joint guarantee compatibility | consistent |
| guarantee constraining (13 of 13) | consistent |

Findings: one `guarantees_compatible`, five `activation_reachable`, thirteen
`constraining_guarantee`. No vacuity, redundancy or incomplete-analysis finding
was produced, and no bound was exhausted.

Witness replay was exercised on the shared-assumption model
(`pilot-witness.json`, five total tables):

```
cairn requirement-analysis inspect --case pilot-case.json \
  --registry pilot-registry.json --root . --witness pilot-witness.json
```

`pilot-report-with-witness.json` records `witness_replay.replays: true` for
`assumptions_satisfiable` across the same five requirements.

## Boundary

This pilot binds the five authored contracts to current native rows and applies
the bounded finite queries to them. It does not establish that those contracts
capture the requirements' full meaning, that the requirements are mutually
consistent outside the declared finite domains, or that any gate verdict should
change. The analysis remains advisory, and correspondence between the five
contracts is not declared, so their equally named observations stay independent.
