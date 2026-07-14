# Cross-repository change dependencies

Cross-repository dependencies are explicit receipt links. They are separate from local `depends_on` edges: local edges order active changes in one repository, while cross-repository rows require operator-supplied upstream evidence and never create implicit remote graph nodes.

## Metadata

Add `cross_repo_dependencies` to an active change's `metadata.json`:

```json
{
  "depends_on": ["local-prerequisite"],
  "cross_repo_dependencies": [
    {
      "id": "valence-canonical-evidence",
      "repo": "valence",
      "change": "canonical-evidence",
      "requires": "archive_receipt",
      "expected_status": "pass",
      "expected_policy_hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      "expected_subject_hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
      "expected_receipt_hash": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
      "non_claims": [
        "dependency satisfaction proves presented receipt linkage only",
        "dependency satisfaction does not prove upstream correctness or remote trust",
        "dependency satisfaction does not prove downstream integration correctness or release eligibility"
      ]
    }
  ]
}
```

Supported receipt kinds are `proposal_gate`, `design_gate`, `tasks_gate`, `validation_receipt`, `sync_receipt`, and `archive_receipt`. Every declared hash is binding. `pass` is the sole configured expected status.

## Explicit evidence

Cairn accepts a bounded summary envelope from an explicit file or bundle directory:

```json
{
  "schema": "cairn.cross-repo-receipt-summary.v1",
  "repo": "valence",
  "change": "canonical-evidence",
  "receipt_kind": "archive_receipt",
  "status": "pass",
  "policy_hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "subject_hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
  "receipt_hash": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
  "diagnostic_summary": "current passing upstream receipt"
}
```

When an optional `receipt` object is embedded, `receipt_hash` must equal Cairn's canonical BLAKE3 hash of that object. Evidence files are bounded, symlinks are rejected, bundle traversal is local-only, and diagnostics may not contain raw credentials, prompts, environment values, absolute private paths, or unbounded logs.

```console
cairn validate --root . --cross-repo-evidence evidence/valence-archive.json --strict
cairn sync downstream-change --root . --cross-repo-evidence evidence/ --execute
cairn archive downstream-change --root . --cross-repo-evidence evidence/ --execute
cairn release-readiness --root . --cross-repo-evidence evidence/
```

MCP `cairn_validate`, sync, and archive tools accept the same paths in a `cross_repo_evidence` string array. The shell never inspects sibling checkouts, fetches remotes, runs upstream commands, or treats transport success as receipt success.

## Pass-only feedback

Only explicit current passing evidence matching repository, change, kind, policy, subject, and receipt identities can satisfy a row. Failed, blocked, skipped, deferred, timed-out, dry-run, cancelled, infrastructure-failed, malformed, unsupported, unknown, stale, missing, and mismatched evidence remains blocked.

Blocked rows include stable diagnostic and dependency IDs, expected/actual safe identities, status, remediation, explicit evidence guidance, safe artifact references, severity, and non-claims. Mutation preflight binds the dependency receipt hash and reopens evidence before the first write.

## Claim boundary

Satisfaction proves presented receipt linkage only. It does not prove upstream correctness, social or cryptographic trust in a remote repository, downstream integration correctness, or release eligibility by itself.
