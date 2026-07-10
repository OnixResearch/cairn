## Design

Cairn treats digest spelling normalization as input parsing, not proof. The CLI shell accepts only the spellings documented for each producer field and normalizes them to lowercase BLAKE3 hex before the pure readiness evaluator runs. The evaluator still checks field role, source, receipt, graph, and release binding identity separately.

### Decisions

1. **Producer-specific adapters.** Valence, Mantle, and Kamacite fields declare whether `b3:<hex>` or bare hex is accepted.
2. **Pure evaluator receives canonical digests.** `evaluate_function_address_evidence` sees already-normalized typed values.
3. **Malformed spelling is an issue.** Bad prefix, bad length, non-hex, uppercase when unsupported, and empty strings produce deterministic issues.
4. **No domain substitution.** A valid digest string cannot be reused in the wrong readiness field without a role/domain issue.

### Validation shape

Positive fixtures cover supported spellings at every producer boundary. Negative fixtures cover malformed hashes, wrong prefixes, cross-field substitution, and optional-present invalid evidence.
