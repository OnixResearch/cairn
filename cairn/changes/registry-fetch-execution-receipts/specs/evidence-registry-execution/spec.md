## ADDED Requirements

### Requirement: Registry fetch execution receipt

r[evidence_registry_execution.fetch.behavior] The system MUST provide a deterministic registry fetch execution receipt that consumes a registry fetch plan, verifies planned artifacts against expected hashes, and records local materialization results without exposing host-specific secrets.

#### Scenario: Fetch plan materializes verified artifacts

r[evidence_registry_execution.fetch.behavior.fetch.plan.materializes.verified.artifacts]

- GIVEN a verified registry fetch plan whose referenced artifacts are available in a local mirror or source directory
- WHEN the operator runs the fetch execution command
- THEN the receipt reports every materialized artifact, expected hash, observed hash, safe relative destination, and `valid=true`

#### Scenario: Hash mismatch fails closed

r[evidence_registry_execution.fetch.behavior.hash.mismatch.fails.closed]

- GIVEN a fetch plan artifact whose observed bytes do not match the expected hash
- WHEN the operator runs the fetch execution command
- THEN the receipt reports `valid=false`, identifies the mismatch with a stable issue code, and does not claim successful materialization

### Requirement: Fetch execution preserves safety boundaries

r[evidence_registry_execution.fetch.safety] The system MUST reject unsafe paths, missing plan verification, missing artifacts, and ambient network assumptions with deterministic failure receipts.

#### Scenario: Unsafe path is rejected

r[evidence_registry_execution.fetch.safety.unsafe.path.is.rejected]

- GIVEN a fetch plan with an absolute path or parent-directory traversal destination
- WHEN the operator runs fetch execution
- THEN the command emits a deterministic failure receipt and does not write outside the allowed output root

#### Scenario: No hidden network dependency

r[evidence_registry_execution.fetch.safety.no.hidden.network.dependency]

- GIVEN a fetch execution test fixture uses local file inputs only
- WHEN the operator runs the test suite
- THEN the behavior is verified without requiring network access or remote credentials


### Requirement: Implementation Task Traceability

r[evidence_registry_execution.fetch.receipt.task_traceability] The system MUST keep implementation tasks traceable to scenario-level verification targets during this change.

#### Scenario: Materializes task coverage

r[evidence_registry_execution.fetch.receipt.materializes]

- GIVEN the change package tasks for `registry-fetch-execution-receipts`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.

#### Scenario: Unsafe Path task coverage

r[evidence_registry_execution.fetch.safety.unsafe_path]

- GIVEN the change package tasks for `registry-fetch-execution-receipts`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.

#### Scenario: Hash Mismatch task coverage

r[evidence_registry_execution.fetch.receipt.hash_mismatch]

- GIVEN the change package tasks for `registry-fetch-execution-receipts`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.

#### Scenario: No Network task coverage

r[evidence_registry_execution.fetch.safety.no_network]

- GIVEN the change package tasks for `registry-fetch-execution-receipts`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.
