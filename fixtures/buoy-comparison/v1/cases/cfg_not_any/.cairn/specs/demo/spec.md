# Buoy comparison fixture

## Purpose

Defines the declared structural-traceability comparison fixture.

## Requirements

### Requirement: Declared marker binding

r[cairn.buoy_comparison.demo] Cairn MUST bind one declared source marker for the comparison fixture.

#### Scenario: Marker is current

- GIVEN a declared source marker
- WHEN Cairn exports the trace index
- THEN the marker MUST resolve to the content-bound requirement row
