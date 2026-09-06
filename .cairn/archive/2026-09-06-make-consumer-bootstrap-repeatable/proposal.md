# Make consumer bootstrap repeatable

## Why

A new consumer needs a complete policy and guidance path, not only empty lifecycle directories.
The installed build created `.cairn/`, but validation in the new directory failed without an explicit policy path.
The same validation passed with an explicit checked policy. Installed agent guidance also used the obsolete visible lifecycle path.

Cairn needs a repeatable setup and upgrade path that preserves consumer-owned policy and files.

## What Changes

- Add preview-first consumer setup and policy-upgrade plans with exact source, tool, profile, and destination identities.
- Resolve the policy from an explicit option or checked project declaration, with the source visible in each receipt.
- Compose Nickel Export, existing generated agent workflows, Cap Root, and Durable File Publication through narrow shell adapters.
- Extend doctor with policy compatibility, policy freshness, and declared generated-guidance findings.
- Preserve explicit legacy migration and the existing low-level `init` behavior.

The proposed public surface is `cairn setup` with separate plan, apply, and check operations. Implementation must publish the exact syntax in its contract.

## Impact

Maintenance owner: Cairn policy and operator-workflow maintainers.
Current consumers: first-time Cairn adopters and the maintained disposable consumer fixture.
Immediate outcome: a clean project reaches a valid native lifecycle without policy copying or ambient source-checkout assumptions.
Durable contribution: one content-bound installation and upgrade mechanism with ownership-aware recovery.

Candidate surfaces are `native/authoring.rs`, `support/layout.rs`, generated agent workflows, Nickel policy, fixtures, and operator docs.
Normal validation and gates remain free of runtime Nickel evaluation.
Compatibility tests must preserve explicit `--policy` behavior and existing receipt fields.

The source and immutable reuse revisions are in `docs/cairn-utility-program.md`.
No dependency is admitted merely because a sibling checkout exists.

## Repeatability evidence

Retain a clean-root setup-to-validation test and an explicit-policy control.
Add negative fixtures for policy drift, unsupported cohorts, modified guidance, symlinks, stale plans, interrupted publication, and failed recovery.
Repeat the accepted path from a fresh root with no user-global Cairn configuration.

## Out of Scope

- Silent upgrades or fallback after an explicit policy fails
- Global agent configuration edits or ambient guidance discovery
- Automatic legacy deletion, source commits, network acquisition, or publication
- A new schema-lineage engine or general package manager
- Claims that setup proves consumer correctness or release readiness
