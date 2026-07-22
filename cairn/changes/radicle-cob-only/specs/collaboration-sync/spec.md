# Collaboration Sync Specification

## ADDED Requirements

### Requirement: Collaboration sync retirement
r[cairn.collaboration_sync.retired] Cairn MUST reject collaboration sync commands because COB reference transfer is outside the local COB-only provider boundary.

#### Scenario: Sync is unavailable
r[cairn.collaboration_sync.retired.rejected]
- GIVEN an operator requests collaboration sync through any backend or compatibility namespace
- WHEN Cairn parses the command
- THEN the command MUST fail before locks, Git writes, sockets, node control, or network activity
- AND the diagnostic MUST direct the operator to manage external Git transport separately.

## REMOVED Requirements

### Requirement: Side-effect-free explicit preview
r[cairn.collaboration_sync.preview] This obsolete sync-preview requirement MUST be removed with the runtime transport surface.

### Requirement: Deterministic provider-specific planning
r[cairn.collaboration_sync.planning] This obsolete network-plan requirement MUST be removed with the runtime transport surface.

### Requirement: Serialized bounded Radicle execution
r[cairn.collaboration_sync.execution] This obsolete node-execution requirement MUST be removed with the runtime transport surface.

### Requirement: Exact typed peer evidence
r[cairn.collaboration_sync.integration] This obsolete peer-evidence requirement MUST be removed with the runtime transport surface.

### Requirement: Fail-closed negative paths
r[cairn.collaboration_sync.blocked] This obsolete peer-failure requirement MUST be removed with the runtime transport surface.

### Requirement: Deterministic bounded receipts
r[cairn.collaboration_sync.receipts] This obsolete sync-receipt requirement MUST be removed with the runtime transport surface.

### Requirement: Explicit claim boundary
r[cairn.collaboration_sync.non_claims] This obsolete sync-claim requirement MUST be removed with the runtime transport surface.

### Requirement: Reproducible validation
r[cairn.collaboration_sync.validation] This obsolete live-sync validation requirement MUST be removed with the runtime transport surface.
