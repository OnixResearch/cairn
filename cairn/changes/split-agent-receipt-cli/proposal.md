## Why

Agent receipt validation and bundle verification are important evidence surfaces, but the command-family implementation still lives in `main.rs`. A module split will clarify ownership and reduce coupling before further release-evidence work.

## What Changes

- Move `agent-receipt` command parsing and handlers into `crates/cairn-cli/src/agent_receipt.rs`.
- Preserve validation, bundle verification, MCP smoke, deterministic issue-code, and JSON output contracts.
- Keep bridge exports narrow if release-readiness or evidence commands consume receipt helpers.

## Scope

In scope: Make agent receipt behavior easier to review., Avoid changing receipt schemas while moving code., Verify with focused agent receipt tests and full gates..

Out of scope: Changing agent receipt fixture hashes unless existing deterministic inputs intentionally change., Adding new receipt families or release-readiness requirements in this slice..

## Verification

Run Cairn validation and proposal/design/tasks gates for this change package. During implementation, run focused tests for the moved or added command family, then the maintained repository gates before commit.
