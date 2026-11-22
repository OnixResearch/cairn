# Consumer-selected release gates

## ADDED Requirements

### Requirement: Consumer-selected release gate list

r[release_evidence.consumer_gate_selection] `release-readiness` MUST report a consuming repository's `release_readiness_policy.required_external_commands` as its repository gate list when the admitted policy declares a well-formed list, MUST report the maintained-local gates when the policy omits the list, and MUST refuse a malformed list without substituting the maintained-local gates.

A well-formed list is a non-empty array of unique, non-blank strings inside a `release_readiness_policy` object. Listing a command MUST NOT be reported as executing it.

#### Scenario: A well-formed list is reported as selected

r[release_evidence.consumer_gate_selection.selected]
- GIVEN an admitted policy whose `release_readiness_policy.required_external_commands` is a non-empty array of unique, non-blank strings
- WHEN `release-readiness` runs
- THEN `repository_gate_summary.policy` MUST be `consumer-selected-gates`
- AND `repository_gate_summary.required_commands` MUST equal the declared list in order.

#### Scenario: An absent list keeps the maintained gates

r[release_evidence.consumer_gate_selection.absent]
- GIVEN an admitted policy without `release_readiness_policy`, or with a `release_readiness_policy` object that has no `required_external_commands` key
- WHEN `release-readiness` runs
- THEN `repository_gate_summary.policy` MUST be `maintained-local-gates`
- AND the `repository_gate_policy_listed` check MUST pass.

#### Scenario: A malformed list is refused

r[release_evidence.consumer_gate_selection.malformed]
- GIVEN an admitted policy whose `release_readiness_policy` is not an object, or whose `required_external_commands` is not an array, is empty, or contains a non-string, blank or duplicate entry
- WHEN `release-readiness` runs
- THEN `repository_gate_summary.policy` MUST be `invalid-consumer-selected-gates` with a null `required_commands` and an `issue` that names the field and entry indices but never an entry's text
- AND the `repository_gate_policy_listed` check MUST fail with that issue
- AND the receipt verdict MUST be `fail`
- AND the maintained-local gates MUST NOT be reported in place of the declared list.
