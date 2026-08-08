# Radicle Change COB Specification

## MODIFIED Requirements

### Requirement: Native Radicle custom object adapter
r[cairn.radicle_change_cob.radicle_adapter] The system MUST expose the active-change reducer as the Radicle custom object type `xyz.onixresearch.cairn-change` through direct `radicle-cob` storage traits without moving repository access into the pure core or requiring a Radicle runtime.

#### Scenario: Radicle operations map into pure reducer inputs
r[cairn.radicle_change_cob.radicle_adapter.mapping]
- GIVEN a signed Radicle COB entry has a valid manifest, action encoding, actor, operation ID, and root-embedded authority
- WHEN the custom object adapter initializes or updates a change
- THEN the adapter MUST verify the signed actor against immutable creator/delegate authority and pass only deterministic in-memory inputs to the pure reducer.

#### Scenario: Invalid adapter inputs fail closed
r[cairn.radicle_change_cob.radicle_adapter.reject]
- GIVEN an entry has malformed actions, a mismatched manifest or namespace signer, unauthorized content mutation, or invalid reducer input
- WHEN Cairn evaluates the custom object history
- THEN the adapter MUST return an error so the invalid operation branch is not materialized as accepted state.

#### Scenario: Aggregate dependencies remain external
r[cairn.radicle_change_cob.radicle_adapter.dependencies]
- GIVEN one change object declares dependencies on other active changes
- WHEN its local reducer evaluates the object
- THEN the reducer MUST preserve the declarations without claiming global existence, acyclicity, readiness, or archival satisfaction, and the Cairn repository shell MUST retain aggregate graph validation authority.
