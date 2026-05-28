# Ironspec Supersession Specification Delta

## ADDED Requirements

### Requirement: Packaged policy export carries Nickel runtime
r[cairn.supersede.policy_export_packaged_runtime] The system MUST make packaged `cairn policy export --check` evaluate Nickel policy without requiring an ambient dev-shell `nickel` on PATH.

#### Scenario: Packaged policy export works from nix run
r[cairn.supersede.policy_export_packaged_runtime.wrapper]
- GIVEN a Cairn package built by the repository flake
- WHEN an operator runs the packaged `cairn policy export --check` command outside the development shell
- THEN the command SHALL find Nickel through the package runtime environment, render the policy, and report freshness for the checked-in generated policy.

#### Scenario: Ambient Nickel is not required
r[cairn.supersede.policy_export_packaged_runtime.check]
- GIVEN the packaged Cairn binary is executed in a stripped environment without caller-provided Nickel on PATH
- WHEN the packaged policy export freshness check runs
- THEN the flake check SHALL pass only if the package supplies the Nickel runtime dependency itself.

#### Scenario: Operator docs use packaged invocation
r[cairn.supersede.policy_export_packaged_runtime.docs]
- GIVEN maintainers follow the documented local gate commands
- WHEN they run the policy export freshness check from the repository root
- THEN the docs SHALL direct them to the packaged CLI invocation that has the same runtime dependency behavior as release and CI checks.

#### Scenario: Package runtime fix is verified
r[cairn.supersede.policy_export_packaged_runtime.validation]
- GIVEN the package runtime fix is implemented
- WHEN focused package checks, Cairn validation, Tracey coverage, and flake checks run
- THEN the package runtime, lifecycle specs, traceability, and existing repository gates SHALL pass.
