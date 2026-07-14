# Lifecycle substance gates

Cairn rejects lifecycle packages that are present but structurally vacuous. The reviewed Nickel `gate_policy.substance` record owns minimum line, requirement, and task counts; placeholder and normative tokens; and dependency-marker semantics. Rust evaluates in-memory snapshots while CLI/MCP shells own filesystem I/O. r[verify cairn.lifecycle_validation.artifact_substance]

Generated templates use `CAIRN_PLACEHOLDER`. Replace every placeholder before expecting a gate to pass. Empty, whitespace-only, or placeholder-only proposals and designs emit `lifecycle_validation.artifact_substance.*` diagnostics.

Accepted and delta specs need at least one `### Requirement:` block with exactly one bare `r[...]` ID and a policy-listed normative token in that requirement's own body. Scenario prose, fenced examples, and comments cannot strengthen a requirement. Vacuity, bad identity cardinality, and weak bodies emit `lifecycle_validation.spec_substance.*`. r[verify cairn.lifecycle_validation.spec_substance]

A substantive task is a recognized checkbox row with a non-placeholder label and either a known delta `r[...]` reference or a valid dependency marker. Dependency markers must be closed, non-empty, and match the configured lifecycle-ID target class. Malformed markers emit `lifecycle_validation.dependency_marker_shape.*` and grant no linkage exemption. r[verify cairn.lifecycle_validation.task_substance]

Gate receipts include deterministic `substance` inventories and machine-readable issue `code` fields. Validation receipts expose equivalent `findings`, `spec_findings`, and `substance_findings` arrays while retaining compatibility issue strings. Task progress (`done`, `in_progress`, `todo`) is reported separately from `substantive_tasks`; checked boxes never override a substance blocker. The same policy-backed findings feed stage gates, repository validation, sync/archive preflight, and MCP receipts. Blocked mutation receipts report `mutated=false` and perform no writes. r[verify cairn.lifecycle_validation.task_progress_non_authority]

These checks prove bounded structural non-vacuity and linkage shape only. They do not prove that prose is true, code is correct, external dependencies exist, or a release is eligible.
