## Why

Operators can run `status`, `validate`, and `gate`, but setup/layout failures still require knowing which directories and policy files the CLI selected. A deterministic `doctor` receipt makes repository health, layout choice, policy availability, and validation issues visible without guessing.

## What Changes

- Add `cairn doctor [--root PATH] [--policy PATH]` as a read-only diagnostics command.
- Keep filesystem and policy-file reads in the CLI shell while pure core classifies the diagnostic snapshot.
- Report deterministic JSON checks for root/layout directories, policy load, repository counts, and validation issue totals.

## Scope

In scope: read-only diagnostics, JSON receipt shape, fixture-backed healthy/unhealthy tests, README and workflow-skill updates.

Out of scope: executing external tools such as Tracey/Nix, mutating repositories, or replacing `validate`/`gate` receipts.

## Verification

Run focused doctor CLI/core tests, `cairn validate --root .`, OpenSpec strict validation, Tracey bootstrap coverage, Slotcar, and the full Nix flake gate.
