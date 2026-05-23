# Semantic Review Readiness Gate

r[cairn.semantic_review.readiness_gate.release_readiness] The system MUST let release readiness consume semantic review ledger evidence when it is supplied or conventionally present.

## Scenarios

### Supplied valid semantic review ledger passes readiness check
r[cairn.semantic_review.readiness_gate.release_readiness.supplied_valid]
- GIVEN a semantic review ledger receipt with `valid=true` and a passing verdict
- WHEN an operator runs `cairn release-readiness --review-ledger <path>`
- THEN the release-readiness receipt includes a `semantic_review_ledger` check marked passed with the ledger receipt hash

### Supplied invalid semantic review ledger fails readiness
r[cairn.semantic_review.readiness_gate.release_readiness.supplied_invalid]
- GIVEN a semantic review ledger receipt with `valid=false` or a failing verdict
- WHEN an operator runs `cairn release-readiness --review-ledger <path>`
- THEN the release-readiness receipt is invalid and includes the failing semantic-review check

### Missing semantic review ledger is explicit
r[cairn.semantic_review.readiness_gate.release_readiness.skipped_absent]
- GIVEN no semantic review ledger argument and no conventional semantic-review ledger artifact
- WHEN an operator runs `cairn release-readiness`
- THEN the release-readiness receipt includes a skipped semantic-review check explaining that Cairn did not run reviewers

r[cairn.semantic_review.readiness_gate.boundary] The system MUST preserve the semantic-review boundary when readiness consumes review evidence.

## Scenarios

### Readiness consumes receipts rather than running reviewers
r[cairn.semantic_review.readiness_gate.boundary.no_reviewer_execution]
- GIVEN a semantic review ledger artifact
- WHEN release readiness evaluates semantic review evidence
- THEN Cairn reads deterministic receipt fields and does not invoke reviewers, model providers, network calls, or hidden semantic judgment
