## Why

Cairn's agent-run completion gate currently treats a successful receipt as verified whenever `verification_manifest.checks` is merely a non-empty array. It does not require a check row to have passed, prove that every repository-required check was selected, bind the row to the final post-mutation subject, reject dry-run or skipped evidence, or compare evidence with a repository-owned verification contract. A self-authored row with a failing status or arbitrary hash can therefore satisfy `evaluate_agent_run_completion`.

That weak gate makes model prose, wrapper assertions, and stale evidence too easy to promote into completion. Agent autonomy should grow only after completion is receipt-authoritative: the repository selects checks, producers execute them, Cairn validates exact coverage and freshness, and only a current passing validation receipt can support a completion claim.

## What Changes

- Introduce a versioned authoritative agent-verification contract that binds repository-owned registry, selection, policy, and final subject identities. r[cairn.supersede.agent_run_receipts.verification_contract]
- Require exact coverage of every selected required check and make `passed` the only status eligible to satisfy completion. r[cairn.supersede.agent_run_receipts.pass_only]
- Reject stale, duplicate, unknown, malformed, prerequisite-incomplete, dry-run, skipped, deferred, blocked, timed-out, cancelled, cleanup-failed, or infrastructure-failed evidence. r[cairn.supersede.agent_run_receipts.freshness]
- Add bounded actionable diagnostics with stable IDs, check status, hashes, repository-relative source locations, remediation, exact reproduction argv, safe artifact references, and explicit non-claims. r[cairn.supersede.agent_run_receipts.actionable_diagnostics]
- Make a passing Cairn completion-validation receipt, rather than model text, task checkboxes, process exit success, or an unvalidated wrapper verdict, the completion authority. r[cairn.supersede.agent_run_receipts.completion_authority]
- Preserve a pure deterministic completion core and keep registry loading, filesystem access, process execution, output capture, and receipt persistence in explicit shells. r[cairn.supersede.agent_run_receipts.execution_boundary]
- Version the public schema and migrate CLI/MCP, fixtures, policy exports, docs, and hash goldens together; legacy v1 receipts remain inspectable but cannot establish authoritative completion. r[cairn.supersede.agent_run_receipts.schema_migration]
- Add positive and negative core, CLI, MCP, schema, determinism, and compatibility fixtures. r[cairn.supersede.agent_run_receipts.authoritative_verification]

## Impact

- **Files**: Nickel policy and generated schemas; `cairn-core` agent receipt DTOs, validation, hashing, and diagnostics; `cairn-cli`/MCP contract and receipt loading; agent receipt fixtures; replay and Preserves goldens; operator documentation.
- **Compatibility**: introduces an explicit authoritative receipt version/profile. Existing v1 receipts may still be parsed for bounded historical inspection, but successful v1 wrapper verdicts are not completion evidence.
- **Testing**: exact required-check coverage, status vocabulary, prerequisite closure, final-subject freshness, registry and selection drift, diagnostic safety, CLI/MCP parity, cross-root determinism, and legacy fail-closed behavior.
