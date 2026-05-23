# semantic-review Delta

## MODIFIED Requirements

### Requirement: Review metrics regression rail [r[cairn.semantic_review.review_metrics_regression_rail]]
The system MUST provide a deterministic rail that keeps metrics-derived review regression categories synchronized across registry entries, fixtures, operator guidance, task/spec references, and sanitized evidence.

#### Scenario: Registry binds all category surfaces [r[cairn.semantic_review.review_metrics_regression_rail.registry_binds_surfaces]]
- GIVEN a metrics-derived review regression category is declared
- WHEN Cairn validates the category registry
- THEN the registry MUST include a stable category code, metrics class/stage/source, diagnostic code, requirement or scenario id, operator-guidance anchor, positive fixture reference, negative fixture reference, and sanitized evidence reference
- AND missing or duplicate bindings MUST produce deterministic fail-closed issues

#### Scenario: Fixture pair proves category enforcement [r[cairn.semantic_review.review_metrics_regression_rail.fixture_pair_enforcement]]
- GIVEN a declared metrics-derived review regression category
- WHEN its fixture coverage is checked
- THEN at least one negative fixture MUST fail with the category diagnostic
- AND at least one positive fixture MUST pass while demonstrating the expected task, spec, evidence, command, helper, or oracle shape

#### Scenario: Guidance and task references stay synchronized [r[cairn.semantic_review.review_metrics_regression_rail.guidance_task_sync]]
- GIVEN a registry entry names a diagnostic code and requirement id
- WHEN Cairn checks guidance and task/spec references
- THEN operator guidance MUST document the diagnostic and remediation shape
- AND the related task/spec artifacts MUST reference the declared requirement or scenario id

#### Scenario: Sanitized evidence boundary is enforced [r[cairn.semantic_review.review_metrics_regression_rail.secret_free_evidence]]
- GIVEN repeated review findings are summarized as metrics evidence
- WHEN Cairn validates the evidence reference for a registry entry
- THEN the evidence MUST include safe category metadata and provenance sufficient for review
- AND it MUST NOT include credentials, tokens, provider payload bodies, private transcript dumps, raw hidden prompts, account identifiers, or connection strings

#### Scenario: Downstream projects retain category ownership [r[cairn.semantic_review.review_metrics_regression_rail.downstream_category_ownership]]
- GIVEN a downstream project has project-specific review diagnostics and fixtures
- WHEN it uses the Cairn review metrics regression rail
- THEN Cairn MUST validate the generic registry/coverage invariant without requiring those project-specific categories to be added to Cairn's own accepted specs
