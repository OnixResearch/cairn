# Readiness-driven workflow guidance delta

## ADDED Requirements

### Requirement: Generated workflows use current readiness
r[cairn.agent_workflow_readiness.guidance] The system MUST derive generated apply, verify, drain, and archive diagnostic routes from readiness while retaining original checks and mutation preflight.

#### Scenario: Guidance requests one relevant artifact
- GIVEN a readiness blocker identifies a supported artifact instruction route
- WHEN Cairn generates workflow guidance
- THEN it directs the agent to that artifact and current evidence prerequisites rather than an unconditional repeated gate sequence

#### Scenario: Unknown remediation does not become a shell command
- GIVEN readiness reports an unresolved diagnostic
- WHEN the workflow renders the next step
- THEN it retains the unresolved state and requests explicit inspection without fabricated repair or execution arguments
