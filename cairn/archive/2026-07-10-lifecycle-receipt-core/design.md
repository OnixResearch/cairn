## Design

The lifecycle receipt core lives in `cairn-core` as pure deterministic DTOs and validators. It takes parsed receipt values, policy metadata, and hash material as inputs and returns normalized receipts or diagnostics. `cairn-cli` continues to load files, compute repository snapshots, write JSON, and exit with command status.

### Decisions

1. **Envelope fields are shared; decisions are not.** Verdict, issue, evidence summary, input hash, policy hash, receipt hash, and command identity shapes are reusable. Gate pass/fail semantics remain command- and policy-specific.

2. **Receipt hash rules are explicit profiles.** Each receipt family declares which fields are excluded from its own hash material and which hash fields must be present.

3. **Diagnostics are typed but bounded.** Shared diagnostic rows carry code, message, scope, path/field when available, and severity/class metadata where the policy supports it. They do not encode downstream business decisions.

4. **Adapters preserve existing receipts.** Cairn receipts and downstream receipt-shaped evidence keep public JSON compatibility through family adapters.

5. **No release promotion.** Passing lifecycle receipt core validation proves envelope syntax, hash consistency, diagnostics shape, and non-claim presence only.

### Validation shape

Positive tests cover valid gate, validate, sync/archive-plan, runtime evidence, and release-evidence-style receipts. Negative tests cover unsupported verdict, missing receipt hash, included self-hash field, stale hash, missing issue fields, weakened non-claims, and text that claims release eligibility or lifecycle readiness from envelope validation alone.
