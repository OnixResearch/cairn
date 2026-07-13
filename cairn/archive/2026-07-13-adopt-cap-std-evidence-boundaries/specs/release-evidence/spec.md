# Release Evidence Specification

## Purpose

Adds capability-bounded local filesystem roots for Cairn evidence, registry, release, replay, and scan shell boundaries.

## Requirements

### Requirement: Cairn evidence filesystem authority is capability-rooted
r[cairn.release_evidence.cap_std_boundary.root_wrappers] Cairn MUST represent repository, evidence bundle, registry, release, replay, and scan filesystem authority as typed capability roots opened by the CLI shell.

#### Scenario: Valid evidence path stays under the declared root
r[cairn.release_evidence.cap_std_boundary.tests.positive]
- GIVEN Cairn has opened an operator-declared repository or artifact root
- WHEN evidence, registry, release, replay, or scan code reads or writes a valid relative path through the typed root
- THEN the operation MUST remain confined to that declared root and preserve existing valid-input behavior.

#### Scenario: Invalid evidence path cannot escape the root
r[cairn.release_evidence.cap_std_boundary.tests.negative]
- GIVEN input names `../` traversal, an absolute path, a missing capability root, or a symlink escape attempt
- WHEN Cairn resolves the path through the typed root
- THEN Cairn MUST fail closed with deterministic diagnostics before reading, writing, scanning, or replaying outside the declared root.

### Requirement: Capability dependency stays outside Cairn core
r[cairn.release_evidence.cap_std_boundary.dependency] Cairn MUST add `cap-std` only to crates or modules that own CLI/support filesystem effects.

#### Scenario: Core remains pure
r[cairn.release_evidence.cap_std_boundary.conversion]
- GIVEN `cairn-core` validation, policy, lifecycle, and receipt logic is reviewed
- WHEN cap-std adoption is complete
- THEN `cairn-core` MUST receive loaded inputs or validated relative locators and MUST NOT depend on `cap-std` or open ambient filesystem paths.

### Requirement: Capability boundary is documented without evidence overclaims
r[cairn.release_evidence.cap_std_boundary.docs] Cairn MUST document that capability roots bound local filesystem authority only and do not prove artifact truth, replay completeness, lifecycle correctness, or release readiness.

#### Scenario: Closeout evidence includes boundary validation
r[cairn.release_evidence.cap_std_boundary.validation]
- GIVEN the cap-std evidence-boundary change is ready to archive
- WHEN focused evidence, registry, release, replay, scan, Cairn validation, Tracey coverage, and gate checks run
- THEN the evidence MUST include positive and negative filesystem-boundary coverage and visible non-claim documentation.
