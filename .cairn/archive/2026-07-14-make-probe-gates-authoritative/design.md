## Context

`evaluate_gate_snapshot` derives `GateReport.verdict` from the issues known to the pure core. `run_gate` then appends the inline-execution denial and `gate_probe_issues`, while `gate_report_json` independently derives `valid` from the enlarged issue list. This creates two outcome authorities.

The pre-mutation helper `mutation_gate_blockers` calls `evaluate_gate_snapshot` but never calls `gate_probe_issues`. In a reproduced fixture, `gate tasks` reported a missing saved receipt while `sync --execute` returned `blocked=false`, `mutated=true`, and changed the accepted spec. `gate_input_json` currently hashes proposal, design, tasks, and delta artifacts but not probe declarations or saved receipts.

## Decisions

### 1. Finalize a gate exactly once

**Choice:** Add a pure gate finalization core that accepts the lifecycle snapshot plus normalized external-evidence observations, combines and deterministically orders all findings, and derives `valid` and `verdict` from the final issue set.

**Rationale:** A receipt must have one outcome authority. It must be impossible to construct `valid=false` with `verdict="PASS"` through post-evaluation mutation.

### 2. Normalize probe evidence in the shell

**Choice:** The CLI shell reads probe declarations and saved receipts, enforces bounded reads and repository-relative paths, computes BLAKE3 identities, and converts read/parse/validation results into typed in-memory observations. The pure core decides how those observations affect the gate.

**Rationale:** Filesystem access stays outside `cairn-core`, while decision logic remains testable without a repository or process.

### 3. Reuse one authoritative gate composition everywhere

**Choice:** `gate tasks`, sync/archive pre-mutation checks, and mutation-capable MCP adapters call the same shell orchestration and pure finalizer. No mutation path may use a reduced gate that omits configured saved evidence.

**Rationale:** Inspection and enforcement must not disagree about readiness.

### 4. Bind evidence without copying payloads into receipts

**Choice:** Gate hash material includes stable probe IDs, declaration hashes, stated and computed saved-receipt hashes, policy linkage, validation status, and deterministic issue codes. It excludes stdout/stderr content, raw environment values, credentials, absolute temporary paths, and other unbounded payloads.

**Rationale:** A consumer can distinguish decisions made over different probe evidence without turning the gate receipt into a data-exfiltration surface.

### 5. Keep inline execution disabled

**Choice:** This change does not execute probes from a gate. `--run-probes` remains a deterministic denial under the default policy, and that denial participates in the final outcome like every other issue.

**Rationale:** The defect is inconsistent evaluation, not a reason to weaken the saved-evidence boundary.

### 6. Fail closed with a receipt

**Choice:** Missing, unreadable, malformed, oversized, stale-policy, hash-mismatched, or semantically invalid saved probe evidence becomes deterministic gate issues whenever a bounded failure receipt can be produced.

**Rationale:** Gate automation needs machine-readable evidence rather than an unstructured early abort.

### 7. Make probe status pass-only

**Choice:** Normalize saved probe outcomes to an explicit vocabulary and accept only a current `passed` result as satisfying required probe evidence. Failed, blocked, skipped, deferred, timed-out, cancelled, dry-run, cleanup-failed, infrastructure-failed, and unknown outcomes remain non-passing and preserve their exact class in the gate receipt.

**Rationale:** An unavailable probe or a successful dry-run plan is useful feedback, but it does not establish the probed property.

### 8. Return actionable bounded diagnostics

**Choice:** Probe blockers include stable diagnostic and probe IDs, result status, applicable declaration/policy/subject/evidence/receipt hashes, bounded repository-relative source span when supplied, remediation label and summary, exact reproduction argv, safe artifact refs, severity, and explicit non-claims. Raw stdout/stderr, prompts, credentials, environment values, absolute private paths, and unbounded logs remain excluded.

**Rationale:** Saved evidence should provide enough backpressure for a host or agent to reproduce and repair the failure without turning the gate receipt into an unsafe transcript.

## Risks / Trade-offs

- Gate input and receipt hashes will change because they finally bind probe evidence; schema and determinism fixtures must be updated together.
- Reusing the full gate on mutation paths can expose previously hidden blockers. That is intentional and requires explicit fixture coverage.
- The gate proves only that declared saved evidence passed Cairn's bounded checks; it does not prove the probe command, assertion oracle, producer honesty, or underlying behavior is correct.
