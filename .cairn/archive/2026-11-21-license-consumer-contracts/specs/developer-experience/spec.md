## ADDED Requirements

### Requirement: Consumer contracts and assurance-targets policy carry a permissive grant

r[cairn.license_boundary.consumer_contracts_grant] Cairn MUST offer the consumer-facing typed Nickel contracts under `contracts/**` and `cairn-policy/assurance-targets.ncl` under `MIT OR Apache-2.0` at every revision of this repository, including the copies other repositories vendored from commits `3dc654d38404d595ff07ca994925789f45941ffe` and `f20bcb4aa00268fdaa630cbce07633250f18a681` and byte-for-byte copies of those files elsewhere, and MUST NOT extend that grant to the generated JSON exports, any other `cairn-policy/` file, the CLI, the exporter/toolchain, repository documentation or third-party material.

#### Scenario: A consumer vendors the covered contracts

- GIVEN a consumer has the consumer-facing contracts or the assurance-targets policy source, at the current revision or as a byte-for-byte copy of either named vendored revision
- WHEN the consumer inspects Cairn's license map and policy license notice
- THEN it MAY choose MIT or Apache-2.0 for those files and redistribute them with the applicable terms and notices
- AND the original Git objects do not need to be changed to receive that choice.

#### Scenario: The grant is missing

- GIVEN a repository state where the new grant row, a covered path or a named vendored revision is absent from the license map or policy notice
- WHEN the license-boundary check runs
- THEN it MUST fail with the missing or misplaced marker
- AND the check MUST NOT accept the state as the declared boundary.
