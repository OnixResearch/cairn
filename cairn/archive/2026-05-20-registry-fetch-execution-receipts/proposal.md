## Why

The registry rail can select indexes and emit metadata-only fetch plans, but there is not yet a first-class receipt for executing a fetch plan into a local mirror or artifact cache. A fetch execution receipt would close the consumer-side evidence chain after selection/fetch-plan without adding network trust assumptions.

## What Changes

- Add a public registry fetch execution command or subcommand that consumes a verified registry fetch plan and records deterministic local materialization evidence.
- Verify fetched or copied artifacts by expected hash before reporting success.
- Emit failure receipts for missing artifacts, unsafe paths, hash mismatches, and unverified plans.

## Scope

In scope: Turn metadata-only fetch plans into auditable local execution evidence., Keep receipts deterministic and secret-free., Preserve offline/local-first semantics; no hidden network fetches in tests..

Out of scope: Adding remote network download clients in the initial slice., Changing existing registry select/fetch-plan receipt shapes except for additive linking fields if required., Claiming artifact correctness beyond hash/path/policy verification..

## Verification

Run Cairn validation and proposal/design/tasks gates for this change package. During implementation, run focused tests for the moved or added command family, then the maintained repository gates before commit.
