# Collaboration Backend Agnostic Specification

## MODIFIED Requirements

### Requirement: Explicit backend-neutral CLI
r[cairn.collaboration_backend.cli] The system MUST expose `cairn collaboration import|status|checkout` with an explicit backend selector and backend-specific arguments while retaining `cairn cob` as the direct entry point for the COB-only provider.

#### Scenario: Portable backend needs no COB context
r[cairn.collaboration_backend.cli.portable]
- GIVEN the operator selects `portable-json.v1`
- WHEN Cairn parses a collaboration command
- THEN `--store` is accepted and no Git COB repository or keystore is required.

#### Scenario: Backend-specific arguments are strict
r[cairn.collaboration_backend.cli.arguments]
- GIVEN the operator supplies missing, duplicate, malformed, cross-backend, or Radicle runtime arguments
- WHEN Cairn parses a collaboration command
- THEN the command MUST reject those arguments deterministically before mutation.
