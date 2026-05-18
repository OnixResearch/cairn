# Cairn Supersession Release-Readiness Bundle Delta

## Purpose

Specify a deterministic local release-readiness receipt bundle that lets operators and agents cite one Cairn artifact for the current checkout's key readiness evidence.

## Requirements

### Requirement: Release-readiness receipt bundle
r[cairn.supersede.release_readiness_bundle] The system MUST provide a local release-readiness receipt bundle that summarizes required Cairn operator evidence for a checkout without mutating lifecycle content.

#### Scenario: Bundle receipt has stable schema
r[cairn.supersede.release_readiness_bundle.schema]
- GIVEN an operator requests release-readiness evidence for a Cairn checkout
- WHEN the release-readiness bundle command emits its JSON receipt
- THEN the receipt MUST include stable top-level fields for command, root, validity, verdict, checks, cited evidence hashes, and receipt hash
- AND each check entry MUST identify its stable check name, pass/fail status, source command or artifact, and any cited receipt hash

#### Scenario: Bundle runs locally without mutation
r[cairn.supersede.release_readiness_bundle.local]
- GIVEN a checkout with native `cairn/` lifecycle content
- WHEN the operator runs the release-readiness bundle command without mutation flags
- THEN the command MUST NOT create, edit, sync, archive, migrate, or delete lifecycle artifacts
- AND it MUST complete using local repository evidence rather than requiring a network service or long-running MCP client

#### Scenario: Bundle cites core readiness evidence
r[cairn.supersede.release_readiness_bundle.evidence]
- GIVEN the checkout has maintained Cairn validation, Tracey coverage, MCP agent smoke, and repository verification evidence
- WHEN the bundle evaluates readiness
- THEN it MUST include or cite Cairn validation evidence
- AND it MUST include or cite Tracey coverage evidence
- AND it MUST include or cite the MCP agent smoke receipt hash
- AND it MUST include or cite repository gate evidence such as formatting, tests, Clippy, Slotcar, or flake checks according to the maintained local policy

#### Scenario: Bundle fails transparently on missing or failing evidence
r[cairn.supersede.release_readiness_bundle.failure]
- GIVEN a required readiness component fails or required evidence is missing
- WHEN the release-readiness bundle command emits its receipt
- THEN the receipt MUST report `valid=false` or a failing verdict
- AND it MUST include a deterministic check or issue naming the failed or missing evidence component
- AND it MUST preserve any available component receipt hashes so operators can diagnose the failure without scraping prose output

#### Scenario: Bundle workflow is documented
r[cairn.supersede.release_readiness_bundle.docs]
- GIVEN an operator wants to prepare a Cairn checkout for release or agent handoff
- WHEN they read the maintained documentation
- THEN it MUST name the release-readiness bundle command
- AND it MUST describe which stable receipt fields and cited hashes are intended as release evidence
- AND it MUST explain that the bundle complements, rather than replaces, the maintained repository gate commands

#### Scenario: Bundle is verified before archive
r[cairn.supersede.release_readiness_bundle.verification]
- GIVEN the release-readiness bundle implementation is complete
- WHEN maintainers run final verification
- THEN focused bundle tests MUST assert stable receipt fields and hash-shaped evidence
- AND negative tests MUST cover missing or failing required evidence where practical
- AND Cairn validation, Tracey coverage, repository formatting, tests, Clippy, Slotcar, and flake checks MUST pass before the change is synced and archived
