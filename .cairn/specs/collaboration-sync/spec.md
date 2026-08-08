# Collaboration Sync Specification

## Purpose

Defines the `collaboration-sync` capability.

## Requirements

### Requirement: Collaboration sync retirement
r[cairn.collaboration_sync.retired] Cairn MUST reject collaboration sync commands because COB reference transfer is outside the local COB-only provider boundary.

#### Scenario: Sync is unavailable
r[cairn.collaboration_sync.retired.rejected]
- GIVEN an operator requests collaboration sync through any backend or compatibility namespace
- WHEN Cairn parses the command
- THEN the command MUST fail before locks, Git writes, sockets, node control, or network activity
- AND the diagnostic MUST direct the operator to manage external Git transport separately.
