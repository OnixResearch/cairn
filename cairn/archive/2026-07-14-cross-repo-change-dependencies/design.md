## Context

Cairn already supports same-repo change dependency ordering through local `depends_on` metadata. That graph answers the question: "Which active change in this repository must drain before another?" Cross-repo dependencies answer a different question: "Which upstream Cairn receipt from another repository must be presented before this downstream change is considered unblocked?"

Those relationships should not rely on sibling checkout paths or network fetches. Cairn should validate presented evidence deterministically and leave trust, transport, and upstream correctness outside the claim boundary.

## Decisions

### 1. Use a separate metadata field

**Choice:** Add `cross_repo_dependencies` alongside local `depends_on` rather than overloading `depends_on`.

Example shape:

```json
{
  "cross_repo_dependencies": [
    {
      "id": "valence-canonical-evidence-ids",
      "repo": "valence",
      "change": "canonical-evidence-ids",
      "requires": "archive_receipt",
      "expected_status": "pass",
      "expected_receipt_hash": "blake3:...",
      "non_claims": [
        "does_not_prove_upstream_correctness",
        "does_not_prove_downstream_integration_correctness"
      ]
    }
  ]
}
```

**Rationale:** Local graph ordering and cross-repo evidence linkage have different satisfaction rules. Separate fields keep receipts clear and avoid accidentally treating remote work as an active local graph node.

### 2. Validate evidence summaries in pure core

**Choice:** `cairn-core` validates normalized dependency rows against normalized upstream receipt summaries. The core does not read paths, inspect sibling repositories, fetch remotes, run commands, or trust clocks.

**Rationale:** Cross-repo validation is still deterministic matching logic: repo identity, change identity, receipt kind, stage/status, policy hash when constrained, and receipt hash when constrained. That belongs in pure functional core and can be tested with fixtures.

### 3. Let the shell resolve evidence explicitly

**Choice:** `cairn-cli` accepts explicit local evidence inputs such as receipt files or evidence bundle directories. It MAY summarize imported Cairn evidence bundles, but it MUST NOT auto-fetch sibling repositories or Git remotes by default.

**Rationale:** Auto-discovery would make validation environment-dependent and could hide trust decisions. Explicit evidence inputs make the operator's claim auditable.

### 4. Treat hashes as binding when declared

**Choice:** A dependency row may constrain an exact `expected_receipt_hash`; when present, Cairn requires an exact match. Cairn-owned dependency receipts use BLAKE3 identities. Imported upstream receipts retain their own declared receipt hash while Cairn's validation receipt is BLAKE3-bound.

**Rationale:** Exact hashes provide immutable evidence linkage without requiring Cairn to reinterpret every upstream receipt field. BLAKE3 remains the stack-owned identity default.

### 5. Fail closed with typed statuses

**Choice:** Validation rows use deterministic statuses such as `satisfied`, `missing`, `failed`, `stale_policy`, `wrong_repo`, `wrong_change`, `wrong_kind`, `unsupported_kind`, and `hash_mismatch`.

**Rationale:** Agents need precise blockers and stable issue codes. A single generic failure would make repair plans ambiguous.

### 6. Make non-claims visible

**Choice:** Every validation report and docs page states that cross-repo dependency satisfaction proves only presented receipt linkage. It does not prove upstream source correctness, social trust in the upstream repo, witness independence, downstream integration correctness, or release eligibility by itself.

**Rationale:** Cairn's evidence model is intentionally bounded. Cross-repo dependencies should improve coordination without overstating assurance.

### 7. Accept only explicit current pass evidence

**Choice:** A dependency is satisfied only by a supported receipt kind with an explicit passing status and every declared repo, change, stage/kind, policy, subject, and receipt-hash constraint matched. Blocked, skipped, deferred, timed-out, dry-run, cancelled, infrastructure-failed, malformed, unknown, and stale evidence remain non-passing. Freshness is based on explicit content and policy identities, never an ambient clock or sibling checkout inspection.

**Rationale:** Evidence availability and transport success do not imply that the upstream requirement passed for the expected subject.

### 8. Make blockers actionable and bounded

**Choice:** Blocked rows carry stable dependency and diagnostic IDs, exact status, expected/actual safe identity fields and hashes, bounded remediation, explicit evidence-input or reproduction guidance, safe artifact refs, severity, and non-claims. Diagnostics exclude credentials, raw receipt payloads, absolute host paths, environment values, and unbounded logs.

**Rationale:** Downstream agents need enough information to request or reproduce the correct upstream artifact without Cairn auto-fetching it or leaking transport details.

## Risks / Trade-offs

- Requiring explicit evidence inputs adds operator steps, but avoids environment-dependent validation.
- Exact receipt hashes can become stale after legitimate upstream policy or receipt-schema changes; diagnostics must distinguish hash mismatch from wrong repo/change.
- The metadata contract needs careful policy/schema versioning so future receipt kinds can be added without breaking old changes.
- This does not replace local same-repo `depends_on` DAG ordering; operators may need both fields in one change.
