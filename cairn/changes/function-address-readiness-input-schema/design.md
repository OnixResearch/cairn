## Design

Cairn keeps function-address readiness as a pure evaluator over typed input records. The CLI shell reads JSON artifacts, normalizes supported versions, and passes typed evidence into `evaluate_function_address_evidence`. Unknown versions, missing required fields, malformed BLAKE3, wrong roles, unsupported claim scopes, and boundary weakening fail closed.

### Decisions

1. **Contracts are explicit.** Cairn documents the exact fields it consumes from Valence receipts, Mantle binding receipts, Kamacite receipts, and graph summaries.
2. **Direct Mantle receipt preferred.** Once Mantle emits a versioned binding receipt, Cairn reads it without wrapper glue.
3. **Optional stays non-blocking.** Optional policy mode still passes with skipped/present disposition according to available evidence.
4. **Required mode is strict.** Required policy mode fails on missing, malformed, stale, or overclaiming evidence.
5. **Boundary remains lifecycle-only.** Cairn validates readiness policy conformance, not Rust semantics, compiler correctness, whole-program safety, or release eligibility.

### Validation shape

Positive fixtures cover absent optional evidence, valid direct Mantle/Valence/Kamacite/graph inputs, and required mode with complete evidence. Negative fixtures cover missing required evidence, malformed digest, wrong role/schema, stale receipt hash, unsupported claim scope, and forbidden non-claim weakening.
