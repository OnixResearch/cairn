# Authoritative agent completion

Cairn treats model success text, process exit status, task checkboxes, wrapper verdicts, and archive presence as progress observations. They do not establish completion. Authoritative completion requires a current repository-owned verification contract and saved passing execution results for the exact final subject.

## Inputs and authority

The repository owns a versioned `cairn.agent-completion.v2` contract. It binds:

- repository and registry IDs;
- BLAKE3 registry, selection, policy, and final-subject hashes;
- the exact required check IDs and argv identities;
- prerequisite edges between selected checks.

The producer supplies saved result rows. A result binds one required check ID, exact argv and command hash, status, final subject, registry, selection, policy, evidence, execution-receipt, and artifact hashes. Producer rows cannot redefine the required set.

`passed` is the only completion-eligible status. `failed`, `blocked`, `skipped`, `deferred`, `timed_out`, `cancelled`, `dry_run`, `cleanup_failed`, `infrastructure_error`, `not_selected`, `unknown`, unsupported values, missing rows, duplicates, unknown rows, and unsatisfied prerequisites all remain blockers.

## CLI flow

Checks execute outside Cairn and produce bounded safe result artifacts. Cairn only reads and validates them:

```console
cairn agent-receipt complete \
  --contract fixtures/agent-completion/contract.json \
  --results fixtures/agent-completion/results.json \
  --subject-hash cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc
```

The command never launches, retries, or escalates a check. A passing response has `classification: "authoritative"`, exact required and passed sets, no non-passing rows, `valid: true`, `verdict: "PASS"`, and a `completion_receipt_hash`. Wrappers may cite that hash and final subject without embedding raw logs.

The MCP tool `cairn_agent_completion_validate` accepts the same contract, results, and subject values inline under the same bounded input limit. CLI and MCP call the same pure finalizer and agree on sets, diagnostics, identities, validity, verdict, and completion receipt hash.

## Legacy compatibility

`cairn agent-receipt validate <v1-receipt.json>` remains an inspection path for historical envelopes, replay linkage, and hashes. Successful v1 producer-authored check arrays are classified `legacy_unscoped`, return `authoritative_completion: false`, and cannot return authoritative completion `PASS`. Migrate wrappers to the contract-aware command before claiming completion.

## Actionable diagnostics

Diagnostics contain stable IDs and codes, applicable check/status and identity hashes, bounded remediation text, exact reproduction argv, the safe `<repository-root>` working label, safe repository-relative source spans and artifact references, severity, expected/actual identity values, and non-claims. Raw prompts, model output, credentials, environment values, absolute private paths, PTY bytes, and unbounded stdout/stderr are not accepted or projected.

Any repository mutation changes the final-subject hash and invalidates earlier results. Registry, selection, or policy changes likewise require a refreshed contract and fresh result rows.

## Non-claims

A passing completion receipt proves exact selected-check coverage and identity linkage for the supplied contract and final subject. It does not prove check-oracle soundness, honest command execution beyond supplied evidence, model correctness, whole-product correctness, or release eligibility outside the selected contract.
